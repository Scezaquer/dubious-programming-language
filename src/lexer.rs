use regex::Regex;
use std::i64;

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
    GreaterOrEqualThan, // >=
    Increment,          // ++
    Decrement,          // --
    LeftShift,          // <<
    RightShift,         // >>
    AddAssign,          // +=
    SubtractAssign,     // -=
    MultiplyAssign,     // *=
    DivideAssign,       // /=
    ModulusAssign,      // %=
    LeftShiftAssign,    // <<=
    RightShiftAssign,   // >>=
    BitwiseAndAssign,   // &=
    BitwiseXorAssign,   // ^=
    BitwiseOrAssign,    // |=
    MemberAccess,       // .
	Comma,              // ,
}

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
    Keyword(String),
    EOF,
}

/// A token with extra debug information to print more useful error messages.
#[derive(Debug)]
#[derive(PartialEq, Clone)]
pub struct TokenWithDebugInfo {
	pub internal_tok: Token,
	pub line: usize,
	pub file: String
}

impl TokenWithDebugInfo {
	pub fn new(internal_tok: Token, line: usize, file: String) -> TokenWithDebugInfo {
		TokenWithDebugInfo {
			internal_tok,
			line,
			file
		}
	}
}

impl PartialEq<TokenWithDebugInfo> for Token {
	fn eq(&self, other: &TokenWithDebugInfo) -> bool {
		self == &other.internal_tok
	}
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
pub fn lex(file: &str) -> Vec<TokenWithDebugInfo> {
    let mut tokens : Vec<TokenWithDebugInfo> = Vec::new();

    // Identifiers can start with a letter or an underscore, followed by any
    // number of letters, numbers, or underscores
    let identifier_re = Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*").unwrap();

    // Primitive types are any of the following strings: int float char void array
    let primitive_type_re = Regex::new(r"^(int|float|char|void|array)").unwrap();

    // Floats are a sequence of digits, followed by a decimal point
    // and more digits
    let float_re = Regex::new(r"^\d+\.\d+").unwrap();

    // Integers are a sequence of digits
    let int_re = Regex::new(r"^\d+").unwrap();

	// Binary literals are a sequence of 0s and 1s, prefixed by 0b
	let bin_re = Regex::new(r"^0b[01]+").unwrap();

	// Hex literals are a sequence of 0-9 and a-f/A-F, prefixed by 0x
	let hex_re = Regex::new(r"^0x[0-9a-fA-F]+").unwrap();

	let bool_re = Regex::new(r"^(true|false)").unwrap();

    // (Short) operators are any of the following characters: + - * / % ^ & | ~ < > = ! . ,
    let operator_re = Regex::new(r"^[\+\-\*/%\^&~\|<>=!\.,]").unwrap();

    // Large operators are any of the following strings: == != <= >= && || ++ -- << >> += -= *= /= %= <<= >>= &= ^= |= ^^
    let large_operator_re = Regex::new(r"^(==|!=|<=|>=|&&|\|\||\+\+|--|<<=|>>=|<<|>>|\+=|-=|\*=|\/=|%=|&=|\^=|\|=|\^\^)").unwrap();

    // Keywords are any of the following strings: if else while for return
    let keyword_re = Regex::new(r"^(if|else|do|while|for|loop|return|fn|let|break|continue|const)").unwrap();

	let whitespace_re = Regex::new(r"^[^\S\n]+").unwrap();

	let preprocessor_re = Regex::new(r"^\#(include|define|undef|ifdef|ifndef|if|elif|else|endif|error|print).*?(\n|$)").unwrap();

	// Since the preprocessor will have removed actual comments, it's safe to assume that
	// any line starting with // was something the preprocesser added
	let imported_code_re = Regex::new(r"^//\s<.*>\n").unwrap();
	let mut current_file = String::new();

    let mut pos: usize = 0; // Current position in the file
	let mut line_hashmap: std::collections::HashMap<String, usize> = std::collections::HashMap::new();

    while pos < file.len() {
        let rest = &file[pos..];
		let tok;
		if let Some(caps) = preprocessor_re.captures(rest){
			// Skip the preprocessor directive
			pos += caps.get(0).unwrap().end();
			continue;
		} else if let Some(caps) = imported_code_re.captures(rest){
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
        } else if rest.starts_with(":") {
            tok = Token::Colon;
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
        } else if let Some(caps) = primitive_type_re.captures(rest) {
            tok = Token::PrimitiveType(caps.get(0).unwrap().as_str().to_string());
            pos += caps.get(0).unwrap().end();
        } else if let Some(caps) = bin_re.captures(rest) {
			tok = Token::BinLiteral(i64::from_str_radix(caps.get(0).unwrap().as_str().strip_prefix("0b").unwrap(), 2).unwrap());
			pos += caps.get(0).unwrap().end();
		} else if let Some(caps) = hex_re.captures(rest) {
			tok = Token::HexLiteral(i64::from_str_radix(caps.get(0).unwrap().as_str().strip_prefix("0x").unwrap(), 16).unwrap());
			pos += caps.get(0).unwrap().end();
		} else if let Some(caps) = float_re.captures(rest) {
            tok = Token::FloatLiteral(caps.get(0).unwrap().as_str().parse().unwrap());
            pos += caps.get(0).unwrap().end();
        } else if let Some(caps) = int_re.captures(rest) {
            tok = Token::IntLiteral(caps.get(0).unwrap().as_str().parse().unwrap());
            pos += caps.get(0).unwrap().end();
        } else if let Some(caps) = bool_re.captures(rest) {
			tok = Token::BoolLiteral(caps.get(0).unwrap().as_str() == "true");
			pos += caps.get(0).unwrap().end();
		} else if let Some(caps) = large_operator_re.captures(rest) {
            let op = match caps.get(0).unwrap().as_str() {
                "==" => Operator::Equal,
                "!=" => Operator::NotEqual,
                "<=" => Operator::LessOrEqualThan,
                ">=" => Operator::GreaterOrEqualThan,
                "&&" => Operator::LogicalAnd,
                "||" => Operator::LogicalOr,
                "++" => Operator::Increment,
                "--" => Operator::Decrement,
                "<<=" => Operator::LeftShiftAssign,
                ">>=" => Operator::RightShiftAssign,
                "<<" => Operator::LeftShift,
                ">>" => Operator::RightShift,
                "+=" => Operator::AddAssign,
                "-=" => Operator::SubtractAssign,
                "*=" => Operator::MultiplyAssign,
                "/=" => Operator::DivideAssign,
                "%=" => Operator::ModulusAssign,
                "&=" => Operator::BitwiseAndAssign,
                "^=" => Operator::BitwiseXorAssign,
                "|=" => Operator::BitwiseOrAssign,
                "^^" => Operator::LogicalXor,
                _ => unreachable!(),
            };
            tok = Token::Operator(op);
            pos += caps.get(0).unwrap().end();
        } else if let Some(caps) = operator_re.captures(rest) {
            let op = match caps.get(0).unwrap().as_str() {
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
                "." => Operator::MemberAccess,
				"," => Operator::Comma,
                _ => unreachable!(),
            };
            tok = Token::Operator(op);
            pos += caps.get(0).unwrap().end();
        } else if let Some(caps) = keyword_re.captures(rest) {
            tok = Token::Keyword(caps.get(0).unwrap().as_str().to_string());
            pos += caps.get(0).unwrap().end();
        } else if let Some(caps) = identifier_re.captures(rest) {
            tok = Token::Identifier(caps.get(0).unwrap().as_str().to_string());
            pos += caps.get(0).unwrap().end();
        } else if let Some(caps) = whitespace_re.captures(rest) {
            pos += caps.get(0).unwrap().end();
			continue;
        } else {
            panic!("Unexpected character: {}", rest.chars().next().unwrap());
        }

		tokens.push(TokenWithDebugInfo::new(tok, line_hashmap[&current_file], current_file.clone()));
    }

    tokens.push(TokenWithDebugInfo::new(Token::EOF, line_hashmap[&current_file], current_file.clone()));

    return tokens;
}