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

    for arg in &args[1..] {
        match arg.as_str() {
            "-h" | "--help" => {
                print_help();
                return;
            }
            "-v" | "--version" => {
                println!("Veyra 0.1.0");
                return;
            }
            "-t" | "--test" => {
                test_mode = true;
            }
            _ => {
                if !arg.starts_with('-') {
                    file = Some(arg.clone());
                }
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

    let lines = lexer::parse_lines(&source, &file);
    let mut rt = runtime::Runtime::new();
    rt.test_mode = test_mode;
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
    println!("Veyra 0.1.0 - A Turkish-flavored programming language");
    println!();
    println!("Usage: veyra [options] <file.vey>");
    println!();
    println!("Options:");
    println!("  -h, --help     Show this help message");
    println!("  -v, --version  Show version");
    println!("  -t, --test     Run test blocks");
}