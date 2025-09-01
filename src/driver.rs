use crate::lexer::Tokenizer;
use crate::parser::Parser;
use std::fs;

pub fn run_file(path: &str) {
    let source_code = fs::read_to_string(path).expect("Could not read source file");

    run_source_code(&source_code);
}
fn run_source_code(source_code: &String) {
    let mut tokenizer = Tokenizer::new(source_code);
    let tokens = tokenizer.tokenize();
    let mut parser = Parser::new(tokens);
    let parsing = parser.parse();

    println!("{:#?}", parsing);
}
