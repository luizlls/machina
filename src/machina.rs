use crate::{
    value::Value,
    ast::{
        Instruction,
        OpCode,
        Operand,
        Register
    },
};

use std::fmt::Debug;

const INITIAL_REG_SIZE: usize = 16;

#[derive(Debug)]
pub struct Environment {
    pub functions: Vec<Function>,
}

impl Environment {

    pub fn new() -> Environment {
        Environment {
            functions: vec![],
        }
    }

    fn get_function(&self, index: usize) -> &Function {
        &self.functions[index]
    }
}

#[derive(Debug)]
pub struct Machine<'a> {
    registers: Vec<Value>,
    bp: usize,
    rp: usize,
    environment: &'a Environment
}

impl<'a> Machine<'a> {
    pub fn new(env: &'a Environment) -> Machine<'a> {
        Machine {
            registers: vec![Value::null(); INITIAL_REG_SIZE],
            bp: 0,
            rp: 0,
            environment: env,
        }
    }

    fn alloc(&mut self, total: usize) {
        self.rp = (self.bp + total as usize) - 1;
    }

    pub fn call(&mut self, index: usize, first: usize, last: usize) -> Value {
        let function = self.environment.get_function(index);

        match function {
            Function::Common(ref common) => {
                self.call_common(common, first, last)
            }
            Function::Native(ref native) => {
                self.call_native(native, first, last)
            }
        }
    }

    fn call_common(&mut self, f: &CommonFunction, first: usize, last: usize) -> Value {

        self.resize_registers(((last - first) + 1) as usize);

        for (idx, reg) in (first ..= last).enumerate() {
            let new = self.rp + idx as usize;
            let old = self.bp + reg as usize;
            self.registers[new] = self.registers[old];
        }

        let _bp = self.bp;
        let _rp = self.rp;
        self.bp = self.rp;

        let value = f.call(self);

        self.rp = _rp;
        self.bp = _bp;

        value
    }

    fn call_native(&mut self, NativeFunction(fun): &NativeFunction, first: usize, last: usize) -> Value {
        fun(&self.registers[self.bp + first ..= self.bp + last])
    }

    #[inline(always)]
    fn set(&mut self, reg: Register, value: Value) {
        self.registers[self.bp + reg as usize] = value;
    }

    #[inline(always)]
    fn get(&self, value: Operand) -> Value {
        match value {
            Operand::Register(r) => {
                self.registers[self.bp + r as usize]
            }
            Operand::Immediate(imm) => {
                Value::from(imm)
            }
            Operand::Constant(_idx) => {
                todo!()
            }
            Operand::Function(idx) => {
                Value::function(idx as u32)
            }
            _ => Value::null()
        }
    }

    #[inline(always)]
    fn get_function(&self, value: Operand) -> usize {
        match value {
            Operand::Register(r) => {
                self.registers[self.bp + r as usize].get_fun_uncheked() as usize
            }
            Operand::Function(idx) => {
                idx as usize
            }
            _ => {
                panic!("Only `Register` or `Function` can be used as function pointers")
            }
        }
    }

    fn resize_registers(&mut self, total: usize) {
        let curr = self.registers.len();
        let diff = (curr as isize) - (self.rp + total) as isize;
        if diff <= 0 {
            let new_size = 1.5 * curr as f32;
            self.registers.resize(new_size as usize, Value::null());
        }
    }
}

#[derive(Debug)]
pub enum Function {
    Common(CommonFunction),
    Native(NativeFunction),
}

pub struct NativeFunction(pub fn(args: &[Value]) -> Value);

impl Debug for NativeFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<<native>>")
    }
}

#[derive(Debug, Clone)]
pub struct CommonFunction {
    pub locals: u8,
    pub instructions: Vec<Instruction>
}

impl CommonFunction {

    pub fn new(locals: u8, instructions: Vec<Instruction>) -> CommonFunction {
        CommonFunction {
            locals,
            instructions
        }
    }

    fn call(&self, vm: &mut Machine) -> Value {
        vm.alloc(self.locals as usize);

        let mut ip  = 0;

        loop {
            let instruction = self.instructions[ip];
            ip += 1;

            match instruction.opcode {
                OpCode::Move => {
                    vm.set(instruction.register(0), vm.get(instruction.get(1)));
                }
                OpCode::Call => {
                    let first = instruction.register(2) as usize;
                    let last  = instruction.register(3) as usize;
                    if first > last {
                        panic!("Invalid register range for CALL instruction")
                    }

                    let fun = vm.get_function(instruction.get(1));
                    let val = vm.call(fun, first, last);

                    vm.set(instruction.register(0), val);
                }
                OpCode::Jmp => {
                    ip = instruction.position(0) as usize;
                }
                OpCode::Jt => {
                    let val = vm.get(instruction.get(1));
                    if val.is_true() {
                        ip = instruction.position(0) as usize;
                    }
                }
                OpCode::Jf => {
                    let val = vm.get(instruction.get(1));
                    if val.is_false() {
                        ip = instruction.position(0) as usize;
                    }
                }
                OpCode::JLt => {
                    let a = vm.get(instruction.get(1));
                    let b = vm.get(instruction.get(2));
                    if a.get_int() < b.get_int() {
                        ip = instruction.position(0) as usize;
                    }
                }
                OpCode::JLe => {
                    let a = vm.get(instruction.get(1));
                    let b = vm.get(instruction.get(2));
                    if a.get_int() <= b.get_int() {
                        ip = instruction.position(0) as usize;
                    }
                }
                OpCode::JGt => {
                    let a = vm.get(instruction.get(1));
                    let b = vm.get(instruction.get(2));
                    if a.get_int() > b.get_int() {
                        ip = instruction.position(0) as usize;
                    }
                }
                OpCode::JGe => {
                    let a = vm.get(instruction.get(1));
                    let b = vm.get(instruction.get(2));
                    if a.get_int() >= b.get_int() {
                        ip = instruction.position(0) as usize;
                    }
                }
                OpCode::JEq => {
                    let a = vm.get(instruction.get(1));
                    let b = vm.get(instruction.get(2));
                    if a.get_int() == b.get_int() {
                        ip = instruction.position(0) as usize;
                    }
                }
                OpCode::JNe => {
                    let a = vm.get(instruction.get(1));
                    let b = vm.get(instruction.get(2));
                    if a.get_int() != b.get_int() {
                        ip = instruction.position(0) as usize;
                    }
                }
                OpCode::Lt => {
                    let a = vm.get(instruction.get(0));
                    let b = vm.get(instruction.get(1));
                    vm.set(instruction.register(0), Value::from(a.get_int() < b.get_int()));
                }
                OpCode::Le => {
                    let a = vm.get(instruction.get(0));
                    let b = vm.get(instruction.get(1));
                    vm.set(instruction.register(0), Value::from(a.get_int() <= b.get_int()));
                }
                OpCode::Gt => {
                    let a = vm.get(instruction.get(0));
                    let b = vm.get(instruction.get(1));
                    vm.set(instruction.register(0), Value::from(a.get_int() > b.get_int()));
                }
                OpCode::Ge => {
                    let a = vm.get(instruction.get(0));
                    let b = vm.get(instruction.get(1));
                    vm.set(instruction.register(0), Value::from(a.get_int() >= b.get_int()));
                }
                OpCode::Eq => {
                    let a = vm.get(instruction.get(0));
                    let b = vm.get(instruction.get(1));
                    vm.set(instruction.register(0), Value::from(a.get_int() == b.get_int()));
                }
                OpCode::Ne => {
                    let a = vm.get(instruction.get(0));
                    let b = vm.get(instruction.get(1));
                    vm.set(instruction.register(0), Value::from(a.get_int() != b.get_int()));
                }
                OpCode::Add => {
                    let a = vm.get(instruction.get(0));
                    let b = vm.get(instruction.get(1));
                    vm.set(instruction.register(0), Value::from(a.get_int() + b.get_int()));
                }
                OpCode::Sub => {
                    let a = vm.get(instruction.get(0));
                    let b = vm.get(instruction.get(1));
                    vm.set(instruction.register(0), Value::from(a.get_int() - b.get_int()));
                }
                OpCode::Mul => {
                    let a = vm.get(instruction.get(0));
                    let b = vm.get(instruction.get(1));
                    vm.set(instruction.register(0), Value::from(a.get_int() * b.get_int()));
                }
                OpCode::Div => {
                    let a = vm.get(instruction.get(0));
                    let b = vm.get(instruction.get(1));
                    vm.set(instruction.register(0), Value::from(a.get_int() / b.get_int()));
                }
                OpCode::Mod => {
                    let a = vm.get(instruction.get(0));
                    let b = vm.get(instruction.get(1));
                    vm.set(instruction.register(0), Value::from(a.get_int() % b.get_int()));
                }
                OpCode::And => {
                    let a = vm.get(instruction.get(0));
                    let b = vm.get(instruction.get(1));
                    vm.set(instruction.register(0), Value::from(a.get_int() & b.get_int()));
                }
                OpCode::Or => {
                    let a = vm.get(instruction.get(0));
                    let b = vm.get(instruction.get(1));
                    vm.set(instruction.register(0), Value::from(a.get_int() | b.get_int()));
                }
                OpCode::Xor => {
                    let a = vm.get(instruction.get(0));
                    let b = vm.get(instruction.get(1));
                    vm.set(instruction.register(0), Value::from(a.get_int() ^ b.get_int()));
                }
                OpCode::Shl => {
                    let a = vm.get(instruction.get(0));
                    let b = vm.get(instruction.get(1));
                    vm.set(instruction.register(0), Value::from(a.get_int() << b.get_int()));
                }
                OpCode::Shr => {
                    let a = vm.get(instruction.get(0));
                    let b = vm.get(instruction.get(1));
                    vm.set(instruction.register(0), Value::from(a.get_int() >> b.get_int()));
                }
                OpCode::Not => {
                    let a = vm.get(instruction.get(0));
                    vm.set(instruction.register(0), Value::from(!a.get_int()));
                }
                OpCode::Ret => {
                    return vm.get(instruction.get(0))
                }
                OpCode::Write => {
                    println!("{:#?}", vm.get(instruction.get(0)));
                }
            }
        }
    }
}
