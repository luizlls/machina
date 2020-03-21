use crate::object::ObjectValue;
use crate::ast::*;

const STACK_SIZE: usize = 64;

macro_rules! pop_binary {
    ($vm:expr, $func:ident) => {
        let a = $vm.stack.pop().unwrap();
        let b = $vm.stack.pop().unwrap();
        $vm.stack.push(a.$func(b));
    };
}

#[derive(Debug, Clone)]
pub struct Machina {
    module: Module,
    stack: Vec<ObjectValue>,
}

impl Machina {
    pub fn new(module: Module) -> Self {
        Machina {
            module,
            stack: Vec::with_capacity(STACK_SIZE),
        }
    }

    pub fn run(&self) {

    }

    fn call(vm: &mut Machina, function: &Function) {
        let mut variables = vec![ObjectValue::Null; function.variables];
        let mut ip = 0;

        while ip < function.instructions.len() {
            let instruction = &function.instructions[ip];
            ip += 1;
            match instruction.kind {
                InstructionKind::Const => {
                    let value = function.constants[instruction.arg].clone();
                    vm.stack.push(value);
                }
                InstructionKind::Load => {
                    let value = variables[instruction.arg].clone();
                    vm.stack.push(value);
                }
                InstructionKind::Store => {
                    variables[instruction.arg] = vm.stack.pop().unwrap();
                }
                InstructionKind::Jump => {
                    ip = instruction.arg;
                }
                InstructionKind::JumpT => {
                    if vm.stack.pop().unwrap().boolean() {
                        ip = instruction.arg;
                    }
                }
                InstructionKind::JumpF => {
                    if !vm.stack.pop().unwrap().boolean() {
                        ip = instruction.arg;
                    }
                }
                InstructionKind::Call => {
                }
                InstructionKind::Add => {
                    pop_binary!(vm, add);
                }
                InstructionKind::Sub => {
                    pop_binary!(vm, sub);
                }
                InstructionKind::Mul => {
                    pop_binary!(vm, mul);
                }
                InstructionKind::Div => {
                    pop_binary!(vm, div);
                }
                InstructionKind::Mod => {
                    pop_binary!(vm, modulus);
                }
                InstructionKind::Eq => {
                    pop_binary!(vm, eq);
                }
                InstructionKind::Ne => {
                    pop_binary!(vm, ne);
                }
                InstructionKind::Lt => {
                    pop_binary!(vm, lt);
                }
                InstructionKind::Lte => {
                    pop_binary!(vm, lte);
                }
                InstructionKind::Gt => {
                    pop_binary!(vm, gt);
                }
                InstructionKind::Gte => {
                    pop_binary!(vm, gte);
                }
                InstructionKind::And => {
                    pop_binary!(vm, bit_and);
                }
                InstructionKind::Or => {
                    pop_binary!(vm, bit_or);
                }
                InstructionKind::Xor => {
                    pop_binary!(vm, bit_xor);
                }
                InstructionKind::Not => {
                    let value = vm.stack.pop().unwrap();
                    vm.stack.push(value.bit_not());
                }
                InstructionKind::Null => {
                    vm.stack.push(ObjectValue::Null);
                }
                InstructionKind::Input => {

                }
                InstructionKind::Output => {
                    println!("{}", vm.stack.pop().unwrap());
                }
                InstructionKind::Return => {
                    return;
                }
            }
        }
    }
}