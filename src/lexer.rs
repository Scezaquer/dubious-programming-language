use regex::Regex;
use std::i64;
use crate::shared::TokenWithDebugInfo;

/// All the operators that the lexer can recognize.
///
/// The Operator enum represents all the operators that the lexer can
/// recognize. This includes arithmetic operators, bitwise operators,
/// comparison operators, logical operators, and assignment operators.
/// The Operator enum is used to represent operators in the Token enum.
/// The Operator enum derives the Debug, PartialEq, and Clone traits.
#[derive(Debug)]
#[derive(PartialEq, Clone, Copy)]
pub enum Operator {
    Add,                // +
    Subtract,           // -
    Multiply,           // *
    Divide,             // /
    Modulus,            // %
    BitwiseAnd,         // &
    BitwiseOr,          // |
    BitwiseXor,         // ^
    BitwiseNot,         // ~
    LessThan,           // <
    GreaterThan,        // >
    Assign,             // =
    LogicalAnd,         // &&
    LogicalOr,          // ||
    LogicalNot,         // !
    LogicalXor,         // ^^
    Equal,              // ==
    NotEqual,           // !=
    LessOrEqualThan,    // <=
    Increment,          // ++
    Decrement,          // --
    LeftShift,          // <<
    AddAssign,          // +=
    SubtractAssign,     // -=
    MultiplyAssign,     // *=
    DivideAssign,       // /=
    ModulusAssign,      // %=
    BitwiseAndAssign,   // &=
    BitwiseXorAssign,   // ^=
    BitwiseOrAssign,    // |=
	Comma,              // ,
	MemberAccess,       // .
	DoubleColon,		// ::
}

// We need special treatment for the operators that start with a '>'.
// Since there is ambiguity at the tokenization level between
// generics syntax and oprators that start with '>', we don't use
// the tokenization level to determine if it's a generic or an operator.

// e.g in 'let a: S<T>= ..;' we don't want to parse the '>=' as
// a 'greater than' operator, but as a generic binding followed
// by an assignment operator.


/// All the tokens that the lexer can recognize.
/// 
/// The Token enum represents all the tokens that the lexer can recognize.
/// This includes identifiers, primitive types, literals, operators, and
/// keywords. The Token enum is used to represent tokens in the list of
/// tokens that the lexer returns. The Token enum derives the Debug, PartialEq,
/// and Clone traits.
#[derive(Debug)]
#[derive(PartialEq, Clone)]
pub enum Token {
    Identifier(String),
    PrimitiveType(String),
    FloatLiteral(f64),
    IntLiteral(i64),
	CharLiteral(String),
	StringLiteral(String),
	BinLiteral(i64),
	HexLiteral(i64),
	BoolLiteral(bool),
    Operator(Operator),
    LParen,
    RParen,
    Comma,
    Semicolon,
    Colon,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
	BeginGeneric, // :<
    Keyword(String),
    EOF
}

impl TokenWithDebugInfo<Token> {
	pub fn new(internal_tok: Token, line: usize, file: String) -> TokenWithDebugInfo<Token> {
		TokenWithDebugInfo {
			internal_tok,
			line,
			file
		}
	}
}

impl PartialEq<TokenWithDebugInfo<Token>> for Token {
	fn eq(&self, other: &TokenWithDebugInfo<Token>) -> bool {
		self == &other.internal_tok
	}
}

fn process_escape_sequence(s: &str) -> String {
	// Process escape sequences in the string literal
	let mut processed = String::new();
	let mut chars = s.chars().peekable();
	while let Some(c) = chars.next() {
		if c == '\\' && chars.peek().is_some() {
			match chars.next().unwrap() {
				'n' => processed.push('\n'),
				'r' => processed.push('\r'),
				't' => processed.push('\t'),
				'\\' => processed.push('\\'),
				'\'' => processed.push('\''),
				'"' => processed.push('"'),
				'0' => processed.push('\0'),
				c => processed.push(c), // Unknown escape, keep as is
			}
		} else {
			processed.push(c);
		}
	}
	return processed;
}

/// Lexes the input file and returns a list of tokens.
/// 
/// The lex function takes a string representing the contents of a file
/// and returns a list of tokens. The lexer recognizes identifiers, primitive
/// types, literals, operators, and keywords. This function uses regular expressions
/// to match the tokens in the input file. It skips whitespace and comments, and panics
/// if it encounters an unexpected character.
/// 
/// # Examples
/// 
/// ```
/// let file = "fn main() { return 0; }";
/// let tokens = lex(file);
/// assert_eq!(tokens.len(), 10);
/// ```
/// 
/// # Panics
/// 
/// The lex function panics if it encounters an unexpected character in the input file.
/// 
/// # Errors
/// 
/// The lex function does not return any errors.
/// 
/// # Safety
/// 
/// The lex function is safe to use with any input file.
/// 
/// # Performance
/// 
/// The lex function has a time complexity of O(n), where n is the length of the input file.
/// The lex function has a space complexity of O(n), where n is the length of the input file.
pub fn lex(file: &str) -> Vec<TokenWithDebugInfo<Token>> {
    let mut tokens : Vec<TokenWithDebugInfo<Token>> = Vec::new();

    // Identifiers can start with a letter or an underscore, followed by any
    // number of letters, numbers, or underscores
    let identifier_re = Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*").unwrap();

    // Primitive types are any of the following strings: int float char void array
	let primitive_type_re = Regex::new(r"^(int|float|bool|char|str|void|array)$").unwrap();

    // Floats are a sequence of digits, followed by a decimal point
    // and more digits
    let float_re = Regex::new(r"^-?\d+\.\d+").unwrap();

    // Integers are a sequence of digits
    let int_re = Regex::new(r"^\d+").unwrap();

	// Binary literals are a sequence of 0s and 1s, prefixed by 0b
	let bin_re = Regex::new(r"^0[bB][01]+").unwrap();

	// Hex literals are a sequence of 0-9 and a-f/A-F, prefixed by 0x
	let hex_re = Regex::new(r"^0[xX][0-9a-fA-F]+").unwrap();

	let bool_re = Regex::new(r"^(true|false)$").unwrap();

    // (Short) operators are any of the following characters: + - * / % ^ & | ~ < > = ! . ,
    let operator_re = Regex::new(r"^[\+\-\*/%\^&~\|<>=!\.,]").unwrap();

    // Large operators are any of the following strings: == != <= >= && || ++ -- << >> += -= *= /= %= <<= >>= &= ^= |= ^^
    let large_operator_re = Regex::new(r"^(==|!=|<=|&&|\|\||\+\+|--|<<=|<<|\+=|-=|\*=|\/=|%=|&=|\^=|\|=|\^\^|::)").unwrap();

    // Keywords are any of the following strings: if else while for return
    let keyword_re = Regex::new(r"^(if|else|do|while|for|loop|return|fn|let|break|continue|const|struct|enum|asm|namespace|spacename)$").unwrap();

	let whitespace_re = Regex::new(r"^[^\S\n]+").unwrap();
	
	let char_re = Regex::new(r"^'(?:\\.|[^\\']){0,8}'").unwrap();

	let string_re = Regex::new(r#"^"(?:\\.|[^\\"])*""#).unwrap();

	// Since the preprocessor will have removed actual comments, it's safe to assume that
	// any line starting with // was something the preprocesser added
	let imported_code_re = Regex::new(r"^//\s<.*>\n").unwrap();
	let mut current_file = String::new();

    let mut pos: usize = 0; // Current position in the file

	// The current line number in the file
	let mut line_hashmap: std::collections::HashMap<String, usize> = std::collections::HashMap::new();

    while pos < file.len() {
        let rest = &file[pos..];
		let tok;
        if rest.starts_with("//") && let Some(caps) = imported_code_re.captures(rest){
			current_file = caps.get(0).unwrap().as_str()
				.trim_start_matches("// <")
				.trim_end_matches(">\n")
				.to_string();
			if !line_hashmap.contains_key(&current_file){
				line_hashmap.insert(current_file.clone(), 1);
			}
			pos += caps.get(0).unwrap().end();
			continue;
		} else if rest.starts_with("\n") {
			if let Some(file) = line_hashmap.get_mut(&current_file){
				*file += 1;
			}
			pos += 1;
			continue;
		} else if rest.starts_with("(") {
            tok = Token::LParen;
            pos += 1;
        } else if rest.starts_with(")") {
            tok = Token::RParen;
            pos += 1;
        } else if rest.starts_with(",") {
            tok = Token::Comma;
            pos += 1;
        } else if rest.starts_with(";") {
            tok = Token::Semicolon;
            pos += 1;
        } else if rest.starts_with("{") {
            tok = Token::LBrace;
            pos += 1;
        } else if rest.starts_with("}") {
            tok = Token::RBrace;
            pos += 1;
        } else if rest.starts_with("[") {
            tok = Token::LBracket;
            pos += 1;
        } else if rest.starts_with("]") {
            tok = Token::RBracket;
            pos += 1;
		} else if rest.starts_with("0") && let Some(caps) = bin_re.captures(rest) {
			tok = Token::BinLiteral(i64::from_str_radix(caps.get(0).unwrap().as_str().strip_prefix("0b").unwrap(), 2).unwrap());
			pos += caps.get(0).unwrap().end();
		} else if rest.starts_with("0") && let Some(caps) = hex_re.captures(rest) {
			tok = Token::HexLiteral(i64::from_str_radix(caps.get(0).unwrap().as_str().strip_prefix("0x").unwrap(), 16).unwrap());
			pos += caps.get(0).unwrap().end();
		} else if let Some(caps) = float_re.captures(rest) {
            let f = caps.get(0).unwrap().as_str().parse().unwrap();
            tok = Token::FloatLiteral(f);
            if f < 0.0 {
                tokens.push(TokenWithDebugInfo::<Token>::new(Token::Operator(Operator::Add), line_hashmap[&current_file], current_file.clone()));
            }
            pos += caps.get(0).unwrap().end();
        } else if let Some(caps) = int_re.captures(rest) {
            tok = Token::IntLiteral(caps.get(0).unwrap().as_str().parse().unwrap());
            pos += caps.get(0).unwrap().end();
        } else if rest.starts_with("'") && let Some(caps) = char_re.captures(rest) {
			pos += caps.get(0).unwrap().end();
			if let Some(file) = line_hashmap.get_mut(&current_file){
				*file += caps.get(0).unwrap().as_str().matches('\n').count();
			}
			tok = Token::CharLiteral(process_escape_sequence(caps.get(0).unwrap().as_str().strip_prefix("'").unwrap().strip_suffix("'").unwrap()));
		} else if rest.starts_with('"') && let Some(caps) = string_re.captures(rest) {
			pos += caps.get(0).unwrap().end();
			if let Some(file) = line_hashmap.get_mut(&current_file){
				*file += caps.get(0).unwrap().as_str().matches('\n').count();
			}
			tok = Token::StringLiteral(process_escape_sequence(caps.get(0).unwrap().as_str().strip_prefix("\"").unwrap().strip_suffix("\"").unwrap()));
		} else if large_operator_re.is_match(rest) {
            let op = match &rest[..2] {
                // Commented out operators are handled in the AST build phase
                // ">=" => Operator::GreaterOrEqualThan,
                // ">>" => Operator::RightShift,
                // ">>=" => Operator::RightShiftAssign,
                // "<<" => Operator::LeftShiftAssign,
                "==" => Operator::Equal,
                "!=" => Operator::NotEqual,
                "<=" => Operator::LessOrEqualThan,
                "&&" => Operator::LogicalAnd,
                "||" => Operator::LogicalOr,
                "++" => Operator::Increment,
                "--" => Operator::Decrement,
                "<<" => Operator::LeftShift,
                "+=" => Operator::AddAssign,
                "-=" => Operator::SubtractAssign,
                "*=" => Operator::MultiplyAssign,
                "/=" => Operator::DivideAssign,
                "%=" => Operator::ModulusAssign,
                "&=" => Operator::BitwiseAndAssign,
                "^=" => Operator::BitwiseXorAssign,
                "|=" => Operator::BitwiseOrAssign,
                "^^" => Operator::LogicalXor,
				"::" => Operator::DoubleColon,
                _ => unreachable!(),
            };
            tok = Token::Operator(op);
            pos += 2;
        } else if operator_re.is_match(rest) {
            let op = match &rest[..1] {
                "+" => Operator::Add,
                "-" => Operator::Subtract,
                "*" => Operator::Multiply,
                "/" => Operator::Divide,
                "%" => Operator::Modulus,
                "&" => Operator::BitwiseAnd,
                "|" => Operator::BitwiseOr,
                "<" => Operator::LessThan,
                ">" => Operator::GreaterThan,
                "=" => Operator::Assign,
                "!" => Operator::LogicalNot,
                "~" => Operator::BitwiseNot,
                "^" => Operator::BitwiseXor,
				"," => Operator::Comma,
				"." => Operator::MemberAccess,
                _ => unreachable!(),
            };
            tok = Token::Operator(op);
            pos += 1;
        } else if rest.starts_with(":<") {
			tok = Token::BeginGeneric;
			pos += 2;
		} else if rest.starts_with(":") {
            tok = Token::Colon;
            pos += 1;
        } else if identifier_re.is_match(rest) && let Some(caps) = identifier_re.captures(rest) {
			if primitive_type_re.is_match(caps.get(0).unwrap().as_str()) {
				tok = Token::PrimitiveType(caps.get(0).unwrap().as_str().to_string());
			} else if keyword_re.is_match(caps.get(0).unwrap().as_str()) {
				tok = Token::Keyword(caps.get(0).unwrap().as_str().to_string());
			} else if bool_re.is_match(caps.get(0).unwrap().as_str()) {
				tok = Token::BoolLiteral(caps.get(0).unwrap().as_str() == "true");
			} else {
				tok = Token::Identifier(caps.get(0).unwrap().as_str().to_string());
			}
            pos += caps.get(0).unwrap().end();
        } else if let Some(caps) = whitespace_re.captures(rest) {
            pos += caps.get(0).unwrap().end();
			continue;
        } else {
            panic!("Unexpected character: {}", rest.chars().next().unwrap());
        }

		tokens.push(TokenWithDebugInfo::<Token>::new(tok, line_hashmap[&current_file], current_file.clone()));
    }

    tokens.push(TokenWithDebugInfo::<Token>::new(Token::EOF, line_hashmap[&current_file], current_file.clone()));

    return tokens;
}