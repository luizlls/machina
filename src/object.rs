#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ObjectKind {
    String,
    Decimal,
    Integer,
    Null,
}

#[derive(Debug, Clone)]
pub enum ObjectValue {
    Integer(i64),
    Decimal(f64),
    String(String),
    Null,
}

impl ObjectValue {
    pub fn is(&self, kind: ObjectKind) -> bool {
        match self {
            ObjectValue::String (_) => kind == ObjectKind::String,
            ObjectValue::Decimal(_) => kind == ObjectKind::Decimal,
            ObjectValue::Integer(_) => kind == ObjectKind::Integer,
            ObjectValue::Null       => kind == ObjectKind::Null,
        }
    }
}