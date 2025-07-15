mod tokenizer;
mod parser;
mod environment_symbol_table;
mod interpreter_engine;

use tokenizer::Tokenizer;
use parser::Parser;
use interpreter_engine::Interpreter;

use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cargo run <filename.ns>");
        process::exit(1);
    }

    let filename = &args[1];

    // Check if file has .ns extension
    if !filename.ends_with(".ns") {
        eprintln!("Error: File must have .ns extension");
        eprintln!("Usage: cargo run <filename.ns>");
        process::exit(1);
    }

    let contents = fs::read_to_string(filename).unwrap_or_else(|err| {
        eprintln!("Error reading file {}: {}", filename, err);
        process::exit(1);
    });

    // Step 1: Tokenize
    let mut tokenizer = Tokenizer::new(&contents);
    let tokens = tokenizer.tokenize();

    // Step 2: Parse
    let mut parser = Parser::new(tokens);
    match parser.parse() {
        Ok(statements) => {
            println!("Parsed AST:");
            for stmt in &statements {
                println!("{:#?}", stmt);
            }
            
            // Step 3: Interpret
            let mut interpreter = Interpreter::new();
            match interpreter.interpret(statements) {
                Ok(_) => {
                    println!("\nProgram executed successfully!");
                }
                Err(err) => {
                    eprintln!("Runtime Error: {}", err);
                    process::exit(1);
                }
            }
        }
        Err(err) => {
            eprintln!("Parser Error: {}", err);
            process::exit(1);
        }
    }
}