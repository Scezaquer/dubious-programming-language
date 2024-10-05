mod ast;
mod code_generator;
mod lexer;

use ast::build_ast::parse;
use code_generator::generator::generate;
use lexer::lex::lex;
use std::fs;
use std::path::Path;
use std::process::Command;

use clap::Parser;

/// A simple compiler for the Dubious programming language (DPL)
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input file to read (typically .dpl)
    input_file: String,

    /// Output file to write
    #[arg(short, long, default_value = "out")]
    output_file: String,

    /// Print the AST
    #[arg(short, long, default_value_t = false)]
    ast: bool,

    /// Print the tokens
    #[arg(short, long, default_value_t = false)]
    tokens: bool,

    /// Output an assembly file instead of a binary
    #[arg(short = 'S', default_value_t = false)]
    output_asm: bool,
}

fn main() {
    let args = Args::parse();

    if !Path::new(&args.input_file).exists() {
        eprintln!("File not found: {}", args.input_file);
        std::process::exit(1);
    }

    let file = fs::read_to_string(&args.input_file).expect("Failed to read file");
    //let file = fs::read_to_string("return_2.dpl").expect("Failed to read file");

    let tokens = lex(&file);

    if args.tokens {
        dbg!(&tokens);
    }

    let ast = parse(&tokens);

    if args.ast {
        dbg!(&ast);
        println!("{}", ast.pretty_print());
    }

    // We make a .s file if the user wants to see the assembly code,
    // otherwise don't use any extension.
    // There is an edgecase where the user wants to output to a file called
    // "out", and uses -S, which will cause the output file to be "out.s"
    // instead of "out".
    let output_file;
    if args.output_asm && args.output_file == "out" {
        output_file = "out.s".to_string();
    } else {
        output_file = args.output_file.clone();
    }

    generate(&ast, &output_file);
    if !args.output_asm {
        // nasm -f elf64 out.s -o out.o
        Command::new("nasm")
            .args([
                "-f",
                "elf64",
                format!("{}", args.output_file).as_str(),
                "-o",
                format!("{}.o", &args.output_file).as_str(),
            ])
            .output()
            .expect("Failed to compile assembly file");

        // ld out.o -o out
        Command::new("ld")
            .args([
                format!("{}.o", &args.output_file).as_str(),
                "-o",
                format!("{}", &args.output_file).as_str(),
            ])
            .output()
            .expect("Failed to link object file");

        // rm out.o
        Command::new("rm")
            .args([format!("{}.o", &args.output_file).as_str()])
            .output()
            .expect("Failed to remove intermediary object file");
    }
}
