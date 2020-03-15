use crate::lexer::{Lexer, Token, TokenKind};
use crate::error::{MachinaError, ErrorKind};
use crate::ast::*;

use std::collections::VecDeque;


type ParserResult<T> = Result<T, MachinaError>;

#[derive(Debug, Clone)]
pub struct Parser<'a> {
    lexer: Lexer<std::str::Chars<'a>>,
    curr: Option<Token>,
    peek: Option<Token>,
    tokens: VecDeque<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Parser {
        Parser {
            lexer: Lexer::new(source.chars()),
            curr: None,
            peek: None,
            tokens: VecDeque::with_capacity(0),
        }
    }

    pub fn parse(&mut self) -> Result<Module, Vec<MachinaError>> {
        let mut module = Module::new();
        let mut errors = vec![];

        let _ = self.next(); // curr
        let _ = self.next(); // peek

        while self.curr.is_some() {
            match self.parse_basic_function() {
                Ok(function) => {
                    let name = function.name.clone().0;
                    module.functions.insert(
                        name,
                        Function::BasicFunction(function));
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

    fn parse_basic_function(&mut self) -> ParserResult<BasicFunction> {
        self.eat(TokenKind::Define)?;

        let line = self.line();
        let name = self.parse_label()?;

        let mut tokens = vec![];

        while !self.curr_is(TokenKind::End)
           && !self.curr_is(TokenKind::EOF) {
            tokens.push(self.curr.clone().unwrap());
            self.next()?;
        }

        self.eat(TokenKind::End)?;

        Ok(BasicFunction {
            name,
            line,
            tokens
        })
    }

    pub fn parse_final_function(&mut self, function: BasicFunction) -> ParserResult<FinalFunction> {
        self.tokens = VecDeque::from(function.tokens);

        self.next()?; // curr
        self.next()?; // peek

        let args = if self.curr_is(TokenKind::LParen) {
            self.parse_args()?
        } else {
            Vec::with_capacity(0)
        };

        self.eat(TokenKind::Colon)?;

        Ok(FinalFunction {
            name: function.name,
            line: function.line,
            registers_size: 0,
            blocks: Vec::with_capacity(0),
        })
    }

    fn parse_instruction(&mut self) -> ParserResult<Instruction> {
        match self.curr_kind() {
            TokenKind::Exec => {
                self.eat(TokenKind::Exec)?;
                let target = Target::Variable(self.parse_variable()?);
                Ok(Instruction::Exec(target))
            }
            TokenKind::If => {
                self.eat(TokenKind::If)?;
                let test = self.parse_value()?;
                let then = self.parse_label()?;
                self.eat(TokenKind::Semicolon)?;
                let else_ = self.parse_label()?;
                Ok(Instruction::If(test, then, else_))
            }
            TokenKind::Jmp => {
                self.eat(TokenKind::Jmp)?;
                let dest = self.parse_label()?;
                Ok(Instruction::Jmp(dest))
            }
            TokenKind::JmpT => {
                self.eat(TokenKind::JmpT)?;
                let test = self.parse_value()?;
                let dest = self.parse_label()?;
                Ok(Instruction::JmpT(test, dest))
            }
            TokenKind::JmpF => {
                self.eat(TokenKind::JmpF)?;
                let test = self.parse_value()?;
                let dest = self.parse_label()?;
                Ok(Instruction::JmpT(test, dest))
            }
            TokenKind::Out => {
                self.eat(TokenKind::Out)?;
                let value = self.parse_value()?;
                Ok(Instruction::Output(value))
            }
            TokenKind::Variable => {
                let target = Target::Variable(self.parse_variable()?);
                let expr   = self.parse_expression()?;
                Ok(Instruction::Assignment(target, expr))
            }
            _ => {
                Err(self.unexpected(&[
                    TokenKind::Exec,
                    TokenKind::If,
                    TokenKind::Jmp,
                    TokenKind::JmpT,
                    TokenKind::JmpF,
                    TokenKind::Out,
                    TokenKind::Variable,
                ]))
            }
        }
    }

    fn parse_expression(&mut self) -> ParserResult<Expression> {
        match self.curr_kind() {
            TokenKind::String
          | TokenKind::Integer
          | TokenKind::Decimal
          | TokenKind::Variable
          | TokenKind::Null => {
                Ok(Expression::Value(self.parse_value()?))
            }
            TokenKind::In => {
                self.eat(TokenKind::In)?;
                Ok(Expression::Input)
            }
            TokenKind::Call => {
                self.eat(TokenKind::Call)?;
                let func = self.parse_label()?;
                let args = self.parse_seq_of(TokenKind::Comma, Self::parse_value)?;
                Ok(Expression::Call(func, args))
            }
            TokenKind::Case => {
                self.eat(TokenKind::Case)?;
                let cases = self.parse_seq_of(TokenKind::Semicolon, |this| {
                    let test   = this.parse_value()?;
                    let target = this.parse_label()?;
                    Ok((test, target))
                })?;
                Ok(Expression::Case(cases))
            }
              TokenKind::Add
            | TokenKind::Sub
            | TokenKind::Mul
            | TokenKind::Div
            | TokenKind::Mod
            | TokenKind::Eq
            | TokenKind::Neq
            | TokenKind::Lt
            | TokenKind::Lte
            | TokenKind::Gt
            | TokenKind::Gte
            | TokenKind::And
            | TokenKind::Or
            | TokenKind::Xor => {
                self.eat(self.curr_kind())?;
                let lhs = self.parse_value()?;
                let rhs = self.parse_value()?;
                Ok(Expression::Binary(Binary::from(self.curr_kind()), lhs, rhs))
            }
            TokenKind::Not => {
                self.eat(self.curr_kind())?;
                let rhs = self.parse_value()?;
                Ok(Expression::Unary(Unary::from(self.curr_kind()), rhs))
            }
            _ => {
                Err(self.unexpected(&[]))
            }
        }
    }

    fn parse_args(&mut self) -> ParserResult<Vec<Variable>> {
        self.eat(TokenKind::LParen)?;
        let args = self.parse_seq_of(TokenKind::Comma, Self::parse_variable)?;
        self.eat(TokenKind::RParen)?;
        Ok(args)
    }

    fn parse_variable(&mut self) -> ParserResult<Variable> {
        match self.curr.clone() {
            Some(Token { kind: TokenKind::Variable, value: Some(variable), line }) => {
                let _ = self.next()?;
                Ok(Variable(variable))
            }
            _ => Err(self.unexpected(&[TokenKind::Variable]))
        }
    }

    fn parse_label(&mut self) -> ParserResult<Label> {
        match self.curr.clone() {
            Some(Token { kind: TokenKind::Label, value: Some(label), line }) => {
                self.next()?;
                Ok(Label(label))
            }
            _ => Err(self.unexpected(&[TokenKind::String])),
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
            Some(Token { kind: TokenKind::Variable, value: Some(variable), .. }) => {
                let _ = self.next()?;
                let target = Target::Variable(Variable(variable));
                Ok(Value::Target(target))
            }
            _ => Err(self.unexpected(&[
                TokenKind::String,
                TokenKind::Integer,
                TokenKind::Decimal,
                TokenKind::Variable,
                TokenKind::Null,
            ])),
        }
    }

    fn parse_seq_of<T, F>(&mut self, sep: TokenKind, mut f: F) -> ParserResult<Vec<T>>
    where F: FnMut(&mut Self) -> ParserResult<T>, T: std::fmt::Debug {
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

        if !self.tokens.is_empty() {
            self.peek = self.tokens.pop_front();
        } else {
            self.peek = if let Some(token) = self.lexer.next() {
                Some(token?)
            } else {
                None
            };
        }

        Ok(())
    }

    fn eat(&mut self, tkn: TokenKind) -> ParserResult<()> {
        if self.curr_kind() == tkn {
            let _ = self.next()?;
            Ok(())
        } else {
            Err(self.unexpected(&[tkn]))
        }
    }

    fn try_eat(&mut self, kind: TokenKind) -> ParserResult<bool> {
        if self.curr_is(kind) {
            let _ = self.next()?;
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
        while !(self.curr_is(TokenKind::EOF)
             || self.curr_is(TokenKind::Define)) {
            match self.next() {
                Err(err) => {
                    errors.push(err)
                }
                _ => {}
            }
        }
    }

    fn unexpected(&mut self, expected: &[TokenKind]) -> MachinaError {
        let tokens = expected.iter().map(|x| format!("`{}`", x)).collect::<Vec<String>>();
        MachinaError {
            kind: ErrorKind::UnexpectedToken(tokens, format!("`{}`", self.curr_kind())),
            line: self.line()
        }
    }
}