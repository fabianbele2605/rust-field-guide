mod lexer;
mod parser;
mod interpreter;
use lexer::tokenize;
use parser::Parser;
use interpreter::Interpreter;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Uso: toylang <archivo.toy>");
        return;
    }
    let source = fs::read_to_string(&args[1]).expect("No se pudo leer el archivo");
    
    let tokens = tokenize(&source);
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program();
    
    let mut interpreter = Interpreter::new();
    interpreter.run(&program);
}