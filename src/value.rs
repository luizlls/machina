use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    Number(f64),
    Boolean(bool),
    Object(HashMap<String, Value>),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ValueKind {
    String,
    Number,
    Boolean,
    Object,
}

impl Value {
    pub fn is(&self, kind: ValueKind) -> bool {
        match self {
            Value::String(_)  => kind == ValueKind::String,
            Value::Number(_)  => kind == ValueKind::Number,
            Value::Boolean(_) => kind == ValueKind::Boolean,
            Value::Object(_)  => kind == ValueKind::Object,
        }
    }
}