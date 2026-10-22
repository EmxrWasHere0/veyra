use crate::modules::VeyraModule;
use crate::runtime::Value;

use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};

use std::sync::Mutex;

pub struct RandomModule {
    rng: Mutex<StdRng>,
}

impl RandomModule {
    pub fn new() -> Self {
        Self {
            rng: Mutex::new(StdRng::from_entropy()),
        }
    }
}

fn expect_count(args: &[Value], expected: usize, func: &str) -> Result<(), String> {
    if args.len() != expected {
        return Err(format!("random.{} expects {} argument(s)", func, expected));
    }

    Ok(())
}

fn number(value: &Value, func: &str) -> Result<f64, String> {
    match value {
        Value::Num(n) if n.is_finite() => Ok(*n),
        _ => Err(format!("random.{} expects a finite number", func)),
    }
}

fn integer(value: &Value, func: &str) -> Result<i64, String> {
    let n = number(value, func)?;

    if n.fract() != 0.0 || n < i64::MIN as f64 || n >= i64::MAX as f64 {
        return Err(format!("random.{} expects an integer", func));
    }

    Ok(n as i64)
}

impl VeyraModule for RandomModule {
    fn name(&self) -> &str {
        "random"
    }

    fn call_func(&self, func: &str, args: Vec<Value>) -> Result<Value, String> {
        let mut rng = self
            .rng
            .lock()
            .map_err(|_| "Random generator lock poisoned".to_string())?;

        match func {
            "seed" => {
                expect_count(&args, 1, func)?;

                let seed = integer(&args[0], func)? as u64;

                *rng = StdRng::seed_from_u64(seed);

                Ok(Value::Null)
            }

            "random" => {
                expect_count(&args, 0, func)?;

                Ok(Value::Num(rng.gen::<f64>()))
            }

            "randint" => {
                expect_count(&args, 2, func)?;

                let a = integer(&args[0], func)?;
                let b = integer(&args[1], func)?;

                if a > b {
                    return Err("random.randint minimum cannot exceed maximum".into());
                }

                Ok(Value::Num(rng.gen_range(a..=b) as f64))
            }

            "uniform" => {
                expect_count(&args, 2, func)?;

                let a = number(&args[0], func)?;
                let b = number(&args[1], func)?;

                if a > b {
                    return Err("random.uniform minimum cannot exceed maximum".into());
                }

                if a == b {
                    return Ok(Value::Num(a));
                }

                Ok(Value::Num(rng.gen_range(a..b)))
            }

            "boolean" => {
                expect_count(&args, 0, func)?;

                Ok(Value::Bool(rng.gen_bool(0.5)))
            }

            "choice" => {
                expect_count(&args, 1, func)?;

                let items = match &args[0] {
                    Value::List(items) => items,
                    _ => return Err("random.choice expects a list".into()),
                };

                items
                    .choose(&mut *rng)
                    .cloned()
                    .ok_or_else(|| "Cannot choose from an empty list".into())
            }

            "shuffle" => {
                expect_count(&args, 1, func)?;

                let mut items = match &args[0] {
                    Value::List(items) => items.clone(),
                    _ => return Err("random.shuffle expects a list".into()),
                };

                items.shuffle(&mut *rng);

                Ok(Value::List(items))
            }

            "sample" => {
                expect_count(&args, 2, func)?;

                let items = match &args[0] {
                    Value::List(items) => items,
                    _ => return Err("random.sample expects a list".into()),
                };

                let count = integer(&args[1], func)?;

                if count < 0 || count as usize > items.len() {
                    return Err("Sample size out of range".into());
                }

                let sampled = items
                    .choose_multiple(&mut *rng, count as usize)
                    .cloned()
                    .collect();

                Ok(Value::List(sampled))
            }

            _ => Err(format!("random has no member '{}'", func)),
        }
    }

    fn list_members(&self) -> Vec<String> {
        [
            "seed", "random", "randint", "uniform", "boolean", "choice", "shuffle", "sample",
        ]
        .iter()
        .map(|name| name.to_string())
        .collect()
    }

    fn is_function(&self, _name: &str) -> bool {
        true
    }
}
