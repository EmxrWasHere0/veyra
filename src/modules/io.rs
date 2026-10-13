use std::fs;
use std::io::Write;
use std::process::Command;

use crate::modules::VeyraModule;
use crate::runtime::Value;

pub struct IoModule;

impl VeyraModule for IoModule {
    fn name(&self) -> &str {
        "io"
    }

    fn members(&self) -> Vec<String> {
        vec![
            "read".into(),
            "write".into(),
            "append".into(),
            "exists".into(),
            "remove".into(),
            "rename".into(),
            "mkdir".into(),
            "rmdir".into(),
            "listdir".into(),
            "cwd".into(),
            "chdir".into(),
            "run".into(),
            "shell".into(),
        ]
    }

    fn is_function(&self, _name: &str) -> bool {
        true // all members are functions
    }

    fn get_value(&self, _name: &str) -> Option<Value> {
        None
    }

    fn call(&self, name: &str, args: Vec<Value>) -> Result<Value, String> {
        match name {
            // ---- File operations ----

            "read" => {
                need(name, &args, 1)?;
                let path = as_text(name, &args[0])?;
                fs::read_to_string(&path)
                    .map(Value::Text)
                    .map_err(|e| format!("io.read: {}", e))
            }

            "write" => {
                need(name, &args, 2)?;
                let path = as_text(name, &args[0])?;
                let content = as_text(name, &args[1])?;
                fs::write(&path, &content).map_err(|e| format!("io.write: {}", e))?;
                Ok(Value::Null)
            }

            "append" => {
                need(name, &args, 2)?;
                let path = as_text(name, &args[0])?;
                let content = as_text(name, &args[1])?;
                let mut file = fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&path)
                    .map_err(|e| format!("io.append: {}", e))?;
                file.write_all(content.as_bytes())
                    .map_err(|e| format!("io.append: {}", e))?;
                Ok(Value::Null)
            }

            "exists" => {
                need(name, &args, 1)?;
                let path = as_text(name, &args[0])?;
                Ok(Value::Bool(std::path::Path::new(&path).exists()))
            }

            "remove" => {
                need(name, &args, 1)?;
                let path = as_text(name, &args[0])?;
                fs::remove_file(&path).map_err(|e| format!("io.remove: {}", e))?;
                Ok(Value::Null)
            }

            "rename" => {
                need(name, &args, 2)?;
                let from = as_text(name, &args[0])?;
                let to = as_text(name, &args[1])?;
                fs::rename(&from, &to).map_err(|e| format!("io.rename: {}", e))?;
                Ok(Value::Null)
            }

            // ---- Directory operations ----

            "mkdir" => {
                need(name, &args, 1)?;
                let path = as_text(name, &args[0])?;
                fs::create_dir_all(&path).map_err(|e| format!("io.mkdir: {}", e))?;
                Ok(Value::Null)
            }

            "rmdir" => {
                need(name, &args, 1)?;
                let path = as_text(name, &args[0])?;
                fs::remove_dir_all(&path).map_err(|e| format!("io.rmdir: {}", e))?;
                Ok(Value::Null)
            }

            "listdir" => {
                need(name, &args, 1)?;
                let path = as_text(name, &args[0])?;
                let entries: Vec<Value> = fs::read_dir(&path)
                    .map_err(|e| format!("io.listdir: {}", e))?
                    .filter_map(|entry| entry.ok())
                    .map(|e| Value::Text(e.file_name().to_string_lossy().to_string()))
                    .collect();
                Ok(Value::List(entries))
            }

            "cwd" => {
                if !args.is_empty() {
                    return Err("io.cwd() takes no arguments".into());
                }
                std::env::current_dir()
                    .map(|p| Value::Text(p.to_string_lossy().to_string()))
                    .map_err(|e| format!("io.cwd: {}", e))
            }

            "chdir" => {
                need(name, &args, 1)?;
                let path = as_text(name, &args[0])?;
                std::env::set_current_dir(&path)
                    .map_err(|e| format!("io.chdir: {}", e))?;
                Ok(Value::Null)
            }

            // ---- Subprocess ----

            "run" => {
                // io.run(program, [...args]) -> [exit_code, stdout, stderr]
                if args.is_empty() || args.len() > 2 {
                    return Err("io.run(program, [args]) takes 1-2 arguments".into());
                }
                let program = as_text(name, &args[0])?;
                let cmd_args: Vec<String> = if args.len() == 2 {
                    match &args[1] {
                        Value::List(items) => items
                            .iter()
                            .map(|v| match v {
                                Value::Text(s) => Ok(s.clone()),
                                other => Ok(other.to_string()),
                            })
                            .collect::<Result<Vec<_>, String>>()?,
                        other => return Err(format!("io.run: expected list for args, found: {}", other)),
                    }
                } else {
                    Vec::new()
                };

                let output = Command::new(&program)
                    .args(&cmd_args)
                    .output()
                    .map_err(|e| format!("io.run: {}", e))?;

                Ok(Value::List(vec![
                    Value::Num(output.status.code().unwrap_or(-1) as f64),
                    Value::Text(String::from_utf8_lossy(&output.stdout).to_string()),
                    Value::Text(String::from_utf8_lossy(&output.stderr).to_string()),
                ]))
            }

            "shell" => {
                // io.shell(command_string) -> [exit_code, stdout, stderr]
                need(name, &args, 1)?;
                let cmd = as_text(name, &args[0])?;

                let output = if cfg!(target_os = "windows") {
                    Command::new("cmd")
                        .args(["/C", &cmd])
                        .output()
                } else {
                    Command::new("sh")
                        .args(["-c", &cmd])
                        .output()
                }
                .map_err(|e| format!("io.shell: {}", e))?;

                Ok(Value::List(vec![
                    Value::Num(output.status.code().unwrap_or(-1) as f64),
                    Value::Text(String::from_utf8_lossy(&output.stdout).to_string()),
                    Value::Text(String::from_utf8_lossy(&output.stderr).to_string()),
                ]))
            }

            _ => Err(format!("io.{} is not a function", name)),
        }
    }
}

fn need(func: &str, args: &[Value], n: usize) -> Result<(), String> {
    if args.len() == n {
        Ok(())
    } else {
        Err(format!(
            "io.{} takes {} argument(s), {} given",
            func,
            n,
            args.len()
        ))
    }
}

fn as_text(func: &str, val: &Value) -> Result<String, String> {
    match val {
        Value::Text(s) => Ok(s.clone()),
        other => Err(format!("io.{}: expected text, found: {}", func, other)),
    }
}