use crate::modules::VeyraModule;
use crate::runtime::Value;

pub struct SystemModule;

impl VeyraModule for SystemModule {
    fn name(&self) -> &str {
        "system"
    }

    fn call_func(&self, func: &str, args: Vec<Value>) -> Result<Value, String> {
        match func {
            // ── Constants (called with no args) ──
            "arch" => Ok(Value::Text(std::env::consts::ARCH.to_string())),
            "os" => Ok(Value::Text(std::env::consts::OS.to_string())),
            "family" => Ok(Value::Text(std::env::consts::FAMILY.to_string())),
            "version" => Ok(Value::Text("0.1.0".to_string())),

            // ── Functions ──
            "args" => {
                let items: Vec<Value> = std::env::args().map(Value::Text).collect();
                Ok(Value::List(items))
            }
            "env" => {
                let mut items = Vec::new();
                for (k, v) in std::env::vars() {
                    items.push(Value::List(vec![Value::Text(k), Value::Text(v)]));
                }
                Ok(Value::List(items))
            }
            "get_env" => {
                if args.len() != 1 {
                    return Err("get_env() takes 1 argument".into());
                }
                let key = args[0].to_string();
                Ok(std::env::var(&key).map(Value::Text).unwrap_or(Value::Null))
            }
            "set_env" => {
                if args.len() != 2 {
                    return Err("set_env() takes 2 arguments".into());
                }
                let key = args[0].to_string();
                let val = args[1].to_string();
                std::env::set_var(&key, &val);
                Ok(Value::Null)
            }
            "exit" => {
                let code = if args.is_empty() {
                    0
                } else {
                    match &args[0] {
                        Value::Num(n) => *n as i32,
                        _ => 0,
                    }
                };
                std::process::exit(code);
            }
            _ => Err(format!("system has no member '{}'", func)),
        }
    }

    fn list_members(&self) -> Vec<String> {
        vec![
            "arch", "os", "family", "version", "args", "env", "get_env", "set_env", "exit",
        ]
        .into_iter()
        .map(String::from)
        .collect()
    }

    fn is_function(&self, name: &str) -> bool {
        matches!(name, "args" | "env" | "get_env" | "set_env" | "exit")
    }
}