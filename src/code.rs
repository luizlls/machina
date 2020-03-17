use crate::ast;

use std::collections::HashMap;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum InstructionKind {
    Get,
    Set,
    Const,
    If,
    Jmp,
    JmpT,
    JmpF,
    Call,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Ne,
    Lt,
    Lte,
    Gt,
    Gte,
    And,
    Or,
    Xor,
    Not,
    Input,
    Output,
    Return,
}

#[derive(Debug, Clone)]
pub struct Instruction {
    pub kind: InstructionKind,
    pub args: Vec<u16>,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub local_values: Vec<ast::Value>,
    pub instructions: Vec<Instruction>,
}

impl Function {
    fn from(ast_function: ast::Function) -> Function {
        let mut function = Function {
            local_values: vec![],
            instructions: vec![]
        };

        let mut blocks = HashMap::new();

        for block in ast_function.blocks {
            blocks.insert(block.label, function.instructions.len());

            for instruction in block.instructions {
                function.convert_instruction(instruction);
            }
        }

        function
    }

    fn convert_instruction(&mut self, inst: ast::Instruction) {
        match inst {
            ast::Instruction::Assign(target, expr) => {

            }
            ast::Instruction::Switch(cases) => {

            }        
            ast::Instruction::If(test, then, else_) => {

            }        
            ast::Instruction::Jmp(dest) => {

            }        
            ast::Instruction::JmpT(test, dest) => {

            }        
            ast::Instruction::JmpF(test, dest) => {

            }        
            ast::Instruction::Call(func, arguments) => {

            }        
            ast::Instruction::Output(value) => {

            }        
            ast::Instruction::Return(value) => {

            }
        }
    }

    fn convert_expression(&mut self, expr: ast::Expression) {
        match expr {
            ast::Expression::Input => {
                self.instructions.push(
                    Instruction {
                        kind: InstructionKind::Input,
                        args: Vec::with_capacity(0)
                    }
                )
            }
            ast::Expression::Binary(op, lhs, rhs) => {

            }
            ast::Expression::Unary(op, lhs) => {

            }
            ast::Expression::Call(func, arguments) => {

            }
            ast::Expression::Value(value) => {

            }
        }
    }
}

