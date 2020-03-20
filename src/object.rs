use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ObjectKind {
    String,
    Decimal,
    Integer,
    Null,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ObjectValue {
    Integer(i64),
    Decimal(f64),
    String(String),
    Null,
}

impl Hash for ObjectValue {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        match self {
            ObjectValue::String(s) => {
                s.hash(state);
            }
            ObjectValue::Integer(i) => {
                i.hash(state);
            }
            ObjectValue::Decimal(d) => {
                state.write_u64(d.to_bits());
            }
            ObjectValue::Null => {
                state.write_u8(0);
            }
        }
        state.finish();
    }
}

impl Eq for ObjectValue {}

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