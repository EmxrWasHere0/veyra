use crate::modules::VeyraModule;
use crate::runtime::Value;

use chrono::{
    Datelike, Duration, Local, NaiveDate, NaiveDateTime, Timelike, Utc,
};

use std::sync::OnceLock;
use std::time::Instant;

pub struct TimeModule;

static START: OnceLock<Instant> = OnceLock::new();

fn start_instant() -> &'static Instant {
    START.get_or_init(Instant::now)
}

fn number(value: &Value, name: &str) -> Result<f64, String> {
    match value {
        Value::Num(n) if n.is_finite() => Ok(*n),
        _ => Err(format!("{} expects a finite number", name)),
    }
}

fn integer(value: &Value, name: &str) -> Result<i64, String> {
    let n = number(value, name)?;

    if n.fract() != 0.0 || n < i64::MIN as f64 || n >= i64::MAX as f64 {
        return Err(format!("{} expects an integer", name));
    }

    Ok(n as i64)
}

fn text<'a>(value: &'a Value, name: &str) -> Result<&'a str, String> {
    match value {
        Value::Text(s) => Ok(s),
        _ => Err(format!("{} expects text", name)),
    }
}

fn check_args(args: &[Value], expected: usize, name: &str) -> Result<(), String> {
    if args.len() != expected {
        return Err(format!("time.{} expects {} argument(s)", name, expected));
    }

    Ok(())
}

fn parse_datetime(value: &Value) -> Result<NaiveDateTime, String> {
    let s = text(value, "Datetime")?;

    NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S")
        .or_else(|_| {
            NaiveDate::parse_from_str(s, "%Y-%m-%d").map(|date| date.and_hms_opt(0, 0, 0).unwrap())
        })
        .map_err(|_| format!("Invalid datetime '{}'", s))
}

fn format_datetime(date: NaiveDateTime) -> String {
    date.format("%Y-%m-%d %H:%M:%S").to_string()
}

impl VeyraModule for TimeModule {
    fn name(&self) -> &str {
        "time"
    }

    fn call_func(&self, func: &str, args: Vec<Value>) -> Result<Value, String> {
        match func {
            "now" => {
                check_args(&args, 0, func)?;

                Ok(Value::Text(
                    Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                ))
            }

            "utcnow" => {
                check_args(&args, 0, func)?;

                Ok(Value::Text(
                    Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                ))
            }

            "timestamp" => {
                check_args(&args, 0, func)?;

                Ok(Value::Num(Utc::now().timestamp_millis() as f64 / 1000.0))
            }

            "date" => {
                check_args(&args, 3, func)?;

                let year = integer(&args[0], func)?;
                let month = integer(&args[1], func)?;
                let day = integer(&args[2], func)?;

                let date = NaiveDate::from_ymd_opt(year as i32, month as u32, day as u32)
                    .ok_or("Invalid date")?;

                Ok(Value::Text(date.format("%Y-%m-%d").to_string()))
            }

            "parse" => {
                check_args(&args, 2, func)?;

                let input = text(&args[0], func)?;
                let pattern = text(&args[1], func)?;

                let date = NaiveDateTime::parse_from_str(input, pattern)
                    .map_err(|e| format!("time.parse error: {}", e))?;

                Ok(Value::Text(format_datetime(date)))
            }

            "format" => {
                check_args(&args, 2, func)?;

                let date = parse_datetime(&args[0])?;
                let pattern = text(&args[1], func)?;

                Ok(Value::Text(date.format(pattern).to_string()))
            }

            "year" | "month" | "day" | "hour" | "minute" | "second" | "weekday" => {
                check_args(&args, 1, func)?;

                let date = parse_datetime(&args[0])?;

                let value = match func {
                    "year" => date.year() as f64,
                    "month" => date.month() as f64,
                    "day" => date.day() as f64,
                    "hour" => date.hour() as f64,
                    "minute" => date.minute() as f64,
                    "second" => date.second() as f64,
                    "weekday" => date.weekday().num_days_from_monday() as f64,
                    _ => unreachable!(),
                };

                Ok(Value::Num(value))
            }

            "add_days" | "add_hours" => {
                check_args(&args, 2, func)?;

                let date = parse_datetime(&args[0])?;
                let amount = integer(&args[1], func)?;

                let duration = if func == "add_days" {
                    Duration::try_days(amount)
                } else {
                    Duration::try_hours(amount)
                }
                .ok_or("Duration out of range")?;

                let result = date
                    .checked_add_signed(duration)
                    .ok_or("Datetime out of range")?;

                Ok(Value::Text(format_datetime(result)))
            }

            "diff" => {
                check_args(&args, 2, func)?;

                let a = parse_datetime(&args[0])?;
                let b = parse_datetime(&args[1])?;

                Ok(Value::Num((b - a).num_seconds() as f64))
            }

            "sleep" | "sleep_ms" => {
                check_args(&args, 1, func)?;

                let amount = number(&args[0], func)?;

                if amount < 0.0 {
                    return Err("Sleep duration cannot be negative".into());
                }

                let seconds = if func == "sleep_ms" {
                    amount / 1000.0
                } else {
                    amount
                };

                let duration = std::time::Duration::try_from_secs_f64(seconds)
                    .map_err(|_| "Sleep duration out of range")?;

                std::thread::sleep(duration);

                Ok(Value::Null)
            }

            "monotonic" => {
                check_args(&args, 0, func)?;

                Ok(Value::Num(start_instant().elapsed().as_secs_f64()))
            }

            "monotonic_ms" => {
                check_args(&args, 0, func)?;

                Ok(Value::Num(start_instant().elapsed().as_secs_f64() * 1000.0))
            }

            "is_leap" => {
                check_args(&args, 1, func)?;

                let year = integer(&args[0], func)?;

                let leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);

                Ok(Value::Bool(leap))
            }

            "days_in_month" => {
                check_args(&args, 2, func)?;

                let year = integer(&args[0], func)? as i32;
                let month = integer(&args[1], func)? as u32;

                let first =
                    NaiveDate::from_ymd_opt(year, month, 1).ok_or("Invalid year or month")?;

                let next = if month == 12 {
                    NaiveDate::from_ymd_opt(year + 1, 1, 1)
                } else {
                    NaiveDate::from_ymd_opt(year, month + 1, 1)
                }
                .ok_or("Invalid next month")?;

                Ok(Value::Num((next - first).num_days() as f64))
            }

            _ => Err(format!("time has no member '{}'", func)),
        }
    }

    fn list_members(&self) -> Vec<String> {
        [
            "now",
            "utcnow",
            "timestamp",
            "date",
            "parse",
            "format",
            "year",
            "month",
            "day",
            "hour",
            "minute",
            "second",
            "weekday",
            "add_days",
            "add_hours",
            "diff",
            "sleep",
            "sleep_ms",
            "monotonic",
            "monotonic_ms",
            "is_leap",
            "days_in_month",
        ]
        .iter()
        .map(|name| name.to_string())
        .collect()
    }

    fn is_function(&self, _name: &str) -> bool {
        true
    }
}
