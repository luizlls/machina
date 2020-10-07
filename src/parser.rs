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

    fn parse(mut self) -> Result<Module, MachinaError> {
        let _ = self.next();

        while !self.curr_is(Token::EOF) {
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
        
        while !self.curr_is(Token::Function) {
            let label = self.eat_value(Token::Label)?;
            let block = self.parse_block(count)?;
            count = count + block.instructions.len();
            
            blocks.insert(label, block);
        }

        Ok((name, Function { locals: 0, instructions: vec![] }))
    }

    fn parse_block(&mut self, line: usize) -> ParserResult<Block> {
        let mut instructions = vec![];

        while !self.curr_is(Token::Label) {
            instructions.push(self.parse_pre_instruction()?);
        }

        Ok(Block { line, instructions })
    }

    fn parse_pre_instruction(&mut self) -> ParserResult<PreInstruction> {
        Ok(PreInstruction {
            opcode: OpCode::Write,
            operands: vec![]
        })
    }

    fn parse_pre_operand(&mut self) -> ParserResult<PreOperand> {
        let operand = match self.token {
            Token::String => {
                PreOperand::String(self.eat_value(Token::String)?)
            }
            Token::Number => {
                PreOperand::Number(self.eat_value(Token::Number)?)
            }
            Token::Function => {
                PreOperand::Function(self.eat_value(Token::Function)?)
            }
            Token::Register => {
                PreOperand::Register(self.eat_value(Token::Register)?)
            }
            Token::Label => {
                PreOperand::Label(self.eat_value(Token::Label)?)
            }
            _ => {
                return Err(
                    self.unexpected(&[
                        Token::String,
                        Token::Number,
                        Token::Function,
                        Token::Register,
                        Token::Label
                    ])
                )
            }
        };

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
            return Err(self.unexpected(&[tkn]))
        };

        self.next()?;

        Ok(value)
    }

    fn curr_is(&self, tkn: Token) -> bool {
        self.token == tkn
    }

    
    fn unexpected(&mut self, tokens: &[Token]) -> MachinaError {
        let expected = tokens
            .iter()
            .map(|t| format!("`{}`", t)).collect::<Vec<String>>().join(", ");

        MachinaError {
            kind: MachinaErrorKind::Expected(format!("one of {}", expected)),
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
