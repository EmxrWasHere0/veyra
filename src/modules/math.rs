use crate::modules::VeyraModule;
use crate::runtime::Value;

pub struct MathModule;

impl VeyraModule for MathModule {
    fn name(&self) -> &str {
        "math"
    }

    fn call_func(&self, func: &str, args: Vec<Value>) -> Result<Value, String> {
        match func {
            "PI" => Ok(Value::Num(std::f64::consts::PI)),
            "E" => Ok(Value::Num(std::f64::consts::E)),
            "TAU" => Ok(Value::Num(std::f64::consts::TAU)),
            "INF" => Ok(Value::Num(f64::INFINITY)),

            "abs" | "sqrt" | "floor" | "ceil" | "round" | "sin" | "cos" | "tan" | "asin"
            | "acos" | "atan" | "ln" | "log10" | "exp" => {
                if args.len() != 1 {
                    return Err(format!("math.{} expects 1 argument", func));
                }

                let n = match &args[0] {
                    Value::Num(n) => *n,
                    _ => return Err(format!("math.{} expects a number", func)),
                };

                let result = match func {
                    "abs" => n.abs(),
                    "sqrt" => n.sqrt(),
                    "floor" => n.floor(),
                    "ceil" => n.ceil(),
                    "round" => n.round(),
                    "sin" => n.sin(),
                    "cos" => n.cos(),
                    "tan" => n.tan(),
                    "asin" => n.asin(),
                    "acos" => n.acos(),
                    "atan" => n.atan(),
                    "ln" => n.ln(),
                    "log10" => n.log10(),
                    "exp" => n.exp(),
                    _ => unreachable!(),
                };

                Ok(Value::Num(result))
            }

            "pow" | "atan2" | "min" | "max" => {
                if args.len() != 2 {
                    return Err(format!("math.{} expects 2 arguments", func));
                }

                let a = match &args[0] {
                    Value::Num(n) => *n,
                    _ => return Err(format!("math.{} expects numbers", func)),
                };

                let b = match &args[1] {
                    Value::Num(n) => *n,
                    _ => return Err(format!("math.{} expects numbers", func)),
                };

                let result = match func {
                    "pow" => a.powf(b),
                    "atan2" => a.atan2(b),
                    "min" => a.min(b),
                    "max" => a.max(b),
                    _ => unreachable!(),
                };

                Ok(Value::Num(result))
            }

            "clamp" => {
                if args.len() != 3 {
                    return Err("math.clamp expects 3 arguments".into());
                }

                let mut numbers = Vec::new();

                for arg in args {
                    match arg {
                        Value::Num(n) => numbers.push(n),
                        _ => return Err("math.clamp expects numbers".into()),
                    }
                }

                let (value, minimum, maximum) = (numbers[0], numbers[1], numbers[2]);

                if minimum > maximum {
                    return Err("math.clamp minimum cannot exceed maximum".into());
                }

                Ok(Value::Num(value.clamp(minimum, maximum)))
            }

            _ => Err(format!("math has no member '{}'", func)),
        }
    }

    fn list_members(&self) -> Vec<String> {
        [
            "PI", "E", "TAU", "INF", "abs", "sqrt", "floor", "ceil", "round", "sin", "cos", "tan",
            "asin", "acos", "atan", "ln", "log10", "exp", "pow", "atan2", "min", "max", "clamp",
        ]
        .iter()
        .map(|name| name.to_string())
        .collect()
    }

    fn is_function(&self, name: &str) -> bool {
        !matches!(name, "PI" | "E" | "TAU" | "INF")
    }
}
