use crate::lexer::{Token, TokenKind};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    /// target, expr
    Assignment(Target, Expression),

    /// test, then, else
    If(Value, Label, Label),

    /// possibilities (test, result)
    Case(Vec<(Value, Label)>),

    /// destination
    Jmp(Label),

    /// test, destination
    JmpT(Value, Label),

    /// test, destination
    JmpF(Value, Label),

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
    Neq,
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
            TokenKind::Eq => Binary::Eq,
            TokenKind::Neq => Binary::Neq,
            TokenKind::Lt => Binary::Lt,
            TokenKind::Lte => Binary::Lte,
            TokenKind::Gt => Binary::Gt,
            TokenKind::Gte => Binary::Gte,
            TokenKind::And => Binary::And,
            TokenKind::Or => Binary::Or,
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
pub enum Target {
    Variable(Variable),
    Register(Register),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Register(pub u32);

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// value
    String(String),

    /// value
    Integer(i64),

    /// value
    Decimal(f64),

    /// target
    Target(Target),

    /// -
    Null,
}

#[derive(Debug, Clone)]
pub struct BasicFunction {
    pub name: Label,
    pub line: u32,
    pub tokens: Vec<Token>,
}

#[derive(Debug, Clone)]
pub struct FinalFunction {
    pub name: Label,
    pub line: u32,
    pub args: Vec<Variable>,
    pub registers_size: u32,
    pub blocks: Vec<Block>,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub label: Label,
    pub line: u32,
    pub instructions: Vec<Instruction>,
}

#[derive(Debug, Clone)]
pub enum Function {
    BasicFunction(BasicFunction),
    FinalFunction(FinalFunction)
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