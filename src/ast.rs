use crate::lexer::Token;

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
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    
    /// value
    Value(Value),

    /// -
    Null,
    
    /// -
    Input,

    Assignment(String, Box<Expression>),

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
pub struct Label(String);

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// value
    String(String),

    /// value
    Integer(i64),

    /// value
    Decimal(f64),

    /// label
    Variable(String),

    /// label
    Label(Label),
}

#[derive(Debug, Clone)]
pub struct RawFunction {
    pub tokens: Vec<Token>,
    pub metadata: MetaData,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub instructions: Vec<Instruction>,
}

#[derive(Debug, Clone)]
pub struct MetaData {
    pub name: String,
    pub line: u32,
}
