use crate::modules::VeyraModule;
use crate::runtime::Value;

pub struct BaseModule;

impl VeyraModule for BaseModule {
    fn name(&self) -> &str {
        "base"
    }

    fn call_func(&self, func: &str, _args: Vec<Value>) -> Result<Value, String> {
        match func {
            // ── Constants ──
            "os" => Ok(Value::Text(std::env::consts::OS.to_string())),
            "arch" => Ok(Value::Text(std::env::consts::ARCH.to_string())),
            "family" => Ok(Value::Text(std::env::consts::FAMILY.to_string())),

            // ── Functions ──
            "hostname" => {
                if cfg!(target_os = "windows") {
                    Ok(Value::Text(
                        std::env::var("COMPUTERNAME").unwrap_or_else(|_| "unknown".into()),
                    ))
                } else {
                    let output = std::process::Command::new("hostname")
                        .output()
                        .map_err(|e| format!("hostname error: {}", e))?;
                    Ok(Value::Text(
                        String::from_utf8_lossy(&output.stdout).trim().to_string(),
                    ))
                }
            }
            "username" => {
                let name = if cfg!(target_os = "windows") {
                    std::env::var("USERNAME").unwrap_or_else(|_| "unknown".into())
                } else {
                    std::env::var("USER").unwrap_or_else(|_| "unknown".into())
                };
                Ok(Value::Text(name))
            }
            "home_dir" => {
                let home = if cfg!(target_os = "windows") {
                    std::env::var("USERPROFILE").unwrap_or_else(|_| ".".into())
                } else {
                    std::env::var("HOME").unwrap_or_else(|_| "/".into())
                };
                Ok(Value::Text(home))
            }
            "exe_path" => std::env::current_exe()
                .map(|p| Value::Text(p.to_string_lossy().to_string()))
                .map_err(|e| format!("exe_path error: {}", e)),
            "pid" => Ok(Value::Num(std::process::id() as f64)),
            "cores" => {
                let n = std::thread::available_parallelism()
                    .map(|n| n.get() as f64)
                    .unwrap_or(1.0);
                Ok(Value::Num(n))
            }
            _ => Err(format!("base has no member '{}'", func)),
        }
    }

    fn list_members(&self) -> Vec<String> {
        vec![
            "os", "arch", "family", "hostname", "username", "home_dir", "exe_path", "pid", "cores",
        ]
        .into_iter()
        .map(String::from)
        .collect()
    }

    fn is_function(&self, name: &str) -> bool {
        matches!(
            name,
            "hostname" | "username" | "home_dir" | "exe_path" | "pid" | "cores"
        )
    }
}
