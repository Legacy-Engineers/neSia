mod tokenizer;
mod parser;

use tokenizer::Tokenizer;
use parser::Parser;

use std::env;
use std::fs;
use std::process;

fn main() {
    // You can uncomment this and pass args later
    // let args: Vec<String> = env::args().collect();

    // if args.len() < 2 {
    //     eprintln!("Usage: cargo run <filename>");
    //     process::exit(1);
    // }

    // let filename = &args[1];

    // let contents = fs::read_to_string(filename).unwrap_or_else(|err| {
    //     eprintln!("Error reading file {}: {}", filename, err);
    //     process::exit(1);
    // });

    let src = "numbers = 42 + 5;";

    // Step 1: Tokenize
    let mut tokenizer = Tokenizer::new(src);
    let tokens = tokenizer.tokenize();

    // Step 2: Parse
    let mut parser = Parser::new(tokens);
    match parser.parse() {
        Ok(statements) => {
            println!("Parsed AST:");
            for stmt in statements {
                println!("{:#?}", stmt);
            }
        }
        Err(err) => {
            eprintln!("Parser Error: {}", err);
            process::exit(1);
        }
    }
}
