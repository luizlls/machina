use crate::lexer::{Lexer, Token, TokenKind};
use crate::error::{MachinaError, ErrorKind};
use crate::ast::*;


type ParserResult<T> = Result<T, MachinaError>;

#[derive(Debug, Clone)]
pub struct Parser<'a> {
    lexer: Lexer<std::str::Chars<'a>>,
    curr: Option<Token>,
    peek: Option<Token>,
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
            match self.parse_raw_function() {
                Ok(function) => {
                    let name = function.name.clone().0;
                    module.functions.insert(
                        name,
                        Function::RawParsedFunction(function));
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

    fn parse_raw_function(&mut self) -> ParserResult<RawParsedFunction> {
        self.eat(TokenKind::Define)?;

        let line = self.curr_line();
        let name = self.parse_label()?;

        let mut tokens = vec![];

        while !self.curr_is(TokenKind::End)
           && !self.curr_is(TokenKind::EOF) {
            tokens.push(self.curr.clone().unwrap());
            self.next()?;
        }

        self.eat(TokenKind::End)?;

        Ok(RawParsedFunction {
            name,
            line,
            tokens
        })
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

    fn next(&mut self) -> ParserResult<()> {
        self.curr = self.peek.clone();
        self.peek = if let Some(res) = self.lexer.next() {
            Some(res?)
        } else {
            None
        };
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

    fn curr_line(&self) -> u32 {
        if let Some(tok) = &self.curr { tok.line } else { 0 }
    }

    fn peek_line(&self) -> u32 {
        if let Some(tok) = &self.curr { tok.line } else { 0 }
    }

    fn curr_is(&self, tt: TokenKind) -> bool {
        self.curr_kind() == tt
    }

    fn peek_is(&self, tt: TokenKind) -> bool {
        self.peek_kind() == tt
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
            line: self.curr_line()
        }
    }
}