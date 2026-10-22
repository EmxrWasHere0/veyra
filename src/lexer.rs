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
    Colon,
    LBrace,
    RBrace,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub kind: Tok,
    pub text: String,
}

pub fn lex(source: &str) -> Result<Vec<Token>, String> {
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

        // Comments
        if c == '#' {
            while i < len && chars[i] != '\n' {
                i += 1;
            }
            continue;
        }

        // Strings: single and double quotes
        if c == '"' || c == '\'' {
            let quote = c;
            let start = i;

            i += 1;

            let mut closed = false;

            while i < len {
                if chars[i] == '\\' {
                    if i + 1 >= len {
                        break;
                    }

                    i += 2;
                    continue;
                }

                if chars[i] == quote {
                    i += 1;
                    closed = true;
                    break;
                }

                i += 1;
            }

            if !closed {
                return Err(format!(
                    "Unterminated string starting at column {}",
                    start + 1
                ));
            }

            let text: String = chars[start..i].iter().collect();

            tokens.push(Token {
                kind: Tok::Val,
                text,
            });

            continue;
        }

        // Numbers
        if c.is_ascii_digit() || (c == '.' && i + 1 < len && chars[i + 1].is_ascii_digit()) {
            let start = i;
            let mut has_dot = false;

            while i < len {
                if chars[i].is_ascii_digit() {
                    i += 1;
                } else if chars[i] == '.' && !has_dot {
                    has_dot = true;
                    i += 1;
                } else {
                    break;
                }
            }

            let text: String = chars[start..i].iter().collect();

            tokens.push(Token {
                kind: Tok::Val,
                text,
            });

            continue;
        }

        // Identifiers and keywords
        if c.is_alphabetic() || c == '_' {
            let start = i;

            while i < len && (chars[i].is_alphanumeric() || chars[i] == '_') {
                i += 1;
            }

            let text: String = chars[start..i].iter().collect();

            let kind = match text.as_str() {
                "func" => Tok::FuncKw,

                "true" | "false" | "null" => Tok::Val,

                "and" | "or" | "not" => Tok::Op,

                _ => Tok::Id,
            };

            tokens.push(Token { kind, text });

            continue;
        }

        // Spread operator
        if c == '.' && i + 2 < len && chars[i + 1] == '.' && chars[i + 2] == '.' {
            tokens.push(Token {
                kind: Tok::Spread,
                text: "...".into(),
            });

            i += 3;
            continue;
        }

        // Two-character operators
        if i + 1 < len {
            let two: String = chars[i..i + 2].iter().collect();

            let kind = match two.as_str() {
                "??" | "==" | "!=" | "<=" | ">=" => Some(Tok::Op),

                "?=" => Some(Tok::QuestionEq),
                "=>" => Some(Tok::Arrow),

                _ => None,
            };

            if let Some(kind) = kind {
                tokens.push(Token { kind, text: two });

                i += 2;
                continue;
            }
        }

        // Single-character tokens
        let kind = match c {
            '(' => Some(Tok::LParen),
            ')' => Some(Tok::RParen),

            '[' => Some(Tok::LBracket),
            ']' => Some(Tok::RBracket),

            '{' => Some(Tok::LBrace),
            '}' => Some(Tok::RBrace),

            ',' => Some(Tok::Comma),

            '.' => Some(Tok::Dot),
            '@' => Some(Tok::At),

            '?' => Some(Tok::Question),

            ':' => Some(Tok::Colon),

            '+' | '-' | '*' | '/' | '%' | '<' | '>' | '=' | '!' => Some(Tok::Op),

            _ => None,
        };

        if let Some(kind) = kind {
            tokens.push(Token {
                kind,
                text: c.to_string(),
            });

            i += 1;
            continue;
        }

        // Unknown characters must not be silently ignored.
        return Err(format!("Unexpected character '{}' at column {}", c, i + 1));
    }

    tokens.push(Token {
        kind: Tok::Eof,
        text: String::new(),
    });

    Ok(tokens)
}

pub fn parse_lines(source: &str, filename: &str) -> Result<Vec<Line>, String> {
    let mut lines = Vec::new();
    let source_name = Rc::new(filename.to_string());

    let mut pending_tokens: Vec<Token> = Vec::new();
    let mut pending_line = 0usize;

    let mut delimiters: Vec<(Tok, usize)> = Vec::new();

    for (index, raw_line) in source.lines().enumerate() {
        let line_number = index + 1;
        let trimmed = raw_line.trim();

        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        // Lex the original line to preserve column positions.
        let tokens = lex(raw_line)
            .map_err(|error| format!("{}:{}: {}", filename, line_number, error))?;

        let meaningful: Vec<Token> = tokens
            .into_iter()
            .filter(|token| token.kind != Tok::Eof)
            .collect();

        if meaningful.is_empty() {
            continue;
        }

        if pending_tokens.is_empty() {
            pending_line = line_number;
        }

        for token in meaningful {
            match token.kind {
                Tok::LParen | Tok::LBracket | Tok::LBrace => {
                    delimiters.push((token.kind.clone(), line_number));
                }

                Tok::RParen => {
                    match delimiters.pop() {
                        Some((Tok::LParen, _)) => {}

                        _ => {
                            return Err(format!(
                                "{}:{}: Unexpected ')'",
                                filename, line_number
                            ));
                        }
                    }
                }

                Tok::RBracket => {
                    match delimiters.pop() {
                        Some((Tok::LBracket, _)) => {}

                        _ => {
                            return Err(format!(
                                "{}:{}: Unexpected ']'",
                                filename, line_number
                            ));
                        }
                    }
                }

                Tok::RBrace => {
                    match delimiters.pop() {
                        Some((Tok::LBrace, _)) => {}

                        _ => {
                            return Err(format!(
                                "{}:{}: Unexpected '}}'",
                                filename, line_number
                            ));
                        }
                    }
                }

                _ => {}
            }

            pending_tokens.push(token);
        }

        // All delimiters closed: complete the logical line.
        if delimiters.is_empty() {
            lines.push(Line {
                tokens: std::mem::take(&mut pending_tokens),
                source: source_name.clone(),
                number: pending_line,
            });
        }
    }

    if let Some((kind, opened_at)) = delimiters.last() {
        let symbol = match kind {
            Tok::LParen => "(",
            Tok::LBracket => "[",
            Tok::LBrace => "{",
            _ => "?",
        };

        return Err(format!(
            "{}:{}: Unclosed '{}'",
            filename, opened_at, symbol
        ));
    }

    Ok(lines)
}