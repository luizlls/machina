use std::error::Error;
use std::fmt;
use std::fmt::{Display};

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind {
    InvalidEscapeCharacter,
    InvalidCharacter(char),
    UnterminatedString,
     // expected, found
    UnexpectedToken(Vec<String>, String),
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorKind::InvalidEscapeCharacter => {
                write!(f, "Invalid escape character")
            }
            ErrorKind::InvalidCharacter(chr) => {
                write!(f, "Invalid character `{}`", chr)
            }
            ErrorKind::UnterminatedString => {
                write!(f, "Unterminated string")
            }
            ErrorKind::UnexpectedToken(expected, found) => {
                write!(f, "Expected {}, found {}", expected.join(" or "), found)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct MachinaError {
    pub kind: ErrorKind,
    pub line: u32
}

impl Error for ErrorKind { }