use crate::modules::VeyraModule;
use crate::runtime::Value;

pub struct JsonModule;

impl VeyraModule for JsonModule {
    fn name(&self) -> &str {
        "json"
    }

    fn call_func(&self, func: &str, args: Vec<Value>) -> Result<Value, String> {
        match func {
            "parse" => {
                require_args("json.parse", &args, 1)?;

                let text = match &args[0] {
                    Value::Text(text) => text,
                    _ => return Err("json.parse() requires text".into()),
                };

                let parsed: serde_json::Value =
                    serde_json::from_str(text).map_err(|e| format!("JSON parse error: {}", e))?;

                Ok(from_json(parsed))
            }

            "stringify" => {
                require_args("json.stringify", &args, 1)?;

                let json_value = to_json(&args[0])?;

                serde_json::to_string(&json_value)
                    .map(Value::Text)
                    .map_err(|e| format!("JSON stringify error: {}", e))
            }

            "get" => {
                require_args("json.get", &args, 2)?;

                let key = match &args[1] {
                    Value::Text(key) => key,
                    _ => return Err("json.get() key must be text".into()),
                };

                match &args[0] {
                    Value::Object(entries) => Ok(entries
                        .iter()
                        .find(|(name, _)| name == key)
                        .map(|(_, value)| value.clone())
                        .unwrap_or(Value::Null)),

                    _ => Err("json.get() requires an object".into()),
                }
            }

            "has" => {
                require_args("json.has", &args, 2)?;

                let key = match &args[1] {
                    Value::Text(key) => key,
                    _ => return Err("json.has() key must be text".into()),
                };

                match &args[0] {
                    Value::Object(entries) => {
                        Ok(Value::Bool(entries.iter().any(|(name, _)| name == key)))
                    }

                    _ => Err("json.has() requires an object".into()),
                }
            }

            "keys" => {
                require_args("json.keys", &args, 1)?;

                match &args[0] {
                    Value::Object(entries) => Ok(Value::List(
                        entries
                            .iter()
                            .map(|(key, _)| Value::Text(key.clone()))
                            .collect(),
                    )),

                    _ => Err("json.keys() requires an object".into()),
                }
            }

            "values" => {
                require_args("json.values", &args, 1)?;

                match &args[0] {
                    Value::Object(entries) => Ok(Value::List(
                        entries.iter().map(|(_, value)| value.clone()).collect(),
                    )),

                    _ => Err("json.values() requires an object".into()),
                }
            }

            _ => Err(format!("json has no member '{}'", func)),
        }
    }

    fn list_members(&self) -> Vec<String> {
        vec!["parse", "stringify", "get", "has", "keys", "values"]
            .into_iter()
            .map(String::from)
            .collect()
    }

    fn is_function(&self, name: &str) -> bool {
        matches!(
            name,
            "parse" | "stringify" | "get" | "has" | "keys" | "values"
        )
    }
}

fn require_args(name: &str, args: &[Value], expected: usize) -> Result<(), String> {
    if args.len() != expected {
        return Err(format!(
            "{}() requires {} argument(s), got {}",
            name,
            expected,
            args.len()
        ));
    }

    Ok(())
}

fn from_json(value: serde_json::Value) -> Value {
    match value {
        serde_json::Value::Null => Value::Null,

        serde_json::Value::Bool(value) => Value::Bool(value),

        serde_json::Value::Number(number) => Value::Num(number.as_f64().unwrap_or(0.0)),

        serde_json::Value::String(text) => Value::Text(text),

        serde_json::Value::Array(items) => Value::List(items.into_iter().map(from_json).collect()),

        serde_json::Value::Object(entries) => Value::Object(
            entries
                .into_iter()
                .map(|(key, value)| (key, from_json(value)))
                .collect(),
        ),
    }
}

fn to_json(value: &Value) -> Result<serde_json::Value, String> {
    match value {
        Value::Null => Ok(serde_json::Value::Null),

        Value::Bool(value) => Ok(serde_json::Value::Bool(*value)),

        Value::Num(number) => {
            let number = serde_json::Number::from_f64(*number)
                .ok_or_else(|| "Cannot serialize non-finite number".to_string())?;

            Ok(serde_json::Value::Number(number))
        }

        Value::Text(text) => Ok(serde_json::Value::String(text.clone())),

        Value::List(items) => {
            let mut result = Vec::new();

            for item in items {
                result.push(to_json(item)?);
            }

            Ok(serde_json::Value::Array(result))
        }

        Value::Object(entries) => {
            let mut result = serde_json::Map::new();

            for (key, value) in entries {
                result.insert(key.clone(), to_json(value)?);
            }

            Ok(serde_json::Value::Object(result))
        }

        Value::Func(_) | Value::Native(_) => Err("Cannot serialize a function to JSON".into()),
    }
}
