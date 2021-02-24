use crate::lexer::*;
use text_io::scan;

macro_rules! stack_pop {
    ($stack:ident) => {
        match $stack.pop() {
            Some(e) => e,
            None => 0,
        }
    };
}

/* macro_rules! match_null {
    ($value:ident) => {
        match $value {
            Some(e) => e,
            None => 0,
        }
    };
} */

macro_rules! match_empty {
    ($value:ident) => {
        match $value {
            Some(e) => e,
            None => ' ',
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
    RecV,
    RecVC,
    PrintS,
    Call,
}

#[derive(Debug)]
pub enum Signal {
    None,
    Call { pointer: u32 },
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
                    "rcv" => instructions.push(Instruction::RecV),
                    "recv" => instructions.push(Instruction::RecVC),
                    "prints" => instructions.push(Instruction::PrintS),
                    "call" => instructions.push(Instruction::Call),
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
    pub fn execute(&self, stack: &mut Vec<u32>, ca: usize) -> Signal {
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
            Instruction::RecV => {
                let num: u32;
                scan!("{}", num);
                stack.push(num);
            }
            Instruction::RecVC => {
                let chr: char;
                scan!("{}", chr);
                stack.push(chr as u32);
            }
            Instruction::PrintS => {
                let s = stack_pop!(stack);
                let u = std::char::from_u32(s);
                println!("{}", match_empty!(u));
            }
            Instruction::Call => {
                let c_addr = stack_pop!(stack);
                stack.push(ca as u32);
                return Signal::Call { pointer:  c_addr};
            }
        }
        Signal::None
    }
}
