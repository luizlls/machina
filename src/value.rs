use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Value {
    String {
        value: String
    },
    Integer {
        value: i64
    },
    Decimal {
        value: f64
    },
    Array {
        value: Vec<Value>
    },
    Object {
        value: HashMap<String, Value>
    },
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ValueKind {
    String,
    Decimal,
    Integer,
    Array,
    Object,
}

impl Value {
    pub fn is(&self, kind: ValueKind) -> bool {
        match self {
            Value::String {..} => kind == ValueKind::String,
            Value::Decimal{..} => kind == ValueKind::Decimal,
            Value::Integer{..} => kind == ValueKind::Integer,
            Value::Array  {..} => kind == ValueKind::Array,
            Value::Object {..} => kind == ValueKind::Object,
        }
    }
}