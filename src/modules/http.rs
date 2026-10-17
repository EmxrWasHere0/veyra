use std::time::Duration;

use crate::modules::VeyraModule;
use crate::runtime::Value;

pub struct HttpModule;

impl VeyraModule for HttpModule {
    fn name(&self) -> &str {
        "http"
    }

    fn call_func(
        &self,
        func: &str,
        args: Vec<Value>,
    ) -> Result<Value, String> {
        match func {
            "get" => {
                if args.len() < 1 || args.len() > 2 {
                    return Err(
                        "http.get() requires 1 or 2 arguments".into()
                    );
                }

                let url = expect_text(&args[0], "URL")?;

                let headers = if args.len() == 2 {
                    parse_headers(&args[1])?
                } else {
                    Vec::new()
                };

                send_request(
                    "GET",
                    &url,
                    None,
                    headers,
                )
            }

            "post" | "put" | "patch" => {
                if args.len() < 2 || args.len() > 3 {
                    return Err(format!(
                        "http.{}() requires 2 or 3 arguments",
                        func
                    ));
                }

                let url = expect_text(&args[0], "URL")?;

                let body = value_to_body(&args[1])?;

                let headers = if args.len() == 3 {
                    parse_headers(&args[2])?
                } else {
                    Vec::new()
                };

                send_request(
                    &func.to_uppercase(),
                    &url,
                    Some(body),
                    headers,
                )
            }

            "delete" => {
                if args.len() < 1 || args.len() > 2 {
                    return Err(
                        "http.delete() requires 1 or 2 arguments".into()
                    );
                }

                let url = expect_text(&args[0], "URL")?;

                let headers = if args.len() == 2 {
                    parse_headers(&args[1])?
                } else {
                    Vec::new()
                };

                send_request(
                    "DELETE",
                    &url,
                    None,
                    headers,
                )
            }

            "request" => {
                if args.len() != 4 {
                    return Err(
                        "http.request() requires 4 arguments".into()
                    );
                }

                let method = expect_text(&args[0], "method")?;
                let url = expect_text(&args[1], "URL")?;

                let body = if matches!(args[2], Value::Null) {
                    None
                } else {
                    Some(value_to_body(&args[2])?)
                };

                let headers = parse_headers(&args[3])?;

                send_request(
                    &method,
                    &url,
                    body,
                    headers,
                )
            }

            _ => Err(format!(
                "http has no member '{}'",
                func
            )),
        }
    }

    fn list_members(&self) -> Vec<String> {
        vec![
            "get",
            "post",
            "put",
            "patch",
            "delete",
            "request",
        ]
        .into_iter()
        .map(String::from)
        .collect()
    }

    fn is_function(&self, name: &str) -> bool {
        matches!(
            name,
            "get"
                | "post"
                | "put"
                | "patch"
                | "delete"
                | "request"
        )
    }
}

// ============================================================
// HTTP REQUEST
// ============================================================

fn send_request(
    method: &str,
    url: &str,
    body: Option<String>,
    headers: Vec<(String, String)>,
) -> Result<Value, String> {
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(30))
        .connect_timeout(Duration::from_secs(10))
        .user_agent(format!(
            "Veyra/{}",
            env!("CARGO_PKG_VERSION")
        ))
        .build()
        .map_err(|e| {
            format!("HTTP client error: {}", e)
        })?;

    let method = reqwest::Method::from_bytes(
        method.as_bytes()
    )
    .map_err(|e| {
        format!("Invalid HTTP method: {}", e)
    })?;

    let mut request = client.request(method, url);

    let mut has_content_type = false;

    for (name, value) in headers {
        if name.eq_ignore_ascii_case("content-type") {
            has_content_type = true;
        }

        let header_name =
            reqwest::header::HeaderName::from_bytes(
                name.as_bytes()
            )
            .map_err(|e| {
                format!("Invalid HTTP header name: {}", e)
            })?;

        let header_value =
            reqwest::header::HeaderValue::from_str(&value)
                .map_err(|e| {
                    format!("Invalid HTTP header value: {}", e)
                })?;

        request = request.header(
            header_name,
            header_value,
        );
    }

    if let Some(body) = body {
        if !has_content_type {
            request = request.header(
                reqwest::header::CONTENT_TYPE,
                "application/json",
            );
        }

        request = request.body(body);
    }

    let response = request
        .send()
        .map_err(|e| {
            format!("HTTP request failed: {}", e)
        })?;

    let status = response.status().as_u16();

    let response_headers = response
        .headers()
        .iter()
        .map(|(name, value)| {
            (
                name.as_str().to_string(),
                Value::Text(
                    String::from_utf8_lossy(
                        value.as_bytes()
                    )
                    .to_string()
                ),
            )
        })
        .collect();

    let body = response
        .text()
        .map_err(|e| {
            format!("HTTP response read error: {}", e)
        })?;

    Ok(Value::List(vec![
        Value::Num(status as f64),
        Value::Text(body),
        Value::Object(response_headers),
    ]))
}

// ============================================================
// HELPERS
// ============================================================

fn expect_text(
    value: &Value,
    name: &str,
) -> Result<String, String> {
    match value {
        Value::Text(text) => Ok(text.clone()),

        _ => Err(format!(
            "{} must be text",
            name
        )),
    }
}

fn parse_headers(
    value: &Value,
) -> Result<Vec<(String, String)>, String> {
    match value {
        Value::Null => Ok(Vec::new()),

        Value::Object(entries) => {
            let mut result = Vec::new();

            for (name, value) in entries {
                result.push((
                    name.clone(),
                    value.to_string(),
                ));
            }

            Ok(result)
        }

        Value::List(items) => {
            let mut result = Vec::new();

            for item in items {
                match item {
                    Value::List(pair) if pair.len() == 2 => {
                        let name = expect_text(
                            &pair[0],
                            "Header name",
                        )?;

                        let value = expect_text(
                            &pair[1],
                            "Header value",
                        )?;

                        result.push((name, value));
                    }

                    _ => {
                        return Err(
                            "Headers must contain [name, value] pairs"
                                .into()
                        );
                    }
                }
            }

            Ok(result)
        }

        _ => Err(
            "Headers must be an object or list of pairs".into()
        ),
    }
}

fn value_to_body(
    value: &Value,
) -> Result<String, String> {
    match value {
        Value::Text(text) => Ok(text.clone()),

        Value::Object(_) | Value::List(_) => {
            let json_value = to_json_body(value)?;

            serde_json::to_string(&json_value)
                .map_err(|e| {
                    format!("HTTP body serialization error: {}", e)
                })
        }

        Value::Null => Ok("null".into()),

        Value::Num(_) | Value::Bool(_) => {
            Ok(value.to_string())
        }

        _ => Err(
            "Cannot use function as HTTP body".into()
        ),
    }
}

fn to_json_body(
    value: &Value,
) -> Result<serde_json::Value, String> {
    match value {
        Value::Null => {
            Ok(serde_json::Value::Null)
        }

        Value::Bool(value) => {
            Ok(serde_json::Value::Bool(*value))
        }

        Value::Num(number) => {
            let number =
                serde_json::Number::from_f64(*number)
                    .ok_or(
                        "Cannot serialize non-finite number"
                    )?;

            Ok(serde_json::Value::Number(number))
        }

        Value::Text(text) => {
            Ok(serde_json::Value::String(
                text.clone()
            ))
        }

        Value::List(items) => {
            let mut result = Vec::new();

            for item in items {
                result.push(to_json_body(item)?);
            }

            Ok(serde_json::Value::Array(result))
        }

        Value::Object(entries) => {
            let mut result = serde_json::Map::new();

            for (key, value) in entries {
                result.insert(
                    key.clone(),
                    to_json_body(value)?,
                );
            }

            Ok(serde_json::Value::Object(result))
        }

        _ => Err(
            "Cannot serialize function to JSON".into()
        ),
    }
}