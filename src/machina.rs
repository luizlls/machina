use crate::{
    bytecode::{
        Constant,
        Function,
        OpCode,
        Operand,
        Register,
    },
    value::Value,
};

use std::fmt::Debug;

const INITIAL_REG_SIZE: usize = 16;


#[derive(Debug)]
pub struct Environment {
    pub functions: Vec<Function>,
    pub constants: Vec<Constant>,
}

impl Environment {

    pub fn new() -> Environment {
        Environment {
            constants: vec![],
            functions: vec![],
        }
    }

    fn get_function(&self, index: usize) -> &Function {
        &self.functions[index]
    }
}

#[derive(Debug)]
pub struct Machina<'a> {
    registers: Vec<Value>,
    bp: usize,
    rp: usize,
    environment: &'a Environment
}

impl<'a> Machina<'a> {
    pub fn new(env: &'a Environment) -> Machina<'a> {
        Machina {
            registers: vec![Value::null(); INITIAL_REG_SIZE],
            bp: 0,
            rp: 0,
            environment: env,
        }
    }

    pub fn call(&mut self, index: usize, first: Register, last: Register) -> Value {

        let function = self.environment.get_function(index);

        self.resize_registers(((last - first) + 1) as usize);

        for (idx, reg) in (first ..= last).enumerate() {
            let new = self.rp + idx as usize;
            let old = self.bp + reg as usize;
            self.registers[new] = self.registers[old];
        }

        let _bp = self.bp;
        let _rp = self.rp;
        self.bp = self.rp;

        let value = self.eval(function);

        self.rp = _rp;
        self.bp = _bp;

        value
    }

    fn eval(&mut self, function: &Function) -> Value {
        self.alloc(function.locals as usize);

        let mut ip  = 0;

        loop {
            let instruction = function.instructions[ip];
            ip += 1;

            match instruction.opcode {
                OpCode::Move => {
                    self.set(instruction.register(0), self.get(instruction.get(1)));
                }
                OpCode::Call => {
                    let first = instruction.register(2);
                    let last  = instruction.register(3);

                    if first > last {
                        panic!("Invalid register range for CALL instruction")
                    }

                    let val = self.call(instruction.function(0) as usize, first, last);

                    self.set(instruction.register(1), val);
                }
                OpCode::Jmp => {
                    ip = instruction.position(0) as usize;
                }
                OpCode::Jt => {
                    let val = self.get(instruction.get(1));
                    if val.is_true() {
                        ip = instruction.position(0) as usize;
                    }
                }
                OpCode::Jf => {
                    let val = self.get(instruction.get(1));
                    if val.is_false() {
                        ip = instruction.position(0) as usize;
                    }
                }
                OpCode::JLt => jump_op!(self, instruction, ip, <),
                OpCode::JLe => jump_op!(self, instruction, ip, <=),
                OpCode::JGt => jump_op!(self, instruction, ip, >),
                OpCode::JGe => jump_op!(self, instruction, ip, >=),
                OpCode::JEq => jump_op!(self, instruction, ip, ==),
                OpCode::JNe => jump_op!(self, instruction, ip, !=),
                OpCode::Lt  => binary_op!(self, instruction, <),
                OpCode::Le  => binary_op!(self, instruction, <=),
                OpCode::Gt  => binary_op!(self, instruction, >),
                OpCode::Ge  => binary_op!(self, instruction, >=),
                OpCode::Eq  => binary_op!(self, instruction, ==),
                OpCode::Ne  => binary_op!(self, instruction, !=),
                OpCode::Add => binary_op!(self, instruction, +),
                OpCode::Sub => binary_op!(self, instruction, -),
                OpCode::Mul => binary_op!(self, instruction, *),
                OpCode::Div => binary_op!(self, instruction, /),
                OpCode::Mod => integer_op!(self, instruction, %),
                OpCode::And => integer_op!(self, instruction, &),
                OpCode::Or  => integer_op!(self, instruction, |),
                OpCode::Xor => integer_op!(self, instruction, ^),
                OpCode::Shl => integer_op!(self, instruction, <<),
                OpCode::Shr => integer_op!(self, instruction, >>),
                OpCode::Not => unary_op!(self, instruction, !),
                OpCode::Ret => {
                    return self.get(instruction.get(0));
                }
                OpCode::Write => {
                    if instruction.get(0) == Operand::None {
                        println!("\n");
                    } else {
                        println!("{}", self.get(instruction.get(0)));
                    }
                }
            }
        }
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
            Operand::Constant(idx) => {
                match self.environment.constants[idx as usize] {
                    Constant::String(_) => {
                        todo!()
                    }
                    Constant::Number(num) => Value::from(num.value()),
                }
            }
            _ => Value::null()
        }
    }

    fn alloc(&mut self, total: usize) {
        self.rp = (self.bp + total as usize) - 1;
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
