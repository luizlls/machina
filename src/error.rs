use std::error::Error;
use std::fmt;
use std::fmt::{Display};

#[derive(Debug, Clone, PartialEq)]
pub enum MachinaErrorKind {
    InvalidCharacter(char),
    InvalidInstruction(String),
    UnterminatedString,
    Expected(String),
}

impl Display for MachinaErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MachinaErrorKind::InvalidCharacter(chr) => {
                write!(f, "Invalid character `{}`", chr)
            }
            MachinaErrorKind::UnterminatedString => {
                write!(f, "Unterminated string")
            }
            MachinaErrorKind::InvalidInstruction(ins) => {
                write!(f, "Invalid instruction `{}`", ins)
            }
            MachinaErrorKind::Expected(token) => {
                write!(f, "Expected {}", token)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct MachinaError {
    pub kind: MachinaErrorKind,
    pub line: usize
}

impl Error for MachinaErrorKind { }