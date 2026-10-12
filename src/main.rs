use std::collections::{HashMap, HashSet};
use std::fmt;
use std::io::{self, Write};
use std::rc::Rc;

const LOOP_LIMIT: usize = 1_000_000;
const CALL_LIMIT: usize = 1_000;

/* ============================ DEĞERLER ============================ */

#[derive(Clone, Debug)]
struct Param {
    name: String,
    default: Option<String>, // kaynak metni; her çağrıda yeniden hesaplanır
    variadic: bool,
}

#[derive(Debug)]
struct Function {
    name: String,
    params: Vec<Param>,
    body: Vec<Line>,
}

#[derive(Clone, Debug)]
enum Value {
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
        match (self, other) {
            (Value::Num(a), Value::Num(b)) => a == b,
            (Value::Text(a), Value::Text(b)) => a == b,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::List(a), Value::List(b)) => a == b,
            (Value::Func(a), Value::Func(b)) => Rc::ptr_eq(a, b),
            (Value::Native(a), Value::Native(b)) => a == b,
            (Value::Null, Value::Null) => true,
            _ => false,
        }
    }
}

impl Value {
    fn number(&self) -> Result<f64, String> {
        match self {
            Value::Num(n) => Ok(*n),
            other => Err(format!("Number expected, found: {}", other)),
        }
    }

    fn truthy(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            Value::Num(n) => *n != 0.0,
            Value::Text(s) => !s.is_empty(),
            Value::List(items) => !items.is_empty(),
            Value::Func(_) | Value::Native(_) => true,
            Value::Null => false,
        }
    }

    fn index(&self) -> Result<usize, String> {
        let n = self.number()?;
        if n < 0.0 || n.fract() != 0.0 || n > usize::MAX as f64 {
            Err("The list index must be a non-negative integer.".into())
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

fn binop(op: &str, a: Value, b: Value) -> Result<Value, String> {
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
            _ => return Err("Compatible values ​​are required for '+'".into()),
        },
        "/" => {
            let d = b.number()?;
            if d == 0.0 {
                return Err("Division by zero is not possible.z".into());
            }
            Value::Num(a.number()? / d)
        }
        "%" => {
            let d = b.number()?;
            if d == 0.0 {
                return Err("The modulo operation cannot be performed with zero.".into());
            }
            Value::Num(a.number()? % d)
        }
        _ => return Err(format!("Unsupported operator: {}", op)),
    })
}

/* ========================= YERLEŞİKLER ============================ */

const NATIVES: [&str; 4] = ["len", "num", "text", "type"];

fn call_native(name: &str, args: Vec<Value>) -> Result<Value, String> {
    let need = |n: usize| -> Result<(), String> {
        if args.len() == n {
            Ok(())
        } else {
            Err(format!("{}() takes {} arguments, {} were given", name, n, args.len()))
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
                        "A list or text is required for len; found: {}",
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
                Value::Text(s) => match s.trim().parse::<f64>() {
                    Ok(n) => Value::Num(n),
                    Err(_) => return Err(format!("Could not be converted to a number: {:?}", s)),
                },
                other => return Err(format!("Could not be converted to a number: {}", other)),
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
                    Value::Num(_) => "number",
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

/* ============================= LEXER ============================== */

#[derive(Clone, Debug, PartialEq)]
enum Tok {
    Val(Value),
    Id(String),
    Op(String),
    LParen,
    RParen,
    LBracket,
    RBracket,
    Comma,
    Question,
    Spread, // ...
    Arrow,  // =>
    FuncKw, // func
    Eof,
}

const TWO_CHAR_OPS: [&str; 9] = ["==", "!=", "<=", ">=", "+=", "-=", "*=", "/=", "%="];

fn lex(input: &str) -> Result<Vec<Tok>, String> {
    let chars: Vec<char> = input.chars().collect();
    let mut out = Vec::new();
    let mut i = 0usize;

    while i < chars.len() {
        let c = chars[i];

        if c.is_whitespace() {
            i += 1;
            continue;
        }

        // --- metin ---
        if c == '"' || c == '\'' {
            let quote = c;
            i += 1;
            let mut value = String::new();
            let mut closed = false;

            while i < chars.len() {
                let ch = chars[i];
                i += 1;
                if ch == quote {
                    closed = true;
                    break;
                }
                if ch == '\\' && i < chars.len() {
                    let esc = chars[i];
                    i += 1;
                    value.push(match esc {
                        'n' => '\n',
                        't' => '\t',
                        'r' => '\r',
                        other => other,
                    });
                } else {
                    value.push(ch);
                }
            }

            if !closed {
                return Err("Unclosed text expression".into());
            }
            out.push(Tok::Val(Value::Text(value)));
            continue;
        }

        // --- sayı ---
        if c.is_ascii_digit()
            || (c == '.' && matches!(chars.get(i + 1), Some(d) if d.is_ascii_digit()))
        {
            let start = i;
            let mut dot = false;
            while i < chars.len() {
                match chars[i] {
                    d if d.is_ascii_digit() => i += 1,
                    '.' if !dot => {
                        dot = true;
                        i += 1;
                    }
                    _ => break,
                }
            }
            let text: String = chars[start..i].iter().collect();
            let n = text
                .parse::<f64>()
                .map_err(|_| format!("Invalid number: {}", text))?;
            out.push(Tok::Val(Value::Num(n)));
            continue;
        }

        // --- isim / anahtar kelime ---
        if c.is_ascii_alphabetic() || c == '_' {
            let start = i;
            while i < chars.len() && (chars[i].is_ascii_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            let name: String = chars[start..i].iter().collect();
            out.push(match name.as_str() {
                "true" => Tok::Val(Value::Bool(true)),
                "false" => Tok::Val(Value::Bool(false)),
                "null" => Tok::Val(Value::Null),
                "and" | "or" | "not" => Tok::Op(name),
                "func" => Tok::FuncKw,
                _ => Tok::Id(name),
            });
            continue;
        }

        // --- ... ---
        if c == '.'
            && matches!(chars.get(i + 1), Some('.'))
            && matches!(chars.get(i + 2), Some('.'))
        {
            out.push(Tok::Spread);
            i += 3;
            continue;
        }

        // --- => ---
        if c == '=' && matches!(chars.get(i + 1), Some('>')) {
            out.push(Tok::Arrow);
            i += 2;
            continue;
        }

        // --- iki karakterli operatörler ---
        let pair: String = chars[i..(i + 2).min(chars.len())].iter().collect();
        if TWO_CHAR_OPS.contains(&pair.as_str()) {
            out.push(Tok::Op(pair));
            i += 2;
            continue;
        }

        // --- tek karakterli tokenlar ---
        let tok = match c {
            '(' => Tok::LParen,
            ')' => Tok::RParen,
            '[' => Tok::LBracket,
            ']' => Tok::RBracket,
            ',' => Tok::Comma,
            '?' => Tok::Question,
            '+' | '-' | '*' | '/' | '%' | '<' | '>' | '=' | '!' => Tok::Op(c.to_string()),
            _ => return Err(format!("Unknown character: {}", c)),
        };
        out.push(tok);
        i += 1;
    }

    out.push(Tok::Eof);
    Ok(out)
}

/// Token dizisini yeniden lex edilebilir kaynak metnine çevirir.
fn toks_to_source(toks: &[Tok]) -> String {
    let mut out = String::new();
    for t in toks {
        if !out.is_empty() {
            out.push(' ');
        }
        match t {
            Tok::Val(Value::Text(s)) => out.push_str(&format!("{:?}", s)),
            Tok::Val(v) => out.push_str(&v.to_string()),
            Tok::Id(n) => out.push_str(n),
            Tok::Op(o) => out.push_str(o),
            Tok::LParen => out.push('('),
            Tok::RParen => out.push(')'),
            Tok::LBracket => out.push('['),
            Tok::RBracket => out.push(']'),
            Tok::Comma => out.push(','),
            Tok::Question => out.push('?'),
            Tok::Spread => out.push_str("..."),
            Tok::Arrow => out.push_str("=>"),
            Tok::FuncKw => out.push_str("func"),
            Tok::Eof => {}
        }
    }
    out
}

/* ============================= PARSER ============================= */

struct Parser<'a> {
    toks: Vec<Tok>,
    pos: usize,
    rt: &'a mut Runtime,
}

impl<'a> Parser<'a> {
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

    /// Değerlendirmeden, en dıştaki ',' ')' ']' ya da Eof'a kadar token toplar.
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

    fn parse_params(&mut self) -> Result<Vec<Param>, String> {
        if !matches!(self.take(), Tok::LParen) {
            return Err("'(' was expected".into());
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
                    return Err("There can be only one '...args' parameter.".into());
                }
                variadic_seen = true;
                true
            } else {
                false
            };

            let name = match self.take() {
                Tok::Id(n) => n,
                other => {
                    return Err(format!("Parameter name expected, found: {:?}", other))
                }
            };

            if params.iter().any(|p| p.name == name) {
                return Err(format!("Parameter name is duplicated: {}", name));
            }

            let default = if matches!(self.peek(), Tok::Op(o) if o == "=") {
                if is_variadic {
                    return Err("A variadic parameter cannot have a default value.".into());
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
                other => {
                    return Err(format!("',' or ')' expected, found: {:?}", other))
                }
            }
        }

        Ok(params)
    }

    fn parse(mut self) -> Result<Value, String> {
        let v = self.expr(1)?;
        if !matches!(self.peek(), Tok::Eof) {
            return Err(format!("Unexpected expression: {:?}", self.peek()));
        }
        Ok(v)
    }

    /// İfadeyi değerlendirir; ifade tek bir isimden ibaretse o ismin
    /// kanal etiketini de döndürür. (Kanal bağının argümanla taşınması için.)
    fn expr_tagged(&mut self, min_prec: u8) -> Result<(Value, Option<String>), String> {
        let hint = match self.peek() {
            Tok::Id(name) => {
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
            }
            _ => None,
        };

        Ok((self.expr(min_prec)?, hint))
    }

    fn expr(&mut self, min_prec: u8) -> Result<Value, String> {
        let mut left = match self.take() {
            Tok::Val(v) => v,

            Tok::Id(name) => self
                .rt
                .get(&name)
                .ok_or_else(|| format!("Undefined variable: {}", name))?,

            Tok::Op(op) if op == "-" => Value::Num(-self.expr(6)?.number()?),
            Tok::Op(op) if op == "+" => Value::Num(self.expr(6)?.number()?),
            Tok::Op(op) if op == "not" || op == "!" => Value::Bool(!self.expr(6)?.truthy()),

            // lambda: func(p, q = ifade) => ifade
            Tok::FuncKw => {
                let params = self.parse_params()?;
                if !matches!(self.take(), Tok::Arrow) {
                    return Err("'=>' was expected".into());
                }
                let body_toks = self.collect_tail();
                if body_toks.is_empty() {
                    return Err("Lambda is empty".into());
                }
                let body_src = toks_to_source(&body_toks);
                let body = vec![Line { number: 0, text: format!("return {}", body_src) }];
                Value::Func(Rc::new(Function { name: "lambda".into(), params, body }))
            }

            Tok::LParen => {
                let v = self.expr(1)?;
                if !matches!(self.take(), Tok::RParen) {
                    return Err("')' bekleniyordu".into());
                }
                v
            }

            Tok::LBracket => {
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
                    return Err("']' expected for call".into());
                }
                Value::List(items)
            }

            other => return Err(format!("Invalid expression: {:?}", other)),
        };

        loop {
            // --- fonksiyon çağrısı ---
            if matches!(self.peek(), Tok::LParen) {
                self.take();
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
                    return Err("')' expected for call".into());
                }

                left = match left {
                    Value::Func(f) => self.rt.call_tagged(&f, args)?,
                    Value::Native(n) => {
                        call_native(n, args.into_iter().map(|(v, _)| v).collect())?
                    }
                    other => return Err(format!("Not callable: {}", other)),
                };
                continue;
            }

            // --- indeksleme ---
            if matches!(self.peek(), Tok::LBracket) {
                self.take();
                let idx = self.expr(1)?.index()?;
                if !matches!(self.take(), Tok::RBracket) {
                    return Err("']' expected for index".into());
                }

                left = match left {
                    Value::List(items) => {
                        let len = items.len();
                        items.into_iter().nth(idx).ok_or_else(|| {
                            format!("List index out of range: {} (length: {})", idx, len)
                        })?
                    }
                    other => {
                        return Err(format!("A list is required for indexing; found: {}", other))
                    }
                };
                continue;
            }

            // --- kanal okuma: varsayilan ? ad ---
            // 'ad' bir kanal etiketi taşıyorsa (örn. parametreyle taşınmışsa)
            // gerçek kanal adına çözülür; yoksa 'ad'ın kendisi kanal adı sayılır.
            if matches!(self.peek(), Tok::Question) {
                self.take();
                let written = match self.take() {
                    Tok::Id(n) => n,
                    other => return Err(format!("Channel name expected, found: {:?}", other)),
                };
                let effective = self.rt.tag_of(&written).unwrap_or(written);
                left = self.rt.channels.get(&effective).cloned().unwrap_or(left);
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
}

/* ============================ RUNTIME ============================= */

#[derive(Clone, Copy, PartialEq, Debug)]
enum Flow {
    Normal,
    Stop,
    Forward,
    Return,
}

struct Runtime {
    scopes: Vec<HashMap<String, Value>>,
    consts: Vec<HashSet<String>>,
    tags: Vec<HashMap<String, String>>, // değişken adı -> kanal adı
    channels: HashMap<String, Value>,
    functions: HashMap<String, Rc<Function>>, // 'add' ile tanımlananlar (global)
    call_depth: usize,
    return_value: Option<Value>,
    last_value: Option<Value>,
}

impl Runtime {
    fn new() -> Self {
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

    fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
        self.consts.push(HashSet::new());
        self.tags.push(HashMap::new());
    }

    fn pop_scope(&mut self) {
        self.scopes.pop();
        self.consts.pop();
        self.tags.pop();
    }

    fn is_const(&self, name: &str) -> bool {
        self.consts.iter().any(|s| s.contains(name))
    }

    /// Bu adın bağlı olduğu kanal adı (varsa).
    fn tag_of(&self, name: &str) -> Option<String> {
        self.tags.iter().rev().find_map(|t| t.get(name)).cloned()
    }

    fn tag_as(&mut self, var_name: &str, channel_name: &str) {
        self.tags
            .last_mut()
            .unwrap()
            .insert(var_name.to_string(), channel_name.to_string());
    }

    /// Arama sırası: kapsamlar → global fonksiyonlar → yerleşikler
    fn get(&self, name: &str) -> Option<Value> {
        if let Some(v) = self.scopes.iter().rev().find_map(|s| s.get(name)) {
            return Some(v.clone());
        }
        if let Some(f) = self.functions.get(name) {
            return Some(Value::Func(f.clone()));
        }
        NATIVES
            .iter()
            .find(|n| **n == name)
            .map(|n| Value::Native(n))
    }

    fn declare_const(&mut self, name: &str, value: Value) {
        self.scopes.last_mut().unwrap().insert(name.to_string(), value);
        self.consts.last_mut().unwrap().insert(name.to_string());
        // Sabit, kendi adıyla bir kanal açar.
        self.tag_as(name, name);
    }

    fn assign(&mut self, name: &str, value: Value) -> Result<(), String> {
        // Sabite yazmak, bağlı olduğu KANALI günceller (sabitin kendisi değişmez).
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

    fn set_index(&mut self, name: &str, idx: usize, value: Value) -> Result<(), String> {
        if self.is_const(name) {
            return Err(format!("The elements of the constant '{}' cannot be modified.", name));
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

    fn eval(&mut self, src: &str) -> Result<Value, String> {
        if src.trim().is_empty() {
            return Err("Missing expression".into());
        }
        let toks = lex(src)?;
        Parser { toks, pos: 0, rt: self }.parse()
    }

    /// Etiketsiz çağrı (varsayılan değer hesaplama, iç kullanım).
    #[allow(dead_code)]
    fn call(&mut self, func: &Rc<Function>, args: Vec<Value>) -> Result<Value, String> {
        self.call_tagged(func, args.into_iter().map(|v| (v, None)).collect())
    }

    /// Argümanlarla birlikte kanal etiketlerini de taşıyan çağrı.
    fn call_tagged(
        &mut self,
        func: &Rc<Function>,
        args: Vec<(Value, Option<String>)>,
    ) -> Result<Value, String> {
        if self.call_depth >= CALL_LIMIT {
            return Err(format!("Call stack limit reached ({})", CALL_LIMIT));
        }

        let mut values = Vec::with_capacity(args.len());
        let mut arg_tags = Vec::with_capacity(args.len());
        for (v, t) in args {
            values.push(v);
            arg_tags.push(t);
        }

        let planned = plan_args(func, values)?;
        let param_tags = map_tags_to_params(func, &arg_tags);

        // Kapalı kapsam: dış değişkenler görünmez
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
        // Varsayılanlar fonksiyonun KENDİ kapsamında hesaplanır:
        // önceki parametreleri görebilir, dış değişkenleri göremez.
        let value = match slot {
            Some(v) => v,
            None => match &p.default {
                Some(src) => rt.eval(src)?,
                None => {
                    return Err(format!(
                        "The parameter '{}' is missing in the '{}' function.",
                        func.name, p.name
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
            Err("'stop'/'fwd' can only be used within a loop.".into())
        }
    }
}

/// Argümanları parametre yuvalarına dağıtır. None = varsayılan kullanılacak.
fn plan_args(func: &Function, args: Vec<Value>) -> Result<Vec<Option<Value>>, String> {
    let params = &func.params;
    let n = args.len();
    let mut out: Vec<Option<Value>> = vec![None; params.len()];

    match params.iter().position(|p| p.variadic) {
        None => {
            if n > params.len() {
                return Err(format!(
                    "'{}' takes at most {} arguments, {} were given",
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
            let before = vp;
            let after = params.len() - vp - 1;

            // Sondaki sabit parametreler önce pay alır, kalanı variadic yutar.
            let tail_take = after.min(n);
            let tail_start = n - tail_take;
            let head_take = before.min(tail_start);

            let mut args = args;
            let tail: Vec<Value> = args.split_off(tail_start);
            let middle: Vec<Value> = args.split_off(head_take);
            let head: Vec<Value> = args;

            for (i, a) in head.into_iter().enumerate() {
                out[i] = Some(a);
            }
            out[vp] = Some(Value::List(middle));

            let first_filled = after - tail_take;
            for (k, a) in tail.into_iter().enumerate() {
                out[vp + 1 + first_filled + k] = Some(a);
            }
        }
    }

    Ok(out)
}

/// Argüman konumundaki kanal etiketlerini parametre konumuna haritalar.
/// Variadic parametre etiket taşımaz (bir liste üretir).
fn map_tags_to_params(func: &Function, arg_tags: &[Option<String>]) -> Vec<Option<String>> {
    let params = &func.params;
    let n = arg_tags.len();
    let mut out: Vec<Option<String>> = vec![None; params.len()];

    match params.iter().position(|p| p.variadic) {
        None => {
            for (i, t) in arg_tags.iter().take(params.len()).enumerate() {
                out[i] = t.clone();
            }
        }
        Some(vp) => {
            let before = vp;
            let after = params.len() - vp - 1;
            let tail_take = after.min(n);
            let tail_start = n - tail_take;
            let head_take = before.min(tail_start);

            for i in 0..head_take {
                out[i] = arg_tags[i].clone();
            }
            let first_filled = after - tail_take;
            for k in 0..tail_take {
                out[vp + 1 + first_filled + k] = arg_tags[tail_start + k].clone();
            }
        }
    }

    out
}

/* =========================== YARDIMCILAR =========================== */

#[derive(Clone, Debug)]
struct Line {
    number: usize,
    text: String,
}

const KEYWORDS: [&str; 7] = ["true", "false", "null", "and", "or", "not", "func"];

fn is_ident(s: &str) -> bool {
    !s.is_empty()
        && !KEYWORDS.contains(&s)
        && s.chars().next().is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
        && s.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
}

/// `line`, `word` anahtar kelimesiyle gerçekten başlıyorsa kalanını döndürür.
fn kw<'a>(line: &'a str, word: &str) -> Option<&'a str> {
    let rest = line.strip_prefix(word)?;
    if rest.is_empty() || rest.starts_with(char::is_whitespace) || rest.starts_with('(') {
        Some(rest.trim())
    } else {
        None
    }
}

fn is_block_start(t: &str) -> bool {
    kw(t, "if").is_some() || kw(t, "repeat").is_some() || kw(t, "add").is_some()
}

fn strip_comment(line: &str) -> &str {
    let mut quote: Option<char> = None;
    let mut escaped = false;
    for (i, c) in line.char_indices() {
        if escaped {
            escaped = false;
        } else if quote.is_some() && c == '\\' {
            escaped = true;
        } else if c == '"' || c == '\'' {
            quote = if quote == Some(c) { None } else { quote.or(Some(c)) };
        } else if c == '#' && quote.is_none() {
            return &line[..i];
        }
    }
    line
}

/// `hedef`, `op`, `ifade` üçlüsünü ayırır; parantez/köşeli parantez içini atlar.
fn split_assignment(src: &str) -> Option<(String, String, String)> {
    let ch: Vec<char> = src.chars().collect();
    let mut quote: Option<char> = None;
    let mut escaped = false;
    let mut dp = 0i32;
    let mut db = 0i32;

    for i in 0..ch.len() {
        let c = ch[i];

        if escaped {
            escaped = false;
            continue;
        }
        if quote.is_some() && c == '\\' {
            escaped = true;
            continue;
        }
        if c == '"' || c == '\'' {
            quote = if quote == Some(c) { None } else { quote.or(Some(c)) };
            continue;
        }
        if quote.is_some() {
            continue;
        }

        match c {
            '(' => dp += 1,
            ')' => dp -= 1,
            '[' => db += 1,
            ']' => db -= 1,
            _ => {}
        }
        if dp != 0 || db != 0 || c != '=' {
            continue;
        }

        let prev = if i > 0 { ch[i - 1] } else { '\0' };
        let next = *ch.get(i + 1).unwrap_or(&'\0');

        // ==, !=, <=, >=, =>  atla
        if next == '=' || next == '>' || matches!(prev, '=' | '!' | '<' | '>') {
            continue;
        }

        let (lhs, op) = if "+-*/%".contains(prev) {
            (ch[..i - 1].iter().collect::<String>(), format!("{}=", prev))
        } else {
            (ch[..i].iter().collect::<String>(), "=".to_string())
        };

        let lhs = lhs.trim().to_string();
        if lhs.is_empty() {
            return None;
        }
        let rhs = ch[i + 1..].iter().collect::<String>().trim().to_string();
        return Some((lhs, op, rhs));
    }

    None
}

fn split_index_target(lhs: &str) -> Option<(String, String)> {
    let lhs = lhs.trim();
    let open = lhs.strip_suffix(']')?.rfind('[')?;
    let name = lhs[..open].trim();
    let index = lhs[open + 1..lhs.len() - 1].trim();
    if is_ident(name) && !index.is_empty() {
        Some((name.to_string(), index.to_string()))
    } else {
        None
    }
}

fn matching_end(lines: &[Line], head: usize) -> Result<usize, String> {
    let mut depth = 0usize;
    for i in head + 1..lines.len() {
        let t = lines[i].text.as_str();
        if is_block_start(t) {
            depth += 1;
        } else if t == "end" {
            if depth == 0 {
                return Ok(i);
            }
            depth -= 1;
        }
    }
    Err(format!("{}: 'end' not found for block", lines[head].number))
}

/// `add isim(...)` başlığını ada ve "(...)" metnine ayırır.
fn parse_add_header(rest: &str) -> Result<(String, String), String> {
    let open = rest.find('(').ok_or("'(' was expected in the function definition.")?;
    let name = rest[..open].trim().to_string();
    if !is_ident(&name) {
        return Err(format!("Invalid function name: {}", name));
    }
    if !rest.trim_end().ends_with(')') {
        return Err("')' expected in function definition".into());
    }
    Ok((name, rest[open..].trim_end().to_string()))
}

/* =========================== ÇALIŞTIRMA =========================== */

fn run(
    lines: &[Line],
    from: usize,
    to: usize,
    rt: &mut Runtime,
    in_loop: bool,
) -> Result<Flow, String> {
    let mut i = from;

    while i < to {
        let s = lines[i].text.as_str();
        let no = lines[i].number;
        let err = move |e: String| format!("{}: {}", no, e);

        if s.is_empty() {
            i += 1;
            continue;
        }

        if s == "end" {
            return Err(err("Unawaited 'end'".into()));
        }

        if kw(s, "else").is_some() {
            return Err(err("Unawaited 'else'".into()));
        }

        if s == "stop" || s == "fwd" {
            if !in_loop {
                return Err(err(format!("'{}' can only be used in loops", s)));
            }
            return Ok(if s == "stop" { Flow::Stop } else { Flow::Forward });
        }

        if let Some(rest) = kw(s, "return") {
            rt.return_value = Some(if rest.is_empty() {
                Value::Null
            } else {
                rt.eval(rest).map_err(err)?
            });
            return Ok(Flow::Return);
        }

        if let Some(rest) = kw(s, "say") {
            if rest.is_empty() {
                println!();
            } else {
                println!("{}", rt.eval(rest).map_err(err)?);
            }
            i += 1;
            continue;
        }

        if let Some(rest) = kw(s, "get") {
            if !rest.is_empty() {
                print!("{}", rt.eval(rest).map_err(err)?);
                io::stdout().flush().map_err(|e| err(e.to_string()))?;
            }
            let mut input = String::new();
            io::stdin().read_line(&mut input).map_err(|e| err(e.to_string()))?;
            rt.assign("_input", Value::Text(input.trim_end().to_string()))
                .map_err(err)?;
            i += 1;
            continue;
        }

        if let Some(rest) = kw(s, "set") {
            let (name, op, expr) = split_assignment(rest)
                .ok_or_else(|| err("Usage: set name = expression".into()))?;
            if op != "=" {
                return Err(err("Only '=' can be used in the constant definition.".into()));
            }
            if !is_ident(&name) {
                return Err(err(format!("Invalid constant name: {}", name)));
            }
            if rt.get(&name).is_some() || rt.is_const(&name) {
                return Err(err(format!("'{}' already assigned", name)));
            }
            let value = rt.eval(&expr).map_err(err)?;
            rt.declare_const(&name, value);
            i += 1;
            continue;
        }

        if let Some(rest) = kw(s, "add") {
            let (name, params_src) = parse_add_header(rest).map_err(err)?;
            let end = matching_end(lines, i)?;

            let toks = lex(&params_src).map_err(err)?;
            let params = {
                let mut p = Parser { toks, pos: 0, rt: &mut *rt };
                p.parse_params().map_err(err)?
            };

            let body: Vec<Line> = lines[i + 1..end].to_vec();
            rt.functions
                .insert(name.clone(), Rc::new(Function { name, params, body }));

            i = end + 1;
            continue;
        }

        if let Some(cond) = kw(s, "if") {
            let end = matching_end(lines, i)?;
            let mut branches: Vec<(Option<&str>, usize, usize)> = Vec::new();
            let mut current = Some(cond);
            let mut start = i + 1;
            let mut depth = 0usize;

            for j in i + 1..end {
                let t = lines[j].text.as_str();
                if is_block_start(t) {
                    depth += 1;
                } else if t == "end" {
                    depth = depth.saturating_sub(1);
                } else if depth == 0 {
                    if let Some(tail) = kw(t, "else") {
                        branches.push((current, start, j));
                        current = match kw(tail, "if") {
                            Some(c) => Some(c),
                            None if tail.is_empty() => None,
                            None => {
                                return Err(format!(
                                    "{}: waited 'else' or 'else if'.",
                                    lines[j].number
                                ))
                            }
                        };
                        start = j + 1;
                    }
                }
            }
            branches.push((current, start, end));

            for (c, a, b) in branches {
                let hit = match c {
                    Some(c) => rt.eval(c).map_err(err)?.truthy(),
                    None => true,
                };
                if hit {
                    let flow = run(lines, a, b, rt, in_loop)?;
                    if flow != Flow::Normal {
                        return Ok(flow);
                    }
                    break;
                }
            }

            i = end + 1;
            continue;
        }

        if let Some(rest) = kw(s, "repeat") {
            let end = matching_end(lines, i)?;
            let body_start = i + 1;

            let run_body = |rt: &mut Runtime| -> Result<Flow, String> {
                rt.push_scope();
                let r = run(lines, body_start, end, rt, true);
                rt.pop_scope();
                r
            };

            if let Some(cond) = kw(rest, "while") {
                let mut turns = 0usize;
                while rt.eval(cond).map_err(err)?.truthy() {
                    if turns >= LOOP_LIMIT {
                        return Err(err("The loop has reached the safety limit.".into()));
                    }
                    turns += 1;
                    match run_body(rt)? {
                        Flow::Stop => break,
                        Flow::Return => return Ok(Flow::Return),
                        _ => {}
                    }
                }
            } else {
                let n = rt.eval(rest).map_err(err)?.number().map_err(err)?;
                if n < 0.0 || n.fract() != 0.0 || n > LOOP_LIMIT as f64 {
                    return Err(err(
                        "An integer between 0 and 1,000,000 is required for 'repeat'.".into()
                    ));
                }
                for _ in 0..n as usize {
                    match run_body(rt)? {
                        Flow::Stop => break,
                        Flow::Return => return Ok(Flow::Return),
                        _ => {}
                    }
                }
            }

            i = end + 1;
            continue;
        }

        if let Some((target, op, rhs)) = split_assignment(s) {
            let value = rt.eval(&rhs).map_err(err)?;

            if let Some((name, idx_expr)) = split_index_target(&target) {
                if op != "=" {
                    return Err(err("Only '=' is supported for list elements.".into()));
                }
                let idx = rt.eval(&idx_expr).map_err(err)?.index().map_err(err)?;
                rt.set_index(&name, idx, value).map_err(err)?;
            } else {
                if !is_ident(&target) {
                    return Err(err(format!("Invalid assignment target: {}", target)));
                }
                let value = if op == "=" {
                    value
                } else {
                    let old = rt
                        .get(&target)
                        .ok_or_else(|| err(format!("Undefined variable: {}", target)))?;
                    binop(&op[..1], old, value).map_err(err)?
                };
                rt.assign(&target, value).map_err(err)?;
            }

            i += 1;
            continue;
        }

        // Çıplak ifade: değeri "son ifade" olarak saklanır (otomatik dönüş)
        let value = rt.eval(s).map_err(err)?;
        rt.last_value = Some(value);
        i += 1;
    }

    Ok(Flow::Normal)
}

/* ============================== MAIN ============================== */

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let Some(filename) = args.get(1) else {
        eprintln!("Veyra v0.1.0\nTo run a Veyra file, please provide file name as the first argument.\ne.g. `veyra filename.vey`");
        return;
    };

    let source = match std::fs::read_to_string(filename) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Couldn't read the file: {}", e);
            std::process::exit(1);
        }
    };

    let lines: Vec<Line> = source
        .lines()
        .enumerate()
        .map(|(i, l)| Line { number: i + 1, text: strip_comment(l).trim().to_string() })
        .collect();

    let mut rt = Runtime::new();
    if let Err(e) = run(&lines, 0, lines.len(), &mut rt, false) {
        eprintln!("Veyra error: {}", e);
        std::process::exit(1);
    }
}