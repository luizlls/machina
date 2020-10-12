use std::{cmp::Ordering, hash::{Hash, Hasher}, ops::Deref};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OpCode {
    Call,
    Ret,
    Move,
    Jmp,
    Jt,
    Jf,
    JLt,
    JLe,
    JGt,
    JGe,
    JEq,
    JNe,
    Lt,
    Le,
    Gt,
    Ge,
    Eq,
    Ne,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Not,
    And,
    Or,
    Xor,
    Shl,
    Shr,
    Write,
}

pub type Immediate = i32;
pub type Position  = u16;
pub type Register  = u16;
pub type ConstantIdx = u16;
pub type FunctionIdx = u16;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operand {
    None,
    Immediate(Immediate),
    Position(Position),
    Register(Register),
    Function(FunctionIdx),
    Constant(ConstantIdx),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Instruction {
    pub opcode: OpCode,
    pub operands: [Operand; 4]
}

impl Instruction {

    #[inline(always)]
    pub fn new(opcode: OpCode, operands: [Operand; 4]) -> Instruction {
        Instruction {
            opcode,
            operands
        }
    }

    #[inline(always)]
    pub fn get(&self, arg: usize) -> Operand {
        self.operands[arg]
    }

    #[inline(always)]
    pub fn register(&self, arg: usize) -> Register {
        if let Operand::Register(r) = self.operands[arg] {
            r
        } else {
            panic!("Operand is not a register")
        }
    }

    #[inline(always)]
    pub fn position(&self, arg: usize) -> Position {
        if let Operand::Position(p) = self.operands[arg] {
            p
        } else {
            panic!("Operand is not a position")
        }
    }

    #[inline(always)]
    pub fn immediate(&self, arg: usize) -> Immediate {
        if let Operand::Immediate(i) = self.operands[arg] {
            i
        } else {
            panic!("Operand is not a immediate value")
        }
    }

    #[inline(always)]
    pub fn function(&self, arg: usize) -> FunctionIdx {
        if let Operand::Function(f) = self.operands[arg] {
            f
        } else {
            panic!("Operand is not a function index")
        }
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Constant {

    String(String),

    Number(Number),
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


#[derive(Debug, Clone)]
pub struct Function {
    pub locals: u8,
    pub instructions: Vec<Instruction>
}

impl Function {

    pub fn new(locals: u8, instructions: Vec<Instruction>) -> Function {
        Function {
            locals,
            instructions
        }
    }
}


#[derive(Debug, Clone)]
pub struct Module {
    pub functions: Vec<Function>,
    pub constants: Vec<Constant>,
}