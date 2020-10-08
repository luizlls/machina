use std::collections::HashMap;

use crate::{
    bytecode::{
        OpCode,
        Module,
        Function,
        Constant,
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
    functions: Vec<Function>,
    constants: Vec<Constant>,
    function_indexes: HashMap<String, usize>,
    constant_indexes: HashMap<String, usize>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Parser {
        Parser {
            lexer: Lexer::new(source),
            token: Token::EOF,
            functions: vec![],
            constants: vec![],
            function_indexes: HashMap::new(),
            constant_indexes: HashMap::new(),
        }
    }

    pub fn parse(mut self) -> Result<Module, MachinaError> {
        let _ = self.next();

        while !self.token_is(Token::EOF) {
            let (name, function) = self.parse_function()?;
            let index = self.functions.len();
            self.functions.push(function);
            self.function_indexes.insert(name, index);
        }

        Ok(Module {
            constants: self.constants,
            functions: self.functions,
        })
    }

    fn parse_function(&mut self) -> ParserResult<(String, Function)> {

        let name = self.eat_value(Token::Function)?;

        let mut blocks = HashMap::new();
        let mut count = 0;

        let initial_block = self.parse_block(0)?;
        count += initial_block.instructions.len();
        blocks.insert("<main>".into(), initial_block);
        
        while self.token_is(Token::Label) {
            let label = self.eat_value(Token::Label)?;
            let block = self.parse_block(count)?;
            count = count + block.instructions.len();
            
            blocks.insert(label, block);
        }

        Ok((name, Function { locals: 0, instructions: vec![] }))
    }

    fn parse_block(&mut self, line: usize) -> ParserResult<Block> {
        let mut instructions = vec![];

        while !self.token_is(Token::Label)
          &&  !self.token_is(Token::Function)
          &&  !self.token_is(Token::EOF) {
            instructions.push(self.parse_pre_instruction()?);
        }

        Ok(Block { line, instructions })
    }

    fn parse_pre_instruction(&mut self) -> ParserResult<PreInstruction> {
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
            self.parse_pre_operand(Token::Function, true)?,
            self.parse_pre_operand(Token::Register, true)?,
            self.parse_pre_operand(Token::Register, true)?,
            self.parse_pre_operand(Token::Register, false)?,
        ];

        Ok(PreInstruction { opcode: OpCode::Call, operands })
    }

    fn parse_move_instruction(&mut self) -> ParserResult<PreInstruction> {
        self.eat(Token::Move)?;

        let operands = vec![
            self.parse_pre_operand(Token::Register, true)?,
            self.parse_pre_operand(Token::Operand, false)?,
        ];

        Ok(PreInstruction { opcode: OpCode::Move, operands })
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
            self.parse_pre_operand(Token::Label, true)?
        ];

        match opcode {
            OpCode::Jmp => {},
            OpCode::Jt
          | OpCode::Jf => {
                operands.push(self.parse_pre_operand(Token::Register, true)?);
            }
            OpCode::JLt
          | OpCode::JLe
          | OpCode::JGt
          | OpCode::JGe
          | OpCode::JEq
          | OpCode::JNe => {
                operands.push(self.parse_pre_operand(Token::Register, true)?);
                operands.push(self.parse_pre_operand(Token::Operand, false)?);
          }
            _ => unreachable!()
        };

        Ok(PreInstruction { opcode, operands })
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
            self.parse_pre_operand(Token::Register, false)?,
        ];

        Ok(PreInstruction { opcode, operands })
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
            self.parse_pre_operand(Token::Register, true)?,
            self.parse_pre_operand(Token::Operand, false)?,
        ];

        Ok(PreInstruction { opcode, operands })
    }

    fn parse_pre_operand(&mut self, kind: Token, eat_comma: bool) -> ParserResult<PreOperand> {
        if kind == Token::Operand {
            self.expect_one_of(&[Token::String, Token::Number, Token::Register])?;
        } else {
            self.expect_one_of(&[kind])?;
        }

        let operand = match self.token {
            Token::String => PreOperand::String(self.eat_value(Token::String)?),
            Token::Number => PreOperand::Number(self.eat_value(Token::Number)?),
            Token::Register => PreOperand::Register(self.eat_value(Token::Register)?),
            Token::Function => PreOperand::Function(self.eat_value(Token::Function)?),
            Token::Label => PreOperand::Label(self.eat_value(Token::Label)?),
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

    fn eat_value(&mut self, tkn: Token) -> ParserResult<String> {
        let value = if self.token == tkn {
            self.lexer.value().unwrap()
        } else {
            return Err(self.unexpected(&[tkn]));
        };

        self.next()?;

        Ok(value)
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
            .map(|t| format!("`{}`", t)).collect::<Vec<String>>().join(", ");

        MachinaError {
            kind: MachinaErrorKind::Expected(format!("one of {}", expected), format!("`{}`", self.token)),
            line: self.lexer.line(),
        }
    }
}


#[derive(Debug, Clone)]
struct Block {
    line: usize,
    instructions: Vec<PreInstruction>,
}

#[derive(Debug, Clone)]
pub struct PreInstruction {
    pub opcode: OpCode,
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
