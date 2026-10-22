mod lexer;
mod modules;
mod runtime;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        print_help();
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
                print_help();
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
                for e in &rt.test_stats.errors {
                    println!("  FAIL: {}", e);
                }
                if rt.test_stats.failed > 0 {
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("Runtime Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn print_help() {
    println!(
        "Veyra {} - a programming language made for fun",
        env!("CARGO_PKG_VERSION")
    );
    println!();
    println!("Usage: veyra [options] <file.vey>");
    println!();
    println!("Options:");
    println!("  -h, --help     Show this help message");
    println!("  -v, --version  Show version");
    println!("  -t, --test     Run test blocks");
}
