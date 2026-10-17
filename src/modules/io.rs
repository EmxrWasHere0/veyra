use crate::modules::VeyraModule;
use crate::runtime::Value;

pub struct IoModule;

impl VeyraModule for IoModule {
    fn name(&self) -> &str {
        "io"
    }

    fn call_func(&self, func: &str, args: Vec<Value>) -> Result<Value, String> {
        match func {
            "read" => {
                if args.len() != 1 {
                    return Err("io.read() takes 1 argument".into());
                }
                let path = args[0].to_string();
                std::fs::read_to_string(&path)
                    .map(Value::Text)
                    .map_err(|e| format!("io.read error: {}", e))
            }
            "write" => {
                if args.len() != 2 {
                    return Err("io.write() takes 2 arguments".into());
                }
                let path = args[0].to_string();
                let content = args[1].to_string();
                std::fs::write(&path, &content)
                    .map(|_| Value::Null)
                    .map_err(|e| format!("io.write error: {}", e))
            }
            "append" => {
                if args.len() != 2 {
                    return Err("io.append() takes 2 arguments".into());
                }
                let path = args[0].to_string();
                let content = args[1].to_string();
                std::fs::OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open(&path)
                    .and_then(|mut f| {
                        use std::io::Write;
                        f.write_all(content.as_bytes())
                    })
                    .map(|_| Value::Null)
                    .map_err(|e| format!("io.append error: {}", e))
            }
            "exists" => {
                if args.len() != 1 {
                    return Err("io.exists() takes 1 argument".into());
                }
                let path = args[0].to_string();
                Ok(Value::Bool(std::path::Path::new(&path).exists()))
            }
            "remove" => {
                if args.len() != 1 {
                    return Err("io.remove() takes 1 argument".into());
                }
                let path = args[0].to_string();
                let p = std::path::Path::new(&path);
                let result = if p.is_dir() {
                    std::fs::remove_dir(&path)
                } else {
                    std::fs::remove_file(&path)
                };
                result
                    .map(|_| Value::Null)
                    .map_err(|e| format!("io.remove error: {}", e))
            }
            "rename" => {
                if args.len() != 2 {
                    return Err("io.rename() takes 2 arguments".into());
                }
                let from = args[0].to_string();
                let to = args[1].to_string();
                std::fs::rename(&from, &to)
                    .map(|_| Value::Null)
                    .map_err(|e| format!("io.rename error: {}", e))
            }
            "mkdir" => {
                if args.len() != 1 {
                    return Err("io.mkdir() takes 1 argument".into());
                }
                let path = args[0].to_string();
                std::fs::create_dir_all(&path)
                    .map(|_| Value::Null)
                    .map_err(|e| format!("io.mkdir error: {}", e))
            }
            "rmdir" => {
                if args.len() != 1 {
                    return Err("io.rmdir() takes 1 argument".into());
                }
                let path = args[0].to_string();
                std::fs::remove_dir(&path)
                    .map(|_| Value::Null)
                    .map_err(|e| format!("io.rmdir error: {}", e))
            }
            "listdir" => {
                if args.len() != 1 {
                    return Err("io.listdir() takes 1 argument".into());
                }
                let path = args[0].to_string();
                let entries = std::fs::read_dir(&path)
                    .map_err(|e| format!("io.listdir error: {}", e))?;
                let mut items = Vec::new();
                for entry in entries.flatten() {
                    items.push(Value::Text(
                        entry.file_name().to_string_lossy().to_string(),
                    ));
                }
                Ok(Value::List(items))
            }
            "cwd" => std::env::current_dir()
                .map(|p| Value::Text(p.to_string_lossy().to_string()))
                .map_err(|e| format!("io.cwd error: {}", e)),
            "chdir" => {
                if args.len() != 1 {
                    return Err("io.chdir() takes 1 argument".into());
                }
                let path = args[0].to_string();
                std::env::set_current_dir(&path)
                    .map(|_| Value::Null)
                    .map_err(|e| format!("io.chdir error: {}", e))
            }
            "run" => {
                if args.is_empty() {
                    return Err("io.run() takes at least 1 argument".into());
                }
                let prog = args[0].to_string();
                let cmd_args: Vec<String> = match args.get(1) {
                    Some(Value::List(items)) if args.len() == 2 => {
                        items.iter()
                            .map(|value| value.to_string())
                            .collect()
                    }

                    _ => args[1..]
                        .iter()
                        .map(|value| value.to_string())
                        .collect(),
                };
                let output = std::process::Command::new(&prog)
                    .args(&cmd_args)
                    .output()
                    .map_err(|e| format!("io.run error: {}", e))?;
                Ok(Value::List(vec![
                    Value::Num(output.status.code().unwrap_or(-1) as f64),
                    Value::Text(String::from_utf8_lossy(&output.stdout).to_string()),
                    Value::Text(String::from_utf8_lossy(&output.stderr).to_string()),
                ]))
            }
            "shell" => {
                if args.len() != 1 {
                    return Err("io.shell() takes 1 argument".into());
                }
                let cmd = args[0].to_string();
                let (shell, flag) = if cfg!(target_os = "windows") {
                    ("cmd", "/C")
                } else {
                    ("sh", "-c")
                };
                let output = std::process::Command::new(shell)
                    .arg(flag)
                    .arg(&cmd)
                    .output()
                    .map_err(|e| format!("io.shell error: {}", e))?;
                Ok(Value::List(vec![
                    Value::Num(output.status.code().unwrap_or(-1) as f64),
                    Value::Text(String::from_utf8_lossy(&output.stdout).to_string()),
                    Value::Text(String::from_utf8_lossy(&output.stderr).to_string()),
                ]))
            }
            _ => Err(format!("io has no member '{}'", func)),
        }
    }

    fn list_members(&self) -> Vec<String> {
        vec![
            "read", "write", "append", "exists", "remove", "rename", "mkdir", "rmdir", "listdir",
            "cwd", "chdir", "run", "shell",
        ]
        .into_iter()
        .map(String::from)
        .collect()
    }

    fn is_function(&self, _name: &str) -> bool {
        true // io has only functions, no constants
    }
}