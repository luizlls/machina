use std::hash::{Hash, Hasher};
use std::fmt;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ObjectKind {
    Null,
    String,
    Decimal,
    Integer,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ObjectValue {
    Null,
    Integer(i64),
    Decimal(f64),
    String(String),
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
                state.write_usize(0);
            }
        }
        state.finish();
    }
}

impl Eq for ObjectValue {}

impl fmt::Display for ObjectValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ObjectValue::Null => write!(f, "null"),
            ObjectValue::Decimal(d) => write!(f, "{}", d),
            ObjectValue::Integer(i) => write!(f, "{}", i),
            ObjectValue::String (s) => write!(f, "{}", s),
        }
    }
}

macro_rules! expr {
    ($e:expr) => { $e };
}

macro_rules! binary_numeric {
    ($name:ident, $op:tt) => {
        pub fn $name(self, other: ObjectValue) -> ObjectValue {
            let error = format!("Cannot call {} on values of type {} and {}", stringify!($name), self.name(), other.name());
            use ObjectValue::*;
            match (self, other) {
                (Decimal(s), Decimal(o)) => Decimal(expr!(s $op o)),
                (Integer(s), Integer(o)) => Integer(expr!(s $op o)),
                (Integer(s), Decimal(o)) => Decimal(expr!(s as f64 $op o)),
                (Decimal(s), Integer(o)) => Decimal(expr!(s $op o as f64)),
                _ => {
                    panic!(error);
                }
            }
        }
    };
}

macro_rules! binary_integer {
    ($name:ident, $op:tt) => {
        pub fn $name(self, other: ObjectValue) -> ObjectValue {
            let error = format!("Cannot call {} on values of type {} and {}", stringify!($name), self.name(), other.name());
            use ObjectValue::*;
            match (self, other) {
                (Integer(s), Integer(o)) => Integer(expr!(s $op o) as i64),
                _ => {
                    panic!(error);
                }
            }
        }
    };
}

macro_rules! binary_compare {
    ($name:ident, $op:tt) => {
        pub fn $name(self, other: ObjectValue) -> ObjectValue {
            let error = format!("Cannot call {} on values of type {} and {}", stringify!($name), self.name(), other.name());
            use ObjectValue::*;
            match (self, other) {
                (Null, Null) => Integer(false as i64),
                (Integer(s), Integer(o)) => Integer(expr!(s $op o) as i64),
                (Decimal(s), Decimal(o)) => Integer(expr!(s $op o) as i64),
                (String (s), String (o)) => Integer(expr!(s $op o) as i64),
                _ => {
                    panic!(error);
                }
            }
        }  
    };
}

impl ObjectValue {

    binary_numeric!(add, +);
    binary_numeric!(sub, -);
    binary_numeric!(mul, *);
    binary_numeric!(div, /);
    binary_numeric!(modulus, %);
    binary_compare!(lt, <);
    binary_compare!(gt, >);
    binary_compare!(lte, <=);
    binary_compare!(gte, >=);
    binary_integer!(bit_and, &);
    binary_integer!(bit_or,  |);
    binary_integer!(bit_xor, ^);

    pub fn eq(self, other: ObjectValue) -> ObjectValue {
        ObjectValue::Integer((self == other) as i64)
    }

    pub fn ne(self, other: ObjectValue) -> ObjectValue {
        ObjectValue::Integer((self != other) as i64)
    }

    pub fn bit_not(self) -> ObjectValue {
        match self {
            ObjectValue::Integer(i) => ObjectValue::Integer(!i),
            _ => {
                panic!("Cannot call not on values of type {}", self.name());
            }
        }
    }

    pub fn is(&self, kind: ObjectKind) -> bool {
        match self {
            ObjectValue::Null => kind == ObjectKind::Null,
            ObjectValue::Decimal(_) => kind == ObjectKind::Decimal,
            ObjectValue::Integer(_) => kind == ObjectKind::Integer,
            ObjectValue::String (_) => kind == ObjectKind::String,
        }
    }
    pub fn name<'a>(&self) -> &'a str {
        match self {
            ObjectValue::Null => "null",
            ObjectValue::Decimal(_) => "string",
            ObjectValue::Integer(_) => "integer",
            ObjectValue::String (_) => "decimal",
        }
    }

    pub fn boolean(&self) -> bool {
        match self {
            ObjectValue::Null => false,
            ObjectValue::Decimal(d) => *d != 0.0,
            ObjectValue::Integer(i) => *i != 0,
            ObjectValue::String (s) => !s.is_empty()
        }
    }
}