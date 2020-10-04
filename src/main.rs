use machina::{
    ast::{
        Operand,
        OpCode,
        Instruction,
    },
    machina::{
        Environment,
        Function,
        CommonFunction,
        Machine,
    }
};

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() <= 1 {
        println!("Machina v {}", env!("CARGO_PKG_VERSION"));
        println!("Use 'machina <file name>' to compile and/or execute a file");
    } else {
        // file(args.get(1).unwrap().to_string());
        fibonacci();
    }
}

fn file(_file: String) {
    // let input = fs::read_to_string(file.clone()).expect("Couldn't open the file");
    // exec(input);
}

fn exec(_source: String) {
    // let mut machina = Machina::new();
    // match Parser::new(&source).parse() {
    //     Ok(module) => {
    //         machina.run(module);
    //     }
    //     Err(errors) => {
    //         for err in errors {
    //             println!("Error [line: {}] {}", err.line, err.kind);
    //         }
    //     }
    // }
}

fn fibonacci() {
    let entrypoint = vec![

        Instruction::new(
            OpCode::Move,
            [
                Operand::Register(0),
                Operand::Immediate(35),
                Operand::None,
                Operand::None,
            ]
        ),

        Instruction::new(
            OpCode::Call,
            [
                Operand::Register(0),
                Operand::Function(0),
                Operand::Register(0),
                Operand::Register(0),
            ]
        ),

        Instruction::new(
            OpCode::Write,
            [
                Operand::Register(0),
                Operand::None,
                Operand::None,
                Operand::None,
            ]
        ),

        Instruction::new(
            OpCode::Ret,
            [
                Operand::Register(0),
                Operand::None,
                Operand::None,
                Operand::None,
            ]
        )
    ];


    let fibonacci = vec![
        // if n < 2
        Instruction::new(
            OpCode::JLe,
            [
                Operand::Position(9),
                Operand::Register(0),
                Operand::Immediate(1),
                Operand::None
            ]
        ),

        // fib (n - 1)
        Instruction::new(
            OpCode::Move,
            [
                Operand::Register(1),
                Operand::Register(0),
                Operand::None,
                Operand::None,
            ]
        ),

        Instruction::new(
            OpCode::Sub,
            [
                Operand::Register(1),
                Operand::Immediate(1),
                Operand::None,
                Operand::None,
            ]
        ),

        Instruction::new(
            OpCode::Call,
            [
                Operand::Register(1),
                Operand::Function(0),
                Operand::Register(1),
                Operand::Register(1),
            ]
        ),


        // fib (n - 2)
        Instruction::new(
            OpCode::Move,
            [
                Operand::Register(2),
                Operand::Register(0),
                Operand::None,
                Operand::None,
            ]
        ),

        Instruction::new(
            OpCode::Sub,
            [
                Operand::Register(2),
                Operand::Immediate(2),
                Operand::None,
                Operand::None,
            ]
        ),

        Instruction::new(
            OpCode::Call,
            [
                Operand::Register(2),
                Operand::Function(0),
                Operand::Register(2),
                Operand::Register(2),
            ]
        ),

        Instruction::new(
            OpCode::Add,
            [
                Operand::Register(1),
                Operand::Register(2),
                Operand::None,
                Operand::None,
            ]
        ),
        Instruction::new(
            OpCode::Move,
            [
                Operand::Register(0),
                Operand::Register(1),
                Operand::None,
                Operand::None,
            ]
        ),

        // L0
        Instruction::new(
            OpCode::Ret,
            [
                Operand::Register(0),
                Operand::None,
                Operand::None,
                Operand::None,
            ]
        )

    ];

    let functions = vec![
        Function::Common(CommonFunction::new(3, fibonacci)),
        Function::Common(CommonFunction::new(1, entrypoint)),
    ];

    let environment = Environment {
        functions,
    };

    Machine::new(&environment).call(1, 0, 0);
}
