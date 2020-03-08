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
}