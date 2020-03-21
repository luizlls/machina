use crate::object::ObjectValue;
use crate::ast::*;

const STACK_SIZE: usize = 64;

macro_rules! binary {
    ($stack:expr, $function:ident) => {
        let rhs = $stack.pop().unwrap();
        let lhs = $stack.pop().unwrap();
        let result = lhs.$function(rhs);
        $stack.push(result);
    };
}

macro_rules! unary {
    ($stack:expr, $function:ident) => {
        let rhs = $stack.pop().unwrap();
        let result = rhs.$function();
        $stack.push(result);
    };
}

type Stack = Vec<ObjectValue>;

#[derive(Debug, Clone)]
pub struct Machina {
    module: Module,
    stack: Stack,
}

impl Machina {
    pub fn new(module: Module) -> Self {
        Machina {
            module,
            stack: Vec::with_capacity(STACK_SIZE),
        }
    }

    pub fn run(&mut self) {
        Machina::call(&mut self.stack, &self.module, &"main".into());
    }

    fn call(stack: &mut Stack, module: &Module, function_name: &String) {
        let function = module.functions.get(function_name).unwrap();

        let mut variables = vec![ObjectValue::Null; function.variables];
        let mut ip = 0;

        while ip < function.instructions.len() {
            let instruction = &function.instructions[ip];
            ip += 1;
            match instruction.kind {
                InstructionKind::Const => {
                    stack.push(function.constants[instruction.arg].clone());
                }
                InstructionKind::Load => {
                    stack.push(variables[instruction.arg].clone());
                }
                InstructionKind::Store => {
                    variables[instruction.arg] = stack.pop().unwrap();
                }
                InstructionKind::Jump => {
                    ip = instruction.arg;
                }
                InstructionKind::JumpT => {
                    if stack.pop().unwrap().boolean() {
                        ip = instruction.arg;
                    }
                }
                InstructionKind::JumpF => {
                    if !stack.pop().unwrap().boolean() {
                        ip = instruction.arg;
                    }
                }
                InstructionKind::Call => {
                    if let ObjectValue::String(name) = &function.constants[instruction.arg] {
                        Machina::call(stack, module, name)
                    }
                }
                InstructionKind::Add => {
                    binary!(stack, add);
                }
                InstructionKind::Sub => {
                    binary!(stack, sub);
                }
                InstructionKind::Mul => {
                    binary!(stack, mul);
                }
                InstructionKind::Div => {
                    binary!(stack, div);
                }
                InstructionKind::Mod => {
                    binary!(stack, modulus);
                }
                InstructionKind::Eq => {
                    binary!(stack, eq);
                }
                InstructionKind::Ne => {
                    binary!(stack, ne);
                }
                InstructionKind::Lt => {
                    binary!(stack, lt);
                }
                InstructionKind::Lte => {
                    binary!(stack, lte);
                }
                InstructionKind::Gt => {
                    binary!(stack, gt);
                }
                InstructionKind::Gte => {
                    binary!(stack, gte);
                }
                InstructionKind::And => {
                    binary!(stack, bit_and);
                }
                InstructionKind::Or => {
                    binary!(stack, bit_or);
                }
                InstructionKind::Xor => {
                    binary!(stack, bit_xor);
                }
                InstructionKind::Not => {
                    unary!(stack, bit_not);
                }
                InstructionKind::Null => {
                    stack.push(ObjectValue::Null);
                }
                InstructionKind::Input => {

                }
                InstructionKind::Output => {
                    println!("{}", stack.pop().unwrap());
                }
                InstructionKind::Return => {
                    return;
                }
            }
        }
    }
}