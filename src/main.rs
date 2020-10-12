use std::fs;

use machina::{
    bytecode::{
        Module,
    },
    machina::{
        Environment,
        Machina,
    },
    parser::Parser
};

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
    match Parser::new(&source).parse() {
        Ok(module) => {
            eval(module)
        }
        Err(error) => {
            eprintln!("{}", error)
        }
    }
}

fn eval(module: Module) {

    let Module { functions, .. } = module;

    let environment = Environment {
        functions,
    };

    Machina::new(&environment).call(0, 0, 0, 0);
}
