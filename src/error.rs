use std::error::Error;
use std::fmt;
use std::fmt::{Display};

#[derive(Debug, Clone, PartialEq)]
pub enum MachinaErrorKind {
    InvalidCharacter(char),
    InvalidInstruction(String),
    UnterminatedString,
    Expected(String, String),
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
            MachinaErrorKind::Expected(token, found) => {
                write!(f, "Expected {}, but found {}", token, found)
            }
        }
    }
}

impl Error for MachinaErrorKind { }

#[derive(Debug, Clone)]
pub struct MachinaError {
    pub kind: MachinaErrorKind,
    pub line: usize
}

impl Display for MachinaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ERROR [{}]: {}", self.line, self.kind)
    }
}

impl Error for MachinaError {}
