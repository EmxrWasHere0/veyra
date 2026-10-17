use std::rc::Rc;
use crate::runtime::Line;

#[derive(Clone, Debug, PartialEq)]
pub enum Tok {
    Val,
    Id,
    Op,
    LParen,
    RParen,
    LBracket,
    RBracket,
    Comma,
    Question,
    QuestionEq,
    Spread,
    Arrow,
    FuncKw,
    Dot,
    At,
    Eof,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub kind: Tok,
    pub text: String,
}

pub fn lex(source: &str) -> Vec<Token> {
    let chars: Vec<char> = source.chars().collect();
    let len = chars.len();
    let mut i = 0;
    let mut tokens = Vec::new();

    while i < len {
        let c = chars[i];

        // Whitespace
        if c.is_whitespace() {
            i += 1;
            continue;
        }

        // Comment
        if c == '#' {
            while i < len && chars[i] != '\n' {
                i += 1;
            }
            continue;
        }

        // String
        if c == '"' {
            let start = i;
            i += 1;
            while i < len && chars[i] != '"' {
                if chars[i] == '\\' { i += 1; }
                i += 1;
            }
            if i < len { i += 1; } // closing "
            let text: String = chars[start..i].iter().collect();
            tokens.push(Token { kind: Tok::Val, text });
            continue;
        }

        // Number
        if c.is_ascii_digit() || (c == '.' && i + 1 < len && chars[i + 1].is_ascii_digit()) {
            let start = i;
            while i < len && (chars[i].is_ascii_digit() || chars[i] == '.') {
                i += 1;
            }
            let text: String = chars[start..i].iter().collect();
            tokens.push(Token { kind: Tok::Val, text });
            continue;
        }

        // Identifier / keyword
        if c.is_alphabetic() || c == '_' {
            let start = i;
            while i < len && (chars[i].is_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            let text: String = chars[start..i].iter().collect();
            let kind = match text.as_str() {
                "func" => Tok::FuncKw,
                "true" | "false" | "null" => Tok::Val,
                _ => Tok::Id,
            };
            tokens.push(Token { kind, text });
            continue;
        }

        // Three-char tokens
        if c == '.' && i + 2 < len && chars[i + 1] == '.' && chars[i + 2] == '.' {
            tokens.push(Token { kind: Tok::Spread, text: "...".into() });
            i += 3;
            continue;
        }

        // Two-char tokens
        if i + 1 < len {
            let two: String = chars[i..i + 2].iter().collect();
            match two.as_str() {
                "??" => {
                    tokens.push(Token { kind: Tok::Op, text: "??".into() });
                    i += 2;
                    continue;
                }
                "?=" => {
                    tokens.push(Token { kind: Tok::QuestionEq, text: "?=".into() });
                    i += 2;
                    continue;
                }
                "=>" => {
                    tokens.push(Token { kind: Tok::Arrow, text: "=>".into() });
                    i += 2;
                    continue;
                }
                "==" => {
                    tokens.push(Token { kind: Tok::Op, text: "==".into() });
                    i += 2;
                    continue;
                }
                "!=" => {
                    tokens.push(Token { kind: Tok::Op, text: "!=".into() });
                    i += 2;
                    continue;
                }
                "<=" => {
                    tokens.push(Token { kind: Tok::Op, text: "<=".into() });
                    i += 2;
                    continue;
                }
                ">=" => {
                    tokens.push(Token { kind: Tok::Op, text: ">=".into() });
                    i += 2;
                    continue;
                }
                _ => {}
            }
        }

        // Single-char tokens
        match c {
            '(' => { tokens.push(Token { kind: Tok::LParen, text: "(".into() }); i += 1; }
            ')' => { tokens.push(Token { kind: Tok::RParen, text: ")".into() }); i += 1; }
            '[' => { tokens.push(Token { kind: Tok::LBracket, text: "[".into() }); i += 1; }
            ']' => { tokens.push(Token { kind: Tok::RBracket, text: "]".into() }); i += 1; }
            ',' => { tokens.push(Token { kind: Tok::Comma, text: ",".into() }); i += 1; }
            '.' => { tokens.push(Token { kind: Tok::Dot, text: ".".into() }); i += 1; }
            '@' => { tokens.push(Token { kind: Tok::At, text: "@".into() }); i += 1; }
            '?' => { tokens.push(Token { kind: Tok::Question, text: "?".into() }); i += 1; }
            '+' | '-' | '*' | '/' | '%' | '<' | '>' | '=' | '!' => {
                tokens.push(Token { kind: Tok::Op, text: c.to_string() });
                i += 1;
            }
            _ => {
                // Skip unknown characters
                i += 1;
            }
        }
    }

    tokens.push(Token { kind: Tok::Eof, text: String::new() });
    tokens
}

pub fn parse_lines(source: &str, filename: &str) -> Vec<Line> {
    let mut lines = Vec::new();
    let source_name = Rc::new(filename.to_string());

    for (index, raw_line) in source.lines().enumerate() {
        let trimmed = raw_line.trim();

        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        let tokens = lex(trimmed);

        let meaningful: Vec<Token> = tokens
            .into_iter()
            .filter(|token| token.kind != Tok::Eof)
            .collect();

        if !meaningful.is_empty() {
            lines.push(Line {
                tokens: meaningful,
                source: source_name.clone(),
                number: index + 1,
            });
        }
    }

    lines
}