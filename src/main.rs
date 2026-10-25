mod lexer;
mod modules;
mod runtime;

use std::io::{self, Write};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // Start REPL when no arguments are provided.
    if args.len() < 2 {
        run_repl();
        return;
    }

    let mut test_mode = false;
    let mut file: Option<String> = None;
    let mut script_args = Vec::new();

    for arg in &args[1..] {
        if file.is_some() {
            script_args.push(arg.clone());
            continue;
        }

        match arg.as_str() {
            "-h" | "--help" => {
                print_help();
                return;
            }

            "-v" | "--version" => {
                println!("Veyra {}", env!("CARGO_PKG_VERSION"));
                return;
            }

            "-t" | "--test" => {
                test_mode = true;
            }

            _ if arg.starts_with('-') => {
                eprintln!("Unknown option: {}", arg);
                std::process::exit(1);
            }

            _ => {
                file = Some(arg.clone());
            }
        }
    }

    let file = match file {
        Some(f) => f,
        None => {
            if test_mode {
                eprintln!("Error: --test requires a file");
            } else {
                run_repl();
            }
            return;
        }
    };

    let source = match std::fs::read_to_string(&file) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error reading '{}': {}", file, e);
            std::process::exit(1);
        }
    };

    let lines = match lexer::parse_lines(&source, &file) {
        Ok(lines) => lines,

        Err(error) => {
            eprintln!("Lexer Error: {}", error);
            std::process::exit(1);
        }
    };

    let mut rt = runtime::Runtime::new();
    rt.test_mode = test_mode;

    rt.available_modules.insert(
        "system".into(),
        Box::new(modules::system::SystemModule { script_args }),
    );

    rt.script_dir = std::path::Path::new(&file)
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|| ".".to_string());

    match rt.run(&lines) {
        Ok(_) => {
            if test_mode {
                println!(
                    "\nTest Results: {} passed, {} failed",
                    rt.test_stats.passed, rt.test_stats.failed
                );

                for error in &rt.test_stats.errors {
                    println!("  FAIL: {}", error);
                }

                if rt.test_stats.failed > 0 {
                    std::process::exit(1);
                }
            }
        }

        Err(error) => {
            eprintln!("Runtime Error: {}", error);
            std::process::exit(1);
        }
    }
}

fn run_repl() {
    println!("Veyra {}", env!("CARGO_PKG_VERSION"));
    println!("Type 'exit' or 'quit' to leave.");
    println!();

    let mut rt = runtime::Runtime::new();
    rt.script_dir = ".".to_string();

    // Preserve variables and imports between commands.
    let mut buffer = String::new();

    loop {
        let prompt = if buffer.is_empty() { ">>> " } else { "... " };

        print!("{}", prompt);

        if io::stdout().flush().is_err() {
            break;
        }

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                println!();
                break;
            }

            Ok(_) => {}

            Err(error) => {
                eprintln!("Input Error: {}", error);
                continue;
            }
        }

        let trimmed = input.trim();

        if buffer.is_empty() && (trimmed == "exit" || trimmed == "quit") {
            break;
        }

        if buffer.is_empty() && trimmed.is_empty() {
            continue;
        }

        buffer.push_str(&input);

        // Wait until all blocks and delimiters are closed.
        if !repl_input_complete(&buffer) {
            continue;
        }

        match lexer::parse_lines(&buffer, "<repl>") {
            Ok(lines) => {
                if let Err(error) = rt.run(&lines) {
                    eprintln!("Runtime Error: {}", error);
                }
            }

            Err(error) => {
                eprintln!("Lexer Error: {}", error);
            }
        }

        buffer.clear();
    }
}

fn repl_input_complete(source: &str) -> bool {
    let mut block_depth = 0usize;
    let mut delimiters: Vec<char> = Vec::new();

    let mut triple_quote: Option<char> = None;

    for raw_line in source.lines() {
        let chars: Vec<char> = raw_line.chars().collect();

        let mut position = 0usize;
        let mut normal_quote: Option<char> = None;
        let mut visible = String::new();

        while position < chars.len() {
            let character = chars[position];

            // Handle an active multiline string.
            if let Some(quote) = triple_quote {
                if character == '\\' && position + 1 < chars.len() {
                    position += 2;
                    continue;
                }

                if position + 2 < chars.len()
                    && chars[position] == quote
                    && chars[position + 1] == quote
                    && chars[position + 2] == quote
                {
                    triple_quote = None;
                    position += 3;
                    continue;
                }

                position += 1;
                continue;
            }

            // Handle a regular quoted string.
            if let Some(quote) = normal_quote {
                if character == '\\' && position + 1 < chars.len() {
                    position += 2;
                    continue;
                }

                if character == quote {
                    normal_quote = None;
                }

                position += 1;
                continue;
            }

            // Ignore comments outside strings.
            if character == '#' {
                break;
            }

            // Detect quoted strings.
            if character == '"' || character == '\'' {
                if position + 2 < chars.len()
                    && chars[position + 1] == character
                    && chars[position + 2] == character
                {
                    triple_quote = Some(character);
                    position += 3;
                } else {
                    normal_quote = Some(character);
                    position += 1;
                }

                visible.push(' ');
                continue;
            }

            // Track expression delimiters.
            match character {
                '(' | '[' | '{' => delimiters.push(character),

                ')' => {
                    if delimiters.last() == Some(&'(') {
                        delimiters.pop();
                    } else {
                        return true;
                    }
                }

                ']' => {
                    if delimiters.last() == Some(&'[') {
                        delimiters.pop();
                    } else {
                        return true;
                    }
                }

                '}' => {
                    if delimiters.last() == Some(&'{') {
                        delimiters.pop();
                    } else {
                        return true;
                    }
                }

                _ => {}
            }

            visible.push(character);
            position += 1;
        }

        // Lines inside multiline strings do not affect block depth.
        let line = visible.trim();

        if line.is_empty() {
            continue;
        }

        let words: Vec<&str> = line.split_whitespace().collect();

        let first = words.first().copied().unwrap_or("");

        // Support labeled repeat loops.
        let keyword = if first.starts_with('@') {
            words.get(1).copied().unwrap_or("")
        } else {
            first
        };

        if matches!(keyword, "if" | "while" | "repeat" | "try" | "test" | "func") {
            block_depth += 1;
        } else if keyword == "add"
            && line.contains('(')
            && !line.contains("=>")
        {
            block_depth += 1;
        } else if keyword == "end" {
            block_depth = block_depth.saturating_sub(1);
        }
    }

    triple_quote.is_none() && delimiters.is_empty() && block_depth == 0
}

fn print_help() {
    println!(
        "Veyra {} - a programming language made for fun",
        env!("CARGO_PKG_VERSION")
    );

    println!();
    println!("Usage: veyra [options] [file.vey]");
    println!();

    println!("Options:");
    println!("  -h, --help     Show this help message");
    println!("  -v, --version  Show version");
    println!("  -t, --test     Run test blocks");
    println!();
    println!("Run without arguments to start the interactive REPL.");
}
