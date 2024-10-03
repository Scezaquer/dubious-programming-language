mod lexer;
use std::env;
use std::fs;
use std::path::Path;
use lexer::lex::lex;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <file>", args[0]);
        std::process::exit(1);
    }

    if !Path::new(&args[1]).exists() {
        eprintln!("File not found: {}", args[1]);
        std::process::exit(1);
    }

    let file = fs::read_to_string(&args[1]).expect("Failed to read file");

    dbg!(&file);

    let tokens = lex(&file);

    dbg!(&tokens);
}