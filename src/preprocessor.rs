use std::collections::HashMap;
use regex::Regex;

/// This module is responsible for preprocessing the input file
/// Preprocessor directives are as follows:
/// - #include <file>
/// - #define <identifier> <replacement>
/// - #ifdef <identifier>
/// - #ifndef <identifier>
/// - #endif
/// - #error <message>
/// - #warning <message>
/// - #line <number> <file>
/// - #print <message>
/// - #undef <identifier>
/// - #if <expression>
/// - #elif <expression>
/// - #else
pub fn preprocessor(file: &str, filename: &str) -> String {
	let mut defined_expressions:HashMap<String, String> = HashMap::new();

	let preprocessor_re = Regex::new(r"^\#(include|define|undef|ifdef|ifndef|else|endif|error|print)").unwrap();

	let identifier_re = Regex::new(r"^(?:\s*)[a-zA-Z_][a-zA-Z0-9_]*").unwrap();
	let replacement_re = Regex::new(r"^\s*(.*)\s*?(?:\n|$)").unwrap();
	let file_re = Regex::new(r"^(?:\s*)<(.*)>").unwrap();

    let comments_re = Regex::new(r"^\/\/.*(?:\n|$)").unwrap();
    let multiline_comments_re = Regex::new(r"^\/\*[\s\S]*?\*\/").unwrap();

	let mut processed_file = String::new();

	let mut pos = 0;
	let mut line = 1;
	while pos < file.len(){
		let rest = &file[pos..];
		if let Some(caps) = preprocessor_re.captures(&file[pos..]){
			let directive = caps.get(1).unwrap().as_str();
			match directive{
				"include" => {
					// #include <file>
					// Include the contents of the file
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

					// Read the file
					let included_file = match std::fs::read_to_string(file){
						Ok(file) => file,
						Err(_) => panic!("{} Line {}: File not found: {}", filename, line, file)
					};

					// Preprocess the included file
					let included_file = preprocessor(&included_file, file);
					processed_file.push_str(&included_file);
				},
				"define" => {
					// #define <identifier> <replacement>
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

						// Warning if the identifier is already defined
						if defined_expressions.contains_key(identifier){
							println!("{} Line {} Warning: #define identifier '{}' is already defined", filename, line, identifier);
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
					defined_expressions.insert(identifier.to_string(), replacement.to_string());
					processed_file.push('\n');	// Keep the line count the same
					line += 1;
				},
				"undef" => {
					// #undef <identifier>
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

					// Warning if the identifier is not defined
					if !defined_expressions.contains_key(identifier){
						println!("{} Line {} Warning: #undef identifier '{}' is not defined", filename, line, identifier);
					}

					// Remove the macro from the defined expressions
					defined_expressions.remove(identifier);
					processed_file.push('\n');	// Keep the line count the same
					line += 1;
				},
				"ifdef" => {
					// #ifdef <identifier>
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
				},
				"ifndef" => {
					// #ifndef <identifier>
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

					// Check if the identifier is defined, and skip the code until #endif or #else
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
				},
				"else" => {
					// #else
					// Skip the code until #endif
					// Note: These can't be nested

					let mut rest = &file[pos..];
					while !rest.starts_with("#endif"){
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
					// #error <message>
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
					// #print <message>
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
				_ => {}
			}
		} else if rest.starts_with("\n") {
			processed_file.push('\n');
			pos += 1;
			line += 1;
		} else if let Some(caps) = comments_re.captures(rest) {
			pos += caps.get(0).unwrap().end();
			line += 1;
		} else if let Some(caps) = multiline_comments_re.captures(rest) {
			pos += caps.get(0).unwrap().end();
			line += caps.get(0).unwrap().as_str().matches('\n').count();
		} else if let Some(caps) = identifier_re.captures(rest) {
			let identifier = caps.get(0).unwrap().as_str();
			if defined_expressions.contains_key(identifier){
				processed_file.push_str(defined_expressions.get(identifier).unwrap());
			} else {
				processed_file.push_str(identifier);
			}
			pos += caps.get(0).unwrap().end();
		} else {
			processed_file.push(file.chars().nth(pos).unwrap());
			pos += 1;
		}
	}
	return processed_file;
}