use crate::lexer::{TokenKind};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    /// target, expr
    Assignment(Variable, Expression),

    /// possibilities (test, dest)
    Switch(Vec<(Value, Label)>),

    /// test, then, else
    If(Value, Label, Label),

    /// dest
    Jmp(Label),

    /// test, dest
    JmpT(Value, Label),

    /// test, dest
    JmpF(Value, Label),

    /// func, arguments
    Call(Label, Vec<Value>),

    /// value
    Output(Value),

    /// value
    Return(Option<Value>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {

    /// operation, lhs, rhs
    Binary(Binary, Value, Value),

    /// operation, value
    Unary(Unary, Value),

    /// func, arguments
    Call(Label, Vec<Value>),

    /// value
    Value(Value),

    /// -
    Input,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Binary {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Ne,
    Lt,
    Lte,
    Gt,
    Gte,
    And,
    Or,
    Xor,
}

impl From<TokenKind> for Binary {
    fn from(kind: TokenKind) -> Self {
        match kind {
            TokenKind::Add => Binary::Add,
            TokenKind::Sub => Binary::Sub,
            TokenKind::Mul => Binary::Mul,
            TokenKind::Div => Binary::Div,
            TokenKind::Mod => Binary::Mod,
            TokenKind::Eq  => Binary::Eq,
            TokenKind::Ne  => Binary::Ne,
            TokenKind::Lt  => Binary::Lt,
            TokenKind::Lte => Binary::Lte,
            TokenKind::Gt  => Binary::Gt,
            TokenKind::Gte => Binary::Gte,
            TokenKind::And => Binary::And,
            TokenKind::Or  => Binary::Or,
            TokenKind::Xor => Binary::Xor,
            _ => unreachable!()
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Unary {
    Not,
}

impl From<TokenKind> for Unary {
    fn from(kind: TokenKind) -> Self {
        match kind {
            TokenKind::Not => Unary::Not,
            _ => unreachable!()
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Label(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct Variable(pub String);

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// value
    String(String),

    /// value
    Integer(i64),

    /// value
    Decimal(f64),

    /// variable
    Variable(Variable),

    /// -
    Null,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: Label,
    pub line: u32,
    pub args: Vec<Variable>,
    pub blocks: Vec<Block>,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub label: Label,
    pub line: u32,
    pub instructions: Vec<Instruction>,
}

#[derive(Debug, Clone)]
pub struct Module {
    pub functions: HashMap<String, Function>
}

impl Module {
    pub fn new() -> Module {
        Module {
            functions: HashMap::new()
        }
    }
}