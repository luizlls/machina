mod value;
mod error;
mod lexer;

use crate::lexer::{Lexer};
use std::fs;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() <= 1 {
        println!("Machina v{}", env!("CARGO_PKG_VERSION"));
        println!("Use: machina <filename> to execute a file");
    } else {
        file(args.get(1).unwrap().to_string());
    }
}

fn file(file: String) {
    let input = fs::read_to_string(file.clone()).expect("Couldn't open the file");
    exec(input);
}

fn exec(source: String) {
    for res in Lexer::new(source.chars()) {
        match res {
            Ok(token) => {
                println!("{:?}", token);
            }
            Err(err) => {
                println!("Error {:?}", err);
            }
        }
    }
}