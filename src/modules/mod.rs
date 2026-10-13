pub mod system;
pub mod io;
pub mod base;

use std::collections::HashMap;
use crate::runtime::Value;

/// Every built-in module implements this trait.
pub trait VeyraModule {
    fn name(&self) -> &str;

    /// List all available members (functions + constants).
    fn members(&self) -> Vec<String>;

    /// Get a constant value (non-callable member).
    fn get_value(&self, name: &str) -> Option<Value>;

    /// Call a function member.
    fn call(&self, name: &str, args: Vec<Value>) -> Result<Value, String>;

    /// Check whether a member is a function (true) or a constant (false).
    fn is_function(&self, name: &str) -> bool;
}

/// Returns all built-in modules, keyed by name.
pub fn builtin_modules() -> HashMap<String, Box<dyn VeyraModule>> {
    let mut map: HashMap<String, Box<dyn VeyraModule>> = HashMap::new();
    map.insert("system".into(), Box::new(system::SystemModule));
    map.insert("io".into(), Box::new(io::IoModule));
    map.insert("base".into(), Box::new(base::BaseModule));
    map
}