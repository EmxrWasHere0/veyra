pub mod base;
pub mod http;
pub mod io;
pub mod json;
pub mod system;
pub mod websocket;
pub mod math;
pub mod time;
pub mod fs;
pub mod random;

use crate::runtime::Value;

#[allow(dead_code)]
pub trait VeyraModule {
    fn name(&self) -> &str;
    fn call_func(&self, func: &str, args: Vec<Value>) -> Result<Value, String>;
    fn list_members(&self) -> Vec<String>;
    fn is_function(&self, name: &str) -> bool;
}

pub fn builtin_modules() -> Vec<(String, Box<dyn VeyraModule>)> {
    vec![
        (
            "system".into(),
            Box::new(system::SystemModule {
                script_args: Vec::new(),
            }),
        ),
        ("io".into(), Box::new(io::IoModule)),
        ("base".into(), Box::new(base::BaseModule)),
        ("json".into(), Box::new(json::JsonModule)),
        ("http".into(), Box::new(http::HttpModule)),
        (
            "websocket".into(),
            Box::new(websocket::WebSocketModule::new()),
        ),
        ("math".into(), Box::new(math::MathModule)),
        ("time".into(), Box::new(time::TimeModule)),
        ("fs".into(), Box::new(fs::FsModule)),
        ("random".into(), Box::new(random::RandomModule::new())),
    ]
}
