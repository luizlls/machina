mod ast;
mod error;
mod lexer;
mod parser;
mod value;
mod machina;

use crate::parser::{Parser};
use std::fs;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() <= 1 {
        println!("Machina v {}", env!("CARGO_PKG_VERSION"));
        println!("Use 'machina <file name>' to compile and/or execute a file");
    } else {
        file(args.get(1).unwrap().to_string());
    }
}

fn file(file: String) {
    let input = fs::read_to_string(file.clone()).expect("Couldn't open the file");
    exec(input);
}

fn exec(source: String) {
    let mut parser = Parser::new(&source);
    let parsed = parser.parse();
    match parsed {
        Ok(module) => {
            for (name, function) in module.functions {
                println!("function {} - {:#?}", name, function);
            }
        }
        Err(errors) => {
            for err in errors {
                println!("Error [line: {}] {}", err.line, err.kind);
            }
        }
    }
}