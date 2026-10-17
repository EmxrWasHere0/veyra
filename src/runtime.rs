use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::rc::Rc;

use crate::lexer::{Tok, Token};
use crate::modules::{builtin_modules, VeyraModule};

pub const CALL_LIMIT: usize = 1000;
pub const LOOP_LIMIT: usize = 1_000_000;
pub const NATIVES: [&str; 4] = ["len", "num", "text", "type"];

// ============================================================
// DATA TYPES
// ============================================================

#[derive(Clone, Debug)]
pub enum Value {
    Num(f64),
    Text(String),
    Bool(bool),
    List(Vec<Value>),
    Object(Vec<(String, Value)>),
    Func(Rc<Function>),
    Native(&'static str),
    Null,
}

#[derive(Clone, Debug)]
pub struct Param {
    pub name: String,
    pub default: Option<Vec<Token>>,
    pub variadic: bool,
}

#[derive(Clone, Debug)]
pub struct Line {
    pub tokens: Vec<Token>,
    pub source: Rc<String>,
    pub number: usize,
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct Function {
    pub name: String,
    pub params: Vec<Param>,
    pub body: Vec<Line>,
    pub is_closure: bool,
}

#[derive(Clone, Debug)]
pub enum Flow {
    Normal,
    Stop(Option<String>),
    Forward(Option<String>),
    Return,
}

#[derive(Clone, Debug)]
pub struct TestStats {
    pub passed: usize,
    pub failed: usize,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct VeyraMod {
    pub name: String,
    pub constants: HashMap<String, Value>,
    pub functions: HashMap<String, Rc<Function>>,
}

// ============================================================
// VALUE
// ============================================================

impl Value {
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Num(_) => "num",
            Value::Text(_) => "text",
            Value::Bool(_) => "bool",
            Value::List(_) => "list",
            Value::Func(_) | Value::Native(_) => "function",
            Value::Null => "null",
            Value::Object(_) => "object",
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Num(n) => *n != 0.0,
            Value::Text(s) => !s.is_empty(),
            Value::Bool(b) => *b,
            Value::List(items) => !items.is_empty(),
            Value::Func(_) | Value::Native(_) => true,
            Value::Null => false,
            Value::Object(items) => !items.is_empty(),
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Num(n) => {
                if n.is_finite()
                    && *n >= i64::MIN as f64
                    && *n < i64::MAX as f64
                    && n.fract() == 0.0
                {
                    write!(f, "{}", *n as i64)
                } else {
                    write!(f, "{}", n)
                }
            }

            Value::Text(s) => write!(f, "{}", s),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Null => write!(f, "null"),

            Value::Func(func) => write!(f, "<func {}>", func.name),
            Value::Native(name) => write!(f, "<native {}>", name),

            Value::List(items) => {
                write!(f, "[")?;

                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }

                    match item {
                        Value::Text(s) => write!(f, "\"{}\"", s)?,
                        other => write!(f, "{}", other)?,
                    }
                }

                write!(f, "]")
            }
            Value::Object(entries) => {
                write!(f, "{{")?;

                for (index, (key, value)) in entries.iter().enumerate() {
                    if index > 0 {
                        write!(f, ", ")?;
                    }

                    write!(f, "\"{}\": ", key)?;

                    match value {
                        Value::Text(text) => write!(f, "\"{}\"", text)?,
                        other => write!(f, "{}", other)?,
                    }
                }

                write!(f, "}}")
            }
        }
    }
}

// ============================================================
// RUNTIME
// ============================================================

pub struct Runtime {
    pub scopes: Vec<HashMap<String, Value>>,
    pub consts: Vec<HashSet<String>>,
    pub tags: Vec<HashMap<String, String>>,
    pub channels: HashMap<String, Value>,

    pub functions: HashMap<String, Rc<Function>>,

    pub call_depth: usize,
    pub return_value: Option<Value>,
    pub last_value: Option<Value>,

    pub available_modules: HashMap<String, Box<dyn VeyraModule>>,
    pub loaded_modules: HashSet<String>,
    pub imported_names: HashMap<String, (String, String)>,
    pub veyra_modules: HashMap<String, VeyraMod>,

    pub script_dir: String,
    pub test_mode: bool,
    pub test_stats: TestStats,

    pub active_module: Option<String>,

    // Module loading state.
    pub loading_modules: HashSet<String>,

    // Module-local private variables and dependencies.
    module_scopes: HashMap<String, HashMap<String, Value>>,
    module_imports: HashMap<String, HashMap<String, (String, String)>>,
    module_dependencies: HashMap<String, HashSet<String>>,

    // Function execution state.
    function_depth: usize,
    loop_depth: usize,
}

fn normalize_index(index: &Value, length: usize) -> Result<usize, String> {
    let number = match index {
        Value::Num(number) => *number,
        _ => return Err("Index must be a number".into()),
    };

    if !number.is_finite() || number.fract() != 0.0 {
        return Err("Index must be a finite integer".into());
    }

    if number < i64::MIN as f64 || number >= i64::MAX as f64 {
        return Err("Index is out of range".into());
    }

    let mut index = number as i64;

    if index < 0 {
        index += length as i64;
    }

    if index < 0 || index >= length as i64 {
        return Err("Index out of bounds".into());
    }

    Ok(index as usize)
}

fn values_equal(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Num(x), Value::Num(y)) => x == y,
        (Value::Text(x), Value::Text(y)) => x == y,
        (Value::Bool(x), Value::Bool(y)) => x == y,
        (Value::Null, Value::Null) => true,

        (Value::List(x), Value::List(y)) => {
            x.len() == y.len() && x.iter().zip(y.iter()).all(|(a, b)| values_equal(a, b))
        }

        (Value::Object(x), Value::Object(y)) => {
            x.len() == y.len()
                && x.iter().all(|(key, value)| {
                    y.iter().any(|(other_key, other_value)| {
                        key == other_key && values_equal(value, other_value)
                    })
                })
        }

        (Value::Func(x), Value::Func(y)) => Rc::ptr_eq(x, y),

        (Value::Native(x), Value::Native(y)) => x == y,

        _ => false,
    }
}

impl Runtime {
    pub fn new() -> Self {
        let mut runtime = Self {
            scopes: vec![HashMap::new()],
            consts: vec![HashSet::new()],
            tags: vec![HashMap::new()],
            channels: HashMap::new(),

            functions: HashMap::new(),

            call_depth: 0,
            return_value: None,
            last_value: None,

            available_modules: HashMap::new(),
            loaded_modules: HashSet::new(),
            imported_names: HashMap::new(),
            veyra_modules: HashMap::new(),

            script_dir: ".".into(),
            test_mode: false,

            test_stats: TestStats {
                passed: 0,
                failed: 0,
                errors: Vec::new(),
            },

            active_module: None,
            loading_modules: HashSet::new(),

            module_scopes: HashMap::new(),
            module_imports: HashMap::new(),
            module_dependencies: HashMap::new(),

            function_depth: 0,
            loop_depth: 0,
        };

        for (name, module) in builtin_modules() {
            runtime.available_modules.insert(name, module);
        }

        runtime
    }

    // ========================================================
    // SCOPES
    // ========================================================

    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
        self.consts.push(HashSet::new());
        self.tags.push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
            self.consts.pop();
            self.tags.pop();
        }
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        for scope in self.scopes.iter().rev() {
            if let Some(value) = scope.get(name) {
                return Some(value.clone());
            }
        }

        if let Some(module_name) = &self.active_module {
            if let Some(private) = self.module_scopes.get(module_name) {
                if let Some(value) = private.get(name) {
                    return Some(value.clone());
                }
            }

            if let Some(module) = self.veyra_modules.get(module_name) {
                if let Some(func) = module.functions.get(name) {
                    return Some(Value::Func(func.clone()));
                }

                if let Some(value) = module.constants.get(name) {
                    return Some(value.clone());
                }
            }
        } else if let Some(func) = self.functions.get(name) {
            return Some(Value::Func(func.clone()));
        }

        NATIVES
            .iter()
            .find(|native| **native == name)
            .map(|native| Value::Native(native))
    }

    pub fn is_const(&self, name: &str) -> bool {
        self.consts.iter().rev().any(|scope| scope.contains(name))
    }

    pub fn define_const(&mut self, name: &str, value: Value) {
        self.scopes
            .last_mut()
            .unwrap()
            .insert(name.to_string(), value.clone());

        self.consts.last_mut().unwrap().insert(name.to_string());

        self.channels.insert(name.to_string(), value);
    }

    pub fn define_var(&mut self, name: &str, value: Value) {
        self.scopes
            .last_mut()
            .unwrap()
            .insert(name.to_string(), value);
    }

    pub fn assign(&mut self, name: &str, value: Value) -> Result<(), String> {
        for index in (0..self.scopes.len()).rev() {
            if self.scopes[index].contains_key(name) {
                if self.consts[index].contains(name) {
                    let channel = self.tags[index]
                        .get(name)
                        .cloned()
                        .unwrap_or_else(|| name.to_string());

                    self.channels.insert(channel, value);
                } else {
                    self.scopes[index].insert(name.to_string(), value);
                }

                return Ok(());
            }
        }

        if let Some(module_name) = &self.active_module {
            if let Some(private) = self.module_scopes.get_mut(module_name) {
                if private.contains_key(name) {
                    private.insert(name.to_string(), value);
                    return Ok(());
                }
            }

            if self
                .veyra_modules
                .get(module_name)
                .map(|module| module.constants.contains_key(name))
                .unwrap_or(false)
            {
                self.channels.insert(name.to_string(), value);
                return Ok(());
            }
        }

        Err(format!("Undefined variable '{}'", name))
    }

    // ========================================================
    // EXPRESSIONS
    // ========================================================

    pub fn eval(&mut self, tokens: &[Token]) -> Result<Value, String> {
        if tokens.is_empty() {
            return Ok(Value::Null);
        }

        let mut position = 0;
        let result = self.expr(tokens, &mut position, 0)?;

        if position < tokens.len() {
            return Err(format!("Unexpected token '{}'", tokens[position].text));
        }

        Ok(result)
    }

    fn postfix(&mut self, tokens: &[Token], position: &mut usize) -> Result<Value, String> {
        let mut value = self.primary(tokens, position)?;

        while *position < tokens.len() {
            if tokens[*position].kind != Tok::LBracket {
                break;
            }

            *position += 1;

            let index = self.expr(tokens, position, 0)?;

            expect_token(tokens, position, Tok::RBracket)?;

            value = match value {
                Value::List(items) => {
                    let index = normalize_index(&index, items.len())?;

                    items[index].clone()
                }

                Value::Text(text) => {
                    let chars: Vec<char> = text.chars().collect();

                    let index = normalize_index(&index, chars.len())?;

                    Value::Text(chars[index].to_string())
                }

                _ => {
                    return Err("Indexing requires a list or text".into());
                }
            };
        }

        Ok(value)
    }

    fn expr(
        &mut self,
        tokens: &[Token],
        position: &mut usize,
        min_precedence: u8,
    ) -> Result<Value, String> {
        let mut left = self.unary(tokens, position)?;

        while *position < tokens.len() {
            let token = &tokens[*position];

            if token.kind == Tok::Question {
                if 1 < min_precedence {
                    break;
                }

                *position += 1;

                if *position >= tokens.len() || tokens[*position].kind != Tok::Id {
                    return Err("Expected channel name after '?'".into());
                }

                let name = tokens[*position].text.clone();
                *position += 1;

                let channel_name = self.lookup_tag(&name).unwrap_or(name);

                left = self.channels.get(&channel_name).cloned().unwrap_or(left);

                continue;
            }

            let op = token.text.as_str();
            let precedence = precedence(op);

            if precedence == 0 || precedence < min_precedence {
                break;
            }

            *position += 1;

            let right = self.expr(tokens, position, precedence + 1)?;

            if op == "??" {
                if matches!(left, Value::Null) {
                    left = right;
                }
            } else {
                left = self.binop(left, op, right)?;
            }
        }

        Ok(left)
    }

    fn unary(&mut self, tokens: &[Token], position: &mut usize) -> Result<Value, String> {
        if *position >= tokens.len() {
            return Err("Expected expression".into());
        }

        let token = &tokens[*position];

        if token.text == "-" || token.text == "+" {
            let op = token.text.clone();
            *position += 1;

            let value = self.unary(tokens, position)?;

            return match (op.as_str(), value) {
                ("-", Value::Num(n)) => Ok(Value::Num(-n)),
                ("+", Value::Num(n)) => Ok(Value::Num(n)),
                _ => Err("Unary operator requires num".into()),
            };
        }

        if token.text == "not" || token.text == "!" {
            *position += 1;

            let value = self.unary(tokens, position)?;

            return Ok(Value::Bool(!value.is_truthy()));
        }

        self.postfix(tokens, position)
    }

    fn call_list_method(
        &mut self,
        name: &str,
        method: &str,
        args: Vec<Value>,
    ) -> Result<Value, String> {
        let value = self
            .get(name)
            .ok_or_else(|| format!("Undefined '{}'", name))?;

        let mut items = match value {
            Value::List(items) => items,
            _ => return Err(format!("'{}' is not a list", name)),
        };

        match method {
            "len" => {
                if !args.is_empty() {
                    return Err("list.len expects no arguments".into());
                }

                Ok(Value::Num(items.len() as f64))
            }

            "append" => {
                if args.len() != 1 {
                    return Err("list.append expects 1 argument".into());
                }

                items.push(args[0].clone());

                self.assign(name, Value::List(items))?;

                Ok(Value::Null)
            }

            "pop" => {
                if args.len() > 1 {
                    return Err("list.pop expects 0 or 1 arguments".into());
                }

                if items.is_empty() {
                    return Err("Cannot pop from an empty list".into());
                }

                let index = if args.is_empty() {
                    items.len() - 1
                } else {
                    normalize_index(&args[0], items.len())?
                };

                let removed = items.remove(index);

                self.assign(name, Value::List(items))?;

                Ok(removed)
            }

            "index" => {
                if args.len() != 1 {
                    return Err("list.index expects 1 argument".into());
                }

                for (index, item) in items.iter().enumerate() {
                    if values_equal(item, &args[0]) {
                        return Ok(Value::Num(index as f64));
                    }
                }

                Ok(Value::Num(-1.0))
            }

            _ => Err(format!("Unknown list method '{}'", method)),
        }
    }

    fn primary(&mut self, tokens: &[Token], position: &mut usize) -> Result<Value, String> {
        if *position >= tokens.len() {
            return Err("Expected value".into());
        }

        let token = tokens[*position].clone();

        match token.kind {
            Tok::Val => {
                *position += 1;

                match token.text.as_str() {
                    "true" => return Ok(Value::Bool(true)),
                    "false" => return Ok(Value::Bool(false)),
                    "null" => return Ok(Value::Null),
                    _ => {}
                }

                let is_double_quoted = token.text.starts_with('"') && token.text.ends_with('"');

                let is_single_quoted = token.text.starts_with('\'') && token.text.ends_with('\'');

                if (is_double_quoted || is_single_quoted) && token.text.len() >= 2 {
                    let inner = &token.text[1..token.text.len() - 1];

                    return Ok(Value::Text(unescape_string(inner)));
                }

                if let Ok(number) = token.text.parse::<f64>() {
                    return Ok(Value::Num(number));
                }

                Err(format!("Invalid literal '{}'", token.text))
            }

            Tok::Id => {
                *position += 1;

                let name = token.text;

                // Module member access or list method.
                if *position < tokens.len() && tokens[*position].kind == Tok::Dot {
                    *position += 1;

                    if *position >= tokens.len() {
                        return Err("Expected member name".into());
                    }

                    let member = tokens[*position].text.clone();
                    *position += 1;

                    // List methods.
                    if let Some(Value::List(_)) = self.get(&name) {
                        expect_token(tokens, position, Tok::LParen)?;

                        let arguments = self.parse_call_args(tokens, position)?;

                        let values = arguments.into_iter().map(|argument| argument.0).collect();

                        return self.call_list_method(&name, &member, values);
                    }

                    // Existing module behavior.
                    if *position < tokens.len() && tokens[*position].kind == Tok::LParen {
                        *position += 1;

                        let arguments = self.parse_call_args(tokens, position)?;

                        let (values, tags): (Vec<_>, Vec<_>) = arguments.into_iter().unzip();

                        return self.module_call(&name, &member, values, tags);
                    }

                    return self.module_constant(&name, &member);
                }

                // Function call.
                if *position < tokens.len() && tokens[*position].kind == Tok::LParen {
                    *position += 1;

                    let arguments = self.parse_call_args(tokens, position)?;

                    if let Some((module_name, member)) = self.resolve_import(&name) {
                        return self.module_call(
                            &module_name,
                            &member,
                            arguments.iter().map(|a| a.0.clone()).collect(),
                            arguments.iter().map(|a| a.1.clone()).collect(),
                        );
                    }

                    match self.get(&name) {
                        Some(Value::Func(func)) => {
                            return self.call_tagged(&func, arguments);
                        }

                        Some(Value::Native(native)) => {
                            let values = arguments.into_iter().map(|argument| argument.0).collect();

                            return self.call_native(native, values);
                        }

                        Some(_) => {
                            return Err(format!("'{}' is not a function", name));
                        }

                        None => {
                            return Err(format!("Undefined function '{}'", name));
                        }
                    }
                }

                if let Some(value) = self.get(&name) {
                    return Ok(value);
                }

                if let Some((module_name, member)) = self.resolve_import(&name) {
                    return self.module_constant(&module_name, &member);
                }

                Err(format!("Undefined '{}'", name))
            }

            Tok::LParen => {
                *position += 1;

                let value = self.expr(tokens, position, 0)?;

                expect_token(tokens, position, Tok::RParen)?;

                Ok(value)
            }

            Tok::LBracket => {
                *position += 1;

                let mut items = Vec::new();

                while *position < tokens.len() && tokens[*position].kind != Tok::RBracket {
                    items.push(self.expr(tokens, position, 0)?);

                    if *position < tokens.len() && tokens[*position].kind == Tok::Comma {
                        *position += 1;
                    } else {
                        break;
                    }
                }

                expect_token(tokens, position, Tok::RBracket)?;

                Ok(Value::List(items))
            }

            Tok::FuncKw => {
                // Lambda: func(x, y) => x + y
                *position += 1;

                expect_token(tokens, position, Tok::LParen)?;

                let params = self.parse_params(tokens, position)?;

                expect_token(tokens, position, Tok::Arrow)?;

                let expression = tokens[*position..].to_vec();
                *position = tokens.len();

                if expression.is_empty() {
                    return Err("Lambda requires an expression".into());
                }

                let mut return_tokens = vec![Token {
                    kind: Tok::Id,
                    text: "return".into(),
                }];

                return_tokens.extend(expression);

                Ok(Value::Func(Rc::new(Function {
                    name: "<lambda>".into(),
                    params,
                    body: vec![Line {
                        tokens: return_tokens,
                        source: Rc::new("<lambda>".to_string()),
                        number: 1,
                    }],
                    is_closure: true,
                })))
            }

            _ => Err(format!("Unexpected token '{}'", token.text)),
        }
    }

    fn parse_call_args(
        &mut self,
        tokens: &[Token],
        position: &mut usize,
    ) -> Result<Vec<(Value, Option<String>)>, String> {
        let mut arguments = Vec::new();

        while *position < tokens.len() && tokens[*position].kind != Tok::RParen {
            let start = *position;

            let value = self.expr(tokens, position, 0)?;

            let tag = if *position == start + 1 && tokens[start].kind == Tok::Id {
                let name = &tokens[start].text;

                self.lookup_tag(name).or_else(|| {
                    if self.is_const(name) {
                        Some(name.clone())
                    } else {
                        None
                    }
                })
            } else {
                None
            };

            arguments.push((value, tag));

            if *position < tokens.len() && tokens[*position].kind == Tok::Comma {
                *position += 1;
            } else {
                break;
            }
        }

        expect_token(tokens, position, Tok::RParen)?;

        Ok(arguments)
    }

    fn parse_params(&self, tokens: &[Token], position: &mut usize) -> Result<Vec<Param>, String> {
        let mut params = Vec::new();
        let mut variadic_count = 0;

        while *position < tokens.len() && tokens[*position].kind != Tok::RParen {
            let variadic = if tokens[*position].kind == Tok::Spread {
                *position += 1;
                variadic_count += 1;
                true
            } else {
                false
            };

            if variadic_count > 1 {
                return Err("Only one variadic parameter is allowed".into());
            }

            if *position >= tokens.len() || tokens[*position].kind != Tok::Id {
                return Err("Expected parameter name".into());
            }

            let name = tokens[*position].text.clone();
            *position += 1;

            let mut default = None;

            if *position < tokens.len() && tokens[*position].text == "=" {
                *position += 1;

                let start = *position;
                let mut depth = 0usize;

                while *position < tokens.len() {
                    match tokens[*position].kind {
                        Tok::LParen | Tok::LBracket => depth += 1,

                        Tok::RParen | Tok::RBracket => {
                            if depth == 0 {
                                break;
                            }
                            depth -= 1;
                        }

                        Tok::Comma if depth == 0 => break,

                        _ => {}
                    }

                    *position += 1;
                }

                if start == *position {
                    return Err(format!("Missing default value for '{}'", name));
                }

                default = Some(tokens[start..*position].to_vec());
            }

            if variadic && default.is_some() {
                return Err("Variadic parameters cannot have defaults".into());
            }

            params.push(Param {
                name,
                default,
                variadic,
            });

            if *position < tokens.len() && tokens[*position].kind == Tok::Comma {
                *position += 1;
            } else {
                break;
            }
        }

        expect_token(tokens, position, Tok::RParen)?;

        Ok(params)
    }

    fn lookup_tag(&self, name: &str) -> Option<String> {
        for tags in self.tags.iter().rev() {
            if let Some(tag) = tags.get(name) {
                return Some(tag.clone());
            }
        }

        None
    }

    // ========================================================
    // OPERATORS
    // ========================================================

    fn binop(&self, left: Value, op: &str, right: Value) -> Result<Value, String> {
        match op {
            "+" => match (left, right) {
                (Value::Num(a), Value::Num(b)) => Ok(Value::Num(a + b)),

                (Value::Text(a), Value::Text(b)) => Ok(Value::Text(a + &b)),

                (Value::List(mut a), Value::List(b)) => {
                    a.extend(b);
                    Ok(Value::List(a))
                }

                _ => Err("Invalid operands for '+'".into()),
            },

            "-" => numeric_op(left, right, "-", |a, b| a - b),

            "*" => match (left, right) {
                (Value::Num(a), Value::Num(b)) => Ok(Value::Num(a * b)),

                (Value::Text(text), Value::Num(count)) => {
                    if !count.is_finite() || count < 0.0 || count > 1_000_000.0 {
                        return Err("Invalid text repetition count".into());
                    }

                    Ok(Value::Text(text.repeat(count as usize)))
                }

                _ => Err("Invalid operands for '*'".into()),
            },

            "/" => {
                if matches!(right, Value::Num(0.0)) {
                    return Err("Division by zero".into());
                }

                numeric_op(left, right, "/", |a, b| a / b)
            }

            "%" => {
                if matches!(right, Value::Num(0.0)) {
                    return Err("Modulo by zero".into());
                }

                numeric_op(left, right, "%", |a, b| a % b)
            }

            "==" => Ok(Value::Bool(values_equal(&left, &right))),
            "!=" => Ok(Value::Bool(!values_equal(&left, &right))),

            "<" | ">" | "<=" | ">=" => {
                let ordering = compare_values(&left, &right).ok_or_else(|| {
                    format!(
                        "Cannot compare {} and {}",
                        left.type_name(),
                        right.type_name()
                    )
                })?;

                let result = match op {
                    "<" => ordering.is_lt(),
                    ">" => ordering.is_gt(),
                    "<=" => ordering.is_le(),
                    ">=" => ordering.is_ge(),
                    _ => false,
                };

                Ok(Value::Bool(result))
            }

            "and" => Ok(Value::Bool(left.is_truthy() && right.is_truthy())),

            "or" => Ok(Value::Bool(left.is_truthy() || right.is_truthy())),

            _ => Err(format!("Unknown operator '{}'", op)),
        }
    }

    // ========================================================
    // NATIVE FUNCTIONS
    // ========================================================

    fn call_native(&self, name: &str, args: Vec<Value>) -> Result<Value, String> {
        if args.len() != 1 {
            return Err(format!("{}() requires exactly 1 argument", name));
        }

        let value = &args[0];

        match name {
            "len" => match value {
                Value::Text(text) => Ok(Value::Num(text.chars().count() as f64)),

                Value::List(items) => Ok(Value::Num(items.len() as f64)),

                _ => Err("len() requires text or list".into()),
            },

            "num" => match value {
                Value::Num(n) => Ok(Value::Num(*n)),

                Value::Text(text) => text
                    .parse::<f64>()
                    .map(Value::Num)
                    .map_err(|_| format!("Cannot convert '{}' to num", text)),

                Value::Bool(b) => Ok(Value::Num(if *b { 1.0 } else { 0.0 })),

                _ => Err("Cannot convert value to num".into()),
            },

            "text" => Ok(Value::Text(value.to_string())),

            "type" => Ok(Value::Text(value.type_name().to_string())),

            _ => Err(format!("Unknown native function '{}'", name)),
        }
    }

    // ========================================================
    // FUNCTION CALLS
    // ========================================================

    pub fn call_tagged(
        &mut self,
        func: &Rc<Function>,
        args: Vec<(Value, Option<String>)>,
    ) -> Result<Value, String> {
        if self.call_depth >= CALL_LIMIT {
            return Err(format!("Call stack limit reached ({})", CALL_LIMIT));
        }

        let (values, tags): (Vec<_>, Vec<_>) = args.into_iter().unzip();

        let planned_values = plan_values(func, values)?;
        let planned_tags = plan_tags(func, tags);

        // Functions have closed scopes.
        let saved_scopes = std::mem::replace(&mut self.scopes, vec![HashMap::new()]);

        let saved_consts = std::mem::replace(&mut self.consts, vec![HashSet::new()]);

        let saved_tags = std::mem::replace(&mut self.tags, vec![HashMap::new()]);

        let saved_return = self.return_value.take();
        let saved_last = self.last_value.take();
        let saved_loop_depth = self.loop_depth;

        self.loop_depth = 0;
        self.call_depth += 1;
        self.function_depth += 1;

        let result = call_inner(self, func, planned_values, planned_tags);

        self.function_depth -= 1;
        self.call_depth -= 1;
        self.loop_depth = saved_loop_depth;

        self.scopes = saved_scopes;
        self.consts = saved_consts;
        self.tags = saved_tags;

        self.return_value = saved_return;
        self.last_value = saved_last;

        result
    }

    pub fn module_call(
        &mut self,
        mod_name: &str,
        func_name: &str,
        args: Vec<Value>,
        arg_tags: Vec<Option<String>>,
    ) -> Result<Value, String> {
        if !self.module_is_accessible(mod_name) {
            return Err(format!("Module '{}' has not been imported", mod_name));
        }

        if let Some(module) = self.available_modules.get(mod_name) {
            if !module.is_function(func_name) {
                return Err(format!("'{}.{}' is not a function", mod_name, func_name));
            }

            return module.call_func(func_name, args);
        }

        let func = self
            .veyra_modules
            .get(mod_name)
            .and_then(|module| module.functions.get(func_name))
            .cloned();

        if let Some(func) = func {
            if func_name.starts_with('_') && self.active_module.as_deref() != Some(mod_name) {
                return Err(format!("'{}.{}' is private", mod_name, func_name));
            }

            let previous_module = self.active_module.clone();

            self.active_module = Some(mod_name.to_string());

            let arguments = args
                .into_iter()
                .enumerate()
                .map(|(index, value)| {
                    let tag = arg_tags.get(index).cloned().flatten();
                    (value, tag)
                })
                .collect();

            let result = self.call_tagged(&func, arguments);

            self.active_module = previous_module;

            return result;
        }

        if self.veyra_modules.contains_key(mod_name) {
            return Err(format!(
                "Module '{}' has no function '{}'",
                mod_name, func_name
            ));
        }

        Err(format!("Unknown module '{}'", mod_name))
    }

    fn module_constant(&self, mod_name: &str, member: &str) -> Result<Value, String> {
        if !self.module_is_accessible(mod_name) {
            return Err(format!("Module '{}' has not been imported", mod_name));
        }

        if let Some(module) = self.available_modules.get(mod_name) {
            if module.is_function(member) {
                return Err(format!("'{}.{}' requires ()", mod_name, member));
            }

            if !module.list_members().iter().any(|name| name == member) {
                return Err(format!("Module '{}' has no member '{}'", mod_name, member));
            }

            return module.call_func(member, Vec::new());
        }

        if let Some(module) = self.veyra_modules.get(mod_name) {
            if member.starts_with('_') && self.active_module.as_deref() != Some(mod_name) {
                return Err(format!("'{}.{}' is private", mod_name, member));
            }

            if let Some(value) = module.constants.get(member) {
                return Ok(value.clone());
            }

            if module.functions.contains_key(member) {
                return Err(format!("'{}.{}' requires ()", mod_name, member));
            }

            return Err(format!("Module '{}' has no member '{}'", mod_name, member));
        }

        Err(format!("Unknown module '{}'", mod_name))
    }

    // ========================================================
    // LINE EXECUTION
    // ========================================================

    pub fn run_line(&mut self, line: &Line) -> Result<Flow, String> {
        let tokens = &line.tokens;

        if tokens.is_empty() {
            return Ok(Flow::Normal);
        }

        let first = tokens[0].text.as_str();

        match first {
            "stop" => {
                if self.loop_depth == 0 {
                    return Err("stop used outside loop".into());
                }

                return Ok(Flow::Stop(parse_label(tokens)?));
            }

            "fwd" => {
                if self.loop_depth == 0 {
                    return Err("fwd used outside loop".into());
                }

                return Ok(Flow::Forward(parse_label(tokens)?));
            }

            "return" => {
                if self.function_depth == 0 {
                    return Err("return used outside function".into());
                }

                let value = self.eval_return_values(&tokens[1..])?;

                self.return_value = Some(value);

                return Ok(Flow::Return);
            }

            "say" => {
                let value = self.eval(&tokens[1..])?;

                println!("{}", value);

                self.last_value = Some(value);

                return Ok(Flow::Normal);
            }

            "get" => {
                if tokens.len() < 2 {
                    return Err("get requires a variable name".into());
                }

                let name = tokens[1].text.clone();

                let mut input = String::new();

                std::io::stdin()
                    .read_line(&mut input)
                    .map_err(|e| format!("Input error: {}", e))?;

                let input = input.trim_end_matches(['\r', '\n']).to_string();

                self.define_var(&name, Value::Text(input));

                return Ok(Flow::Normal);
            }

            "set" => {
                if tokens.len() < 4 || tokens[2].text != "=" {
                    return Err("Expected: set name = value".into());
                }

                let name = tokens[1].text.clone();
                let value = self.eval(&tokens[3..])?;

                self.define_const(&name, value);

                return Ok(Flow::Normal);
            }

            "add" => {
                if tokens.len() < 4 || tokens[2].text != "=" {
                    return Err("Expected: add name = value".into());
                }

                let name = tokens[1].text.clone();
                let value = self.eval(&tokens[3..])?;

                self.define_var(&name, value);

                return Ok(Flow::Normal);
            }

            "assert" => {
                let value = self.eval(&tokens[1..])?;

                if value.is_truthy() {
                    if self.test_mode {
                        self.test_stats.passed += 1;
                    }
                } else {
                    let expression = tokens[1..]
                        .iter()
                        .map(|token| token.text.as_str())
                        .collect::<Vec<_>>()
                        .join(" ");

                    let error = format!("Assertion failed: {}", expression);

                    if self.test_mode {
                        self.test_stats.failed += 1;
                        self.test_stats.errors.push(error);
                    } else {
                        return Err(error);
                    }
                }

                return Ok(Flow::Normal);
            }

            _ => {}
        }

        if tokens[0].kind == Tok::LBracket {
            return self.handle_destructure(tokens);
        }

        if tokens.len() >= 3 && tokens[0].kind == Tok::Id && tokens[1].kind == Tok::QuestionEq {
            let name = tokens[0].text.clone();

            let value = self.eval(&tokens[2..])?;

            if self.get(&name).is_none() {
                self.define_var(&name, value);
            }

            return Ok(Flow::Normal);
        }

        if tokens.len() >= 3 && tokens[0].kind == Tok::Id && tokens[1].text == "=" {
            let name = tokens[0].text.clone();

            let value = self.eval(&tokens[2..])?;

            self.assign(&name, value)?;

            return Ok(Flow::Normal);
        }

        let value = self.eval(tokens)?;

        self.last_value = Some(value);

        Ok(Flow::Normal)
    }

    fn eval_return_values(&mut self, tokens: &[Token]) -> Result<Value, String> {
        if tokens.is_empty() {
            return Ok(Value::Null);
        }

        let parts = split_top_level(tokens, Tok::Comma);

        let mut values = Vec::new();

        for part in parts {
            values.push(self.eval(part)?);
        }

        if values.len() == 1 {
            Ok(values.remove(0))
        } else {
            Ok(Value::List(values))
        }
    }

    fn handle_destructure(&mut self, tokens: &[Token]) -> Result<Flow, String> {
        let closing = tokens
            .iter()
            .position(|token| token.kind == Tok::RBracket)
            .ok_or("Missing ']' in destructuring")?;

        if closing + 1 >= tokens.len() || tokens[closing + 1].text != "=" {
            return Err("Expected '=' after destructuring pattern".into());
        }

        let value = self.eval(&tokens[closing + 2..])?;

        let items = match value {
            Value::List(items) => items,

            _ => {
                return Err("Destructuring requires a list".into());
            }
        };

        let mut names = Vec::new();
        let mut rest_name = None;

        let mut position = 1;

        while position < closing {
            if tokens[position].kind == Tok::Comma {
                position += 1;
                continue;
            }

            if tokens[position].kind == Tok::Spread {
                position += 1;

                if position >= closing {
                    return Err("Expected name after '...'".into());
                }

                rest_name = Some(tokens[position].text.clone());
            } else {
                names.push(tokens[position].text.clone());
            }

            position += 1;
        }

        for (index, name) in names.iter().enumerate() {
            let value = items.get(index).cloned().unwrap_or(Value::Null);

            self.define_var(name, value);
        }

        if let Some(name) = rest_name {
            let remaining = items.into_iter().skip(names.len()).collect();

            self.define_var(&name, Value::List(remaining));
        }

        Ok(Flow::Normal)
    }

    // ========================================================
    // BLOCK EXECUTION
    // ========================================================

    pub fn run(&mut self, lines: &[Line]) -> Result<Flow, String> {
        let mut position = 0;

        while position < lines.len() {
            let tokens = &lines[position].tokens;

            if tokens.is_empty() {
                position += 1;
                continue;
            }

            let first = tokens[0].text.as_str();

            match first {
                "if" => {
                    let (flow, next) = self
                        .handle_if(lines, position)
                        .map_err(|error| format_error(&lines[position], error))?;

                    position = next;

                    if !matches!(flow, Flow::Normal) {
                        return Ok(flow);
                    }
                }

                "repeat" => {
                    let (flow, next) = self
                        .handle_repeat(lines, position, None)
                        .map_err(|error| format_error(&lines[position], error))?;

                    position = next;

                    if !matches!(flow, Flow::Normal) {
                        return Ok(flow);
                    }
                }

                "while" => {
                    let (flow, next) = self
                        .handle_while(lines, position)
                        .map_err(|error| format_error(&lines[position], error))?;

                    position = next;

                    if !matches!(flow, Flow::Normal) {
                        return Ok(flow);
                    }
                }

                "try" => {
                    let (flow, next) = self
                        .handle_try(lines, position)
                        .map_err(|error| format_error(&lines[position], error))?;

                    position = next;

                    if !matches!(flow, Flow::Normal) {
                        return Ok(flow);
                    }
                }

                "add" | "func" if is_function_definition(tokens) => {
                    position = self
                        .handle_func_def(lines, position)
                        .map_err(|error| format_error(&lines[position], error))?;
                }

                "mod" => {
                    self.handle_mod(tokens)
                        .map_err(|error| format_error(&lines[position], error))?;

                    position += 1;
                }

                "test" => {
                    position = self
                        .handle_test(lines, position)
                        .map_err(|error| format_error(&lines[position], error))?;
                }

                "end" | "else" | "catch" => {
                    return Err(format_error(
                        &lines[position],
                        format!("Unexpected '{}'", first),
                    ));
                }

                _ if tokens[0].kind == Tok::At => {
                    let (flow, next) = self
                        .handle_labeled(lines, position)
                        .map_err(|error| format_error(&lines[position], error))?;

                    position = next;

                    if !matches!(flow, Flow::Normal) {
                        return Ok(flow);
                    }
                }

                _ => {
                    let flow = self
                        .run_line(&lines[position])
                        .map_err(|error| format_error(&lines[position], error))?;

                    if !matches!(flow, Flow::Normal) {
                        return Ok(flow);
                    }

                    position += 1;
                }
            }
        }

        Ok(Flow::Normal)
    }

    fn find_end(&self, lines: &[Line], start: usize) -> Result<usize, String> {
        let mut depth = 1usize;

        for index in start + 1..lines.len() {
            let tokens = &lines[index].tokens;

            if tokens.is_empty() {
                continue;
            }

            if is_block_start(tokens) {
                depth += 1;
            } else if tokens[0].text == "end" {
                depth -= 1;

                if depth == 0 {
                    return Ok(index);
                }
            }
        }

        Err(format!(
            "Missing 'end' for block starting at line {}",
            start + 1
        ))
    }

    fn find_branch(
        &self,
        lines: &[Line],
        start: usize,
        end: usize,
        keyword: &str,
    ) -> Option<usize> {
        let mut depth = 0usize;

        for index in start + 1..end {
            let tokens = &lines[index].tokens;

            if tokens.is_empty() {
                continue;
            }

            if is_block_start(tokens) {
                depth += 1;
            } else if tokens[0].text == "end" {
                depth = depth.saturating_sub(1);
            } else if depth == 0 && tokens[0].text == keyword {
                return Some(index);
            }
        }

        None
    }

    fn handle_if(&mut self, lines: &[Line], start: usize) -> Result<(Flow, usize), String> {
        let end = self.find_end(lines, start)?;

        let condition = self.eval(&lines[start].tokens[1..])?;

        let else_position = self.find_branch(lines, start, end, "else");

        let flow = if condition.is_truthy() {
            let body_end = else_position.unwrap_or(end);

            self.run(&lines[start + 1..body_end])?
        } else if let Some(else_index) = else_position {
            self.run(&lines[else_index + 1..end])?
        } else {
            Flow::Normal
        };

        Ok((flow, end + 1))
    }

    fn handle_repeat(
        &mut self,
        lines: &[Line],
        start: usize,
        label: Option<String>,
    ) -> Result<(Flow, usize), String> {
        let end = self.find_end(lines, start)?;

        let tokens = &lines[start].tokens;

        let expression_start = if tokens[0].kind == Tok::At { 3 } else { 1 };

        let value = self.eval(&tokens[expression_start..])?;

        let count = match value {
            Value::Num(n) if n.is_finite() && n >= 0.0 && n.fract() == 0.0 => n as usize,

            _ => return Err("repeat requires a non-negative integer".into()),
        };

        if count > LOOP_LIMIT {
            return Err(format!("Loop iteration limit reached ({})", LOOP_LIMIT));
        }

        let body = &lines[start + 1..end];

        self.loop_depth += 1;

        let result = (|| -> Result<Flow, String> {
            for _ in 0..count {
                let flow = self.run(body)?;

                match flow {
                    Flow::Normal => {}

                    Flow::Return => return Ok(Flow::Return),

                    Flow::Stop(None) => break,

                    Flow::Forward(None) => continue,

                    Flow::Stop(Some(target)) => {
                        if label.as_deref() == Some(target.as_str()) {
                            break;
                        }

                        return Ok(Flow::Stop(Some(target)));
                    }

                    Flow::Forward(Some(target)) => {
                        if label.as_deref() == Some(target.as_str()) {
                            continue;
                        }

                        return Ok(Flow::Forward(Some(target)));
                    }
                }
            }

            Ok(Flow::Normal)
        })();

        self.loop_depth -= 1;

        Ok((result?, end + 1))
    }

    fn handle_while(&mut self, lines: &[Line], start: usize) -> Result<(Flow, usize), String> {
        let end = self.find_end(lines, start)?;
        let body = &lines[start + 1..end];

        self.loop_depth += 1;

        let result = (|| -> Result<Flow, String> {
            let mut iterations = 0usize;

            loop {
                let condition = self.eval(&lines[start].tokens[1..])?;

                if !condition.is_truthy() {
                    break;
                }

                if iterations >= LOOP_LIMIT {
                    return Err(format!("Loop iteration limit reached ({})", LOOP_LIMIT));
                }

                iterations += 1;

                let flow = self.run(body)?;

                match flow {
                    Flow::Normal => {}
                    Flow::Stop(None) => break,
                    Flow::Forward(None) => continue,
                    other => return Ok(other),
                }
            }

            Ok(Flow::Normal)
        })();

        self.loop_depth -= 1;

        Ok((result?, end + 1))
    }

    fn handle_labeled(&mut self, lines: &[Line], start: usize) -> Result<(Flow, usize), String> {
        let tokens = &lines[start].tokens;

        if tokens.len() < 4 || tokens[1].kind != Tok::Id || tokens[2].text != "repeat" {
            return Err("Labels are only supported on repeat loops".into());
        }

        let label = tokens[1].text.clone();

        self.handle_repeat(lines, start, Some(label))
    }

    fn handle_try(&mut self, lines: &[Line], start: usize) -> Result<(Flow, usize), String> {
        let end = self.find_end(lines, start)?;

        let catch_position = self.find_branch(lines, start, end, "catch");

        let try_end = catch_position.unwrap_or(end);

        match self.run(&lines[start + 1..try_end]) {
            Ok(flow) => Ok((flow, end + 1)),

            Err(error) => {
                let catch_index = match catch_position {
                    Some(index) => index,
                    None => return Err(error),
                };

                self.push_scope();

                if let Some(token) = lines[catch_index].tokens.get(1) {
                    self.define_var(&token.text, Value::Text(error));
                }

                let result = self.run(&lines[catch_index + 1..end]);

                self.pop_scope();

                Ok((result?, end + 1))
            }
        }
    }

    fn handle_func_def(&mut self, lines: &[Line], start: usize) -> Result<usize, String> {
        let end = self.find_end(lines, start)?;

        let tokens = &lines[start].tokens;

        let name = tokens[1].text.clone();

        let mut position = 3;

        let params = self.parse_params(tokens, &mut position)?;

        let body = lines[start + 1..end].to_vec();

        let func = Rc::new(Function {
            name: name.clone(),
            params,
            body,
            is_closure: false,
        });

        self.functions.insert(name, func);

        Ok(end + 1)
    }

    fn handle_test(&mut self, lines: &[Line], start: usize) -> Result<usize, String> {
        let end = self.find_end(lines, start)?;

        if self.test_mode {
            self.run(&lines[start + 1..end])?;
        }

        Ok(end + 1)
    }

    // ========================================================
    // MODULE SYSTEM
    // ========================================================

    fn module_is_accessible(&self, name: &str) -> bool {
        if self.active_module.as_deref() == Some(name) {
            return true;
        }

        if let Some(current) = &self.active_module {
            return self
                .module_dependencies
                .get(current)
                .map(|dependencies| dependencies.contains(name))
                .unwrap_or(false);
        }

        self.loaded_modules.contains(name)
    }

    fn resolve_import(&self, name: &str) -> Option<(String, String)> {
        if let Some(current) = &self.active_module {
            return self
                .module_imports
                .get(current)
                .and_then(|imports| imports.get(name))
                .cloned();
        }

        self.imported_names.get(name).cloned()
    }

    fn handle_mod(&mut self, tokens: &[Token]) -> Result<(), String> {
        if tokens.len() < 2 || tokens[1].kind != Tok::Id {
            return Err("mod requires a module name".into());
        }

        let name = tokens[1].text.clone();

        let (style, members) = if tokens.len() >= 3 && tokens[2].text == "*" {
            ("star", Vec::new())
        } else if tokens.len() >= 3 && tokens[2].kind == Tok::LBracket {
            let members = tokens[3..]
                .iter()
                .filter(|token| token.kind == Tok::Id)
                .map(|token| token.text.clone())
                .collect();

            ("selective", members)
        } else {
            ("qualified", Vec::new())
        };

        if self.loading_modules.contains(&name) {
            return Err(format!("Circular module import detected: '{}'", name));
        }

        if !self.loaded_modules.contains(&name) {
            if self.available_modules.contains_key(&name) {
                self.loaded_modules.insert(name.clone());
            } else {
                self.load_veyra_module(&name)?;
            }
        }

        if let Some(current) = &self.active_module {
            self.module_dependencies
                .entry(current.clone())
                .or_default()
                .insert(name.clone());
        }

        self.setup_imports(&name, style, &members)
    }

    fn setup_imports(
        &mut self,
        module_name: &str,
        style: &str,
        members: &[String],
    ) -> Result<(), String> {
        let available = self.module_members(module_name)?;

        let selected: Vec<String> = match style {
            "star" => available,

            "selective" => {
                for member in members {
                    if !available.contains(member) {
                        return Err(format!(
                            "Module '{}' has no public member '{}'",
                            module_name, member
                        ));
                    }
                }

                members.to_vec()
            }

            _ => Vec::new(),
        };

        for member in selected {
            let destination = if let Some(current) = &self.active_module {
                self.module_imports.entry(current.clone()).or_default()
            } else {
                &mut self.imported_names
            };

            destination.insert(member.clone(), (module_name.to_string(), member));
        }

        Ok(())
    }

    fn module_members(&self, name: &str) -> Result<Vec<String>, String> {
        if let Some(module) = self.available_modules.get(name) {
            return Ok(module.list_members());
        }

        if let Some(module) = self.veyra_modules.get(name) {
            let mut members = Vec::new();

            for name in module.constants.keys() {
                if !name.starts_with('_') {
                    members.push(name.clone());
                }
            }

            for name in module.functions.keys() {
                if !name.starts_with('_') {
                    members.push(name.clone());
                }
            }

            members.sort();
            members.dedup();

            return Ok(members);
        }

        Err(format!("Unknown module '{}'", name))
    }

    fn find_module_file(&self, name: &str) -> Result<PathBuf, String> {
        let filename = format!("{}.veymod", name);

        let mut directories = vec![
            PathBuf::from(&self.script_dir),
            PathBuf::from(&self.script_dir).join("lib"),
            PathBuf::from("."),
            PathBuf::from("./lib"),
        ];

        #[cfg(unix)]
        {
            directories.push(PathBuf::from("/usr/share/vey/libs"));
        }

        #[cfg(windows)]
        {
            if let Ok(program_files) = std::env::var("ProgramFiles") {
                directories.push(PathBuf::from(program_files).join("vey").join("libs"));
            }
        }

        for directory in directories {
            let path = directory.join(&filename);

            if path.is_file() {
                return Ok(path);
            }
        }

        Err(format!("Module '{}' not found", name))
    }

    fn load_veyra_module(&mut self, name: &str) -> Result<(), String> {
        if self.loading_modules.contains(name) {
            return Err(format!("Circular module import detected: '{}'", name));
        }

        let path = self.find_module_file(name)?;

        let source = std::fs::read_to_string(&path)
            .map_err(|error| format!("Cannot read module '{}': {}", path.display(), error))?;

        self.loading_modules.insert(name.to_string());

        // Module execution is isolated from the caller's variables.
        let saved_scopes = std::mem::replace(&mut self.scopes, vec![HashMap::new()]);

        let saved_consts = std::mem::replace(&mut self.consts, vec![HashSet::new()]);

        let saved_tags = std::mem::replace(&mut self.tags, vec![HashMap::new()]);

        let saved_functions = std::mem::take(&mut self.functions);

        let saved_imports = std::mem::take(&mut self.imported_names);

        let saved_module = self.active_module.take();

        self.active_module = Some(name.to_string());

        let saved_script_dir = self.script_dir.clone();

        let saved_return = self.return_value.take();
        let saved_last = self.last_value.take();

        if let Some(parent) = path.parent() {
            self.script_dir = parent.to_string_lossy().to_string();
        }

        // During initialization, module-local imports are collected
        // using the normal import table.
        let lines = crate::lexer::parse_lines(&source, &path.to_string_lossy())?;

        let result = self.run(&lines);

        let mut constants = HashMap::new();
        let mut private = HashMap::new();

        if let Some(scope) = self.scopes.first() {
            let consts = self.consts.first().cloned().unwrap_or_default();

            for (key, value) in scope {
                if consts.contains(key) {
                    constants.insert(key.clone(), value.clone());
                } else {
                    private.insert(key.clone(), value.clone());
                }
            }
        }

        let functions = std::mem::take(&mut self.functions);

        let imports = std::mem::take(&mut self.imported_names);

        self.scopes = saved_scopes;
        self.consts = saved_consts;
        self.tags = saved_tags;
        self.functions = saved_functions;
        self.imported_names = saved_imports;

        self.active_module = saved_module;
        self.script_dir = saved_script_dir;

        self.return_value = saved_return;
        self.last_value = saved_last;

        self.loading_modules.remove(name);

        result.map_err(|error| format!("{}: {}", path.display(), error))?;

        self.veyra_modules.insert(
            name.to_string(),
            VeyraMod {
                name: name.to_string(),
                constants,
                functions,
            },
        );

        self.module_scopes.insert(name.to_string(), private);

        self.module_imports
            .entry(name.to_string())
            .or_default()
            .extend(imports);

        self.loaded_modules.insert(name.to_string());

        Ok(())
    }

    #[allow(dead_code)]
    pub fn is_bare_import_function(&self, name: &str) -> bool {
        if let Some((module_name, member)) = self.resolve_import(name) {
            if let Some(module) = self.available_modules.get(&module_name) {
                return module.is_function(&member);
            }

            if let Some(module) = self.veyra_modules.get(&module_name) {
                return module.functions.contains_key(&member);
            }
        }

        false
    }
}

// ============================================================
// FUNCTION EXECUTION
// ============================================================

fn call_inner(
    runtime: &mut Runtime,
    func: &Rc<Function>,
    planned: Vec<Option<Value>>,
    tags: Vec<Option<String>>,
) -> Result<Value, String> {
    for ((param, slot), tag) in func.params.iter().zip(planned).zip(tags) {
        let value = match slot {
            Some(value) => value,

            None => match &param.default {
                Some(expression) => runtime.eval(expression)?,

                None => {
                    return Err(format!(
                        "Missing parameter '{}' for function '{}'",
                        param.name, func.name
                    ));
                }
            },
        };

        runtime.define_var(&param.name, value);

        if let Some(tag) = tag {
            runtime
                .tags
                .last_mut()
                .unwrap()
                .insert(param.name.clone(), tag);
        }
    }

    let flow = runtime.run(&func.body)?;

    match flow {
        Flow::Stop(Some(label)) => {
            return Err(format!("stop @{} used outside loop", label));
        }

        Flow::Stop(None) => {
            return Err("stop used outside loop".into());
        }

        Flow::Forward(Some(label)) => {
            return Err(format!("fwd @{} used outside loop", label));
        }

        Flow::Forward(None) => {
            return Err("fwd used outside loop".into());
        }

        _ => {}
    }

    // Explicit return takes priority over implicit return.
    if let Some(value) = runtime.return_value.take() {
        return Ok(value);
    }

    Ok(runtime.last_value.take().unwrap_or(Value::Null))
}

// ============================================================
// ARGUMENT PLANNING
// ============================================================

pub fn plan_values(func: &Function, args: Vec<Value>) -> Result<Vec<Option<Value>>, String> {
    let mut result = vec![None; func.params.len()];

    let variadic = func.params.iter().position(|param| param.variadic);

    if let Some(index) = variadic {
        let trailing = func.params.len() - index - 1;

        let prefix_count = index.min(args.len());

        for i in 0..prefix_count {
            result[i] = Some(args[i].clone());
        }

        let remaining = args.len().saturating_sub(prefix_count);

        let trailing_count = trailing.min(remaining);

        let variadic_end = args.len() - trailing_count;

        result[index] = Some(Value::List(args[prefix_count..variadic_end].to_vec()));

        for i in 0..trailing_count {
            let param_index = func.params.len() - trailing_count + i;

            let arg_index = args.len() - trailing_count + i;

            result[param_index] = Some(args[arg_index].clone());
        }
    } else {
        if args.len() > func.params.len() {
            return Err(format!("Too many arguments for function '{}'", func.name));
        }

        for (index, value) in args.into_iter().enumerate() {
            result[index] = Some(value);
        }
    }

    Ok(result)
}

pub fn plan_tags(func: &Function, tags: Vec<Option<String>>) -> Vec<Option<String>> {
    let mut result = vec![None; func.params.len()];

    let variadic = func.params.iter().position(|param| param.variadic);

    if let Some(index) = variadic {
        let trailing = func.params.len() - index - 1;

        let prefix_count = index.min(tags.len());

        for i in 0..prefix_count {
            result[i] = tags[i].clone();
        }

        let remaining = tags.len().saturating_sub(prefix_count);
        let trailing_count = trailing.min(remaining);

        for i in 0..trailing_count {
            let param_index = func.params.len() - trailing_count + i;

            let arg_index = tags.len() - trailing_count + i;

            result[param_index] = tags[arg_index].clone();
        }
    } else {
        for (index, tag) in tags.into_iter().enumerate() {
            if index >= result.len() {
                break;
            }

            result[index] = tag;
        }
    }

    result
}

// ============================================================
// HELPERS
// ============================================================

fn precedence(op: &str) -> u8 {
    match op {
        "??" => 1,
        "or" => 2,
        "and" => 3,

        "==" | "!=" | "<" | ">" | "<=" | ">=" => 4,

        "+" | "-" => 5,

        "*" | "/" | "%" => 6,

        _ => 0,
    }
}

fn numeric_op(
    left: Value,
    right: Value,
    op: &str,
    operation: impl FnOnce(f64, f64) -> f64,
) -> Result<Value, String> {
    match (left, right) {
        (Value::Num(a), Value::Num(b)) => Ok(Value::Num(operation(a, b))),

        _ => Err(format!("Invalid operands for '{}'", op)),
    }
}

fn compare_values(a: &Value, b: &Value) -> Option<std::cmp::Ordering> {
    match (a, b) {
        (Value::Num(x), Value::Num(y)) => x.partial_cmp(y),

        (Value::Text(x), Value::Text(y)) => Some(x.cmp(y)),

        _ => None,
    }
}

fn expect_token(tokens: &[Token], position: &mut usize, expected: Tok) -> Result<(), String> {
    if *position >= tokens.len() {
        return Err(format!("Expected {:?}, found end of expression", expected));
    }

    if tokens[*position].kind != expected {
        return Err(format!(
            "Expected {:?}, found '{}'",
            expected, tokens[*position].text
        ));
    }

    *position += 1;

    Ok(())
}

fn parse_label(tokens: &[Token]) -> Result<Option<String>, String> {
    if tokens.len() == 1 {
        return Ok(None);
    }

    if tokens.len() == 3 && tokens[1].kind == Tok::At && tokens[2].kind == Tok::Id {
        return Ok(Some(tokens[2].text.clone()));
    }

    Err("Invalid loop label syntax".into())
}

fn is_function_definition(tokens: &[Token]) -> bool {
    tokens.len() >= 3
        && (tokens[0].text == "add" || tokens[0].text == "func")
        && tokens[1].kind == Tok::Id
        && tokens[2].kind == Tok::LParen
}

fn is_block_start(tokens: &[Token]) -> bool {
    if tokens.is_empty() {
        return false;
    }

    if tokens[0].kind == Tok::At {
        return tokens.len() >= 3 && tokens[2].text == "repeat";
    }

    matches!(
        tokens[0].text.as_str(),
        "if" | "repeat" | "while" | "try" | "test"
    ) || is_function_definition(tokens)
}

fn split_top_level(tokens: &[Token], separator: Tok) -> Vec<&[Token]> {
    let mut parts = Vec::new();
    let mut start = 0;
    let mut depth = 0usize;

    for (index, token) in tokens.iter().enumerate() {
        match token.kind {
            Tok::LParen | Tok::LBracket => depth += 1,

            Tok::RParen | Tok::RBracket => {
                depth = depth.saturating_sub(1);
            }

            _ => {}
        }

        if token.kind == separator && depth == 0 {
            parts.push(&tokens[start..index]);
            start = index + 1;
        }
    }

    parts.push(&tokens[start..]);

    parts
}

fn unescape_string(text: &str) -> String {
    let mut result = String::new();
    let mut chars = text.chars();

    while let Some(ch) = chars.next() {
        if ch != '\\' {
            result.push(ch);
            continue;
        }

        match chars.next() {
            Some('n') => result.push('\n'),
            Some('r') => result.push('\r'),
            Some('t') => result.push('\t'),
            Some('"') => result.push('"'),
            Some('\\') => result.push('\\'),
            Some('\'') => result.push('\''),

            Some(other) => {
                result.push('\\');
                result.push(other);
            }

            None => result.push('\\'),
        }
    }

    result
}
fn format_error(line: &Line, error: String) -> String {
    // Errors already containing a source location are preserved.
    if error.contains(".vey:") || error.contains(".veymod:") {
        return error;
    }

    format!("{}:{}: {}", line.source, line.number, error)
}
