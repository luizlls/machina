use crate::lexer::Token;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    /// value
    Exec(Label),

    /// test, then, else
    If(Value, Label, Label),

    /// destination
    Jmp(Label),

    /// test, destination
    JmpT(Value, Label),

    /// test, destination
    JmpF(Value, Label),

    /// value
    Output(Value),

    /// variable_name, expr
    VariableAssignment(Variable, Box<Expression>),

    /// register_num, expr
    RegisterAssignment(Register, Box<Expression>)
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {

    /// value
    Value(Value),

    /// -
    Null,

    /// -
    Input,

    /// proc, arguments
    Call(Label, Vec<Value>),

    /// possibilities (test, result)
    Case(Vec<(Value, Label)>),

    /// operation, lhs, rhs
    Binary(Binary, Value, Value),

    /// operation, value
    Unary(Unary, Value),
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

#[derive(Debug, Clone, PartialEq)]
pub enum Unary {
    Not,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Label(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct Variable(pub String);

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

    /// label
    Variable(Variable),

    /// label
    Label(Label),
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