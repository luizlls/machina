#[derive(Debug, Clone)]
pub enum Value {
    Null,
    Integer(i64),
    Decimal(f64),
    String(String),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ValueKind {
    Null,
    String,
    Decimal,
    Integer,
}

impl Value {
    pub fn is(&self, kind: ValueKind) -> bool {
        match self {
            Value::Null => kind == ValueKind::Null,
            Value::String (_) => kind == ValueKind::String,
            Value::Decimal(_) => kind == ValueKind::Decimal,
            Value::Integer(_) => kind == ValueKind::Integer,
        }
    }
}