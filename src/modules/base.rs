use crate::modules::VeyraModule;
use crate::runtime::Value;

pub struct BaseModule;

impl VeyraModule for BaseModule {
    fn name(&self) -> &str { "base" }

    fn members(&self) -> Vec<String> {
        vec![
            "os".into(), "arch".into(), "family".into(),
            "hostname".into(), "username".into(), "home_dir".into(),
            "exe_path".into(), "pid".into(), "cores".into(),
        ]
    }

    fn is_function(&self, name: &str) -> bool {
        matches!(name, "hostname" | "username" | "home_dir" | "exe_path" | "pid" | "cores")
    }

    fn get_value(&self, name: &str) -> Option<Value> {
        match name {
            "os" => Some(Value::Text(std::env::consts::OS.into())),
            "arch" => Some(Value::Text(std::env::consts::ARCH.into())),
            "family" => Some(Value::Text(std::env::consts::FAMILY.into())),
            _ => None,
        }
    }

    fn call(&self, name: &str, args: Vec<Value>) -> Result<Value, String> {
        if !args.is_empty() {
            return Err(format!("base.{}() takes no arguments", name));
        }
        match name {
            "hostname" => {
                #[cfg(unix)]
                {
                    let out = std::process::Command::new("hostname")
                        .output()
                        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
                        .unwrap_or_else(|_| "unknown".into());
                    Ok(Value::Text(out))
                }
                #[cfg(windows)]
                {
                    Ok(Value::Text(
                        std::env::var("COMPUTERNAME").unwrap_or_else(|_| "unknown".into()),
                    ))
                }
                #[cfg(not(any(unix, windows)))]
                {
                    Ok(Value::Text("unknown".into()))
                }
            }
            "username" => {
                Ok(Value::Text(
                    std::env::var("USER")
                        .or_else(|_| std::env::var("USERNAME"))
                        .unwrap_or_else(|_| "unknown".into()),
                ))
            }
            "home_dir" => {
                Ok(Value::Text(
                    std::env::var("HOME")
                        .or_else(|_| std::env::var("USERPROFILE"))
                        .unwrap_or_else(|_| ".".into()),
                ))
            }
            "exe_path" => std::env::current_exe()
                .map(|p| Value::Text(p.to_string_lossy().to_string()))
                .map_err(|e| format!("base.exe_path: {}", e)),
            "pid" => Ok(Value::Num(std::process::id() as f64)),
            "cores" => Ok(Value::Num(
                std::thread::available_parallelism()
                    .map(|n| n.get() as f64)
                    .unwrap_or(1.0),
            )),
            _ => Err(format!("base.{} is not a function", name)),
        }
    }
}