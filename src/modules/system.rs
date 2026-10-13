use crate::modules::VeyraModule;
use crate::runtime::Value;

pub struct SystemModule;

impl SystemModule {
    fn get_arch() -> String {
        std::env::consts::ARCH.to_string()
    }

    fn get_os() -> String {
        std::env::consts::OS.to_string()
    }

    fn get_family() -> String {
        std::env::consts::FAMILY.to_string()
    }
}

impl VeyraModule for SystemModule {
    fn name(&self) -> &str {
        "system"
    }

    fn members(&self) -> Vec<String> {
        vec![
            "arch".into(),
            "os".into(),
            "family".into(),
            "args".into(),
            "env".into(),
            "get_env".into(),
            "set_env".into(),
            "exit".into(),
            "version".into(),
        ]
    }

    fn is_function(&self, name: &str) -> bool {
        matches!(name, "args" | "env" | "get_env" | "set_env" | "exit")
    }

    fn get_value(&self, name: &str) -> Option<Value> {
        match name {
            "arch" => Some(Value::Text(Self::get_arch())),
            "os" => Some(Value::Text(Self::get_os())),
            "family" => Some(Value::Text(Self::get_family())),
            "version" => Some(Value::Text(env!("CARGO_PKG_VERSION").to_string())),
            _ => None,
        }
    }

    fn call(&self, name: &str, args: Vec<Value>) -> Result<Value, String> {
        match name {
            "args" => {
                if !args.is_empty() {
                    return Err("system.args() takes no arguments".into());
                }
                let argv: Vec<Value> = std::env::args()
                    .skip(2) // skip binary name and script name
                    .map(Value::Text)
                    .collect();
                Ok(Value::List(argv))
            }

            "env" => {
                if !args.is_empty() {
                    return Err("system.env() takes no arguments".into());
                }
                let pairs: Vec<Value> = std::env::vars()
                    .map(|(k, v)| Value::List(vec![Value::Text(k), Value::Text(v)]))
                    .collect();
                Ok(Value::List(pairs))
            }

            "get_env" => {
                if args.len() != 1 {
                    return Err("system.get_env(name) takes 1 argument".into());
                }
                let key = match &args[0] {
                    Value::Text(s) => s.clone(),
                    other => return Err(format!("system.get_env: expected text, found: {}", other)),
                };
                Ok(match std::env::var(&key) {
                    Ok(val) => Value::Text(val),
                    Err(_) => Value::Null,
                })
            }

            "set_env" => {
                if args.len() != 2 {
                    return Err("system.set_env(name, value) takes 2 arguments".into());
                }
                let key = match &args[0] {
                    Value::Text(s) => s.clone(),
                    other => {
                        return Err(format!(
                            "system.set_env: expected text for name, found: {}",
                            other
                        ))
                    }
                };
                let val = match &args[1] {
                    Value::Text(s) => s.clone(),
                    other => {
                        return Err(format!(
                            "system.set_env: expected text for value, found: {}",
                            other
                        ))
                    }
                };
                std::env::set_var(&key, &val);
                Ok(Value::Null)
            }

            "exit" => {
                let code = if args.is_empty() {
                    0
                } else if args.len() == 1 {
                    match &args[0] {
                        Value::Num(n) => *n as i32,
                        other => {
                            return Err(format!(
                                "system.exit: expected number, found: {}",
                                other
                            ))
                        }
                    }
                } else {
                    return Err("system.exit([code]) takes 0 or 1 arguments".into());
                };
                std::process::exit(code);
            }

            _ => Err(format!("system.{} is not a function", name)),
        }
    }
}