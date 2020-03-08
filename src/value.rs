use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Value {
    Null,
    String(String),
    Integer(i64),
    Decimal(f64),
    Array(Vec<Value>),
    Object(HashMap<String, Value>)
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ValueKind {
    Null,
    String,
    Decimal,
    Integer,
    Array,
    Object,
}

impl Value {
    pub fn is(&self, kind: ValueKind) -> bool {
        match self {
            Value::Null => kind == ValueKind::Null,
            Value::String (_) => kind == ValueKind::String,
            Value::Decimal(_) => kind == ValueKind::Decimal,
            Value::Integer(_) => kind == ValueKind::Integer,
            Value::Array  (_) => kind == ValueKind::Array,
            Value::Object (_) => kind == ValueKind::Object,
        }
    }
}