use std::collections::{HashMap, HashSet};
use regex::Regex;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref INCLUDED_FILES: Mutex<HashMap<String, HashSet<String>>> = {
        let m = HashMap::new();
        Mutex::new(m)
    };
	static ref DEFINED_EXPRESSIONS: Mutex<HashMap<String, String>> = {
		let m = HashMap::new();
		Mutex::new(m)
	};
}

/// This module is responsible for preprocessing the input file.
/// 
/// # Preprocessor directives
/// 
/// ## `#include <[FILE]>`
/// Includes the contents of another file at the current position.
/// 
/// ## `#define <identifier> <replacement>`
/// Defines a macro that will be replaced throughout the code.
/// Issues a warning if the identifier is already defined.
/// 
/// ## `#undef [IDENTIFIER]`
/// Removes a previously defined macro.
/// Issues a warning if the identifier isn't defined.
/// 
/// ## `#ifdef [IDENTIFIER]`
/// Conditionally includes code if the identifier is defined.
/// Can be paired with `#else` and must end with `#endif`.
/// 
/// ## `#ifndef [IDENTIFIER]`
/// Conditionally includes code if the identifier is not defined.
/// Can be paired with `#else` and must end with `#endif`.
/// 
/// ## `#else`
/// Alternative branch for `#ifdef` or `#ifndef`.
/// Must be terminated with `#endif`.
/// 
/// ## `#endif`
/// Terminates a conditional block started by `#ifdef` or `#ifndef`.
/// 
/// ## `#error [MESSAGE]`
/// Stops compilation and displays an error message.
/// 
/// ## `#print [MESSAGE]`
/// Displays a message during compilation.
pub fn preprocessor(file: &str, filename: &str, include_path: HashSet<String>, mut full_namespace: Vec<String>) -> String {
	{
		// Lock the mutex to access the included files
		// This is in a separate block to ensure the lock is released
		let mut included_files = INCLUDED_FILES.lock().unwrap();
		let namespace = full_namespace.join("::");
		included_files.entry(namespace).or_insert_with(HashSet::new).insert(filename.to_string());
	}

	let mut include_path = include_path;
	include_path.insert(filename.to_string());

	let preprocessor_re = Regex::new(r"^\#(include|define|undef|ifdef|ifndef|else|endif|error|print|namespace|spacename)").unwrap();

	//TODO: Things in strings should not be interpreted as procesor directives

	let identifier_re = Regex::new(r"^(?:\s*)[a-zA-Z_][a-zA-Z0-9_]*").unwrap();
	let replacement_re = Regex::new(r"^[\t ]*(.*)\s*?(?:\n|$)").unwrap();
	let file_re = Regex::new(r"^(?:\s*)<(.*)>").unwrap();

    let comments_re = Regex::new(r"^\/\/.*(?:\n|$)").unwrap();
    let multiline_comments_re = Regex::new(r"^\/\*[\s\S]*?\*\/").unwrap();

	let mut processed_file = String::new();

	let mut pos = 0;
	let mut line = 1;
	let mut namespace_counter = 0;

	let chars = file.chars().collect::<Vec<_>>();

	processed_file.push_str(format!("// <{}>\n", filename).as_str());
	while pos < file.len(){
		let rest = &file[pos..];
		if let Some(caps) = preprocessor_re.captures(&file[pos..]){
			let directive = caps.get(1).unwrap().as_str();
			match directive{
				"include" => {
					// #include <[FILE]>
					// Include the contents of the file or folder (if folder, looks for include.dpl)
					// If the file is not found, print an error message

					// Get the file
					pos += caps.get(0).unwrap().end();
					let rest = &file[pos..];
					let file;
					if let Some(caps) = file_re.captures(rest){
						file = caps.get(1).unwrap().as_str();
						pos += caps.get(0).unwrap().end();
					}
					else{
						panic!("{} Line {}: Invalid #include file", filename, line);
					}

					// Get the directory of the current file
					let current_dir = std::path::Path::new(filename).parent()
						.unwrap_or(std::path::Path::new("."));

					// Construct the full path by joining the current directory and included file
					let full_path = current_dir.join(file);

					// Check if we are including a directory
					let (actual_path, is_directory) = if full_path.is_dir() {
						// Check if directory contains include.dpl
						let include_file = full_path.join("include.dpl");
						if include_file.exists() {
							(include_file, true)
						} else {
							panic!("{} Line {}: Directory does not contain include.dpl: {}", filename, line, full_path.display());
						}
					} else {
						(full_path.clone(), false)
					};

					// Check if we are in a circular include situation
					if include_path.contains(actual_path.to_str().unwrap()) {
						panic!("{} Line {}: Circular include detected: {}", filename, line, actual_path.display());
					}

					{
						// Lock the mutex to access the included files
						// This is in a separate block to ensure the lock is released
						let included_files = INCLUDED_FILES.lock().unwrap();
						if let Some(x) = included_files.get(&full_namespace.join("::")){
							if x.contains(actual_path.to_str().unwrap()){
								// println!("{} Line {} Warning: File '{}' is already included somewhere else", filename, line, actual_path.display());
								continue;
							}
						}
					}

					// Read the file
					let included_file = match std::fs::read_to_string(&actual_path) {
						Ok(file) => file,
						Err(_) => panic!("{} Line {}: File not found: {}", filename, line, actual_path.display())
					};

					// Preprocess the included file
					let included_file = preprocessor(&included_file, actual_path.to_str().unwrap(), include_path.clone(), full_namespace.clone());

					// Add a comment to indicate the start of the included file to track the source of errors
					if is_directory {
						processed_file.push_str(format!("// <{}/include.dpl>\n", full_path.display()).as_str());
					} else {
						processed_file.push_str(format!("// <{}>\n", actual_path.display()).as_str());
					}
					processed_file.push_str(&included_file);
					processed_file.push_str(format!("// <{}>\n", filename).as_str());
				},
				"define" => {
					// #define [IDENTIFIER] [REPLACEMENT]
					// Define a macro
					// If the identifier is already defined, print a warning message
					
					// Get the identifier
					pos += caps.get(0).unwrap().end();
					let rest = &file[pos..];
					let identifier;
					let replacement;
					if let Some(caps) = identifier_re.captures(rest){
						identifier = caps.get(0).unwrap().as_str();
						pos += caps.get(0).unwrap().end();

						{
							let defined_expressions = DEFINED_EXPRESSIONS.lock().unwrap();
							// Warning if the identifier is already defined
							if defined_expressions.contains_key(identifier){
								println!("{} Line {} Warning: #define identifier '{}' is already defined", filename, line, identifier);
							}
						}
					}
					else{
						panic!("{} Line {}: Invalid #define identifier", filename, line);
					}

					// Get the replacement
					let rest = &file[pos..];
					if let Some(caps) = replacement_re.captures(rest){
						replacement = caps.get(0).unwrap().as_str();
						pos += caps.get(0).unwrap().end();
					}
					else{
						panic!("{} Line {}: Invalid #define replacement", filename, line);
					}

					// Add the macro to the defined expressions
					{
						let mut defined_expressions = DEFINED_EXPRESSIONS.lock().unwrap();
						defined_expressions.insert(identifier.to_string(), replacement.to_string());
					}
					processed_file.push('\n');	// Keep the line count the same
					line += 1;
				},
				"undef" => {
					// #undef [IDENTIFIER]
					// Undefine a macro
					// If the identifier is not defined, print a warning message

					// Get the identifier
					pos += caps.get(0).unwrap().end();
					let rest = &file[pos..];
					let identifier;
					if let Some(caps) = identifier_re.captures(rest){
						identifier = caps.get(0).unwrap().as_str();
						pos += caps.get(0).unwrap().end();
					}
					else{
						panic!("{} Line {}: Invalid #undef identifier", filename, line);
					}

					{
						let mut defined_expressions = DEFINED_EXPRESSIONS.lock().unwrap();
						// Warning if the identifier is not defined
						if !defined_expressions.contains_key(identifier){
							println!("{} Line {} Warning: #undef identifier '{}' is not defined", filename, line, identifier);
						}

						// Remove the macro from the defined expressions
						defined_expressions.remove(identifier);
					}
					processed_file.push('\n');	// Keep the line count the same
					line += 1;
				},
				"ifdef" => {
					// #ifdef [IDENTIFIER]
					// If the identifier is defined, include the code until #endif
					// Otherwise, skip the code until #endif or #else
					// Note: These can't be nested

					// Get the identifier
					pos += caps.get(0).unwrap().end();
					let rest = &file[pos..];
					let identifier;
					if let Some(caps) = identifier_re.captures(rest){
						identifier = caps.get(0).unwrap().as_str();
						pos += caps.get(0).unwrap().end();
					}
					else{
						panic!("{} Line {}: Invalid #ifdef identifier", filename, line);
					}

					{
						let defined_expressions = DEFINED_EXPRESSIONS.lock().unwrap();
						// Check if the identifier is not defined, and skip the code until #endif or #else
						if !defined_expressions.contains_key(identifier){
							let mut rest = &file[pos..];
							while !rest.starts_with("#endif") && !rest.starts_with("#else") && !rest.is_empty(){
								pos += 1;
								if rest.starts_with("\n"){
									line += 1;
								}
								rest = &file[pos..];
							}
							if rest.starts_with("#else"){
								// ignore #else directive and keep the code it contains
								pos += 5;
							}
						}
					}
				},
				"ifndef" => {
					// #ifndef [IDENTIFIER]
					// If the identifier is not defined, include the code until #endif
					// Otherwise, skip the code until #endif or #else

					// Get the identifier
					pos += caps.get(0).unwrap().end();
					let rest = &file[pos..];
					let identifier;
					if let Some(caps) = identifier_re.captures(rest){
						identifier = caps.get(0).unwrap().as_str();
						pos += caps.get(0).unwrap().end();
					}
					else{
						panic!("{} Line {}: Invalid #ifndef identifier", filename, line);
					}

					{
						// Check if the identifier is defined, and skip the code until #endif or #else
						let defined_expressions = DEFINED_EXPRESSIONS.lock().unwrap();
						if defined_expressions.contains_key(identifier){
							let mut rest = &file[pos..];
							while !rest.starts_with("#endif") && !rest.starts_with("#else") && !rest.is_empty(){
								pos += 1;
								if rest.starts_with("\n"){
									line += 1;
								}
								rest = &file[pos..];
							}
							if rest.starts_with("#else"){
								// ignore #else directive and keep the code it contains
								pos += 5;
							}
						}
					}
				},
				"else" => {
					// #else
					// Skip the code until #endif
					// Note: These can't be nested

					let mut rest = &file[pos..];
					while !rest.starts_with("#endif") && !rest.is_empty(){
						pos += 1;
						if rest.starts_with("\n"){
							line += 1;
						}
						rest = &file[pos..];
					}
				},
				"endif" => {
					// #endif
					// End the current conditional block
					// Ignored by itself, only relevant if there is one of #ifdef, #ifndef or #else before it
					pos += caps.get(0).unwrap().end();
				},
				"error" => {
					// #error [MESSAGE]
					// Print an error message and stop the compilation

					// Get the message
					pos += caps.get(0).unwrap().end();
					let rest = &file[pos..];
					let message;
					if let Some(caps) = replacement_re.captures(rest){
						message = caps.get(0).unwrap().as_str();
					}
					else{
						panic!("{} Line {}: Invalid #error message", filename, line);
					}

					panic!("{} Line {}: {}", filename, line, message.trim_end());
				},
				"print" => {
					// #print [MESSAGE]
					// Print a message at compile time

					// Get the message
					pos += caps.get(0).unwrap().end();
					let rest = &file[pos..];
					let message;
					if let Some(caps) = replacement_re.captures(rest){
						message = caps.get(0).unwrap().as_str();
						pos += caps.get(0).unwrap().end();
					}
					else{
						panic!("{} Line {}: Invalid #print message", filename, line);
					}

					println!("{} Line {}: {}", filename, line, message.trim_end());
					line += message.matches('\n').count();
				},
				"namespace" => {
					// #namespace [NAMESPACE]
					// Define a namespace

					// Get the namespace
					pos += caps.get(0).unwrap().end();
					let rest = &file[pos..];
					let namespace;
					if let Some(caps) = identifier_re.captures(rest){
						namespace = caps.get(0).unwrap().as_str();
						pos += caps.get(0).unwrap().end();
					}
					else{
						panic!("{} Line {}: Invalid #namespace identifier", filename, line);
					}

					{
						// Warning if the namespace is already defined
						let defined_expressions = DEFINED_EXPRESSIONS.lock().unwrap();
						if defined_expressions.contains_key(namespace){
							println!("{} Line {} Warning: #namespace identifier '{}' is already defined", filename, line, namespace);
						}
					}

					namespace_counter += 1;
					processed_file.push_str(format!("namespace {};\n", namespace).as_str());
					full_namespace.push(namespace.trim().to_string());
				}
				"spacename" => {
					// #spacename
					// Undefine a namespace
					
					pos += caps.get(0).unwrap().end();
					namespace_counter -= 1;
					if namespace_counter < 0 {
						panic!("{} Line {}: #spacename directive without matching #namespace", filename, line);
					}
					processed_file.push_str("spacename;\n");
					full_namespace.pop();
				}
				_ => {
					panic!("{} Line {}: Invalid preprocessor directive", filename, line);
				}
			}
		} else if rest.starts_with("\n") {
			processed_file.push('\n');
			pos += 1;
			line += 1;
		} else if let Some(caps) = comments_re.captures(rest) {
			pos += caps.get(0).unwrap().end();
			line += 1;
			processed_file.push_str(&"\n");
		} else if let Some(caps) = multiline_comments_re.captures(rest) {
			pos += caps.get(0).unwrap().end();
			let comment_length = caps.get(0).unwrap().as_str().matches('\n').count();
			line += comment_length;
			processed_file.push_str(&"\n".repeat(comment_length));

		} else if let Some(caps) = identifier_re.captures(rest) {
			let identifier = caps.get(0).unwrap().as_str();

			{
				let defined_expressions = DEFINED_EXPRESSIONS.lock().unwrap();
				if defined_expressions.contains_key(identifier){
					processed_file.push_str(defined_expressions.get(identifier).unwrap());
				} else {
					processed_file.push_str(identifier);
				}
			}
			pos += caps.get(0).unwrap().end();
		} else {
			processed_file.push(chars[pos]);
			pos += 1;
		}
	}
	for _ in 0..namespace_counter{
		processed_file.push_str("spacename;\n");
		full_namespace.pop();
	}
	return processed_file;
}