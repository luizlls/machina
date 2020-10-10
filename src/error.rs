use std::error::Error;
use std::fmt;
use std::fmt::{Display};

#[derive(Debug, Clone, PartialEq)]
pub enum MachinaErrorKind {
    UnterminatedString,
    Expected(String, String),
    InvalidCharacter(char),
    InvalidInstruction(String),
    TargetNotFound(String),
    FunctionNotFound(String),
    InvalidRegister(String)
}

impl Display for MachinaErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MachinaErrorKind::UnterminatedString => {
                write!(f, "Unterminated string")
            }
            MachinaErrorKind::Expected(token, found) => {
                write!(f, "Expected one of {}, but found `{}`", token, found)
            }
            MachinaErrorKind::InvalidCharacter(chr) => {
                write!(f, "Invalid character `{}`", chr)
            }
            MachinaErrorKind::InvalidInstruction(ins) => {
                write!(f, "Invalid instruction `{}`", ins)
            }
            MachinaErrorKind::TargetNotFound(label) => {
                write!(f, "Target with label `{}` not found", label)
            }
            MachinaErrorKind::FunctionNotFound(function) => {
                write!(f, "Function with name `{}` not found", function)
            }
            MachinaErrorKind::InvalidRegister(register) => {
                write!(f, "Invalid register `%{}`", register)
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
