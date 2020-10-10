use std::collections::HashMap;

use crate::{
    bytecode::{
        OpCode,
        Module,
        Function,
        Constant,
        Operand,
        Instruction,
        Register,
    },
    error:: {
        MachinaError,
        MachinaErrorKind,
    },
    lexer::{
        Lexer,
        Token,
    }
};

type ParserResult<T> = Result<T, MachinaError>;

#[derive(Debug, Clone)]
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Parser {
        Parser {
            lexer: Lexer::new(source),
            token: Token::EOF,
        }
    }

    pub fn parse(mut self) -> ParserResult<Module> {
        self.next()?;

        let mut functions = vec![];

        while !self.token_is(Token::EOF) {
            functions.push(self.parse_function()?);
        }

        self.build(functions)
    }

    pub fn build(mut self, functions: Vec<PreFunction>) -> ParserResult<Module> {

        let indexes = functions.iter()
            .enumerate()
            .map(|(idx, function)| {
                (function.name.clone(), idx)
            })
            .collect::<HashMap<_,_>>();

        let mut constants = vec![];

        let functions = functions
            .into_iter()
            .map(|function| {
                self.build_function(function, &indexes, &mut constants)
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Module { functions, constants })
    }

    fn build_function(&mut self, function: PreFunction, functions: &HashMap<String, usize>, constants: &mut Vec<Constant>) 
        -> ParserResult<Function>
    {
        let mut labels = HashMap::new();
        let mut count = 0;

        for block in function.blocks.iter() {
            labels.insert(block.label.clone(), count);
            count += block.instructions.len();
        }

        let mut registers = HashMap::new();

        let instructions = function.blocks
            .into_iter()
            .map(|b| b.instructions)
            .flatten()
            .map(|instruction| {
                self.build_instruction(instruction, &labels, &mut registers, functions, constants)
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Function { locals: registers.len() as u8, instructions })
    }

    fn build_instruction(&mut self, function: PreInstruction, labels: &HashMap<String, usize>, registers: &mut HashMap<Register, Register>, functions: &HashMap<String, usize>, constants: &mut Vec<Constant>) 
        -> ParserResult<Instruction>
    {
        let mut operands = [Operand::None; 4];

        for (i, operand) in function.operands.into_iter().enumerate() {
            operands[i] = match operand {
                PreOperand::String(string) => {
                    self.define_constant(Constant::String(string), constants)
                }
                PreOperand::Number(number) => {
                    let num = number.parse::<f64>().unwrap();
                    if num <= f32::MAX as f64 {
                        Operand::Immediate((num as f32) as i32)
                    } else {
                        self.define_constant(Constant::Number(num.into()), constants)
                    }
                }
                PreOperand::Register(register) => {
                    let register = register.parse::<u16>().ok()
                        .ok_or({
                            MachinaError {
                                kind: MachinaErrorKind::InvalidRegister(register),
                                line:  function.line
                            }
                        })?;

                    self.define_register(register, registers)
                }
                PreOperand::Function(name) => {
                    let function = functions.get(&name)
                        .ok_or({
                            MachinaError {
                                kind: MachinaErrorKind::FunctionNotFound(name),
                                line:  function.line
                            }
                        })?;
                    
                    Operand::Function(*function as u16)
                }
                PreOperand::Label(label) => {
                    let position = labels.get(&label)
                        .ok_or({
                            MachinaError {
                                kind: MachinaErrorKind::TargetNotFound(label),
                                line:  function.line
                            }
                        })?;
                    
                    Operand::Position(*position as u16)
                }
            };
        }

        Ok(Instruction { opcode: function.opcode, operands })
    }

    fn define_constant(&self, constant: Constant, constants: &mut Vec<Constant>) -> Operand {
        let index = constants.len();
        constants.push(constant);

        Operand::Constant(index as u16)
    }

    fn define_register(&self, register: Register, registers: &mut HashMap<Register, Register>) -> Operand {
        let index = registers.len() as Register;
        let register = registers.entry(register).or_insert(index);
        
        Operand::Register(*register)
    }

    fn parse_function(&mut self) -> ParserResult<PreFunction> {

        let name = self.take(Token::Function)?;

        let mut blocks = vec![];

        blocks.push(self.parse_block("<main>".into())?);
        
        while self.token_is(Token::Label) {
            let label = self.take(Token::Label)?;
            let block = self.parse_block(label)?;
            blocks.push(block);
        }

        Ok(PreFunction { name, blocks })
    }

    fn parse_block(&mut self, label: String) -> ParserResult<Block> {
        let mut instructions = vec![];

        while !self.token_is(Token::Label)
          &&  !self.token_is(Token::Function)
          &&  !self.token_is(Token::EOF) {
            instructions.push(self.parse_instruction()?);
        }

        Ok(Block { label, instructions })
    }

    fn parse_instruction(&mut self) -> ParserResult<PreInstruction> {
        match self.token {
            Token::Call => self.parse_call_instruction(),
            Token::Move => self.parse_move_instruction(),

            Token::Jmp
          | Token::Jt
          | Token::Jf
          | Token::JLt
          | Token::JLe
          | Token::JGt
          | Token::JGe
          | Token::JEq
          | Token::JNe => self.parse_jump_instructions(),

            Token::Lt
          | Token::Le
          | Token::Gt
          | Token::Ge
          | Token::Eq
          | Token::Ne
          | Token::Add
          | Token::Sub
          | Token::Mul
          | Token::Div
          | Token::Mod
          | Token::And
          | Token::Or
          | Token::Xor
          | Token::Shl
          | Token::Shr => self.parse_binary_instructions(),

            Token::Ret
          | Token::Not
          | Token::Write => self.parse_unary_instructions(),

            _ => {
                return Err(self.unexpected(&[Token::Instruction]));
            }
        }
    }

    fn parse_call_instruction(&mut self) -> ParserResult<PreInstruction> {
        self.eat(Token::Call)?;

        let operands = vec![
            self.parse_operand(Token::Function, true)?,
            self.parse_operand(Token::Register, true)?,
            self.parse_operand(Token::Register, true)?,
            self.parse_operand(Token::Register, false)?,
        ];

        let line = self.line();

        Ok(PreInstruction { opcode: OpCode::Call, line, operands })
    }

    fn parse_move_instruction(&mut self) -> ParserResult<PreInstruction> {
        self.eat(Token::Move)?;

        let operands = vec![
            self.parse_operand(Token::Register, true)?,
            self.parse_operand(Token::Operand, false)?,
        ];

        let line = self.line();

        Ok(PreInstruction { opcode: OpCode::Move, line, operands })
    }

    fn parse_jump_instructions(&mut self) -> ParserResult<PreInstruction> {
        let opcode = match self.token {
            Token::Jmp => OpCode::Jmp,
            Token::Jt  => OpCode::Jt,
            Token::Jf  => OpCode::Jf,
            Token::JLt => OpCode::JLt,
            Token::JLe => OpCode::JLe,
            Token::JGt => OpCode::JGt,
            Token::JGe => OpCode::JGe,
            Token::JEq => OpCode::JEq,
            Token::JNe => OpCode::JNe,
            _ => {
                return Err(self.unexpected(&[Token::Instruction]));
            }
        };

        self.next()?;

        let mut operands = vec![
            self.parse_operand(Token::Label, true)?
        ];

        match opcode {
            OpCode::Jmp => {},
            OpCode::Jt
          | OpCode::Jf => {
                operands.push(self.parse_operand(Token::Register, true)?);
            }
            OpCode::JLt
          | OpCode::JLe
          | OpCode::JGt
          | OpCode::JGe
          | OpCode::JEq
          | OpCode::JNe => {
                operands.push(self.parse_operand(Token::Register, true)?);
                operands.push(self.parse_operand(Token::Operand, false)?);
          }
            _ => unreachable!()
        };

        let line = self.line();

        Ok(PreInstruction { opcode, line, operands })
    }

    fn parse_unary_instructions(&mut self) -> ParserResult<PreInstruction> {
        let opcode = match self.token {
            Token::Not => OpCode::Not,
            Token::Ret => OpCode::Ret,
            Token::Write => OpCode::Write,
            _ => {
                return Err(self.unexpected(&[Token::Instruction]));
            }
        };

        self.next()?;

        let operands = vec![
            self.parse_operand(Token::Register, false)?,
        ];

        let line = self.line();

        Ok(PreInstruction { opcode, line, operands })
    }

    fn parse_binary_instructions(&mut self) -> ParserResult<PreInstruction> {
        let opcode = match self.token {
            Token::Lt => OpCode::Lt,
            Token::Le => OpCode::Le,
            Token::Gt => OpCode::Gt,
            Token::Ge => OpCode::Ge,
            Token::Eq => OpCode::Eq,
            Token::Ne => OpCode::Ne,
            Token::Add => OpCode::Add,
            Token::Sub => OpCode::Sub,
            Token::Mul => OpCode::Mul,
            Token::Div => OpCode::Div,
            Token::Mod => OpCode::Mod,
            Token::And => OpCode::And,
            Token::Or  => OpCode::Or,
            Token::Xor => OpCode::Xor,
            Token::Shl => OpCode::Shl,
            Token::Shr => OpCode::Shr,
            _ => {
                return Err(self.unexpected(&[Token::Instruction]));
            }
        };

        self.next()?;

        let operands = vec![
            self.parse_operand(Token::Register, true)?,
            self.parse_operand(Token::Operand, false)?,
        ];

        let line = self.line();

        Ok(PreInstruction { opcode, line, operands })
    }

    fn parse_operand(&mut self, kind: Token, eat_comma: bool) -> ParserResult<PreOperand> {
        if kind == Token::Operand {
            self.expect_one_of(&[Token::String, Token::Number, Token::Register])?;
        } else {
            self.expect_one_of(&[kind])?;
        }

        let operand = match self.token {
            Token::String => PreOperand::String(self.take(Token::String)?),
            Token::Number => PreOperand::Number(self.take(Token::Number)?),
            Token::Register => PreOperand::Register(self.take(Token::Register)?),
            Token::Function => PreOperand::Function(self.take(Token::Function)?),
            Token::Label => PreOperand::Label(self.take(Token::Label)?),
            _ => {
                return Err(
                    self.unexpected(&[Token::String, Token::Number, Token::Register, Token::Function, Token::Label])
                );
            }
        };

        if eat_comma {
            self.eat(Token::Comma)?;
        }

        Ok(operand)
    }

    fn next(&mut self) -> ParserResult<()> {
        self.token = if let Some(token) = self.lexer.next() {
            token?
        } else {
            Token::EOF
        };
        Ok(())
    }

    fn eat(&mut self, tkn: Token) -> ParserResult<()> {
        if self.token == tkn {
            Ok(self.next()?)
        } else {
            return Err(self.unexpected(&[tkn]))
        }
    }

    fn take(&mut self, tkn: Token) -> ParserResult<String> {
        let value = if self.token == tkn {
            self.lexer.take_value().unwrap()
        } else {
            return Err(self.unexpected(&[tkn]));
        };

        self.next()?;

        Ok(value)
    }

    fn line(&self) -> usize {
        self.lexer.line()
    }

    fn token_is(&self, tkn: Token) -> bool {
        self.token == tkn
    }

    fn expect_one_of(&self, tokens: &[Token]) -> ParserResult<()> {
        if tokens.contains(&self.token) {
            Ok(())
        } else {
            Err(self.unexpected(tokens))
        }
    }
    
    fn unexpected(&self, tokens: &[Token]) -> MachinaError {
        let expected = tokens
            .iter()
            .map(|t| format!("`{}`", t)).collect::<Vec<_>>().join(", ");

        MachinaError {
            kind: MachinaErrorKind::Expected(format!("{}", expected), format!("{}", self.token)),
            line: self.line()
        }
    }
}


#[derive(Debug, Clone)]
pub struct PreFunction {
    name: String,
    blocks: Vec<Block>
}

#[derive(Debug, Clone)]
struct Block {
    label: String,
    instructions: Vec<PreInstruction>,
}

#[derive(Debug, Clone)]
pub struct PreInstruction {
    pub opcode: OpCode,
    pub line: usize,
    pub operands: Vec<PreOperand>,
}

#[derive(Debug, Clone)]
pub enum PreOperand {
    String(String),

    Number(String),

    Register(String),

    Function(String),

    Label(String)
}
