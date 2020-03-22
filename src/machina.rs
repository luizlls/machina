use crate::object::ObjectValue;
use crate::ast::*;

const STACK_SIZE: usize = 64;

macro_rules! binary {
    ($stack:expr, $function:ident) => {
        let rhs = $stack.pop().unwrap();
        let lhs = $stack.pop().unwrap();
        $stack.push(lhs.$function(rhs));
    };
}

macro_rules! unary {
    ($stack:expr, $function:ident) => {
        let rhs = $stack.pop().unwrap();
        $stack.push(rhs.$function());
    };
}

type Stack = Vec<ObjectValue>;

#[derive(Debug, Clone)]
struct Frame {
    ret: usize,
    locals: Vec<ObjectValue>
}

impl Frame {
    fn new(size: usize, ret: usize) -> Frame {
        Frame {
            ret,
            locals: vec![ObjectValue::Null; size],
        }
    }
}

#[derive(Debug, Clone)]
pub struct Machina {
    stack: Stack,
    frames: Vec<Frame>,
}

impl Machina {
    pub fn new() -> Self {
        Machina {
            stack: Vec::with_capacity(STACK_SIZE),
            frames: Vec::with_capacity(1),
        }
    }

    pub fn run(&mut self, module: Module) {
        let mut ip = 0;
        let mut fp = 0;
        self.frames.push(Frame::new(module.variables, 0));

        while self.frames.len() > 0 {
            let instruction = &module.instructions[ip];
            ip += 1;
            match instruction.kind {
                InstructionKind::Const => {
                    self.stack.push(module.constants[instruction.arg].clone());
                }
                InstructionKind::Load => {
                    self.stack.push(self.frames[fp].locals[instruction.arg].clone());
                }
                InstructionKind::Store => {
                    self.frames[fp].locals[instruction.arg] = self.stack.pop().unwrap();
                }
                InstructionKind::Jump => {
                    ip = instruction.arg;
                }
                InstructionKind::JumpT => {
                    if self.stack.pop().unwrap().boolean() {
                        ip = instruction.arg;
                    }
                }
                InstructionKind::JumpF => {
                    if !self.stack.pop().unwrap().boolean() {
                        ip = instruction.arg;
                    }
                }
                InstructionKind::Call => {
                    self.frames.push(Frame::new(module.variables, ip));
                    fp += 1;
                    ip = instruction.arg;
                }
                InstructionKind::Add => {
                    binary!(self.stack, add);
                }
                InstructionKind::Sub => {
                    binary!(self.stack, sub);
                }
                InstructionKind::Mul => {
                    binary!(self.stack, mul);
                }
                InstructionKind::Div => {
                    binary!(self.stack, div);
                }
                InstructionKind::Mod => {
                    binary!(self.stack, modulus);
                }
                InstructionKind::Eq => {
                    binary!(self.stack, eq);
                }
                InstructionKind::Ne => {
                    binary!(self.stack, ne);
                }
                InstructionKind::Lt => {
                    binary!(self.stack, lt);
                }
                InstructionKind::Lte => {
                    binary!(self.stack, lte);
                }
                InstructionKind::Gt => {
                    binary!(self.stack, gt);
                }
                InstructionKind::Gte => {
                    binary!(self.stack, gte);
                }
                InstructionKind::And => {
                    binary!(self.stack, bit_and);
                }
                InstructionKind::Or => {
                    binary!(self.stack, bit_or);
                }
                InstructionKind::Xor => {
                    binary!(self.stack, bit_xor);
                }
                InstructionKind::Not => {
                    unary!(self.stack, bit_not);
                }
                InstructionKind::Null => {
                    self.stack.push(ObjectValue::Null);
                }
                InstructionKind::Input => {}
                InstructionKind::Output => {
                    println!("{}", self.stack.pop().unwrap());
                }
                InstructionKind::Return => {
                    ip = self.frames[fp].ret;
                    if fp >= 1 {
                       fp -= 1;
                    }
                    self.frames.pop();
                }
            }
        }
    }
}