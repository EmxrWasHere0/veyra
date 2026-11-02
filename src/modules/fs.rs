use crate::modules::VeyraModule;
use crate::runtime::Value;

use tempfile::Builder;
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

fn copy_directory(source: &Path, destination: &Path) -> Result<(), String> {
    let metadata = fs::symlink_metadata(source)
        .map_err(|error| format!("fs.copytree error: {}", error))?;

    if metadata.file_type().is_symlink() {
        return Err(format!(
            "fs.copytree error: symbolic links are not supported: '{}'",
            source.display()
        ));
    }

    if !metadata.is_dir() {
        return Err(format!(
            "fs.copytree error: source is not a directory: '{}'",
            source.display()
        ));
    }

    fs::create_dir(destination)
        .map_err(|error| format!("fs.copytree error: {}", error))?;

    for entry in fs::read_dir(source)
        .map_err(|error| format!("fs.copytree error: {}", error))?
    {
        let entry =
            entry.map_err(|error| format!("fs.copytree error: {}", error))?;

        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());

        let file_type = entry
            .file_type()
            .map_err(|error| format!("fs.copytree error: {}", error))?;

        if file_type.is_symlink() {
            return Err(format!(
                "fs.copytree error: symbolic links are not supported: '{}'",
                source_path.display()
            ));
        }

        if file_type.is_dir() {
            copy_directory(&source_path, &destination_path)?;
        } else if file_type.is_file() {
            fs::copy(&source_path, &destination_path)
                .map_err(|error| format!("fs.copytree error: {}", error))?;
        } else {
            return Err(format!(
                "fs.copytree error: unsupported file type: '{}'",
                source_path.display()
            ));
        }
    }

    Ok(())
}

fn replace_directory(source: &Path, destination: &Path) -> Result<(), String> {
    let source_metadata = fs::symlink_metadata(source)
        .map_err(|error| format!("fs.replace_tree error: {}", error))?;

    if source_metadata.file_type().is_symlink() {
        return Err(
            "fs.replace_tree error: source cannot be a symbolic link".into()
        );
    }

    if !source_metadata.is_dir() {
        return Err(
            "fs.replace_tree error: source is not a directory".into()
        );
    }

    let source_root = source
        .canonicalize()
        .map_err(|error| format!("fs.replace_tree error: {}", error))?;

    let parent = destination
        .parent()
        .filter(|path| !path.as_os_str().is_empty())
        .unwrap_or(Path::new("."));

    let parent_root = parent
        .canonicalize()
        .map_err(|error| format!("fs.replace_tree error: {}", error))?;

    let destination_name = destination
        .file_name()
        .ok_or_else(|| {
            "fs.replace_tree error: invalid destination path".to_string()
        })?;

    let destination_root = parent_root.join(destination_name);

    if destination_root.starts_with(&source_root) {
        return Err(
            "fs.replace_tree error: destination cannot be inside source directory".into()
        );
    }

    if source_root.starts_with(&destination_root) {
        return Err(
            "fs.replace_tree error: source cannot be inside destination directory".into()
        );
    }

    // Inspect the destination without following symbolic links.
    let destination_exists = match fs::symlink_metadata(&destination_root) {
        Ok(metadata) => {
            if metadata.file_type().is_symlink() {
                return Err(
                    "fs.replace_tree error: destination cannot be a symbolic link".into()
                );
            }

            if !metadata.is_dir() {
                return Err(
                    "fs.replace_tree error: destination is not a directory".into()
                );
            }

            true
        }

        Err(error) if error.kind() == std::io::ErrorKind::NotFound => false,

        Err(error) => {
            return Err(format!("fs.replace_tree error: {}", error));
        }
    };

    // Stage the new directory before modifying the existing destination.
    let staging = Builder::new()
        .prefix(".veyra-stage-")
        .tempdir_in(&parent_root)
        .map_err(|error| format!("fs.replace_tree error: {}", error))?;

    let staged_package = staging.path().join("package");

    copy_directory(&source_root, &staged_package)?;

    if !destination_exists {
        fs::rename(&staged_package, &destination_root)
            .map_err(|error| format!("fs.replace_tree error: {}", error))?;

        return Ok(());
    }

    // Keep the backup on the same filesystem as the destination.
    let backup = Builder::new()
        .prefix(".veyra-backup-")
        .tempdir_in(&parent_root)
        .map_err(|error| format!("fs.replace_tree error: {}", error))?;

    let backup_package = backup.path().join("package");

    fs::rename(&destination_root, &backup_package)
        .map_err(|error| format!("fs.replace_tree error: {}", error))?;

    match fs::rename(&staged_package, &destination_root) {
        Ok(()) => Ok(()),

        Err(install_error) => {
            match fs::rename(&backup_package, &destination_root) {
                Ok(()) => Err(format!(
                    "fs.replace_tree error: installation failed: {}",
                    install_error
                )),

                Err(restore_error) => {
                    // Preserve the backup for manual recovery.
                    let recovery_path = backup.keep();

                    Err(format!(
                        "fs.replace_tree error: installation failed: {}; \
                         rollback failed: {}; backup preserved at '{}'",
                        install_error,
                        restore_error,
                        recovery_path.display()
                    ))
                }
            }
        }
    }
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

            "copytree" => {
                expect_count(&args, 2, func)?;

                let source = Path::new(text_arg(&args, 0, func)?);
                let destination = Path::new(text_arg(&args, 1, func)?);

                let source_absolute = source
                    .canonicalize()
                    .map_err(|error| format!("fs.copytree error: {}", error))?;

                if !source_absolute.is_dir() {
                    return Err(format!(
                        "fs.copytree error: source is not a directory: '{}'",
                        source.display()
                    ));
                }

                if destination.exists() {
                    return Err(format!(
                        "fs.copytree error: destination already exists: '{}'",
                        destination.display()
                    ));
                }

                let parent = destination
                    .parent()
                    .filter(|path| !path.as_os_str().is_empty())
                    .unwrap_or(Path::new("."));

                let parent_absolute = parent
                    .canonicalize()
                    .map_err(|error| format!("fs.copytree error: {}", error))?;

                let destination_name = destination
                    .file_name()
                    .ok_or_else(|| "fs.copytree error: invalid destination path".to_string())?;

                let destination_absolute = parent_absolute.join(destination_name);

                if destination_absolute.starts_with(&source_absolute) {
                    return Err(
                        "fs.copytree error: destination cannot be inside source directory".into()
                    );
                }

                copy_directory(source, destination)?;

                Ok(Value::Null)
            }

            "replace_tree" => {
                expect_count(&args, 2, func)?;

                let source = Path::new(text_arg(&args, 0, func)?);
                let destination = Path::new(text_arg(&args, 1, func)?);

                replace_directory(source, destination)?;

                Ok(Value::Null)
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
            "copytree",
            "replace_tree",
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
