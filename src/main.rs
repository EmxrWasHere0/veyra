mod lexer;
mod runtime;
mod modules;

use std::io::{self, Write};
use std::process;
use std::rc::Rc;

use lexer::lex;
use runtime::{
    binop, Flow, Function, Parser, Runtime, Value, LOOP_LIMIT,
};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const KEYWORDS: [&str; 7] = ["true", "false", "null", "and", "or", "not", "func"];

#[derive(Clone, Debug)]
pub struct Line {
    pub number: usize,
    pub text: String,
}

/* ============================= HELPERS ============================ */

fn is_ident(s: &str) -> bool {
    !s.is_empty()
        && !KEYWORDS.contains(&s)
        && s.chars().next().is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
        && s.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn kw<'a>(line: &'a str, word: &str) -> Option<&'a str> {
    let rest = line.strip_prefix(word)?;
    if rest.is_empty() || rest.starts_with(char::is_whitespace) || rest.starts_with('(') {
        Some(rest.trim())
    } else {
        None
    }
}

fn is_block_start(t: &str) -> bool {
    kw(t, "if").is_some() || kw(t, "repeat").is_some() || kw(t, "add").is_some()
}

fn strip_comment(line: &str) -> &str {
    let mut quote: Option<char> = None;
    let mut escaped = false;
    for (i, c) in line.char_indices() {
        if escaped { escaped = false; }
        else if quote.is_some() && c == '\\' { escaped = true; }
        else if c == '"' || c == '\'' {
            quote = if quote == Some(c) { None } else { quote.or(Some(c)) };
        } else if c == '#' && quote.is_none() { return &line[..i]; }
    }
    line
}

fn split_assignment(src: &str) -> Option<(String, String, String)> {
    let ch: Vec<char> = src.chars().collect();
    let mut quote: Option<char> = None;
    let mut escaped = false;
    let mut dp = 0i32;
    let mut db = 0i32;

    for i in 0..ch.len() {
        let c = ch[i];
        if escaped { escaped = false; continue; }
        if quote.is_some() && c == '\\' { escaped = true; continue; }
        if c == '"' || c == '\'' {
            quote = if quote == Some(c) { None } else { quote.or(Some(c)) };
            continue;
        }
        if quote.is_some() { continue; }
        match c { '(' => dp += 1, ')' => dp -= 1, '[' => db += 1, ']' => db -= 1, _ => {} }
        if dp != 0 || db != 0 || c != '=' { continue; }

        let prev = if i > 0 { ch[i - 1] } else { '\0' };
        let next = *ch.get(i + 1).unwrap_or(&'\0');
        if next == '=' || next == '>' || matches!(prev, '=' | '!' | '<' | '>') { continue; }

        let (lhs, op) = if "+-*/%".contains(prev) {
            (ch[..i - 1].iter().collect::<String>(), format!("{}=", prev))
        } else {
            (ch[..i].iter().collect::<String>(), "=".to_string())
        };
        let lhs = lhs.trim().to_string();
        if lhs.is_empty() { return None; }
        let rhs = ch[i + 1..].iter().collect::<String>().trim().to_string();
        return Some((lhs, op, rhs));
    }
    None
}

fn split_index_target(lhs: &str) -> Option<(String, String)> {
    let lhs = lhs.trim();
    let open = lhs.strip_suffix(']')?.rfind('[')?;
    let name = lhs[..open].trim();
    let index = lhs[open + 1..lhs.len() - 1].trim();
    if is_ident(name) && !index.is_empty() { Some((name.to_string(), index.to_string())) }
    else { None }
}

fn matching_end(lines: &[Line], head: usize) -> Result<usize, String> {
    let mut depth = 0usize;
    for i in head + 1..lines.len() {
        let t = lines[i].text.as_str();
        if is_block_start(t) { depth += 1; }
        else if t == "end" {
            if depth == 0 { return Ok(i); }
            depth -= 1;
        }
    }
    Err(format!("{}: Missing 'end' for block", lines[head].number))
}

fn parse_add_header(rest: &str) -> Result<(String, String), String> {
    let open = rest.find('(').ok_or("Expected '(' in function definition")?;
    let name = rest[..open].trim().to_string();
    if !is_ident(&name) { return Err(format!("Invalid function name: {}", name)); }
    if !rest.trim_end().ends_with(')') { return Err("Expected ')' in function definition".into()); }
    Ok((name, rest[open..].trim_end().to_string()))
}

/// Parses a `mod` command:
///   mod system              -> (name, None)
///   mod system *            -> (name, Some(Import::Star))
///   mod system [arch, os]   -> (name, Some(Import::Selective(vec)))
enum ImportKind {
    Star,
    Selective(Vec<String>),
}

fn parse_mod_command(rest: &str) -> Result<(String, Option<ImportKind>), String> {
    let rest = rest.trim();

    // Check for selective import: name [a, b, c]
    if let Some(bracket_start) = rest.find('[') {
        let name = rest[..bracket_start].trim();
        if !is_ident(name) { return Err(format!("Invalid module name: {}", name)); }
        let tail = rest[bracket_start..].trim();
        if !tail.ends_with(']') { return Err("Expected ']' in selective import".into()); }
        let inner = tail[1..tail.len() - 1].trim();
        let names: Vec<String> = inner.split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        if names.is_empty() { return Err("Empty selective import list".into()); }
        for n in &names {
            if !is_ident(n) { return Err(format!("Invalid import name: {}", n)); }
        }
        return Ok((name.to_string(), Some(ImportKind::Selective(names))));
    }

    // Check for star import: name *
    if rest.ends_with(" *") || rest.ends_with("\t*") {
        let name = rest[..rest.len() - 1].trim();
        if !is_ident(name) { return Err(format!("Invalid module name: {}", name)); }
        return Ok((name.to_string(), Some(ImportKind::Star)));
    }

    // Simple import: name
    if rest.ends_with('*') && rest.len() > 1 {
        let name = rest[..rest.len() - 1].trim();
        if !is_ident(name) { return Err(format!("Invalid module name: {}", name)); }
        return Ok((name.to_string(), Some(ImportKind::Star)));
    }

    if !is_ident(rest) { return Err(format!("Invalid module name: {}", rest)); }
    Ok((rest.to_string(), None))
}

/* ============================ EXECUTION =========================== */

pub fn run(
    lines: &[Line],
    from: usize,
    to: usize,
    rt: &mut Runtime,
    in_loop: bool,
) -> Result<Flow, String> {
    let mut i = from;

    while i < to {
        let s = lines[i].text.as_str();
        let no = lines[i].number;
        let err = move |e: String| format!("{}: {}", no, e);

        if s.is_empty() { i += 1; continue; }
        if s == "end" { return Err(err("Unexpected 'end'".into())); }
        if kw(s, "else").is_some() { return Err(err("Unexpected 'else'".into())); }

        if s == "stop" || s == "fwd" {
            if !in_loop {
                return Err(err(format!("'{}' can only be used inside a loop", s)));
            }
            return Ok(if s == "stop" { Flow::Stop } else { Flow::Forward });
        }

        if let Some(rest) = kw(s, "return") {
            rt.return_value = Some(if rest.is_empty() { Value::Null }
                else { rt.eval(rest).map_err(err)? });
            return Ok(Flow::Return);
        }

        if let Some(rest) = kw(s, "say") {
            if rest.is_empty() { println!(); }
            else { println!("{}", rt.eval(rest).map_err(err)?); }
            i += 1; continue;
        }

        if let Some(rest) = kw(s, "get") {
            if !rest.is_empty() {
                print!("{}", rt.eval(rest).map_err(err)?);
                io::stdout().flush().map_err(|e| err(e.to_string()))?;
            }
            let mut input = String::new();
            io::stdin().read_line(&mut input).map_err(|e| err(e.to_string()))?;
            rt.assign("_input", Value::Text(input.trim_end().to_string())).map_err(err)?;
            i += 1; continue;
        }

        // Module import
        if let Some(rest) = kw(s, "mod") {
            let (name, kind) = parse_mod_command(rest).map_err(err)?;
            rt.load_module(&name).map_err(err)?;
            match kind {
                Some(ImportKind::Star) => rt.import_star(&name).map_err(err)?,
                Some(ImportKind::Selective(names)) => rt.import_selective(&name, &names).map_err(err)?,
                None => {} // just load, use as module.member
            }
            i += 1; continue;
        }

        if let Some(rest) = kw(s, "set") {
            let (name, op, expr) = split_assignment(rest)
                .ok_or_else(|| err("Usage: set name = expression".into()))?;
            if op != "=" { return Err(err("Only '=' is allowed in constant definitions".into())); }
            if !is_ident(&name) { return Err(err(format!("Invalid constant name: {}", name))); }
            if rt.get(&name).is_some() || rt.is_const(&name) {
                return Err(err(format!("'{}' is already defined", name)));
            }
            let value = rt.eval(&expr).map_err(err)?;
            rt.declare_const(&name, value);
            i += 1; continue;
        }

        if let Some(rest) = kw(s, "add") {
            let (name, params_src) = parse_add_header(rest).map_err(err)?;
            let end = matching_end(lines, i)?;
            let toks = lex(&params_src).map_err(err)?;
            let params = Parser::new(toks, rt).parse_params().map_err(err)?;
            let body: Vec<Line> = lines[i + 1..end].to_vec();
            rt.functions.insert(name.clone(), Rc::new(Function { name, params, body }));
            i = end + 1; continue;
        }

        if let Some(cond) = kw(s, "if") {
            let end = matching_end(lines, i)?;
            let mut branches: Vec<(Option<&str>, usize, usize)> = Vec::new();
            let mut current = Some(cond);
            let mut start = i + 1;
            let mut depth = 0usize;

            for j in i + 1..end {
                let t = lines[j].text.as_str();
                if is_block_start(t) { depth += 1; }
                else if t == "end" { depth = depth.saturating_sub(1); }
                else if depth == 0 {
                    if let Some(tail) = kw(t, "else") {
                        branches.push((current, start, j));
                        current = match kw(tail, "if") {
                            Some(c) => Some(c),
                            None if tail.is_empty() => None,
                            None => return Err(format!("{}: Expected 'else' or 'else if'", lines[j].number)),
                        };
                        start = j + 1;
                    }
                }
            }
            branches.push((current, start, end));

            for (c, a, b) in branches {
                let hit = match c { Some(c) => rt.eval(c).map_err(err)?.truthy(), None => true };
                if hit {
                    let flow = run(lines, a, b, rt, in_loop)?;
                    if flow != Flow::Normal { return Ok(flow); }
                    break;
                }
            }
            i = end + 1; continue;
        }

        if let Some(rest) = kw(s, "repeat") {
            let end = matching_end(lines, i)?;
            let body_start = i + 1;

            let run_body = |rt: &mut Runtime| -> Result<Flow, String> {
                rt.push_scope();
                let r = run(lines, body_start, end, rt, true);
                rt.pop_scope();
                r
            };

            if let Some(cond) = kw(rest, "while") {
                let mut turns = 0usize;
                while rt.eval(cond).map_err(err)?.truthy() {
                    if turns >= LOOP_LIMIT { return Err(err("Loop safety limit reached".into())); }
                    turns += 1;
                    match run_body(rt)? {
                        Flow::Stop => break,
                        Flow::Return => return Ok(Flow::Return),
                        _ => {}
                    }
                }
            } else {
                let n = rt.eval(rest).map_err(err)?.number().map_err(err)?;
                if n < 0.0 || n.fract() != 0.0 || n > LOOP_LIMIT as f64 {
                    return Err(err("repeat requires an integer between 0 and 1,000,000".into()));
                }
                for _ in 0..n as usize {
                    match run_body(rt)? {
                        Flow::Stop => break,
                        Flow::Return => return Ok(Flow::Return),
                        _ => {}
                    }
                }
            }
            i = end + 1; continue;
        }

        if let Some((target, op, rhs)) = split_assignment(s) {
            let value = rt.eval(&rhs).map_err(err)?;
            if let Some((name, idx_expr)) = split_index_target(&target) {
                if op != "=" { return Err(err("Only '=' is supported on list elements".into())); }
                let idx = rt.eval(&idx_expr).map_err(err)?.index().map_err(err)?;
                rt.set_index(&name, idx, value).map_err(err)?;
            } else {
                if !is_ident(&target) { return Err(err(format!("Invalid assignment target: {}", target))); }
                let value = if op == "=" { value } else {
                    let old = rt.get(&target)
                        .ok_or_else(|| err(format!("Undefined variable: {}", target)))?;
                    binop(&op[..1], old, value).map_err(err)?
                };
                rt.assign(&target, value).map_err(err)?;
            }
            i += 1; continue;
        }

        rt.last_value = Some(rt.eval(s).map_err(err)?);
        i += 1;
    }

    Ok(Flow::Normal)
}

/* ============================== MAIN ============================== */

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 { print_help(); return; }

    match args[1].as_str() {
        "-h" | "--help" => return print_help(),
        "-v" | "--version" => return println!("Veyra v{}", VERSION),
        filename => run_file(filename),
    }
}

fn run_file(filename: &str) {
    let source = std::fs::read_to_string(filename).unwrap_or_else(|e| {
        eprintln!("Could not read file: {}", e);
        process::exit(1);
    });

    let lines: Vec<Line> = source.lines().enumerate()
        .map(|(i, l)| Line { number: i + 1, text: strip_comment(l).trim().to_string() })
        .collect();

    let mut rt = Runtime::new();
    if let Err(e) = run(&lines, 0, lines.len(), &mut rt, false) {
        eprintln!("Veyra error: {}", e);
        process::exit(1);
    }
}

fn print_help() {
    println!("Veyra Interpreter v{}", VERSION);
    println!("Usage: veyra <file.vey> [options]");
    println!();
    println!("Options:");
    println!("  -h, --help     Show this help message");
    println!("  -v, --version  Show the interpreter version");
}