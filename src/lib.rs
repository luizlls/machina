#![macro_use]
#![feature(box_syntax)]

#[macro_use]
pub mod macros;

pub mod machina;
pub mod value;
pub mod object;
pub mod error;
pub mod parser;
pub mod lexer;
pub mod bytecode;

