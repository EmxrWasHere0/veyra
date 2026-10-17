use std::collections::HashMap;
use std::io::ErrorKind;
use std::net::TcpStream;
use std::sync::Mutex;
use std::time::Duration;

use tungstenite::stream::MaybeTlsStream;
use tungstenite::{connect, Message, WebSocket};

use crate::modules::VeyraModule;
use crate::runtime::Value;

type Socket = WebSocket<MaybeTlsStream<TcpStream>>;

struct Connection {
    socket: Socket,
    open: bool,
}

struct State {
    next_id: u64,
    connections: HashMap<u64, Connection>,
}

pub struct WebSocketModule {
    state: Mutex<State>,
}

impl WebSocketModule {
    pub fn new() -> Self {
        Self {
            state: Mutex::new(State {
                next_id: 1,
                connections: HashMap::new(),
            }),
        }
    }

    fn with_connection<F>(
        &self,
        id: u64,
        operation: F,
    ) -> Result<Value, String>
    where
        F: FnOnce(&mut Connection) -> Result<Value, String>,
    {
        let mut state = self.state.lock()
            .map_err(|_| "WebSocket state lock failed")?;

        let connection = state.connections.get_mut(&id)
            .ok_or_else(|| {
                format!("Unknown WebSocket connection ID {}", id)
            })?;

        operation(connection)
    }
}

impl VeyraModule for WebSocketModule {
    fn name(&self) -> &str {
        "websocket"
    }

    fn call_func(
        &self,
        func: &str,
        args: Vec<Value>,
    ) -> Result<Value, String> {
        match func {
            "connect" => {
                require_args("connect", &args, 1)?;

                let url = expect_text(&args[0], "URL")?;

                let (socket, _) = connect(url.as_str())
                    .map_err(|e| {
                        format!("WebSocket connection failed: {}", e)
                    })?;

                let mut state = self.state.lock()
                    .map_err(|_| "WebSocket state lock failed")?;

                let id = state.next_id;
                state.next_id += 1;

                state.connections.insert(
                    id,
                    Connection {
                        socket,
                        open: true,
                    },
                );

                Ok(Value::Num(id as f64))
            }

            "send" => {
                require_args("send", &args, 2)?;

                let id = expect_id(&args[0])?;
                let text = expect_text(&args[1], "Message")?;

                self.with_connection(id, |connection| {
                    if !connection.open {
                        return Err("WebSocket connection is closed".into());
                    }

                    connection.socket
                        .send(Message::Text(text.into()))
                        .map_err(|e| {
                            format!("WebSocket send failed: {}", e)
                        })?;

                    Ok(Value::Null)
                })
            }

            "receive" => {
                require_args("receive", &args, 1)?;

                let id = expect_id(&args[0])?;

                self.with_connection(id, |connection| {
                    receive_message(connection, None)
                })
            }

            "receive_timeout" => {
                require_args("receive_timeout", &args, 2)?;

                let id = expect_id(&args[0])?;
                let timeout = expect_timeout(&args[1])?;

                self.with_connection(id, |connection| {
                    receive_message(connection, Some(timeout))
                })
            }

            "close" => {
                require_args("close", &args, 1)?;

                let id = expect_id(&args[0])?;

                let mut state = self.state.lock()
                    .map_err(|_| "WebSocket state lock failed")?;

                let mut connection = state.connections.remove(&id)
                    .ok_or_else(|| {
                        format!("Unknown WebSocket connection ID {}", id)
                    })?;

                connection.socket.close(None)
                    .map_err(|e| {
                        format!("WebSocket close failed: {}", e)
                    })?;

                Ok(Value::Null)
            }

            "is_open" => {
                require_args("is_open", &args, 1)?;

                let id = expect_id(&args[0])?;

                let state = self.state.lock()
                    .map_err(|_| "WebSocket state lock failed")?;

                Ok(Value::Bool(
                    state.connections
                        .get(&id)
                        .map(|connection| connection.open)
                        .unwrap_or(false)
                ))
            }

            _ => Err(format!(
                "websocket has no member '{}'",
                func
            )),
        }
    }

    fn list_members(&self) -> Vec<String> {
        vec![
            "connect",
            "send",
            "receive",
            "receive_timeout",
            "close",
            "is_open",
        ]
        .into_iter()
        .map(String::from)
        .collect()
    }

    fn is_function(&self, name: &str) -> bool {
        matches!(
            name,
            "connect"
                | "send"
                | "receive"
                | "receive_timeout"
                | "close"
                | "is_open"
        )
    }
}

// ============================================================
// RECEIVE
// ============================================================

fn receive_message(
    connection: &mut Connection,
    timeout: Option<Duration>,
) -> Result<Value, String> {
    set_socket_timeout(&mut connection.socket, timeout)?;

    loop {
        match connection.socket.read() {
            Ok(Message::Text(text)) => {
                return Ok(Value::Text(text.to_string()));
            }

            Ok(Message::Binary(bytes)) => {
                return Ok(Value::List(
                    bytes.iter()
                        .map(|byte| Value::Num(*byte as f64))
                        .collect()
                ));
            }

            Ok(Message::Ping(payload)) => {
                connection.socket
                    .send(Message::Pong(payload))
                    .map_err(|e| {
                        format!("WebSocket pong failed: {}", e)
                    })?;
            }

            Ok(Message::Pong(_)) => {
                // Pong is handled internally.
            }

            Ok(Message::Close(_)) => {
                connection.open = false;
                return Ok(Value::Null);
            }

            Ok(Message::Frame(_)) => {}

            Err(tungstenite::Error::Io(error))
                if error.kind() == ErrorKind::WouldBlock
                    || error.kind() == ErrorKind::TimedOut =>
            {
                return Ok(Value::Null);
            }

            Err(tungstenite::Error::ConnectionClosed)
            | Err(tungstenite::Error::AlreadyClosed) => {
                connection.open = false;
                return Ok(Value::Null);
            }

            Err(error) => {
                return Err(format!(
                    "WebSocket receive failed: {}",
                    error
                ));
            }
        }
    }
}

// ============================================================
// SOCKET TIMEOUT
// ============================================================

fn set_socket_timeout(
    socket: &mut Socket,
    timeout: Option<Duration>,
) -> Result<(), String> {
    let stream = socket.get_mut();

    match stream {
        MaybeTlsStream::Plain(tcp) => {
            tcp.set_read_timeout(timeout)
                .map_err(|e| {
                    format!("Cannot set WebSocket timeout: {}", e)
                })
        }

        MaybeTlsStream::Rustls(tls) => {
            tls.sock.set_read_timeout(timeout)
                .map_err(|e| {
                    format!("Cannot set WebSocket timeout: {}", e)
                })
        }

        _ => Err(
            "Unsupported WebSocket stream type".into()
        ),
    }
}

// ============================================================
// ARGUMENT VALIDATION
// ============================================================

fn require_args(
    name: &str,
    args: &[Value],
    expected: usize,
) -> Result<(), String> {
    if args.len() != expected {
        return Err(format!(
            "websocket.{}() requires {} argument(s)",
            name,
            expected
        ));
    }

    Ok(())
}

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

fn expect_id(value: &Value) -> Result<u64, String> {
    match value {
        Value::Num(number)
            if number.is_finite()
                && *number >= 1.0
                && number.fract() == 0.0 =>
        {
            Ok(*number as u64)
        }

        _ => Err(
            "WebSocket connection ID must be a positive integer".into()
        ),
    }
}

fn expect_timeout(value: &Value) -> Result<Duration, String> {
    match value {
        Value::Num(number)
            if number.is_finite()
                && *number >= 0.0
                && *number <= 3_600_000.0 =>
        {
            Ok(Duration::from_millis(*number as u64))
        }

        _ => Err(
            "Timeout must be between 0 and 3600000 milliseconds"
                .into()
        ),
    }
}