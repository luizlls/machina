use std::error::Error;
use std::fmt;
use std::fmt::{Display};

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind {
    InvalidEscapeCharacter,
    InvalidCharacter,
    UnterminatedString,
     // keyword
    InvalidKeyword(String),
     // expected, found
    ExpectedToken(Vec<String>, String),
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorKind::InvalidEscapeCharacter => {
                write!(f, "Invalid escape character")
            }
            ErrorKind::InvalidCharacter => {
                write!(f, "Invalid character")
            }
            ErrorKind::UnterminatedString => {
                write!(f, "Unterminated string")
            }
            ErrorKind::InvalidKeyword(keyword) => {
                write!(f, "Invalid keyword {}", keyword)
            }
            ErrorKind::ExpectedToken(expected, found) => {
                write!(f, "Expected {}, found {}", expected.join(" or "), found)
            }
        }
    }
}

pub type MachinaError = (ErrorKind, u32);

impl Error for ErrorKind { }