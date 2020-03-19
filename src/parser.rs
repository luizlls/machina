use crate::lexer::{Lexer, Token, TokenKind};
use crate::error::{MachinaError, ErrorKind};
use crate::ast::*;

use std::collections::HashMap;

type ParserResult<T> = Result<T, MachinaError>;

#[derive(Debug, Clone)]
pub struct Parser<'a> {
    lexer: Lexer<std::str::Chars<'a>>,
    curr: Option<Token>,
    peek: Option<Token>
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Parser {
        Parser {
            lexer: Lexer::new(source.chars()),
            curr: None,
            peek: None,
        }
    }

    pub fn parse(&mut self) -> Result<Module, Vec<MachinaError>> {
        let mut module = Module::new();
        let mut errors = vec![];

        let _ = self.next(); // curr
        let _ = self.next(); // peek

        while self.curr.is_some() {
            match self.parse_function() {
                Ok(function) => {
                    module.functions.insert(function.name.clone(), function);
                }
                Err(err) => {
                    errors.push(err);
                    self.recover(&mut errors);
                }
            }
        }

        if errors.is_empty() {
            Ok(module)
        } else {
            Err(errors)
        }
    }

    pub fn parse_function(&mut self) -> ParserResult<Function> {
        let line = self.line();
        let name = self.parse_identifier()?.0;

        let args = if self.curr_is(TokenKind::LParen) {
            self.parse_args()?
        } else {
            Vec::with_capacity(0)
        };

        self.eat(TokenKind::Colon)?;

        let mut pre_instructions = vec![];

        let mut count = 0;
        let mut blocks = HashMap::new();

        while !self.is_function_definition()
           && !self.curr_is(TokenKind::EOF) {

            if self.curr_is(TokenKind::Label) {
                let label = self.parse_label()?;
                self.eat(TokenKind::Colon)?;
                blocks.insert(label, count);
            }
            pre_instructions.push(self.parse_instruction()?);
            count += 1;
        }

        dbg!(pre_instructions);

        dbg!(blocks);

        let mut local_values = vec![];

        let mut instructions = vec![];

        Ok(Function {
            name: name,
            line: line,
            local_values,
            instructions,
        })
    }

    fn parse_instruction(&mut self) -> ParserResult<PreInstruction> {
        let line = self.line();
        let kind = InstructionKind::from_token(self.curr_kind());
        if  kind.is_none() {
            return Err(self.unexpected(&[TokenKind::Instruction]));
        }
        self.next()?;
        let kind = kind.unwrap();
        let arg  = match kind {
            InstructionKind::Const
          | InstructionKind::Load
          | InstructionKind::Store
          | InstructionKind::Jump
          | InstructionKind::JumpT
          | InstructionKind::JumpF
          | InstructionKind::Call => {
            Some(self.parse_value()?)
          }
          _ => None
        };
        Ok(PreInstruction { kind, arg, line })
    }

    fn is_function_definition(&self) -> bool {
        self.curr_is(TokenKind::Identifier) 
            && (self.peek_is(TokenKind::Colon)
                || self.peek_is(TokenKind::LParen))
    }

    fn parse_args(&mut self) -> ParserResult<Vec<Variable>> {
        self.eat(TokenKind::LParen)?;
        let args = self.parse_seq_with(TokenKind::Comma, Self::parse_variable)?;
        self.eat(TokenKind::RParen)?;
        Ok(args)
    }

    fn parse_variable(&mut self) -> ParserResult<Variable> {
        match self.curr.clone() {
            Some(Token { kind: TokenKind::Variable, value: Some(variable), .. }) => {
                self.next()?;
                Ok(Variable(variable))
            }
            _ => Err(self.unexpected(&[TokenKind::Variable]))
        }
    }

    fn parse_identifier(&mut self) -> ParserResult<Identifier> {
        match self.curr.clone() {
            Some(Token { kind: TokenKind::Identifier, value: Some(ident), .. }) => {
                self.next()?;
                Ok(Identifier(ident))
            }
            _ => Err(self.unexpected(&[TokenKind::Identifier])),
        }
    }

    fn parse_label(&mut self) -> ParserResult<Label> {
        match self.curr.clone() {
            Some(Token { kind: TokenKind::Label, value: Some(label), .. }) => {
                self.next()?;
                Ok(Label(label))
            }
            _ => Err(self.unexpected(&[TokenKind::Label])),
        }
    }

    fn parse_value(&mut self) -> ParserResult<Value> {
        match self.curr.clone() {
            Some(Token { kind: TokenKind::String, value: Some(s), .. }) => {
                self.next()?;
                Ok(Value::String(s))
            }
            Some(Token { kind: TokenKind::Integer, value: Some(i), .. }) => {
                self.next()?;
                let value = i.parse::<i64>().unwrap();
                Ok(Value::Integer(value))
            }
            Some(Token { kind: TokenKind::Decimal, value: Some(d), .. }) => {
                self.next()?;
                let value = d.parse::<f64>().unwrap();
                Ok(Value::Decimal(value))
            }
            Some(Token { kind: TokenKind::Null, .. }) => {
                self.next()?;
                Ok(Value::Null)
            }
            Some(Token { kind: TokenKind::Identifier, value: Some(identifier), .. }) => {
                self.next()?;
                Ok(Value::Identifier(Identifier(identifier)))
            }
            Some(Token { kind: TokenKind::Label, value: Some(label), .. }) => {
                self.next()?;
                Ok(Value::Label(Label(label)))
            }
            Some(Token { kind: TokenKind::Variable, value: Some(variable), .. }) => {
                self.next()?;
                Ok(Value::Variable(Variable(variable)))
            }
            _ => Err(self.unexpected(&[
                TokenKind::String,
                TokenKind::Integer,
                TokenKind::Decimal,
                TokenKind::Null,
                TokenKind::Identifier,
                TokenKind::Label,
                TokenKind::Variable,
            ])),
        }
    }

    fn parse_seq_with<T, F>(&mut self, sep: TokenKind, mut f: F)
      -> ParserResult<Vec<T>>
      where F: FnMut(&mut Self) -> ParserResult<T> {
        let mut result = vec![];

        loop {
            result.push(f(self)?);
            if !self.try_eat(sep)? {
                break;
            }
        }

        Ok(result)
    }

    fn next(&mut self) -> ParserResult<()> {
        self.curr = self.peek.clone();
        self.peek = if let Some(token) = self.lexer.next() {
            Some(token?)
        } else {
            None
        };
        Ok(())
    }

    fn eat(&mut self, tkn: TokenKind) -> ParserResult<()> {
        if self.curr_kind() == tkn {
            self.next()?;
            Ok(())
        } else {
            Err(self.unexpected(&[tkn]))
        }
    }

    fn try_eat(&mut self, kind: TokenKind) -> ParserResult<bool> {
        if self.curr_is(kind) {
            self.next()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn curr_kind(&self) -> TokenKind {
        if let Some(tok) = &self.curr {
            tok.kind
        } else {
            TokenKind::EOF
        }
    }

    fn peek_kind(&self) -> TokenKind {
        if let Some(tok) = &self.peek {
            tok.kind
        } else {
            TokenKind::EOF
        }
    }

    fn curr_is(&self, kind: TokenKind) -> bool {
        self.curr_kind() == kind
    }

    fn peek_is(&self, kind: TokenKind) -> bool {
        self.peek_kind() == kind
    }

    fn line(&self) -> u32 {
        if let Some(tok) = &self.curr { tok.line } else { 0 }
    }

    fn recover(&mut self, errors: &mut Vec<MachinaError>) {
        while !self.is_function_definition()
           && !self.curr_is(TokenKind::EOF) {
            if let Err(err) = self.next() { errors.push(err) }
        }
    }

    fn unexpected(&mut self, expected: &[TokenKind]) -> MachinaError {
        let tokens = expected.iter().map(|x| format!("`{}`", x)).collect();
        let found  = format!("`{}`", self.curr_kind());
        MachinaError {
            kind: ErrorKind::UnexpectedToken(tokens, found),
            line: self.line()
        }
    }
}