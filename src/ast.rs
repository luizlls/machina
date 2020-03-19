use crate::object::ObjectValue;
use crate::lexer::TokenKind;

use std::collections::HashMap;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum InstructionKind {
    Const,
    Load,
    Store,
    Jump,
    JumpT,
    JumpF,
    Call,
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
    Not,
    Input,
    Output,
    Return,
}

impl InstructionKind {
    pub fn from_token(kind: TokenKind) -> Option<Self> {
        match kind {
            TokenKind::Const  => Some(InstructionKind::Const),
            TokenKind::Load   => Some(InstructionKind::Load),
            TokenKind::Store  => Some(InstructionKind::Store),
            TokenKind::Jump   => Some(InstructionKind::Jump),
            TokenKind::JumpT  => Some(InstructionKind::JumpT),
            TokenKind::JumpF  => Some(InstructionKind::JumpF),
            TokenKind::Call   => Some(InstructionKind::Call),
            TokenKind::Add    => Some(InstructionKind::Add),
            TokenKind::Sub    => Some(InstructionKind::Sub),
            TokenKind::Mul    => Some(InstructionKind::Mul),
            TokenKind::Div    => Some(InstructionKind::Div),
            TokenKind::Mod    => Some(InstructionKind::Mod),
            TokenKind::Eq     => Some(InstructionKind::Eq),
            TokenKind::Ne     => Some(InstructionKind::Ne),
            TokenKind::Lt     => Some(InstructionKind::Lt),
            TokenKind::Lte    => Some(InstructionKind::Lte),
            TokenKind::Gt     => Some(InstructionKind::Gt),
            TokenKind::Gte    => Some(InstructionKind::Gte),
            TokenKind::And    => Some(InstructionKind::And),
            TokenKind::Or     => Some(InstructionKind::Or),
            TokenKind::Xor    => Some(InstructionKind::Xor),
            TokenKind::Not    => Some(InstructionKind::Not),
            TokenKind::Input  => Some(InstructionKind::Input),
            TokenKind::Output => Some(InstructionKind::Output),
            TokenKind::Return => Some(InstructionKind::Return),
            _ => None
        }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Identifier(pub String);

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
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

    /// Label
    Label(Label),

    /// Identifier
    Identifier(Identifier),

    /// -
    Null,
}

#[derive(Debug, Clone)]
pub struct PreInstruction {
    pub kind: InstructionKind,
    pub line: u32,
    pub arg : Option<Value>,
}

#[derive(Debug, Clone)]
pub struct Instruction {
    pub kind: InstructionKind,
    pub arg : u16,
    pub line: u32,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub line: u32,
    pub local_values: Vec<ObjectValue>,
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