#[derive(Debug, Clone)]
pub enum Object {
    Null,
    Integer(i64),
    Decimal(f64),
    String(String),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ObjectKind {
    Null,
    String,
    Decimal,
    Integer,
}

impl Object {
    pub fn is(&self, kind: ObjectKind) -> bool {
        match self {
            Object::Null => kind == ObjectKind::Null,
            Object::String (_) => kind == ObjectKind::String,
            Object::Decimal(_) => kind == ObjectKind::Decimal,
            Object::Integer(_) => kind == ObjectKind::Integer,
        }
    }
}