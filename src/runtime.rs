use std::collections::{HashMap, HashSet};
use std::fmt;
use std::rc::Rc;

use crate::lexer::{lex, toks_to_source, Tok};
use crate::{run, Line};

pub const LOOP_LIMIT: usize = 1_000_000;
pub const CALL_LIMIT: usize = 1_000;

/* ============================= VALUES ============================= */

#[derive(Clone, Debug)]
pub struct Param {
    pub name: String,
    pub default: Option<String>,
    pub variadic: bool,
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub params: Vec<Param>,
    pub body: Vec<Line>,
}

#[derive(Clone, Debug)]
pub enum Value {
    Num(f64),
    Text(String),
    Bool(bool),
    List(Vec<Value>),
    Func(Rc<Function>),
    Native(&'static str),
    Null,
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        use Value::*;
        match (self, other) {
            (Num(a), Num(b)) => a == b,
            (Text(a), Text(b)) => a == b,
            (Bool(a), Bool(b)) => a == b,
            (List(a), List(b)) => a == b,
            (Func(a), Func(b)) => Rc::ptr_eq(a, b),
            (Native(a), Native(b)) => a == b,
            (Null, Null) => true,
            _ => false,
        }
    }
}

impl Value {
    pub fn number(&self) -> Result<f64, String> {
        match self {
            Value::Num(n) => Ok(*n),
            other => Err(format!("Expected a number, found: {}", other)),
        }
    }

    pub fn truthy(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            Value::Num(n) => *n != 0.0,
            Value::Text(s) => !s.is_empty(),
            Value::List(items) => !items.is_empty(),
            Value::Func(_) | Value::Native(_) => true,
            Value::Null => false,
        }
    }

    pub fn index(&self) -> Result<usize, String> {
        let n = self.number()?;
        if n < 0.0 || n.fract() != 0.0 || n > usize::MAX as f64 {
            Err("List index must be a non-negative integer".into())
        } else {
            Ok(n as usize)
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Num(n) if n.fract() == 0.0 && n.is_finite() => write!(f, "{:.0}", n),
            Value::Num(n) => write!(f, "{}", n),
            Value::Text(s) => write!(f, "{}", s),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Null => write!(f, "null"),
            Value::Func(fun) => write!(f, "<func {}>", fun.name),
            Value::Native(n) => write!(f, "<native {}>", n),
            Value::List(items) => {
                let body: Vec<String> = items
                    .iter()
                    .map(|v| match v {
                        Value::Text(t) => format!("{:?}", t),
                        _ => v.to_string(),
                    })
                    .collect();
                write!(f, "[{}]", body.join(", "))
            }
        }
    }
}

pub fn binop(op: &str, a: Value, b: Value) -> Result<Value, String> {
    Ok(match op {
        "or" => Value::Bool(a.truthy() || b.truthy()),
        "and" => Value::Bool(a.truthy() && b.truthy()),
        "==" => Value::Bool(a == b),
        "!=" => Value::Bool(a != b),
        "<" => Value::Bool(a.number()? < b.number()?),
        ">" => Value::Bool(a.number()? > b.number()?),
        "<=" => Value::Bool(a.number()? <= b.number()?),
        ">=" => Value::Bool(a.number()? >= b.number()?),
        "-" => Value::Num(a.number()? - b.number()?),
        "*" => Value::Num(a.number()? * b.number()?),
        "+" => match (a, b) {
            (Value::Num(x), Value::Num(y)) => Value::Num(x + y),
            (Value::Text(x), y) => Value::Text(format!("{}{}", x, y)),
            (x, Value::Text(y)) => Value::Text(format!("{}{}", x, y)),
            _ => return Err("'+' requires compatible operands".into()),
        },
        "/" => {
            let d = b.number()?;
            if d == 0.0 {
                return Err("Division by zero".into());
            }
            Value::Num(a.number()? / d)
        }
        "%" => {
            let d = b.number()?;
            if d == 0.0 {
                return Err("Modulo by zero".into());
            }
            Value::Num(a.number()? % d)
        }
        _ => return Err(format!("Unsupported operator: {}", op)),
    })
}

/* =========================== BUILT-INS ============================ */

pub const NATIVES: [&str; 4] = ["len", "num", "text", "type"];

pub fn call_native(name: &str, args: Vec<Value>) -> Result<Value, String> {
    let need = |n: usize| -> Result<(), String> {
        if args.len() == n {
            Ok(())
        } else {
            Err(format!(
                "{}() takes {} argument(s), {} given",
                name,
                n,
                args.len()
            ))
        }
    };

    match name {
        "len" => {
            need(1)?;
            Ok(Value::Num(match &args[0] {
                Value::List(v) => v.len() as f64,
                Value::Text(s) => s.chars().count() as f64,
                other => {
                    return Err(format!(
                        "len requires a list or text, found: {}",
                        other
                    ))
                }
            }))
        }
        "num" => {
            need(1)?;
            Ok(match &args[0] {
                Value::Num(n) => Value::Num(*n),
                Value::Bool(b) => Value::Num(if *b { 1.0 } else { 0.0 }),
                Value::Text(s) => s
                    .trim()
                    .parse::<f64>()
                    .map(Value::Num)
                    .map_err(|_| format!("Cannot convert to number: {:?}", s))?,
                other => return Err(format!("Cannot convert to number: {}", other)),
            })
        }
        "text" => {
            need(1)?;
            Ok(Value::Text(args[0].to_string()))
        }
        "type" => {
            need(1)?;
            Ok(Value::Text(
                match &args[0] {
                    Value::Num(_) => "num",
                    Value::Text(_) => "text",
                    Value::Bool(_) => "bool",
                    Value::List(_) => "list",
                    Value::Func(_) | Value::Native(_) => "function",
                    Value::Null => "null",
                }
                .into(),
            ))
        }
        _ => Err(format!("Unknown built-in function: {}", name)),
    }
}

/* ============================ RUNTIME ============================= */

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Flow {
    Normal,
    Stop,
    Forward,
    Return,
}

pub struct Runtime {
    pub scopes: Vec<HashMap<String, Value>>,
    pub consts: Vec<HashSet<String>>,
    pub tags: Vec<HashMap<String, String>>,
    pub channels: HashMap<String, Value>,
    pub functions: HashMap<String, Rc<Function>>,
    pub call_depth: usize,
    pub return_value: Option<Value>,
    pub last_value: Option<Value>,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
            consts: vec![HashSet::new()],
            tags: vec![HashMap::new()],
            channels: HashMap::new(),
            functions: HashMap::new(),
            call_depth: 0,
            return_value: None,
            last_value: None,
        }
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
        self.consts.push(HashSet::new());
        self.tags.push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        self.scopes.pop();
        self.consts.pop();
        self.tags.pop();
    }

    pub fn is_const(&self, name: &str) -> bool {
        self.consts.iter().any(|s| s.contains(name))
    }

    pub fn tag_of(&self, name: &str) -> Option<String> {
        self.tags.iter().rev().find_map(|t| t.get(name)).cloned()
    }

    pub fn tag_as(&mut self, var: &str, channel: &str) {
        self.tags
            .last_mut()
            .unwrap()
            .insert(var.to_string(), channel.to_string());
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        if let Some(v) = self.scopes.iter().rev().find_map(|s| s.get(name)) {
            return Some(v.clone());
        }
        if let Some(f) = self.functions.get(name) {
            return Some(Value::Func(f.clone()));
        }
        NATIVES.iter().find(|n| **n == name).map(|n| Value::Native(n))
    }

    pub fn declare_const(&mut self, name: &str, value: Value) {
        self.scopes.last_mut().unwrap().insert(name.to_string(), value);
        self.consts.last_mut().unwrap().insert(name.to_string());
        self.tag_as(name, name);
    }

    pub fn assign(&mut self, name: &str, value: Value) -> Result<(), String> {
        if self.is_const(name) {
            let channel = self.tag_of(name).unwrap_or_else(|| name.to_string());
            self.channels.insert(channel, value);
            return Ok(());
        }
        for scope in self.scopes.iter_mut().rev() {
            if scope.contains_key(name) {
                scope.insert(name.to_string(), value);
                return Ok(());
            }
        }
        self.scopes.last_mut().unwrap().insert(name.to_string(), value);
        Ok(())
    }

    pub fn set_index(&mut self, name: &str, idx: usize, value: Value) -> Result<(), String> {
        if self.is_const(name) {
            return Err(format!("Cannot modify elements of constant '{}'", name));
        }
        for scope in self.scopes.iter_mut().rev() {
            if let Some(slot) = scope.get_mut(name) {
                return match slot {
                    Value::List(items) => {
                        let len = items.len();
                        *items.get_mut(idx).ok_or_else(|| {
                            format!("List index out of range: {} (length: {})", idx, len)
                        })? = value;
                        Ok(())
                    }
                    other => Err(format!("'{}' is not a list: {}", name, other)),
                };
            }
        }
        Err(format!("Undefined variable: {}", name))
    }

    pub fn eval(&mut self, src: &str) -> Result<Value, String> {
        if src.trim().is_empty() {
            return Err("Missing expression".into());
        }
        Parser::new(lex(src)?, self).parse()
    }

    pub fn call_tagged(
        &mut self,
        func: &Rc<Function>,
        args: Vec<(Value, Option<String>)>,
    ) -> Result<Value, String> {
        if self.call_depth >= CALL_LIMIT {
            return Err(format!("Call stack limit reached ({})", CALL_LIMIT));
        }

        let (values, arg_tags): (Vec<_>, Vec<_>) = args.into_iter().unzip();
        let planned = plan_values(func, values)?;
        let param_tags = plan_tags(func, arg_tags);

        let saved_scopes = std::mem::replace(&mut self.scopes, vec![HashMap::new()]);
        let saved_consts = std::mem::replace(&mut self.consts, vec![HashSet::new()]);
        let saved_tags = std::mem::replace(&mut self.tags, vec![HashMap::new()]);
        let saved_return = self.return_value.take();
        let saved_last = self.last_value.take();
        self.call_depth += 1;

        let result = call_inner(self, func, planned, param_tags);

        self.call_depth -= 1;
        self.last_value = saved_last;
        self.return_value = saved_return;
        self.tags = saved_tags;
        self.consts = saved_consts;
        self.scopes = saved_scopes;

        result
    }
}

fn call_inner(
    rt: &mut Runtime,
    func: &Rc<Function>,
    planned: Vec<Option<Value>>,
    param_tags: Vec<Option<String>>,
) -> Result<Value, String> {
    for ((p, slot), tag) in func.params.iter().zip(planned).zip(param_tags) {
        let value = match slot {
            Some(v) => v,
            None => match &p.default {
                Some(src) => rt.eval(src)?,
                None => {
                    return Err(format!(
                        "Missing parameter '{}' for function '{}'",
                        p.name, func.name
                    ))
                }
            },
        };

        rt.scopes.last_mut().unwrap().insert(p.name.clone(), value);

        if !p.variadic {
            if let Some(t) = tag {
                rt.tags.last_mut().unwrap().insert(p.name.clone(), t);
            }
        }
    }

    match run(&func.body, 0, func.body.len(), rt, false)? {
        Flow::Return => Ok(rt.return_value.take().unwrap_or(Value::Null)),
        Flow::Normal => Ok(rt.last_value.take().unwrap_or(Value::Null)),
        Flow::Stop | Flow::Forward => {
            Err("'stop'/'fwd' can only be used inside a loop".into())
        }
    }
}

fn split_three<T>(mut args: Vec<T>, before: usize, after: usize) -> (Vec<T>, Vec<T>, Vec<T>) {
    let n = args.len();
    let tail_take = after.min(n);
    let tail_start = n - tail_take;
    let head_take = before.min(tail_start);
    let tail = args.split_off(tail_start);
    let middle = args.split_off(head_take);
    (args, middle, tail)
}

fn plan_values(func: &Function, args: Vec<Value>) -> Result<Vec<Option<Value>>, String> {
    let params = &func.params;
    let n = args.len();
    let mut out: Vec<Option<Value>> = vec![None; params.len()];

    match params.iter().position(|p| p.variadic) {
        None => {
            if n > params.len() {
                return Err(format!(
                    "'{}' takes at most {} arguments, {} given",
                    func.name,
                    params.len(),
                    n
                ));
            }
            for (i, a) in args.into_iter().enumerate() {
                out[i] = Some(a);
            }
        }
        Some(vp) => {
            let after = params.len() - vp - 1;
            let (head, middle, tail) = split_three(args, vp, after);
            for (i, a) in head.into_iter().enumerate() {
                out[i] = Some(a);
            }
            out[vp] = Some(Value::List(middle));
            let first = after - tail.len();
            for (k, a) in tail.into_iter().enumerate() {
                out[vp + 1 + first + k] = Some(a);
            }
        }
    }
    Ok(out)
}

fn plan_tags(func: &Function, args: Vec<Option<String>>) -> Vec<Option<String>> {
    let params = &func.params;
    let mut out: Vec<Option<String>> = vec![None; params.len()];

    match params.iter().position(|p| p.variadic) {
        None => {
            for (i, t) in args.into_iter().take(params.len()).enumerate() {
                out[i] = t;
            }
        }
        Some(vp) => {
            let after = params.len() - vp - 1;
            let (head, _mid, tail) = split_three(args, vp, after);
            for (i, t) in head.into_iter().enumerate() {
                out[i] = t;
            }
            let first = after - tail.len();
            for (k, t) in tail.into_iter().enumerate() {
                out[vp + 1 + first + k] = t;
            }
        }
    }
    out
}

/* ============================= PARSER ============================= */

pub struct Parser<'a> {
    toks: Vec<Tok>,
    pos: usize,
    rt: &'a mut Runtime,
}

impl<'a> Parser<'a> {
    pub fn new(toks: Vec<Tok>, rt: &'a mut Runtime) -> Self {
        Self { toks, pos: 0, rt }
    }

    fn peek(&self) -> &Tok {
        &self.toks[self.pos]
    }

    fn take(&mut self) -> Tok {
        let t = self.toks[self.pos].clone();
        if !matches!(t, Tok::Eof) {
            self.pos += 1;
        }
        t
    }

    fn prec(op: &str) -> u8 {
        match op {
            "or" => 1,
            "and" => 2,
            "==" | "!=" | "<" | ">" | "<=" | ">=" => 3,
            "+" | "-" => 4,
            "*" | "/" | "%" => 5,
            _ => 0,
        }
    }

    fn collect_tail(&mut self) -> Vec<Tok> {
        let start = self.pos;
        let mut dp = 0i32;
        let mut db = 0i32;

        loop {
            match self.toks[self.pos] {
                Tok::Eof => break,
                Tok::LParen => dp += 1,
                Tok::RParen => {
                    if dp == 0 {
                        break;
                    }
                    dp -= 1;
                }
                Tok::LBracket => db += 1,
                Tok::RBracket => {
                    if db == 0 {
                        break;
                    }
                    db -= 1;
                }
                Tok::Comma if dp == 0 && db == 0 => break,
                _ => {}
            }
            self.pos += 1;
        }

        self.toks[start..self.pos].to_vec()
    }

    pub fn parse_params(&mut self) -> Result<Vec<Param>, String> {
        if !matches!(self.take(), Tok::LParen) {
            return Err("Expected '('".into());
        }

        let mut params: Vec<Param> = Vec::new();
        let mut variadic_seen = false;

        if matches!(self.peek(), Tok::RParen) {
            self.take();
            return Ok(params);
        }

        loop {
            let is_variadic = if matches!(self.peek(), Tok::Spread) {
                self.take();
                if variadic_seen {
                    return Err("Only one '...args' parameter is allowed".into());
                }
                variadic_seen = true;
                true
            } else {
                false
            };

            let name = match self.take() {
                Tok::Id(n) => n,
                other => return Err(format!("Expected parameter name, found: {:?}", other)),
            };

            if params.iter().any(|p| p.name == name) {
                return Err(format!("Duplicate parameter name: {}", name));
            }

            let default = if matches!(self.peek(), Tok::Op(o) if o == "=") {
                if is_variadic {
                    return Err("Variadic parameter cannot have a default value".into());
                }
                self.take();
                let toks = self.collect_tail();
                if toks.is_empty() {
                    return Err(format!("Missing default value for '{}'", name));
                }
                Some(toks_to_source(&toks))
            } else {
                None
            };

            params.push(Param { name, default, variadic: is_variadic });

            match self.take() {
                Tok::Comma => continue,
                Tok::RParen => break,
                other => return Err(format!("Expected ',' or ')', found: {:?}", other)),
            }
        }

        Ok(params)
    }

    pub fn parse(mut self) -> Result<Value, String> {
        let v = self.expr(1)?;
        if !matches!(self.peek(), Tok::Eof) {
            return Err(format!("Unexpected expression: {:?}", self.peek()));
        }
        Ok(v)
    }

    fn expr_tagged(&mut self, min_prec: u8) -> Result<(Value, Option<String>), String> {
        let hint = if let Tok::Id(name) = self.peek() {
            let n = name.clone();
            let standalone = matches!(
                self.toks.get(self.pos + 1),
                Some(Tok::Comma) | Some(Tok::RParen) | Some(Tok::Eof)
            );
            if standalone {
                self.rt.tag_of(&n)
            } else {
                None
            }
        } else {
            None
        };

        Ok((self.expr(min_prec)?, hint))
    }

    fn expr(&mut self, min_prec: u8) -> Result<Value, String> {
        let mut left = self.primary()?;

        loop {
            if matches!(self.peek(), Tok::LParen) {
                left = self.finish_call(left)?;
                continue;
            }

            if matches!(self.peek(), Tok::LBracket) {
                left = self.finish_index(left)?;
                continue;
            }

            if matches!(self.peek(), Tok::Question) {
                left = self.finish_channel(left)?;
                continue;
            }

            let op = match self.peek() {
                Tok::Op(o) => o.clone(),
                _ => break,
            };

            let p = Self::prec(&op);
            if p == 0 || p < min_prec {
                break;
            }

            self.take();
            let right = self.expr(p + 1)?;
            left = binop(&op, left, right)?;
        }

        Ok(left)
    }

    fn primary(&mut self) -> Result<Value, String> {
        match self.take() {
            Tok::Val(v) => Ok(v),

            Tok::Id(name) => self
                .rt
                .get(&name)
                .ok_or_else(|| format!("Undefined variable: {}", name)),

            Tok::Op(op) if op == "-" => Ok(Value::Num(-self.expr(6)?.number()?)),
            Tok::Op(op) if op == "+" => Ok(Value::Num(self.expr(6)?.number()?)),
            Tok::Op(op) if op == "not" || op == "!" => Ok(Value::Bool(!self.expr(6)?.truthy())),

            Tok::FuncKw => self.parse_lambda(),

            Tok::LParen => {
                let v = self.expr(1)?;
                if !matches!(self.take(), Tok::RParen) {
                    return Err("Expected ')'".into());
                }
                Ok(v)
            }

            Tok::LBracket => self.parse_list(),

            other => Err(format!("Invalid expression: {:?}", other)),
        }
    }

    fn parse_lambda(&mut self) -> Result<Value, String> {
        let params = self.parse_params()?;
        if !matches!(self.take(), Tok::Arrow) {
            return Err("Expected '=>'".into());
        }
        let body_toks = self.collect_tail();
        if body_toks.is_empty() {
            return Err("Empty lambda body".into());
        }
        let body_src = toks_to_source(&body_toks);
        let body = vec![Line { number: 0, text: format!("return {}", body_src) }];
        Ok(Value::Func(Rc::new(Function {
            name: "lambda".into(),
            params,
            body,
        })))
    }

    fn parse_list(&mut self) -> Result<Value, String> {
        let mut items = Vec::new();
        while !matches!(self.peek(), Tok::RBracket) {
            items.push(self.expr(1)?);
            if matches!(self.peek(), Tok::Comma) {
                self.take();
            } else {
                break;
            }
        }
        if !matches!(self.take(), Tok::RBracket) {
            return Err("Expected ']' for list".into());
        }
        Ok(Value::List(items))
    }

    fn finish_call(&mut self, callee: Value) -> Result<Value, String> {
        self.take(); // '('
        let mut args: Vec<(Value, Option<String>)> = Vec::new();
        if !matches!(self.peek(), Tok::RParen) {
            loop {
                args.push(self.expr_tagged(1)?);
                if matches!(self.peek(), Tok::Comma) {
                    self.take();
                } else {
                    break;
                }
            }
        }
        if !matches!(self.take(), Tok::RParen) {
            return Err("Expected ')' for call".into());
        }

        match callee {
            Value::Func(f) => self.rt.call_tagged(&f, args),
            Value::Native(n) => call_native(n, args.into_iter().map(|(v, _)| v).collect()),
            other => Err(format!("Not callable: {}", other)),
        }
    }

    fn finish_index(&mut self, target: Value) -> Result<Value, String> {
        self.take(); // '['
        let idx = self.expr(1)?.index()?;
        if !matches!(self.take(), Tok::RBracket) {
            return Err("Expected ']' for index".into());
        }

        match target {
            Value::List(items) => {
                let len = items.len();
                items.into_iter().nth(idx).ok_or_else(|| {
                    format!("List index out of range: {} (length: {})", idx, len)
                })
            }
            other => Err(format!("Indexing requires a list, found: {}", other)),
        }
    }

    fn finish_channel(&mut self, default: Value) -> Result<Value, String> {
        self.take(); // '?'
        let written = match self.take() {
            Tok::Id(n) => n,
            other => return Err(format!("Expected channel name, found: {:?}", other)),
        };
        let effective = self.rt.tag_of(&written).unwrap_or(written);
        Ok(self.rt.channels.get(&effective).cloned().unwrap_or(default))
    }
}