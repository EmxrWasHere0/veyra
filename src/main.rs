mod lexer;
mod runtime;
mod modules;

use std::io::{self, Write};
use std::process;
use std::rc::Rc;

use lexer::lex;
use runtime::{binop, Flow, Function, Parser, Runtime, Value, LOOP_LIMIT};

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

/// If the line starts with `@label`, returns (label, rest_of_line_without_label).
fn parse_label(line: &str) -> Option<(String, &str)> {
    let line = line.trim_start();
    let rest = line.strip_prefix('@')?;
    let end = rest.find(char::is_whitespace).unwrap_or(rest.len());
    let name = &rest[..end];
    if !is_ident(name) { return None; }
    Some((name.to_string(), rest[end..].trim_start()))
}

/// Checks if a line (after optional label) starts a block.
fn is_block_start(t: &str) -> bool {
    let effective = parse_label(t).map(|(_, r)| r).unwrap_or(t);
    kw(effective, "if").is_some()
        || kw(effective, "repeat").is_some()
        || kw(effective, "add").is_some()
        || kw(effective, "test").is_some()
        || kw(effective, "try").is_some()
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

pub fn split_top_level_commas(src: &str) -> Vec<String> {
    let ch: Vec<char> = src.chars().collect();
    let mut parts = Vec::new();
    let mut buf = String::new();
    let mut quote: Option<char> = None;
    let mut escaped = false;
    let mut dp = 0i32;
    let mut db = 0i32;

    for c in ch {
        if escaped { escaped = false; buf.push(c); continue; }
        if quote.is_some() && c == '\\' { escaped = true; buf.push(c); continue; }
        if c == '"' || c == '\'' {
            quote = if quote == Some(c) { None } else { quote.or(Some(c)) };
            buf.push(c);
            continue;
        }
        if quote.is_some() { buf.push(c); continue; }
        match c {
            '(' => { dp += 1; buf.push(c); }
            ')' => { dp -= 1; buf.push(c); }
            '[' => { db += 1; buf.push(c); }
            ']' => { db -= 1; buf.push(c); }
            ',' if dp == 0 && db == 0 => {
                parts.push(buf.trim().to_string());
                buf.clear();
            }
            _ => buf.push(c),
        }
    }
    if !buf.trim().is_empty() {
        parts.push(buf.trim().to_string());
    }
    parts
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
        } else if prev == '?' {
            (ch[..i - 1].iter().collect::<String>(), "?=".to_string())
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
    if lhs.starts_with('[') { return None; }
    let open = lhs.strip_suffix(']')?.rfind('[')?;
    let name = lhs[..open].trim();
    let index = lhs[open + 1..lhs.len() - 1].trim();
    if is_ident(name) && !index.is_empty() { Some((name.to_string(), index.to_string())) }
    else { None }
}

#[derive(Debug)]
pub enum Pattern {
    Name(String),
    Rest(String),
}

fn parse_destructure(src: &str) -> Option<Vec<Pattern>> {
    let s = src.trim();
    if !s.starts_with('[') || !s.ends_with(']') { return None; }
    let inner = &s[1..s.len() - 1];
    let parts = split_top_level_commas(inner);

    let mut out = Vec::with_capacity(parts.len());
    let mut rest_seen = false;
    for part in parts {
        let p = part.trim();
        if let Some(name) = p.strip_prefix("...") {
            let name = name.trim();
            if rest_seen || !is_ident(name) { return None; }
            rest_seen = true;
            out.push(Pattern::Rest(name.to_string()));
        } else {
            if !is_ident(p) { return None; }
            out.push(Pattern::Name(p.to_string()));
        }
    }
    if out.is_empty() { None } else { Some(out) }
}

fn do_destructure(patterns: &[Pattern], value: Value, rt: &mut Runtime) -> Result<(), String> {
    let items = match value {
        Value::List(v) => v,
        other => return Err(format!("Destructuring requires a list, found: {}", other)),
    };
    let rest_pos = patterns.iter().position(|p| matches!(p, Pattern::Rest(_)));

    match rest_pos {
        None => {
            if items.len() < patterns.len() {
                return Err(format!(
                    "Destructuring expected {} values, got {}",
                    patterns.len(), items.len()
                ));
            }
            for (p, v) in patterns.iter().zip(items.into_iter()) {
                if let Pattern::Name(name) = p { rt.assign(name, v)?; }
            }
        }
        Some(rp) => {
            let before = rp;
            let after = patterns.len() - rp - 1;
            if items.len() < before + after {
                return Err(format!(
                    "Destructuring expected at least {} values, got {}",
                    before + after, items.len()
                ));
            }
            let tail_start = items.len() - after;
            let mut items = items;
            let tail: Vec<Value> = items.split_off(tail_start);
            let middle: Vec<Value> = items.split_off(before);
            let head: Vec<Value> = items;

            for (p, v) in patterns[..before].iter().zip(head.into_iter()) {
                if let Pattern::Name(name) = p { rt.assign(name, v)?; }
            }
            if let Pattern::Rest(name) = &patterns[rp] {
                rt.assign(name, Value::List(middle))?;
            }
            for (p, v) in patterns[rp + 1..].iter().zip(tail.into_iter()) {
                if let Pattern::Name(name) = p { rt.assign(name, v)?; }
            }
        }
    }
    Ok(())
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

enum ImportKind { Star, Selective(Vec<String>) }

fn parse_mod_command(rest: &str) -> Result<(String, Option<ImportKind>), String> {
    let rest = rest.trim();
    if let Some(bracket_start) = rest.find('[') {
        let name = rest[..bracket_start].trim();
        if !is_ident(name) { return Err(format!("Invalid module name: {}", name)); }
        let tail = rest[bracket_start..].trim();
        if !tail.ends_with(']') { return Err("Expected ']' in selective import".into()); }
        let inner = tail[1..tail.len() - 1].trim();
        let names: Vec<String> = inner.split(',').map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty()).collect();
        if names.is_empty() { return Err("Empty selective import list".into()); }
        for n in &names {
            if !is_ident(n) { return Err(format!("Invalid import name: {}", n)); }
        }
        return Ok((name.to_string(), Some(ImportKind::Selective(names))));
    }
    if rest.ends_with('*') && rest.len() > 1 {
        let name = rest[..rest.len() - 1].trim();
        if !is_ident(name) { return Err(format!("Invalid module name: {}", name)); }
        return Ok((name.to_string(), Some(ImportKind::Star)));
    }
    if !is_ident(rest) { return Err(format!("Invalid module name: {}", rest)); }
    Ok((rest.to_string(), None))
}

/// Parses `stop` / `stop @label` / `fwd` / `fwd @label`.
/// Returns (kind, Option<label>) where kind is "stop" or "fwd".
fn parse_flow_command(s: &str) -> Option<(&'static str, Option<String>)> {
    let s = s.trim();
    if s == "stop" { return Some(("stop", None)); }
    if s == "fwd" { return Some(("fwd", None)); }
    if let Some(rest) = s.strip_prefix("stop ") {
        let rest = rest.trim();
        if let Some(label) = rest.strip_prefix('@') {
            if is_ident(label) { return Some(("stop", Some(label.to_string()))); }
        }
    }
    if let Some(rest) = s.strip_prefix("fwd ") {
        let rest = rest.trim();
        if let Some(label) = rest.strip_prefix('@') {
            if is_ident(label) { return Some(("fwd", Some(label.to_string()))); }
        }
    }
    None
}

/* ============================ EXECUTION =========================== */

/// `active_loop`: the label (if any) of the innermost enclosing loop.
///                None means we're inside an unlabeled loop or no loop.
///                We also track whether we're in ANY loop separately via `in_loop_count`.
pub fn run(
    lines: &[Line],
    from: usize,
    to: usize,
    rt: &mut Runtime,
    in_loop: Option<&LoopContext>,
) -> Result<Flow, String> {
    let mut i = from;

    while i < to {
        let raw = lines[i].text.as_str();
        let no = lines[i].number;
        let err = move |e: String| format!("{}: {}", no, e);

        // Strip optional label from the beginning
        let (line_label, s): (Option<String>, &str) = match parse_label(raw) {
            Some((lbl, rest)) => (Some(lbl), rest),
            None => (None, raw),
        };

        if s.is_empty() { i += 1; continue; }
        if s == "end" { return Err(err("Unexpected 'end'".into())); }
        if kw(s, "else").is_some() { return Err(err("Unexpected 'else'".into())); }
        if kw(s, "catch").is_some() { return Err(err("Unexpected 'catch'".into())); }

        // Flow commands: stop / fwd / stop @label / fwd @label
        if let Some((kind, label)) = parse_flow_command(s) {
            if in_loop.is_none() {
                return Err(err(format!("'{}' can only be used inside a loop", kind)));
            }
            let flow = if kind == "stop" { Flow::Stop(label) } else { Flow::Forward(label) };
            return Ok(flow);
        }

        if let Some(rest) = kw(s, "return") {
            if rest.is_empty() {
                rt.return_value = Some(Value::Null);
            } else {
                let parts = split_top_level_commas(rest);
                if parts.len() == 1 {
                    rt.return_value = Some(rt.eval(&parts[0]).map_err(err)?);
                } else {
                    let mut values = Vec::with_capacity(parts.len());
                    for part in parts {
                        values.push(rt.eval(&part).map_err(err)?);
                    }
                    rt.return_value = Some(Value::List(values));
                }
            }
            return Ok(Flow::Return);
        }

        if let Some(rest) = kw(s, "assert") {
            let value = rt.eval(rest).map_err(err)?;
            if !value.truthy() {
                return Err(err(format!("Assertion failed: {}", rest)));
            }
            i += 1; continue;
        }

        if let Some(rest) = kw(s, "test") {
            let end = matching_end(lines, i)?;
            if !rt.test_mode { i = end + 1; continue; }

            let test_name = if rest.is_empty() {
                format!("test at line {}", no)
            } else {
                rest.trim_matches('"').trim_matches('\'').to_string()
            };

            rt.push_scope();
            let test_result = run(lines, i + 1, end, rt, None);
            rt.pop_scope();

            match test_result {
                Ok(_) => {
                    rt.test_stats.passed += 1;
                    println!("  ✓ {}", test_name);
                }
                Err(e) => {
                    rt.test_stats.failed += 1;
                    rt.test_stats.failures.push(format!("{}: {}", test_name, e));
                    println!("  ✗ {}: {}", test_name, e);
                }
            }

            i = end + 1;
            continue;
        }

        // try / catch block
        if let Some(_) = kw(s, "try") {
            let end = matching_end(lines, i)?;
            // Find top-level `catch` within this try block
            let mut depth = 0usize;
            let mut catch_line: Option<usize> = None;
            for j in i + 1..end {
                let t = lines[j].text.as_str();
                let (_, tt) = match parse_label(t) {
                    Some((lbl, rest)) => (Some(lbl), rest),
                    None => (None, t),
                };
                if is_block_start(tt) { depth += 1; }
                else if tt == "end" { depth = depth.saturating_sub(1); }
                else if depth == 0 && kw(tt, "catch").is_some() {
                    catch_line = Some(j);
                    break;
                }
            }

            let (try_end, catch_info) = match catch_line {
                Some(cl) => {
                    let tail = &lines[cl].text;
                    let catch_rest = kw(tail, "catch").unwrap_or("");
                    let err_var = if catch_rest.is_empty() { None }
                        else {
                            let name = catch_rest.trim();
                            if !is_ident(name) {
                                return Err(err(format!("Invalid catch variable: {}", name)));
                            }
                            Some(name.to_string())
                        };
                    (cl, Some((cl + 1, err_var)))
                }
                None => (end, None),
            };

            rt.push_scope();
            let try_result = run(lines, i + 1, try_end, rt, in_loop);
            rt.pop_scope();

            match try_result {
                Ok(Flow::Normal) => {
                    // try succeeded, skip catch
                    i = end + 1;
                    continue;
                }
                Ok(flow) => {
                    // return/stop/fwd propagates up through try
                    return Ok(flow);
                }
                Err(e) => {
                    if let Some((catch_start, err_var)) = catch_info {
                        rt.push_scope();
                        if let Some(var) = err_var {
                            rt.assign(&var, Value::Text(e)).map_err(err)?;
                        }
                        let catch_result = run(lines, catch_start, end, rt, in_loop);
                        rt.pop_scope();
                        match catch_result {
                            Ok(Flow::Normal) => { i = end + 1; continue; }
                            Ok(flow) => return Ok(flow),
                            Err(e2) => return Err(e2),
                        }
                    } else {
                        return Err(e);
                    }
                }
            }
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

        if let Some(rest) = kw(s, "mod") {
            let (name, kind) = parse_mod_command(rest).map_err(err)?;
            rt.load_module(&name).map_err(err)?;
            match kind {
                Some(ImportKind::Star) => rt.import_star(&name).map_err(err)?,
                Some(ImportKind::Selective(names)) => rt.import_selective(&name, &names).map_err(err)?,
                None => {}
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
            let my_label = line_label.clone();

            let ctx = LoopContext { label: my_label.clone() };

            let run_body = |rt: &mut Runtime| -> Result<Flow, String> {
                rt.push_scope();
                let r = run(lines, body_start, end, rt, Some(&ctx));
                rt.pop_scope();
                r
            };

            // Helper to decide what to do with a Stop/Forward signal
            // Returns Some(Flow) to re-propagate, or None to consume locally.
            let handle_signal = |flow: Flow| -> (bool, Option<Flow>) {
                // (should_break_loop, propagate_flow)
                match flow {
                    Flow::Normal => (false, None),
                    Flow::Return => (true, Some(Flow::Return)),
                    Flow::Stop(None) => (true, None), // break innermost
                    Flow::Forward(None) => (false, None), // continue innermost
                    Flow::Stop(Some(ref lbl)) => {
                        if my_label.as_deref() == Some(lbl.as_str()) {
                            (true, None) // this loop is the target
                        } else {
                            (true, Some(flow.clone())) // propagate upward
                        }
                    }
                    Flow::Forward(Some(ref lbl)) => {
                        if my_label.as_deref() == Some(lbl.as_str()) {
                            (false, None) // this loop is the target; continue
                        } else {
                            (true, Some(flow.clone())) // propagate
                        }
                    }
                }
            };

            if let Some(cond) = kw(rest, "while") {
                let mut turns = 0usize;
                while rt.eval(cond).map_err(err)?.truthy() {
                    if turns >= LOOP_LIMIT { return Err(err("Loop safety limit reached".into())); }
                    turns += 1;
                    let flow = run_body(rt)?;
                    let (should_break, propagate) = handle_signal(flow);
                    if let Some(f) = propagate { return Ok(f); }
                    if should_break { break; }
                }
            } else {
                let n = rt.eval(rest).map_err(err)?.number().map_err(err)?;
                if n < 0.0 || n.fract() != 0.0 || n > LOOP_LIMIT as f64 {
                    return Err(err("repeat requires an integer between 0 and 1,000,000".into()));
                }
                for _ in 0..n as usize {
                    let flow = run_body(rt)?;
                    let (should_break, propagate) = handle_signal(flow);
                    if let Some(f) = propagate { return Ok(f); }
                    if should_break { break; }
                }
            }
            i = end + 1; continue;
        }

        if let Some((target, op, rhs)) = split_assignment(s) {
            let value = rt.eval(&rhs).map_err(err)?;

            if let Some(patterns) = parse_destructure(&target) {
                if op != "=" {
                    return Err(err("Only '=' is supported with destructuring".into()));
                }
                do_destructure(&patterns, value, rt).map_err(err)?;
                i += 1; continue;
            }

            if let Some((name, idx_expr)) = split_index_target(&target) {
                if op != "=" {
                    return Err(err("Only '=' is supported on list elements".into()));
                }
                let idx = rt.eval(&idx_expr).map_err(err)?.index().map_err(err)?;
                rt.set_index(&name, idx, value).map_err(err)?;
                i += 1; continue;
            }

            if !is_ident(&target) {
                return Err(err(format!("Invalid assignment target: {}", target)));
            }

            let final_value = if op == "=" {
                value
            } else if op == "?=" {
                if rt.is_filled(&target) {
                    i += 1; continue;
                }
                value
            } else {
                let old = rt.get(&target)
                    .ok_or_else(|| err(format!("Undefined variable: {}", target)))?;
                binop(&op[..1], old, value).map_err(err)?
            };
            rt.assign(&target, final_value).map_err(err)?;

            i += 1; continue;
        }

        rt.last_value = Some(rt.eval(s).map_err(err)?);
        i += 1;
    }

    Ok(Flow::Normal)
}

/// Context passed to `run()` indicating the innermost loop (if any) and its label.
pub struct LoopContext {
    pub label: Option<String>,
}

/* ============================== MAIN ============================== */

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 { print_help(); return; }

    let mut test_mode = false;
    let mut filename: Option<&str> = None;

    for arg in args.iter().skip(1) {
        match arg.as_str() {
            "-h" | "--help" => { print_help(); return; }
            "-v" | "--version" => { println!("Veyra v{}", VERSION); return; }
            "--test" | "-t" => test_mode = true,
            other => filename = Some(other),
        }
    }

    let Some(filename) = filename else { print_help(); return; };
    run_file(filename, test_mode);
}

fn run_file(filename: &str, test_mode: bool) {
    let source = std::fs::read_to_string(filename).unwrap_or_else(|e| {
        eprintln!("Could not read file: {}", e);
        process::exit(1);
    });

    let lines: Vec<Line> = source.lines().enumerate()
        .map(|(i, l)| Line { number: i + 1, text: strip_comment(l).trim().to_string() })
        .collect();

    let mut rt = Runtime::new();
    rt.test_mode = test_mode;

    if test_mode {
        println!("Running tests in {}...\n", filename);
    }

    if let Err(e) = run(&lines, 0, lines.len(), &mut rt, None) {
        eprintln!("Veyra error: {}", e);
        process::exit(1);
    }

    if test_mode {
        let stats = &rt.test_stats;
        let total = stats.passed + stats.failed;
        println!();
        println!("Results: {} passed, {} failed, {} total", stats.passed, stats.failed, total);
        if stats.failed > 0 { process::exit(1); }
    }
}

fn print_help() {
    println!("Veyra Interpreter v{}", VERSION);
    println!("Usage: veyra <file.vey> [options]");
    println!();
    println!("Options:");
    println!("  -h, --help     Show this help message");
    println!("  -v, --version  Show the interpreter version");
    println!("  -t, --test     Run test blocks instead of normal execution");
}