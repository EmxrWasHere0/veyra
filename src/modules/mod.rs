pub mod base;
pub mod io;
pub mod system;

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
    ]
}
