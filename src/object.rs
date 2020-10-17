use std::{cmp::Ordering, hash::Hash, hash::Hasher, ops::Deref};

#[derive(Debug, Clone, Hash, PartialOrd, PartialEq)]
pub enum Object {
    String(String),

    Number(Number),

    Integer(i64), // TODO change to bigint

    Boolean(bool),

    // Closure(Vec<Value>),

    // Object(HashMap<String, Box<Value>>),

    // List(Vec<Value>),

    // Tuple(Vec<Value>),

    Null
}

impl Default for Object {
    fn default() -> Object {
        Object::Null
    }
}

#[derive(Debug, Clone, Copy, Default, PartialOrd, PartialEq)]
pub struct Number(f64);

impl Eq for Number {}

impl Hash for Number {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state)
    }
}

impl Ord for Number {
    fn cmp(&self, other: &Number) -> Ordering {
        match self.partial_cmp(&other) {
            Some(ord) => ord,
            None => unreachable!(),
        }
    }
}

impl Deref for Number {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Into<Number> for f64 {
    fn into(self) -> Number {
        Number(self)
    }
}

impl Number {

    #[inline(always)]
    pub fn value(self) -> f64 {
        self.0
    }
}