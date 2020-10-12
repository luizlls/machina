use std::fmt::{Debug, Display};

const MAX_NUM:  u64 = 0xfff8000000000000;
const NAN_TAG:  u64 = MAX_NUM;
const INT_TAG:  u64 = 0xfff9000000000000;
const CHR_TAG:  u64 = 0xfffa000000000000;
const PTR_TAG:  u64 = 0xfffb000000000000;
const TRUE_TAG: u64 = 0xfffc000000000000;
const FLSE_TAG: u64 = 0xfffd000000000000;
const NULL_TAG: u64 = 0xffff000000000000;

#[derive(Clone, Copy, PartialEq)]
pub struct Value(u64);

const TRUE:  Value = Value(TRUE_TAG);
const FALSE: Value = Value(FLSE_TAG);
const NULL:  Value = Value(NULL_TAG);
const QNAN:  Value = Value(NAN_TAG);


impl Value {
    #[inline(always)]
    pub fn is_num(&self) -> bool {
        self.0 < NAN_TAG
    }

    #[inline(always)]
    pub fn is_int(&self) -> bool {
        (self.0 & INT_TAG) == INT_TAG
    }

    #[inline(always)]
    pub fn is_char(&self) -> bool {
        (self.0 & CHR_TAG) == CHR_TAG
    }

    #[inline(always)]
    pub fn is_null(&self) -> bool {
        self == &NULL
    }

    #[inline(always)]
    pub fn is_true(&self) -> bool {
        self == &TRUE
    }

    #[inline(always)]
    pub fn is_false(&self) -> bool {
        self == &FALSE
    }

    #[inline(always)]
    pub fn is_ptr(&self) -> bool {
        (self.0 & PTR_TAG) == PTR_TAG
    }

    #[inline(always)]
    pub const fn raw(v: u64) -> Value {
        Value(v)
    }
    pub fn ptr<T>(ptr: *const T) -> Value {
        Value(PTR_TAG | ptr as u64)
    }

    #[inline(always)]
    pub const fn null() -> Value {
        NULL
    }

    #[inline(always)]
    pub const fn nan() -> Value {
        QNAN
    }

    #[inline(always)]
    pub fn get_raw(&self) -> u64 {
        self.0
    }

    #[inline(always)]
    pub fn get_num(&self) -> f64 {
        if self.is_num() {
            f64::from_bits(self.0)
        } else {
            f64::NAN
        }
    }

    #[inline(always)]
    pub fn get_num_unchecked(&self) -> f64 {
        f64::from_bits(self.0)
    }

    #[inline(always)]
    pub fn as_num(&self) -> f64 {
        if self.is_num() {
            self.get_num_unchecked() as f64
        } else if self.is_int() {
            self.get_int_unchecked() as f64
        } else {
            panic!("Cannot coerse {:?} into a number", self)
        }
    }

    #[inline(always)]
    pub fn get_int(&self) -> i32 {
        assert!(self.is_int());
        (self.0 & !INT_TAG) as i32
    }

    #[inline(always)]
    pub fn get_int_unchecked(&self) -> i32 {
        (self.0 & !INT_TAG) as i32
    }

    #[inline(always)]
    pub fn as_int(&self) -> i64 {
        if self.is_int() {
            self.get_int_unchecked() as i64
        } else if self.is_num() {
            self.get_num_unchecked() as i64
        } else {
            panic!("Cannot coerse {:?} into a int", self)
        }
    }

    #[inline(always)]
    pub fn get_char(&self) -> char {
        assert!(self.is_char());
        std::char::from_u32((self.0 & !CHR_TAG) as u32).unwrap()
    }

    #[inline(always)]
    pub fn get_char_unchecked(&self) -> char {
        std::char::from_u32((self.0 & !CHR_TAG) as u32).unwrap()
    }

    #[inline(always)]
    pub fn get_ptr<T>(&self) -> *const T {
        assert!(self.is_ptr());
        unsafe { ::std::mem::transmute(self.0 & !PTR_TAG) }
    }

    #[inline(always)]
    pub fn get_ptr_unchecked<T>(&self) -> *const T {
        unsafe { ::std::mem::transmute(self.0 & !PTR_TAG) }
    }

    #[inline(always)]
    pub fn get_ptr_mut<T>(&self) -> *mut T {
        assert!(self.is_ptr());
        unsafe { ::std::mem::transmute(self.0 & !PTR_TAG) }
    }

    #[inline(always)]
    pub fn get_ptr_mut_unchecked<T>(&self) -> *mut T {
        unsafe { ::std::mem::transmute(self.0 & !PTR_TAG) }
    }
}

impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_num() {
            write!(f, "NUM {}", self.get_num())
        } else if self.is_int() {
            write!(f, "INT {}", self.get_int())
        } else if self.is_char() {
            write!(f, "CHAR {}", self.get_char())
        } else if self.is_ptr() {
            write!(f, "PTR {}", (self.get_raw() & !PTR_TAG))
        } else if self.is_null() {
            write!(f, "NULL")
        } else if self.is_true() {
            write!(f, "TRUE")
        } else if self.is_false() {
            write!(f, "FALSE")
        } else {
            write!(f, "NAN")
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_num() {
            write!(f, "{}", self.get_num())
        } else if self.is_int() {
            write!(f, "{}", self.get_int())
        } else if self.is_char() {
            write!(f, "{}", self.get_char())
        } else if self.is_ptr() {
            write!(f, "0x{:08X}", (self.get_raw() & !PTR_TAG))
        } else if self.is_null() {
            write!(f, "null")
        } else if self.is_true() {
            write!(f, "true")
        } else if self.is_false() {
            write!(f, "false")
        } else {
            write!(f, "NAN")
        }
    }
}

impl From<bool> for Value {

    #[inline(always)]
    fn from(b: bool) -> Value {
        if b { TRUE } else { FALSE }
    }
}

impl From<f64> for Value {

    #[inline(always)]
    fn from(f: f64) -> Value {
        Value(f.to_bits())
    }
}

impl From<i64> for Value {

    #[inline(always)]
    fn from(i: i64) -> Value {
        if i <= i32::MAX as i64 {
            Value::from(i as i32)
        } else {
            Value((i as f64).to_bits())
        }
    }
}

impl From<i32> for Value {

    #[inline(always)]
    fn from(i: i32) -> Value {
        Value(INT_TAG | i as u64)
    }
}

impl From<char> for Value {

    #[inline(always)]
    fn from(c: char) -> Value {
        Value(CHR_TAG | c as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn booleans() {
        let b = Value::from(2 == 2);
        assert_eq!(b, TRUE);
        let b = Value::from(2 != 2);
        assert_eq!(b, FALSE);
    }

    #[test]
    fn numbers() {
        let a = Value::from(3.141592);
        assert!(a.is_num());
        assert_eq!(a.get_num_unchecked(), 3.141592);
    }

    #[test]
    fn nans() {
        let a = Value::nan();
        assert!(!a.is_num());
        assert!(a.get_num_unchecked().is_nan());
    }

    #[test]
    fn char_simple() {
        let hello = vec![
            Value::from('h'),
            Value::from('e'),
            Value::from('l'),
            Value::from('l'),
            Value::from('o')
        ];

        for c in hello.iter() {
            assert!(c.is_char());
        }

        assert_eq!("hello", hello.iter().map(Value::get_char_unchecked).collect::<String>());
    }

    #[test]
    fn char_complex() {
        let hello = "こんにちは".chars().into_iter().map(Value::from).collect::<Vec<_>>();

        for c in hello.iter() {
            assert!(c.is_char());
        }

        assert_eq!("こんにちは", hello.iter().map(Value::get_char_unchecked).collect::<String>());
    }

    #[test]
    fn int_positive() {
        let a = Value::from((2i64.pow(31) - 1) as i32);
        assert!(a.is_int());
        assert_eq!(a.get_int_unchecked(), 2147483647);
    }

    #[test]
    fn int_negative() {
        let a = Value::from(-321);
        assert!(a.is_int());
        assert_eq!(a.get_int_unchecked(), -321);
    }

    #[test]
    fn nulls() {
        let a = Value::null();
        assert!(a.is_null());
        assert_eq!(a, NULL);
    }

    #[test]
    fn ptrs() {
        let val: Box<Value> = Box::new(Value::from(42));
        let val_ptr: *const Value = &*val;
        let p = Value::ptr(val_ptr);
        assert!(p.is_ptr());
        assert_eq!(p.get_ptr_unchecked(), val_ptr);
        let d = unsafe { *p.get_ptr_unchecked::<Value>() };
        assert_eq!(d, *val);
        assert!(d.is_int());
        assert_eq!(d.get_int_unchecked(), 42);
    }

    #[test]
    fn equality() {
        let a = Value::from(123);
        let b = Value::from(123);
        let c = Value::from(321);
        assert!(a.is_int());
        assert!(b.is_int());
        assert!(c.is_int());
        assert_eq!(a, b);
        assert_ne!(a, c);
        assert_ne!(b, c);
    }
}
