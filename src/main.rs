use avalanche::compiler::*;
use avalanche::config::*;
use avalanche::lexer::*;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;

fn main() {
    let config = Config::new(env::args().collect());
    let mut file = File::open(&config.input).unwrap();
    let mut f_content = String::new();
    match file.read_to_string(&mut f_content) {
        Ok(_e) => (), // fine
        Err(_e) => {
            println!("Cannot open file: {}", config.input);
            process::exit(-1);
        }
    }
    let lexems = Lexer::lex(f_content);
    let instructions = Compiler::compile(lexems);
    let mut stack: Vec<u32> = Vec::new();
    let mut i = 0;
    while i < instructions.len() {
        let signal = instructions[i].execute(&mut stack, i);
        match signal {
            Signal::Call {pointer} => i = pointer as usize,
            _ => i += 1
        }
    }
}
