use regex::Regex;

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
    Exponent,           // **
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
pub fn lex(file: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    // Identifiers can start with a letter or an underscore, followed by any
    // number of letters, numbers, or underscores
    let identifier_re = Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*").unwrap();

    // Primitive types are any of the following strings: int float char void
    let primitive_type_re = Regex::new(r"^(int|float|char|void)").unwrap();

    // Floats are a sequence of digits, followed by a decimal point
    // and more digits
    let float_re = Regex::new(r"^\d+\.\d+").unwrap();

    // Integers are a sequence of digits
    let int_re = Regex::new(r"^\d+").unwrap();

    // (Short) operators are any of the following characters: + - * / % ^ & | ~ < > = ! . ,
    let operator_re = Regex::new(r"^[\+\-\*/%\^&~\|<>=!\.,]").unwrap();

    // Large operators are any of the following strings: == != <= >= && || ** ++ -- << >> += -= *= /= %= <<= >>= &= ^= |= ^^
    let large_operator_re = Regex::new(r"^(==|!=|<=|>=|&&|\|\||\*\*|\+\+|--|<<=|>>=|<<|>>|\+=|-=|\*=|\/=|%=|&=|\^=|\|=|\^\^)").unwrap();

    // Keywords are any of the following strings: if else while for return
    let keyword_re = Regex::new(r"^(if|else|do|while|for|loop|return|fn|let|break|continue)").unwrap();

    let whitespace_re = Regex::new(r"^\s+").unwrap();

    let comments_re = Regex::new(r"^\/\/.*(?:\n|$)").unwrap();

    let multiline_comments_re = Regex::new(r"^\/\*[\s\S]*?\*\/").unwrap();

    let mut pos: usize= 0; // Current position in the file

    while pos < file.len() {
        let rest = &file[pos..];

        if rest.starts_with("(") {
            tokens.push(Token::LParen);
            pos += 1;
        } else if rest.starts_with(")") {
            tokens.push(Token::RParen);
            pos += 1;
        } else if rest.starts_with(",") {
            tokens.push(Token::Comma);
            pos += 1;
        } else if rest.starts_with(";") {
            tokens.push(Token::Semicolon);
            pos += 1;
        } else if rest.starts_with(":") {
            tokens.push(Token::Colon);
            pos += 1;
        } else if rest.starts_with("{") {
            tokens.push(Token::LBrace);
            pos += 1;
        } else if rest.starts_with("}") {
            tokens.push(Token::RBrace);
            pos += 1;
        } else if rest.starts_with("[") {
            tokens.push(Token::LBracket);
            pos += 1;
        } else if rest.starts_with("]") {
            tokens.push(Token::RBracket);
            pos += 1;
        } else if let Some(caps) = comments_re.captures(rest) {
            pos += caps.get(0).unwrap().end();
        } else if let Some(caps) = multiline_comments_re.captures(rest) {
            pos += caps.get(0).unwrap().end();
        } else if let Some(caps) = primitive_type_re.captures(rest) {
            tokens.push(Token::PrimitiveType(caps.get(0).unwrap().as_str().to_string()));
            pos += caps.get(0).unwrap().end();
        } else if let Some(caps) = float_re.captures(rest) {
            tokens.push(Token::FloatLiteral(caps.get(0).unwrap().as_str().parse().unwrap()));
            pos += caps.get(0).unwrap().end();
        } else if let Some(caps) = int_re.captures(rest) {
            tokens.push(Token::IntLiteral(caps.get(0).unwrap().as_str().parse().unwrap()));
            pos += caps.get(0).unwrap().end();
        } else if let Some(caps) = large_operator_re.captures(rest) {
            let op = match caps.get(0).unwrap().as_str() {
                "==" => Operator::Equal,
                "!=" => Operator::NotEqual,
                "<=" => Operator::LessOrEqualThan,
                ">=" => Operator::GreaterOrEqualThan,
                "&&" => Operator::LogicalAnd,
                "||" => Operator::LogicalOr,
                "**" => Operator::Exponent,
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
            tokens.push(Token::Operator(op));
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
            tokens.push(Token::Operator(op));
            pos += caps.get(0).unwrap().end();
        } else if let Some(caps) = keyword_re.captures(rest) {
            tokens.push(Token::Keyword(caps.get(0).unwrap().as_str().to_string()));
            pos += caps.get(0).unwrap().end();
        } else if let Some(caps) = identifier_re.captures(rest) {
            tokens.push(Token::Identifier(caps.get(0).unwrap().as_str().to_string()));
            pos += caps.get(0).unwrap().end();
        } else if let Some(caps) = whitespace_re.captures(rest) {
            pos += caps.get(0).unwrap().end();
        } else {
            panic!("Unexpected character: {}", rest.chars().next().unwrap());
        }
    }

    tokens.push(Token::EOF);

    return tokens;
}