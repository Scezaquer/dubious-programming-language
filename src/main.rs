/// The preprocessor: Handles preprocessor directives and removes comments.
mod preprocessor;
/// Lexer (tokenizer): Turns the input text file into a list of tokens.
mod lexer;
/// Parser: Turns the list of tokens into an abstract syntax tree (AST).
mod ast_build;
/// Pretty printer: Prints the AST in a human-readable format.
mod ast_pretty_print;
/// Checker: Performs type checking and other checks on the AST to ensure it is valid.
mod logic_checker;
/// Code generator: Turns the AST into x86_64 assembly code.
mod code_generator;
// Shared: Contains shared types and functions used across the compiler.
mod shared;

use preprocessor::preprocessor;
use ast_build::parse;
use code_generator::generate;
use lexer::lex;
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::process::Command;
use logic_checker::check_program;
use std::time::Instant;

use clap::Parser;

/// The command line arguments for the compiler
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
    #[arg(short = 'T', long, default_value_t = false)]
    tokens: bool,

    /// Output an assembly file instead of a binary
    #[arg(short = 'S', default_value_t = false)]
    output_asm: bool,

	/// Output both a binary and an assembly file
	#[arg(short, long, default_value_t = false)]
	both: bool,

	/// Print the time taken for each step
	#[arg(short, long, default_value_t = false)]
	time: bool,
}

/// A simple compiler for the Dubious programming language (DPL).
///
/// This function is the entry point for the compiler. It reads the input file, preprocesses it, lexes it, parses it, generates x86_64 code, and then assembles it.
/// This function is meant to be used as a command line tool. Nasm and ld are required to be installed on the system, as they are used to assemble and link the assembly code generated by the compiler.
/// It's syntax is as follows:
/// ```
/// ./dubious <input_file> [options]
/// ```
///
/// # Examples
///
/// ```
/// // This will compile the file example.dpl and output a binary file called out
/// ./dubious example.dpl
///
/// // This will compile the file example.dpl and output an assembly file called out.s
/// ./dubious example.dpl -S
///
/// // This will compile the file example.dpl and output a binary file called example
/// ./dubious example.dpl -o example
///
/// // This will compile the file example.dpl and print the tokens and AST to the console for debugging
/// ./dubious example.dpl --tokens --ast
fn main() {
	let start = Instant::now();
    let args = Args::parse();

    if !Path::new(&args.input_file).exists() {
        eprintln!("File not found: {}", args.input_file);
        std::process::exit(1);
    }

	let now = Instant::now();
    let file = fs::read_to_string(&args.input_file).expect("Failed to read file");
	let elapsed = now.elapsed();
    if args.time { println!("read file elapsed: {:.2?}", elapsed); }

	let now = Instant::now();
	let preprocessed_file = preprocessor(&file, &args.input_file, HashSet::new(), vec!["toplevel".to_string()]);
	let elapsed = now.elapsed();
    if args.time { println!("preprocessor elapsed: {:.2?}", elapsed); }

	let now = Instant::now();
    let tokens = lex(preprocessed_file.as_str());
	let elapsed = now.elapsed();
    if args.time { println!("lexer elapsed: {:.2?}", elapsed); }

    if args.tokens {
        dbg!(&tokens);
    }

	let now = Instant::now();
    let mut ast = parse(&tokens);
	let elapsed = now.elapsed();
    if args.time {  println!("parser elapsed: {:.2?}", elapsed); }

	let now = Instant::now();
	ast = check_program(&ast);
	let elapsed = now.elapsed();
    if args.time { println!("checker elapsed: {:.2?}", elapsed); }

    if args.ast {
        println!("{}", ast);
    }


    // We make a .s file if the user wants to see the assembly code,
    // otherwise don't use any extension.
    // There is an edgecase where the user wants to output to a file called
    // "out", and uses -S, which will cause the output file to be "out.s"
    // instead of "out".
    let asm_output_file;
	let output_file = args.output_file.clone();
	if args.output_asm || args.both {
		if args.output_file == "out" {
			asm_output_file = "out.s".to_string();
		} else {
			asm_output_file = args.output_file.clone() + ".s";
		}
	} else {
		asm_output_file = output_file.clone();
	}

	let now = Instant::now();
    generate(&ast, &asm_output_file);
	let elapsed = now.elapsed();
    if args.time { println!("code generation elapsed: {:.2?}", elapsed); }
    if !args.output_asm {
		let now = Instant::now();
        // nasm -f elf64 out.s -o out.o
		let nasm_output = Command::new("nasm")
			.args([
			"-f",
			"elf64",
			format!("{}", asm_output_file).as_str(),
			"-o",
			format!("{}.o", &args.output_file).as_str(),
			])
			.output()
			.expect("Failed to execute nasm");

		if !nasm_output.status.success() {
			eprintln!(
			"nasm failed with error:\n{}",
			String::from_utf8_lossy(&nasm_output.stderr)
			);
			std::process::exit(1);
		}

		let elapsed = now.elapsed();
    	if args.time { println!("nasm elapsed: {:.2?}", elapsed); }

		let now = Instant::now();
		// ld out.o -o out
		let ld_output = Command::new("ld")
			.args([
			format!("{}.o", &args.output_file).as_str(),
			"-o",
			format!("{}", &args.output_file).as_str(),
			])
			.output()
			.expect("Failed to execute ld");

		if !ld_output.status.success() {
			eprintln!(
			"ld failed with error:\n{}",
			String::from_utf8_lossy(&ld_output.stderr)
			);
			std::process::exit(1);
		}

		let elapsed = now.elapsed();
    	if args.time { println!("ld elapsed: {:.2?}", elapsed); }

		// TODO: should probably check i'm not overwriting a file
		// rm out.o
		let rm_output = Command::new("rm")
			.args([format!("{}.o", &args.output_file).as_str()])
			.output()
			.expect("Failed to execute rm");

		if !rm_output.status.success() {
			eprintln!(
			"rm failed with error:\n{}",
			String::from_utf8_lossy(&rm_output.stderr)
			);
			std::process::exit(1);
		}
	

        // This turns the elf64 into a flat binary file
        // objcopy -O binary out out

        // And this disassembles it
        // ndisasm -b 64 out
    }

	let elapsed = start.elapsed();
	if args.time { println!("total elapsed: {:.2?}", elapsed); }
}
