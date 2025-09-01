mod driver;
mod lexer;
mod parser;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: nesia <source_file>");
        std::process::exit(1);
    }
    let filepath = &args[1];
    driver::run_file(filepath);
}
