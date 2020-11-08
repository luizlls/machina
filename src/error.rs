use std::error::Error;
use std::fmt;
use std::fmt::{Display};

pub type Result<T> = ::std::result::Result<T, MachinaError>;

#[derive(Debug, Clone)]
pub struct Diagnostics {
    errors: Vec<(MachinaError, Option<ErrorMetaData>)>,
}

impl Diagnostics {
    pub fn new() -> Self {
        Diagnostics { errors: vec![] }
    }

    pub fn empty(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn emit(&self) {
        for (error, meta) in self.errors.iter() {
            match meta {
                Some(meta) => {
                    eprintln!("ERROR [{}]: {}", meta.line, error)
                }
                None => {
                    eprintln!("ERROR: {}", error)
                }
            }
        }
    }

    pub fn report<T>(&mut self, error: MachinaError) -> Result<T> {
        self.errors.push((error.clone(), None));

        Err(error)
    }

    pub fn report_with_line<T>(&mut self, error: MachinaError, line: usize) -> Result<T> {
        let meta = Some(ErrorMetaData { line });
        self.errors.push((error.clone(), meta));

        Err(error)
    }
}

#[derive(Debug, Clone)]
pub struct ErrorMetaData {
    line: usize
}


#[derive(Debug, Clone, PartialEq)]
pub enum MachinaError {
    UnterminatedString,
    Expected(String, String),
    InvalidCharacter(char),
    InvalidInstruction(String),
    TargetNotFound(String),
    FunctionNotFound(String),
    InvalidRegister(String),

    OutOfMemory,
}

impl Display for MachinaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MachinaError::UnterminatedString => {
                write!(f, "Unterminated string")
            }
            MachinaError::Expected(expected, found) => {
                write!(f, "Expected {}, but found {}", expected, found)
            }
            MachinaError::InvalidCharacter(chr) => {
                write!(f, "Invalid character `{}`", chr)
            }
            MachinaError::InvalidInstruction(ins) => {
                write!(f, "Invalid instruction `{}`", ins)
            }
            MachinaError::TargetNotFound(label) => {
                write!(f, "Target with label `{}` not found", label)
            }
            MachinaError::FunctionNotFound(function) => {
                write!(f, "Function with name `{}` not found", function)
            }
            MachinaError::InvalidRegister(register) => {
                write!(f, "Invalid register `%{}`", register)
            }
            MachinaError::OutOfMemory => {
                write!(f, "Out of Memory")
            }
        }
    }
}

impl Error for MachinaError { }
