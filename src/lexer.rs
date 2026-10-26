use std::rc::Rc;

use crate::runtime::Line;

#[derive(Clone, Debug, PartialEq)]
pub enum Tok {
    Val,
    InterpolatedString,
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
    pub line: usize,
    pub column: usize,
}

pub fn lex(source: &str) -> Result<Vec<Token>, String> {
    let chars: Vec<char> = source.chars().collect();
    let len = chars.len();

    let mut i = 0usize;
    let mut line = 1usize;
    let mut column = 1usize;
    let mut tokens = Vec::new();

    // Advance the cursor while tracking line and column positions.
    macro_rules! advance {
        () => {{
            if i < len {
                if chars[i] == '\n' {
                    line += 1;
                    column = 1;
                } else {
                    column += 1;
                }

                i += 1;
            }
        }};
    }

    // Advance multiple characters.
    macro_rules! advance_n {
        ($count:expr) => {{
            for _ in 0..$count {
                advance!();
            }
        }};
    }

    // Create a token with its starting position.
    macro_rules! push_token {
        ($kind:expr, $text:expr, $line:expr, $column:expr) => {{
            tokens.push(Token {
                kind: $kind,
                text: $text,
                line: $line,
                column: $column,
            });
        }};
    }

    while i < len {
        let c = chars[i];

        // Whitespace
        if c.is_whitespace() {
            advance!();
            continue;
        }

        // Comments
        if c == '#' {
            while i < len && chars[i] != '\n' {
                advance!();
            }

            continue;
        }

        let start_line = line;
        let start_column = column;

        // Interpolated strings
        if c == 'f'
            && i + 1 < len
            && (chars[i + 1] == '"' || chars[i + 1] == '\'')
        {
            let quote = chars[i + 1];

            advance_n!(2);

            let triple = i + 1 < len
                && chars[i] == quote
                && chars[i + 1] == quote;

            if triple {
                advance_n!(2);
            }

            let mut content = String::new();
            let mut closed = false;

            while i < len {
                if chars[i] == '\\' && i + 1 < len {
                    content.push(chars[i]);
                    advance!();

                    content.push(chars[i]);
                    advance!();

                    continue;
                }

                if triple {
                    if i + 2 < len
                        && chars[i] == quote
                        && chars[i + 1] == quote
                        && chars[i + 2] == quote
                    {
                        advance_n!(3);
                        closed = true;
                        break;
                    }
                } else if chars[i] == quote {
                    advance!();
                    closed = true;
                    break;
                }

                content.push(chars[i]);
                advance!();
            }

            if !closed {
                return Err(format!(
                    "Unterminated interpolated string at line {}, column {}",
                    start_line, start_column
                ));
            }

            push_token!(
                Tok::InterpolatedString,
                content,
                start_line,
                start_column
            );

            continue;
        }

        // Triple-quoted multiline strings
        if (c == '"' || c == '\'')
            && i + 2 < len
            && chars[i + 1] == c
            && chars[i + 2] == c
        {
            let quote = c;

            advance_n!(3);

            let mut content = String::new();
            let mut closed = false;

            while i < len {
                // Preserve escaped characters.
                if chars[i] == '\\' && i + 1 < len {
                    content.push(chars[i]);
                    advance!();

                    content.push(chars[i]);
                    advance!();

                    continue;
                }

                // Closing triple quote.
                if i + 2 < len
                    && chars[i] == quote
                    && chars[i + 1] == quote
                    && chars[i + 2] == quote
                {
                    advance_n!(3);
                    closed = true;
                    break;
                }

                content.push(chars[i]);
                advance!();
            }

            if !closed {
                return Err(format!(
                    "Unterminated multiline string at line {}, column {}",
                    start_line, start_column
                ));
            }

            // Encode physical newlines for the existing string parser.
            let mut encoded = String::new();

            for character in content.chars() {
                match character {
                    '\n' => encoded.push_str("\\n"),
                    '\r' => encoded.push_str("\\r"),
                    '"' => encoded.push_str("\\\""),
                    other => encoded.push(other),
                }
            }

            push_token!(
                Tok::Val,
                format!("\"{}\"", encoded),
                start_line,
                start_column
            );

            continue;
        }

        // Regular strings
        if c == '"' || c == '\'' {
            let quote = c;
            let start = i;

            advance!();

            let mut closed = false;

            while i < len {
                if chars[i] == '\\' {
                    if i + 1 >= len {
                        break;
                    }

                    advance_n!(2);
                    continue;
                }

                if chars[i] == quote {
                    advance!();
                    closed = true;
                    break;
                }

                advance!();
            }

            if !closed {
                return Err(format!(
                    "Unterminated string at line {}, column {}",
                    start_line, start_column
                ));
            }

            let text: String = chars[start..i].iter().collect();

            push_token!(
                Tok::Val,
                text,
                start_line,
                start_column
            );

            continue;
        }

        // Numbers
        if c.is_ascii_digit()
            || (c == '.' && i + 1 < len && chars[i + 1].is_ascii_digit())
        {
            let start = i;
            let mut has_dot = false;

            while i < len {
                if chars[i].is_ascii_digit() {
                    advance!();
                } else if chars[i] == '.' && !has_dot {
                    has_dot = true;
                    advance!();
                } else {
                    break;
                }
            }

            let text: String = chars[start..i].iter().collect();

            push_token!(
                Tok::Val,
                text,
                start_line,
                start_column
            );

            continue;
        }

        // Identifiers and keywords
        if c.is_alphabetic() || c == '_' {
            let start = i;

            while i < len
                && (chars[i].is_alphanumeric() || chars[i] == '_')
            {
                advance!();
            }

            let text: String = chars[start..i].iter().collect();

            let kind = match text.as_str() {
                "func" => Tok::FuncKw,

                "true" | "false" | "null" => Tok::Val,

                "and" | "or" | "not" | "in" => Tok::Op,

                "throw" => Tok::Id,

                _ => Tok::Id,
            };

            push_token!(
                kind,
                text,
                start_line,
                start_column
            );

            continue;
        }

        // Spread operator
        if c == '.'
            && i + 2 < len
            && chars[i + 1] == '.'
            && chars[i + 2] == '.'
        {
            push_token!(
                Tok::Spread,
                "...".into(),
                start_line,
                start_column
            );

            advance_n!(3);
            continue;
        }

        // Two-character operators
        if i + 1 < len {
            let two: String = chars[i..i + 2].iter().collect();

            let kind = match two.as_str() {
                "??" | "==" | "!=" | "<=" | ">="
                | "+=" | "-=" | "*=" | "/=" | "%=" => Some(Tok::Op),

                "?=" => Some(Tok::QuestionEq),

                "=>" => Some(Tok::Arrow),

                _ => None,
            };

            if let Some(kind) = kind {
                push_token!(
                    kind,
                    two,
                    start_line,
                    start_column
                );

                advance_n!(2);
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

            '+' | '-' | '*' | '/' | '%' | '<' | '>' | '=' | '!' => {
                Some(Tok::Op)
            }

            _ => None,
        };

        if let Some(kind) = kind {
            push_token!(
                kind,
                c.to_string(),
                start_line,
                start_column
            );

            advance!();
            continue;
        }

        // Unknown characters must not be silently ignored.
        return Err(format!(
            "Unexpected character '{}' at line {}, column {}",
            c, start_line, start_column
        ));
    }

    // End-of-file token
    push_token!(
        Tok::Eof,
        String::new(),
        line,
        column
    );

    Ok(tokens)
}

pub fn parse_lines(source: &str, filename: &str) -> Result<Vec<Line>, String> {
    let mut lines = Vec::new();
    let source_name = Rc::new(filename.to_string());

    let mut pending_source = String::new();
    let mut pending_line = 0usize;

    let mut delimiters: Vec<(Tok, usize)> = Vec::new();

    // Currently open triple-quoted string, if any.
    let mut triple_quote: Option<char> = None;

    for (index, raw_line) in source.lines().enumerate() {
        let line_number = index + 1;

        if pending_source.is_empty() && triple_quote.is_none() && raw_line.trim().is_empty() {
            continue;
        }

        if pending_source.is_empty() && triple_quote.is_none() && raw_line.trim().starts_with('#') {
            continue;
        }

        if pending_source.is_empty() {
            pending_line = line_number;
        } else {
            pending_source.push('\n');
        }

        pending_source.push_str(raw_line);

        // Detect triple-quoted string boundaries.
        // Ignore delimiters inside normal strings and comments.
        let chars: Vec<char> = raw_line.chars().collect();

        let mut i = 0usize;
        let mut normal_quote: Option<char> = None;

        while i < chars.len() {
            let c = chars[i];

            if let Some(quote) = triple_quote {
                if c == '\\' && i + 1 < chars.len() {
                    i += 2;
                    continue;
                }

                if c == quote
                    && i + 2 < chars.len()
                    && chars[i + 1] == quote
                    && chars[i + 2] == quote
                {
                    triple_quote = None;
                    i += 3;
                    continue;
                }

                i += 1;
                continue;
            }

            if let Some(quote) = normal_quote {
                if c == '\\' && i + 1 < chars.len() {
                    i += 2;
                    continue;
                }

                if c == quote {
                    normal_quote = None;
                }

                i += 1;
                continue;
            }

            if c == '#' {
                break;
            }

            if c == '"' || c == '\'' {
                if i + 2 < chars.len() && chars[i + 1] == c && chars[i + 2] == c {
                    triple_quote = Some(c);
                    i += 3;
                    continue;
                }

                normal_quote = Some(c);
                i += 1;
                continue;
            }

            i += 1;
        }

        // Wait until the multiline string is closed.
        if triple_quote.is_some() {
            continue;
        }

        // Lex the accumulated source.
        let tokens = lex(&pending_source)
            .map_err(|error| format!("{}:{}: {}", filename, pending_line, error))?;

        let meaningful: Vec<Token> = tokens
            .into_iter()
            .filter(|token| token.kind != Tok::Eof)
            .map(|mut token| {
                token.line += pending_line - 1;
                token
            })
            .collect();

        // Rebuild delimiter state from the accumulated tokens.
        delimiters.clear();

        for token in &meaningful {
            match token.kind {
                Tok::LParen | Tok::LBracket | Tok::LBrace => {
                    delimiters.push((token.kind.clone(), pending_line));
                }

                Tok::RParen => match delimiters.pop() {
                    Some((Tok::LParen, _)) => {}

                    _ => {
                        return Err(format!("{}:{}: Unexpected ')'", filename, line_number));
                    }
                },

                Tok::RBracket => match delimiters.pop() {
                    Some((Tok::LBracket, _)) => {}

                    _ => {
                        return Err(format!("{}:{}: Unexpected ']'", filename, line_number));
                    }
                },

                Tok::RBrace => match delimiters.pop() {
                    Some((Tok::LBrace, _)) => {}

                    _ => {
                        return Err(format!("{}:{}: Unexpected '}}'", filename, line_number));
                    }
                },

                _ => {}
            }
        }

        // The logical expression is complete.
        if delimiters.is_empty() {
            if !meaningful.is_empty() {
                lines.push(Line {
                    tokens: meaningful,
                    source: source_name.clone(),
                    number: pending_line,
                });
            }

            pending_source.clear();
        }
    }

    if triple_quote.is_some() {
        return Err(format!(
            "{}:{}: Unterminated multiline string",
            filename, pending_line
        ));
    }

    if let Some((kind, opened_at)) = delimiters.last() {
        let symbol = match kind {
            Tok::LParen => "(",
            Tok::LBracket => "[",
            Tok::LBrace => "{",
            _ => "?",
        };

        return Err(format!("{}:{}: Unclosed '{}'", filename, opened_at, symbol));
    }

    Ok(lines)
}
