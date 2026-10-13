use crate::runtime::Value;

#[derive(Clone, Debug, PartialEq)]
pub enum Tok {
    Val(Value),
    Id(String),
    Op(String),
    LParen,
    RParen,
    LBracket,
    RBracket,
    Comma,
    Question,
    Spread,
    Arrow,
    FuncKw,
    Dot,
    Eof,
}

const TWO_CHAR_OPS: [&str; 9] = ["==", "!=", "<=", ">=", "+=", "-=", "*=", "/=", "%="];

pub fn lex(input: &str) -> Result<Vec<Tok>, String> {
    let chars: Vec<char> = input.chars().collect();
    let mut out = Vec::new();
    let mut i = 0usize;

    while i < chars.len() {
        let c = chars[i];

        if c.is_whitespace() {
            i += 1;
            continue;
        }

        if c == '"' || c == '\'' {
            out.push(read_string(&chars, &mut i, c)?);
            continue;
        }

        // Numbers (digit or dot-digit)
        if c.is_ascii_digit() {
            out.push(read_number(&chars, &mut i)?);
            continue;
        }

        if c.is_ascii_alphabetic() || c == '_' {
            out.push(read_ident(&chars, &mut i));
            continue;
        }

        // Spread ...
        if c == '.'
            && matches!(chars.get(i + 1), Some('.'))
            && matches!(chars.get(i + 2), Some('.'))
        {
            out.push(Tok::Spread);
            i += 3;
            continue;
        }

        // Number starting with dot: .5 etc
        if c == '.' && matches!(chars.get(i + 1), Some(d) if d.is_ascii_digit()) {
            out.push(read_number(&chars, &mut i)?);
            continue;
        }

        // Dot (member access)
        if c == '.' {
            out.push(Tok::Dot);
            i += 1;
            continue;
        }

        // Arrow =>
        if c == '=' && matches!(chars.get(i + 1), Some('>')) {
            out.push(Tok::Arrow);
            i += 2;
            continue;
        }

        // Two-char operators
        let pair: String = chars[i..(i + 2).min(chars.len())].iter().collect();
        if TWO_CHAR_OPS.contains(&pair.as_str()) {
            out.push(Tok::Op(pair));
            i += 2;
            continue;
        }

        // Single-char tokens
        let tok = match c {
            '(' => Tok::LParen,
            ')' => Tok::RParen,
            '[' => Tok::LBracket,
            ']' => Tok::RBracket,
            ',' => Tok::Comma,
            '?' => Tok::Question,
            '+' | '-' | '*' | '/' | '%' | '<' | '>' | '=' | '!' => Tok::Op(c.to_string()),
            _ => return Err(format!("Unknown character: {}", c)),
        };
        out.push(tok);
        i += 1;
    }

    out.push(Tok::Eof);
    Ok(out)
}

fn read_string(chars: &[char], i: &mut usize, quote: char) -> Result<Tok, String> {
    *i += 1;
    let mut value = String::new();

    while *i < chars.len() {
        let ch = chars[*i];
        *i += 1;

        if ch == quote {
            return Ok(Tok::Val(Value::Text(value)));
        }

        if ch == '\\' && *i < chars.len() {
            let esc = chars[*i];
            *i += 1;
            value.push(match esc {
                'n' => '\n',
                't' => '\t',
                'r' => '\r',
                other => other,
            });
        } else {
            value.push(ch);
        }
    }

    Err("Unterminated string literal".into())
}

fn read_number(chars: &[char], i: &mut usize) -> Result<Tok, String> {
    let start = *i;
    let mut dot = false;

    while *i < chars.len() {
        match chars[*i] {
            d if d.is_ascii_digit() => *i += 1,
            '.' if !dot => {
                dot = true;
                *i += 1;
            }
            _ => break,
        }
    }

    let text: String = chars[start..*i].iter().collect();
    text.parse::<f64>()
        .map(|n| Tok::Val(Value::Num(n)))
        .map_err(|_| format!("Invalid number: {}", text))
}

fn read_ident(chars: &[char], i: &mut usize) -> Tok {
    let start = *i;
    while *i < chars.len() && (chars[*i].is_ascii_alphanumeric() || chars[*i] == '_') {
        *i += 1;
    }
    let name: String = chars[start..*i].iter().collect();
    match name.as_str() {
        "true" => Tok::Val(Value::Bool(true)),
        "false" => Tok::Val(Value::Bool(false)),
        "null" => Tok::Val(Value::Null),
        "and" | "or" | "not" => Tok::Op(name),
        "func" => Tok::FuncKw,
        _ => Tok::Id(name),
    }
}

pub fn toks_to_source(toks: &[Tok]) -> String {
    let mut out = String::new();
    for t in toks {
        if !out.is_empty() {
            out.push(' ');
        }
        match t {
            Tok::Val(Value::Text(s)) => out.push_str(&format!("{:?}", s)),
            Tok::Val(v) => out.push_str(&v.to_string()),
            Tok::Id(n) => out.push_str(n),
            Tok::Op(o) => out.push_str(o),
            Tok::LParen => out.push('('),
            Tok::RParen => out.push(')'),
            Tok::LBracket => out.push('['),
            Tok::RBracket => out.push(']'),
            Tok::Comma => out.push(','),
            Tok::Question => out.push('?'),
            Tok::Spread => out.push_str("..."),
            Tok::Arrow => out.push_str("=>"),
            Tok::FuncKw => out.push_str("func"),
            Tok::Dot => out.push('.'),
            Tok::Eof => {}
        }
    }
    out
}