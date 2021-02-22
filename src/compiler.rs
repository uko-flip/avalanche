use crate::lexer::*;

macro_rules! stack_pop {
    ($stack:ident) => {
        match $stack.pop() {
            Some(e) => e,
            None => 0,
        }
    };
}

#[derive(Debug)]
pub enum Instruction {
    Push(u32),
    Pop,

    Test,
    PrintN,
    Add,
}

pub struct Compiler {}

impl Compiler {
    pub fn compile(input: Vec<Token>) -> Vec<Instruction> {
        let mut instructions: Vec<Instruction> = Vec::new();
        let mut push_need_argument = false;
        for i in input {
            match i {
                Token::Word(e) => match e.as_str() {
                    "push" => push_need_argument = true,
                    "pop" => instructions.push(Instruction::Pop),
                    "test" => instructions.push(Instruction::Test),
                    "printn" => instructions.push(Instruction::PrintN),
                    "add" => instructions.push(Instruction::Add),
                    _ => println!("Unsupported command: {}", e),
                },
                Token::Number(e) => {
                    if push_need_argument {
                        instructions.push(Instruction::Push(e));
                        push_need_argument = false;
                    } else {
                        println!("Excess number: {}", e);
                    }
                }
            }
        }
        instructions
    }
}

impl Instruction {
    pub fn execute(&self, stack: &mut Vec<u32>) {
        match *self {
            Instruction::Push(e) => stack.push(e),
            Instruction::Pop => {
                stack_pop!(stack);
            }
            Instruction::Test => println!("Hello, World!"),
            Instruction::PrintN => println!("{}", stack_pop!(stack)),
            Instruction::Add => {
                let op1 = stack_pop!(stack);
                let op2 = stack_pop!(stack);
                stack.push(op1 + op2);
            }
        }
    }
}
