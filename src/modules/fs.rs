use crate::modules::VeyraModule;
use crate::runtime::Value;

use std::fs;
use std::path::{Path, PathBuf};

pub struct FsModule;

fn expect_count(args: &[Value], count: usize, func: &str) -> Result<(), String> {
    if args.len() != count {
        return Err(format!("fs.{} expects {} argument(s)", func, count));
    }

    Ok(())
}

fn text_arg<'a>(args: &'a [Value], index: usize, func: &str) -> Result<&'a str, String> {
    match args.get(index) {
        Some(Value::Text(text)) => Ok(text),
        _ => Err(format!(
            "fs.{} expects text argument at position {}",
            func,
            index + 1
        )),
    }
}

fn path_text(path: &Path) -> String {
    path.to_string_lossy().to_string()
}

impl VeyraModule for FsModule {
    fn name(&self) -> &str {
        "fs"
    }

    fn call_func(&self, func: &str, args: Vec<Value>) -> Result<Value, String> {
        match func {
            "read" => {
                expect_count(&args, 1, func)?;

                let path = text_arg(&args, 0, func)?;

                fs::read_to_string(path)
                    .map(Value::Text)
                    .map_err(|error| format!("fs.read error: {}", error))
            }

            "write" => {
                expect_count(&args, 2, func)?;

                let path = text_arg(&args, 0, func)?;
                let content = text_arg(&args, 1, func)?;

                fs::write(path, content).map_err(|error| format!("fs.write error: {}", error))?;

                Ok(Value::Null)
            }

            "append" => {
                expect_count(&args, 2, func)?;

                let path = text_arg(&args, 0, func)?;
                let content = text_arg(&args, 1, func)?;

                use std::io::Write;

                let mut file = fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(path)
                    .map_err(|error| format!("fs.append error: {}", error))?;

                file.write_all(content.as_bytes())
                    .map_err(|error| format!("fs.append error: {}", error))?;

                Ok(Value::Null)
            }

            "exists" => {
                expect_count(&args, 1, func)?;

                Ok(Value::Bool(Path::new(text_arg(&args, 0, func)?).exists()))
            }

            "is_file" => {
                expect_count(&args, 1, func)?;

                Ok(Value::Bool(Path::new(text_arg(&args, 0, func)?).is_file()))
            }

            "is_dir" => {
                expect_count(&args, 1, func)?;

                Ok(Value::Bool(Path::new(text_arg(&args, 0, func)?).is_dir()))
            }

            "mkdir" => {
                expect_count(&args, 1, func)?;

                fs::create_dir(text_arg(&args, 0, func)?)
                    .map_err(|error| format!("fs.mkdir error: {}", error))?;

                Ok(Value::Null)
            }

            "mkdirs" => {
                expect_count(&args, 1, func)?;

                fs::create_dir_all(text_arg(&args, 0, func)?)
                    .map_err(|error| format!("fs.mkdirs error: {}", error))?;

                Ok(Value::Null)
            }

            "listdir" => {
                expect_count(&args, 1, func)?;

                let path = text_arg(&args, 0, func)?;

                let mut entries = Vec::new();

                for entry in
                    fs::read_dir(path).map_err(|error| format!("fs.listdir error: {}", error))?
                {
                    let entry = entry.map_err(|error| format!("fs.listdir error: {}", error))?;

                    entries.push(Value::Text(entry.file_name().to_string_lossy().to_string()));
                }

                Ok(Value::List(entries))
            }

            "remove" => {
                expect_count(&args, 1, func)?;

                fs::remove_file(text_arg(&args, 0, func)?)
                    .map_err(|error| format!("fs.remove error: {}", error))?;

                Ok(Value::Null)
            }

            "rmdir" => {
                expect_count(&args, 1, func)?;

                fs::remove_dir(text_arg(&args, 0, func)?)
                    .map_err(|error| format!("fs.rmdir error: {}", error))?;

                Ok(Value::Null)
            }

            "rmtree" => {
                expect_count(&args, 1, func)?;

                fs::remove_dir_all(text_arg(&args, 0, func)?)
                    .map_err(|error| format!("fs.rmtree error: {}", error))?;

                Ok(Value::Null)
            }

            "rename" => {
                expect_count(&args, 2, func)?;

                fs::rename(text_arg(&args, 0, func)?, text_arg(&args, 1, func)?)
                    .map_err(|error| format!("fs.rename error: {}", error))?;

                Ok(Value::Null)
            }

            "copy" => {
                expect_count(&args, 2, func)?;

                let bytes = fs::copy(text_arg(&args, 0, func)?, text_arg(&args, 1, func)?)
                    .map_err(|error| format!("fs.copy error: {}", error))?;

                Ok(Value::Num(bytes as f64))
            }

            "size" => {
                expect_count(&args, 1, func)?;

                let metadata = fs::metadata(text_arg(&args, 0, func)?)
                    .map_err(|error| format!("fs.size error: {}", error))?;

                Ok(Value::Num(metadata.len() as f64))
            }

            "absolute" => {
                expect_count(&args, 1, func)?;

                let path = text_arg(&args, 0, func)?;

                let absolute = fs::canonicalize(path)
                    .map_err(|error| format!("fs.absolute error: {}", error))?;

                Ok(Value::Text(path_text(&absolute)))
            }

            "join" => {
                if args.is_empty() {
                    return Err("fs.join expects at least 1 argument".into());
                }

                let mut path = PathBuf::new();

                for (index, arg) in args.iter().enumerate() {
                    match arg {
                        Value::Text(part) => path.push(part),
                        _ => {
                            return Err(format!(
                                "fs.join expects text argument at position {}",
                                index + 1
                            ));
                        }
                    }
                }

                Ok(Value::Text(path_text(&path)))
            }

            "basename" => {
                expect_count(&args, 1, func)?;

                let path = Path::new(text_arg(&args, 0, func)?);

                Ok(Value::Text(
                    path.file_name()
                        .map(|name| name.to_string_lossy().to_string())
                        .unwrap_or_default(),
                ))
            }

            "dirname" => {
                expect_count(&args, 1, func)?;

                let path = Path::new(text_arg(&args, 0, func)?);

                Ok(Value::Text(
                    path.parent().map(path_text).unwrap_or_default(),
                ))
            }

            "extension" => {
                expect_count(&args, 1, func)?;

                let path = Path::new(text_arg(&args, 0, func)?);

                Ok(Value::Text(
                    path.extension()
                        .map(|ext| ext.to_string_lossy().to_string())
                        .unwrap_or_default(),
                ))
            }

            "cwd" => {
                expect_count(&args, 0, func)?;

                std::env::current_dir()
                    .map(|path| Value::Text(path_text(&path)))
                    .map_err(|error| format!("fs.cwd error: {}", error))
            }

            _ => Err(format!("fs has no member '{}'", func)),
        }
    }

    fn list_members(&self) -> Vec<String> {
        [
            "read",
            "write",
            "append",
            "exists",
            "is_file",
            "is_dir",
            "mkdir",
            "mkdirs",
            "listdir",
            "remove",
            "rmdir",
            "rmtree",
            "rename",
            "copy",
            "size",
            "absolute",
            "join",
            "basename",
            "dirname",
            "extension",
            "cwd",
        ]
        .iter()
        .map(|name| name.to_string())
        .collect()
    }

    fn is_function(&self, _name: &str) -> bool {
        true
    }
}
