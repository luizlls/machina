use crate::{
    value::Value,
    bytecode::{
        OpCode,
        Operand,
        Register,
        Function,
        Instruction,
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

    pub fn call(&mut self, index: usize, ret: Register, first: Register, last: Register) {

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

        self.eval(function, ret);

        self.rp = _rp;
        self.bp = _bp;
    }

    fn eval(&mut self, function: &Function, ret: Register) {
        self.alloc(function.locals as usize);

        let mut ip  = 0;

        loop {
            let instruction = function.instructions[ip];
            ip += 1;

            match instruction.opcode {
                OpCode::Move => {
                    self.eval_move(&instruction)
                }
                OpCode::Call => {
                    self.eval_call(&instruction)
                }
                OpCode::Jmp => {
                    self.eval_jmp(&instruction, &mut ip)
                }
                OpCode::Jt => {
                    self.eval_jt(&instruction, &mut ip)
                }
                OpCode::Jf => {
                    self.eval_jf(&instruction, &mut ip)
                }
                OpCode::JLt => {
                    self.eval_jlt(&instruction, &mut ip)
                }
                OpCode::JLe => {
                    self.eval_jle(&instruction, &mut ip)
                }
                OpCode::JGt => {
                    self.eval_jgt(&instruction, &mut ip)
                }
                OpCode::JGe => {
                    self.eval_jge(&instruction, &mut ip)
                }
                OpCode::JEq => {
                    self.eval_jeq(&instruction, &mut ip)
                }
                OpCode::JNe => {
                    self.eval_jne(&instruction, &mut ip)
                }
                OpCode::Lt => {
                    self.eval_lt(&instruction)
                }
                OpCode::Le => {
                    self.eval_le(&instruction)
                }
                OpCode::Gt => {
                    self.eval_gt(&instruction)
                }
                OpCode::Ge => {
                    self.eval_ge(&instruction)
                }
                OpCode::Eq => {
                    self.eval_eq(&instruction)
                }
                OpCode::Ne => {
                    self.eval_ne(&instruction)
                }
                OpCode::Add => {
                    self.eval_add(&instruction)
                }
                OpCode::Sub => {
                    self.eval_sub(&instruction)
                }
                OpCode::Mul => {
                    self.eval_mul(&instruction)
                }
                OpCode::Div => {
                    self.eval_div(&instruction)
                }
                OpCode::Mod => {
                    self.eval_mod(&instruction)
                }
                OpCode::And => {
                    self.eval_and(&instruction)
                }
                OpCode::Or => {
                    self.eval_or(&instruction)
                }
                OpCode::Xor => {
                    self.eval_xor(&instruction)
                }
                OpCode::Shl => {
                    self.eval_shl(&instruction)
                }
                OpCode::Shr => {
                    self.eval_shr(&instruction)
                }
                OpCode::Not => {
                    self.eval_not(&instruction)
                }
                OpCode::Ret => {
                    self.eval_ret(&instruction, ret);
                    break;
                }
                OpCode::Write => {
                    self.eval_write(&instruction)
                }
            }
        }
    }

    #[inline(always)]
    fn eval_move(&mut self, instruction: &Instruction) {
        self.set(instruction.register(0), self.get(instruction.get(1)));
    }

    #[inline(always)]
    fn eval_call(&mut self, instruction: &Instruction) {
        let first = instruction.register(2);
        let last  = instruction.register(3);
        if first > last {
            panic!("Invalid register range for CALL instruction")
        }

        let ret = instruction.register(1);

        self.call(instruction.function(0) as usize, ret, first, last);
    }

    #[inline(always)]
    fn eval_jmp(&mut self, instruction: &Instruction, ip: &mut usize) {
        *ip = instruction.position(0) as usize;
    }

    #[inline(always)]
    fn eval_jt(&mut self, instruction: &Instruction, ip: &mut usize) {
        let val = self.get(instruction.get(1));
        if val.is_true() {
            *ip = instruction.position(0) as usize;
        }
    }

    #[inline(always)]
    fn eval_jf(&mut self, instruction: &Instruction, ip: &mut usize) {
        let val = self.get(instruction.get(1));
        if val.is_false() {
            *ip = instruction.position(0) as usize;
        }
    }

    #[inline(always)]
    fn eval_jlt(&mut self, instruction: &Instruction, ip: &mut usize) {
        let a = self.get(instruction.get(1));
        let b = self.get(instruction.get(2));
        if a.get_int() < b.get_int() {
            *ip = instruction.position(0) as usize;
        }
    }

    #[inline(always)]
    fn eval_jle(&mut self, instruction: &Instruction, ip: &mut usize) {
        let a = self.get(instruction.get(1));
        let b = self.get(instruction.get(2));
        if a.get_int() <= b.get_int() {
            *ip = instruction.position(0) as usize;
        }
    }

    #[inline(always)]
    fn eval_jgt(&mut self, instruction: &Instruction, ip: &mut usize) {
        let a = self.get(instruction.get(1));
        let b = self.get(instruction.get(2));
        if a.get_int() > b.get_int() {
            *ip = instruction.position(0) as usize;
        }
    }

    #[inline(always)]
    fn eval_jge(&mut self, instruction: &Instruction, ip: &mut usize) {
        let a = self.get(instruction.get(1));
        let b = self.get(instruction.get(2));
        if a.get_int() >= b.get_int() {
            *ip = instruction.position(0) as usize;
        }
    }

    #[inline(always)]
    fn eval_jeq(&mut self, instruction: &Instruction, ip: &mut usize) {
        let a = self.get(instruction.get(1));
        let b = self.get(instruction.get(2));
        if a.get_int() == b.get_int() {
            *ip = instruction.position(0) as usize;
        }
    }

    #[inline(always)]
    fn eval_jne(&mut self, instruction: &Instruction, ip: &mut usize) {
        let a = self.get(instruction.get(1));
        let b = self.get(instruction.get(2));
        if a.get_int() != b.get_int() {
            *ip = instruction.position(0) as usize;
        }
    }

    #[inline(always)]
    fn eval_lt(&mut self, instruction: &Instruction) {
        let a = self.get(instruction.get(0));
        let b = self.get(instruction.get(1));
        self.set(instruction.register(0), Value::from(a.get_int() < b.get_int()));
    }

    #[inline(always)]
    fn eval_le(&mut self, instruction: &Instruction) {
        let a = self.get(instruction.get(0));
        let b = self.get(instruction.get(1));
        self.set(instruction.register(0), Value::from(a.get_int() <= b.get_int()));
    }

    #[inline(always)]
    fn eval_gt(&mut self, instruction: &Instruction) {
        let a = self.get(instruction.get(0));
        let b = self.get(instruction.get(1));
        self.set(instruction.register(0), Value::from(a.get_int() > b.get_int()));
    }

    #[inline(always)]
    fn eval_ge(&mut self, instruction: &Instruction) {
        let a = self.get(instruction.get(0));
        let b = self.get(instruction.get(1));
        self.set(instruction.register(0), Value::from(a.get_int() >= b.get_int()));
    }

    #[inline(always)]
    fn eval_eq(&mut self, instruction: &Instruction) {
        let a = self.get(instruction.get(0));
        let b = self.get(instruction.get(1));
        self.set(instruction.register(0), Value::from(a.get_int() == b.get_int()));
    }

    #[inline(always)]
    fn eval_ne(&mut self, instruction: &Instruction) {
        let a = self.get(instruction.get(0));
        let b = self.get(instruction.get(1));
        self.set(instruction.register(0), Value::from(a.get_int() != b.get_int()));
    }

    #[inline(always)]
    fn eval_add(&mut self, instruction: &Instruction) {
        let a = self.get(instruction.get(0));
        let b = self.get(instruction.get(1));
        self.set(instruction.register(0), Value::from(a.get_int() + b.get_int()));
    }

    #[inline(always)]
    fn eval_sub(&mut self, instruction: &Instruction) {
        let a = self.get(instruction.get(0));
        let b = self.get(instruction.get(1));
        self.set(instruction.register(0), Value::from(a.get_int() - b.get_int()));
    }

    #[inline(always)]
    fn eval_mul(&mut self, instruction: &Instruction) {
        let a = self.get(instruction.get(0));
        let b = self.get(instruction.get(1));
        self.set(instruction.register(0), Value::from(a.get_int() * b.get_int()));
    }

    #[inline(always)]
    fn eval_div(&mut self, instruction: &Instruction) {
        let a = self.get(instruction.get(0));
        let b = self.get(instruction.get(1));
        self.set(instruction.register(0), Value::from(a.get_int() / b.get_int()));
    }

    #[inline(always)]
    fn eval_mod(&mut self, instruction: &Instruction) {
        let a = self.get(instruction.get(0));
        let b = self.get(instruction.get(1));
        self.set(instruction.register(0), Value::from(a.get_int() % b.get_int()));
    }

    #[inline(always)]
    fn eval_and(&mut self, instruction: &Instruction) {
        let a = self.get(instruction.get(0));
        let b = self.get(instruction.get(1));
        self.set(instruction.register(0), Value::from(a.get_int() & b.get_int()));
    }

    #[inline(always)]
    fn eval_or(&mut self, instruction: &Instruction) {
        let a = self.get(instruction.get(0));
        let b = self.get(instruction.get(1));
        self.set(instruction.register(0), Value::from(a.get_int() | b.get_int()));
    }

    #[inline(always)]
    fn eval_xor(&mut self, instruction: &Instruction) {
        let a = self.get(instruction.get(0));
        let b = self.get(instruction.get(1));
        self.set(instruction.register(0), Value::from(a.get_int() ^ b.get_int()));
    }

    #[inline(always)]
    fn eval_shl(&mut self, instruction: &Instruction) {
        let a = self.get(instruction.get(0));
        let b = self.get(instruction.get(1));
        self.set(instruction.register(0), Value::from(a.get_int() << b.get_int()));
    }

    #[inline(always)]
    fn eval_shr(&mut self, instruction: &Instruction) {
        let a = self.get(instruction.get(0));
        let b = self.get(instruction.get(1));
        self.set(instruction.register(0), Value::from(a.get_int() >> b.get_int()));
    }

    #[inline(always)]
    fn eval_not(&mut self, instruction: &Instruction) {
        let a = self.get(instruction.get(0));
        self.set(instruction.register(0), Value::from(!a.get_int()));
    }

    #[inline(always)]
    fn eval_ret(&mut self, instruction: &Instruction, ret: Register) {
        self.set(ret, self.get(instruction.get(0)));
    }

    #[inline(always)]
    fn eval_write(&mut self, instruction: &Instruction) {
        println!("{:#?}", self.get(instruction.get(0)));
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
