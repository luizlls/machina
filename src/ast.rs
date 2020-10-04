
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
    operands: [Operand; 4]
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
}

#[derive(Debug, Clone)]
pub enum Constant {

    String(String),

    Number(f64),

    Integer(i32),
}
