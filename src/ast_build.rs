use crate::lexer::Operator;
use crate::lexer::Token;
use crate::lexer::TokenWithDebugInfo;
use core::panic;
use std::slice::Iter;
use std::vec;

// OPERATOR PRECEDENCE TABLE:
// 1. Member access (.)
// 2. Pre increment (++a) Pre decrement (--a) Unary plus (+a) Unary minus (-a) Logical not (!a) Bitwise not (~a) Dereference (*a) Address of (&a)
// 3. Multiplication (a * b) Division (a / b) Modulus (a % b)
// 4. Addition (a + b) Subtraction (a - b)
// 5. Bitwise left shift (a << b) Bitwise right shift (a >> b)
// 6. Less than (a < b) Greater than (a > b) Less than or equal to (a <= b) Greater than or equal to (a >= b)
// 7. Equal to (a == b) Not equal to (a != b)
// 8. Bitwise and (a & b)
// 9. Bitwise xor (a ^ b)
// 10. Bitwise or (a | b)
// 11. Logical and (a && b)
// 12. Logical xor (a ^^ b)
// 13. Logical or (a || b)
// 14. Assignment (a = b) Add assignment (a += b) Subtract assignment (a -= b) Multiply assignment (a *= b) Divide assignment (a /= b) Modulus assignment (a %= b) Left shift assignment (a <<= b) Right shift assignment (a >>= b) Bitwise and assignment (a &= b) Bitwise xor assignment (a ^= b) Bitwise or assignment (a |= b)

/// Represents a literal value in the AST.
#[derive(Debug, Clone)]
pub enum Literal {
    Int(i64),
    Float(f64),
    Hex(i64),
    Binary(i64),
	Char(char),
	String(String),
}

/// Represents an atom in the AST.
///
/// An atom is the smallest unit of an expression. It can be a constant, an expression, a variable, a function call or an array access.
#[derive(Debug, Clone)]
pub enum Atom {
    Literal(Literal),
    Expression(Box<Expression>),
    Variable(String),
    FunctionCall(String, Vec<Expression>),
    ArrayAccess(String, Vec<Expression>), // identifier[exp1, exp2, ...]
    Array(Vec<Expression>, i64),          // Array literal with a given number of dimensions
}

// In let bindings, the left hand side of the assignment
#[derive(Debug, Clone)]
pub enum AssignmentIdentifier {
    Variable(String),
    Dereference(Box<AssignmentIdentifier>),
    Array(String, Vec<Expression>), // identifier[dim1, dim2, ...]
}

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Int,
    Float,
    Void,
	Char,
    Pointer(Box<Type>),
    Array(Box<Type>), // array[type]. Strings are array[char]
    Function(Box<Type>, Vec<Type>),
}

/// Represents a unary operator in the AST.
#[derive(Debug, PartialEq, Clone)]
pub enum UnOp {
    // TODO: We only support prefix unary operators for now
    PreIncrement,
    PreDecrement,
    UnaryPlus,
    UnaryMinus,
    LogicalNot,
    BitwiseNot,
    Dereference,
    AddressOf,
    NotAUnaryOp, // Not pretty but it makes the code nicer
}

/// Represents a binary operator in the AST.
#[derive(Debug, PartialEq, Clone)]
pub enum BinOp {
    MemberAccess,
    Multiply,
    Divide,
    Modulus,
    Add,
    Subtract,
    LeftShift,
    RightShift,
    LessThan,
    GreaterThan,
    LessOrEqualThan,
    GreaterOrEqualThan,
    Equal,
    NotEqual,
    BitwiseAnd,
    BitwiseXor,
    BitwiseOr,
    LogicalAnd,
    LogicalXor,
    LogicalOr,
    NotABinaryOp, // Not pretty but it makes the code nicer
}

/// Represents an assignment operator in the AST.
#[derive(Debug, PartialEq, Clone)]
pub enum AssignmentOp {
    Assign,
    AddAssign,
    SubtractAssign,
    MultiplyAssign,
    DivideAssign,
    ModulusAssign,
    LeftShiftAssign,
    RightShiftAssign,
    BitwiseAndAssign,
    BitwiseXorAssign,
    BitwiseOrAssign,
    NotAnAssignmentOp, // Not pretty but it makes the code nicer
}

/// Represents an operator precedence level in the AST.
///
/// A precedence level is a group of operators that have the same precedence.
/// Precedence levels are used to determine the order of operations in an expression.
///
/// # Precedence levels
/// 1. Member access (.)
/// 2. Pre increment (++a) Pre decrement (--a) Unary plus (+a) Unary minus (-a) Logical not (!a) Bitwise not (~a) Dereference (*a) Address of (&a)
/// 3. Multiplication (a * b) Division (a / b) Modulus (a % b)
/// 4. Addition (a + b) Subtraction (a - b)
/// 5. Bitwise left shift (a << b) Bitwise right shift (a >> b)
/// 6. Less than (a < b) Greater than (a > b) Less than or equal to (a <= b) Greater than or equal to (a >= b)
/// 7. Equal to (a == b) Not equal to (a != b)
/// 8. Bitwise and (a & b)
/// 9. Bitwise xor (a ^ b)
/// 10. Bitwise or (a | b)
/// 11. Logical and (a && b)
/// 12. Logical xor (a ^^ b)
/// 13. Logical or (a || b)
/// 14. Assignment (a = b) Add assignment (a += b) Subtract assignment (a -= b) Multiply assignment (a *= b) Divide assignment (a /= b) Modulus assignment (a %= b) Left shift assignment (a <<= b) Right shift assignment (a >>= b) Bitwise and assignment (a &= b) Bitwise xor assignment (a ^= b) Bitwise or assignment (a |= b)
#[derive(Debug)]
pub struct PrecedenceLevel {
    binary_ops: Vec<BinOp>,
    unary_ops: Vec<UnOp>,
    assignment_ops: Vec<AssignmentOp>,
}

/// Represents an expression in the AST.
///
/// An expression is a combination of atoms and operators that evaluates to a value.
/// Expressions can be atoms, unary operations, binary operations, or assignments.
#[derive(Debug, Clone)]
pub enum Expression {
    Atom(Atom),
    UnaryOp(Box<Expression>, UnOp),
    BinaryOp(Box<Expression>, Box<Expression>, BinOp),
    Assignment(AssignmentIdentifier, Box<Expression>, AssignmentOp),
	TypeCast(Box<Expression>, Type),
}

/// Gets the binary operator corresponding to the token.
fn get_bin_operator_from_token(token: &TokenWithDebugInfo) -> BinOp {
    match token {
        TokenWithDebugInfo {
            internal_tok: Token::Operator(op),
            ..
        } => match op {
            Operator::MemberAccess => BinOp::MemberAccess,
            Operator::Multiply => BinOp::Multiply,
            Operator::Divide => BinOp::Divide,
            Operator::Modulus => BinOp::Modulus,
            Operator::Add => BinOp::Add,
            Operator::Subtract => BinOp::Subtract,
            Operator::LeftShift => BinOp::LeftShift,
            Operator::RightShift => BinOp::RightShift,
            Operator::LessThan => BinOp::LessThan,
            Operator::GreaterThan => BinOp::GreaterThan,
            Operator::LessOrEqualThan => BinOp::LessOrEqualThan,
            Operator::GreaterOrEqualThan => BinOp::GreaterOrEqualThan,
            Operator::Equal => BinOp::Equal,
            Operator::NotEqual => BinOp::NotEqual,
            Operator::BitwiseAnd => BinOp::BitwiseAnd,
            Operator::BitwiseXor => BinOp::BitwiseXor,
            Operator::BitwiseOr => BinOp::BitwiseOr,
            Operator::LogicalAnd => BinOp::LogicalAnd,
            Operator::LogicalXor => BinOp::LogicalXor,
            Operator::LogicalOr => BinOp::LogicalOr,
            _ => BinOp::NotABinaryOp,
        },
        _ => BinOp::NotABinaryOp,
    }
}

/// Gets the assignment operator corresponding to the token.
fn get_assign_operator_from_token(token: &TokenWithDebugInfo) -> AssignmentOp {
    match token {
        TokenWithDebugInfo {
            internal_tok: Token::Operator(op),
            ..
        } => match op {
            Operator::Assign => AssignmentOp::Assign,
            Operator::AddAssign => AssignmentOp::AddAssign,
            Operator::SubtractAssign => AssignmentOp::SubtractAssign,
            Operator::MultiplyAssign => AssignmentOp::MultiplyAssign,
            Operator::DivideAssign => AssignmentOp::DivideAssign,
            Operator::ModulusAssign => AssignmentOp::ModulusAssign,
            Operator::LeftShiftAssign => AssignmentOp::LeftShiftAssign,
            Operator::RightShiftAssign => AssignmentOp::RightShiftAssign,
            Operator::BitwiseAndAssign => AssignmentOp::BitwiseAndAssign,
            Operator::BitwiseXorAssign => AssignmentOp::BitwiseXorAssign,
            Operator::BitwiseOrAssign => AssignmentOp::BitwiseOrAssign,
            _ => AssignmentOp::NotAnAssignmentOp,
        },
        _ => AssignmentOp::NotAnAssignmentOp,
    }
}

/// Gets the unary operator corresponding to the token.
fn get_un_operator_from_token(token: &TokenWithDebugInfo) -> UnOp {
    match token {
        TokenWithDebugInfo {
            internal_tok: Token::Operator(op),
            ..
        } => match op {
            Operator::Increment => UnOp::PreIncrement,
            Operator::Decrement => UnOp::PreDecrement,
            Operator::Add => UnOp::UnaryPlus,
            Operator::Subtract => UnOp::UnaryMinus,
            Operator::LogicalNot => UnOp::LogicalNot,
            Operator::BitwiseNot => UnOp::BitwiseNot,
            Operator::Multiply => UnOp::Dereference,
            Operator::BitwiseAnd => UnOp::AddressOf,
            _ => error_unexpected_token("valid unary operator", token),
        },
        _ => UnOp::NotAUnaryOp,
    }
}

/// Represents a statement in the AST.
///
/// A statement is a single instruction in the program.
/// Statements can be assignments, let bindings, if statements, while loops, loops, do-while loops, for loops, return statements, expressions, compound statements, break statements, or continue statements.
#[derive(Debug, Clone)]
pub enum Statement {
    Let(AssignmentIdentifier, Option<Expression>, Type),
    If(Expression, Box<Statement>, Option<Box<Statement>>),
    While(Expression, Box<Statement>),
    Loop(Box<Statement>),
    Dowhile(Expression, Box<Statement>),
    For(Expression, Expression, Expression, Box<Statement>),
    Return(Expression),
    Expression(Expression),
    Compound(Vec<Statement>),
    Break,
    Continue,
}

/// Represents a function in the AST.
#[derive(Debug, Clone)]
pub enum Function {
	Function(String, Vec<(String, Type)>, Statement, Type),
}

/// Represents a constant in the AST.
/// Constants can only be assigned on declaration, and can only be assigned a literal,
/// so they're not terribly useful as of now. They're basically static globals.
#[derive(Debug, Clone)]
pub enum Constant {
    Constant(String, Literal, Type),
}

/// Represents a program in the AST.
#[derive(Debug, Clone)]
pub enum Program {
    Program(Vec<Function>, Vec<Constant>),
}

/// Represents the abstract syntax tree (AST) of a program.
#[derive(Debug, Clone)]
pub struct Ast {
    pub program: Program,
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Int(i) => write!(f, "{}", i),
            Literal::Float(fl) => write!(f, "{}", fl),
            Literal::Hex(h) => write!(f, "0x{:x}", h),
            Literal::Binary(b) => write!(f, "0b{:b}", b),
			Literal::Char(c) => write!(f, "'{}'", c),
			Literal::String(s) => write!(f, "\"{}\"", s),// TODO: this may be fucked
        }
    }
}

fn error(msg: &str, token: &TokenWithDebugInfo) -> ! {
    panic!("{} Line {}: {}", token.file, token.line, msg);
}

fn error_unexpected_token(expected: &str, token: &TokenWithDebugInfo) -> ! {
    error(
        &format!("Expected {}, found: {:?}", expected, token.internal_tok),
        token,
    );
}

/// Parses a constant from a token.
fn parse_literal(token: &TokenWithDebugInfo) -> Literal {
    match token {
        TokenWithDebugInfo {
            internal_tok: Token::IntLiteral(i),
            ..
        } => Literal::Int(*i),
        TokenWithDebugInfo {
            internal_tok: Token::FloatLiteral(f),
            ..
        } => Literal::Float(*f),
        TokenWithDebugInfo {
            internal_tok: Token::HexLiteral(h),
            ..
        } => Literal::Hex(*h),
        TokenWithDebugInfo {
            internal_tok: Token::BinLiteral(b),
            ..
        } => Literal::Binary(*b),
        TokenWithDebugInfo {
            internal_tok: Token::BoolLiteral(b),
            ..
        } => {
			if *b {
				Literal::Int(1)
			} else {
				Literal::Int(0)
			}
		},
		TokenWithDebugInfo {
			internal_tok: Token::CharLiteral(c),
			..
		} => Literal::Char(*c),
        _ => error_unexpected_token("constant", token),
    }
}

/// Parses an atom from a list of tokens.
fn parse_atom(mut tokens: &mut Iter<TokenWithDebugInfo>) -> Atom {
    let tok = tokens.next().unwrap();

    // Check if what we're parsing is an array literal
    if let TokenWithDebugInfo {
        internal_tok: Token::LBracket,
        ..
    } = tok
    {
        return parse_array(&mut tokens);
    }

    match tok {
        TokenWithDebugInfo {
            internal_tok: Token::LParen,
            ..
        } => {
            let inner_exp = parse_expression(&mut tokens);

            if let TokenWithDebugInfo {
                internal_tok: Token::RParen,
                ..
            } = tokens.next().unwrap()
            {
                return Atom::Expression(Box::new(inner_exp));
            } else {
                error_unexpected_token("closing parenthesis", tok);
            }
        }
        TokenWithDebugInfo {
            internal_tok: Token::IntLiteral(_),
            ..
        }
        | TokenWithDebugInfo {
            internal_tok: Token::FloatLiteral(_),
            ..
        }
        | TokenWithDebugInfo {
            internal_tok: Token::BinLiteral(_),
            ..
        }
        | TokenWithDebugInfo {
            internal_tok: Token::HexLiteral(_),
            ..
        }
        | TokenWithDebugInfo {
            internal_tok: Token::BoolLiteral(_),
            ..
        } 
		| TokenWithDebugInfo {
			internal_tok: Token::CharLiteral(_),
			..
		} => {
            return Atom::Literal(parse_literal(tok));
        },
		TokenWithDebugInfo {
			internal_tok: Token::StringLiteral(s),
			..
		} => {
			return Atom::Array(
				s.chars().map(|c| Expression::Atom(Atom::Literal(Literal::Char(c)))).collect(),
				s.len() as i64
			);
		}
        TokenWithDebugInfo {
            internal_tok: Token::Identifier(s),
            ..
        } => {
            let next_tok = tokens.clone().next().unwrap();
            if let TokenWithDebugInfo {
                internal_tok: Token::LParen,
                ..
            } = next_tok
            {
                // Function call
                tokens.next();
                let mut args = Vec::new();
                loop {
                    let next_tok = tokens.clone().next().unwrap();
                    if let TokenWithDebugInfo {
                        internal_tok: Token::RParen,
                        ..
                    } = next_tok
                    {
                        tokens.next();
                        break;
                    } else if let TokenWithDebugInfo {
                        internal_tok: Token::Comma,
                        ..
                    } = next_tok
                    {
                        tokens.next();
                    } else {
                        args.push(parse_expression(&mut tokens));
                    }
                }
                return Atom::FunctionCall(s.to_string(), args);
            } else if let TokenWithDebugInfo {
                internal_tok: Token::LBracket,
                ..
            } = next_tok
            {
                // Array access
                tokens.next();
                let mut args = Vec::new();
                loop {
                    let next_tok = tokens.clone().next().unwrap();
                    if let TokenWithDebugInfo {
                        internal_tok: Token::RBracket,
                        ..
                    } = next_tok
                    {
                        tokens.next();
                        break;
                    } else if let TokenWithDebugInfo {
                        internal_tok: Token::Comma,
                        ..
                    } = next_tok
                    {
                        tokens.next();
                    } else {
                        args.push(parse_expression(&mut tokens));
                    }
                }
                return Atom::ArrayAccess(s.to_string(), args);
            }

            // Variable
            return Atom::Variable(s.to_string());
        }
        _ => error_unexpected_token("valid atom token", tok),
    }
}

fn parse_assignment_identifier(mut tokens: &mut Iter<TokenWithDebugInfo>) -> AssignmentIdentifier {
    let tok = tokens.next().unwrap();

    match tok {
        TokenWithDebugInfo {
            internal_tok: Token::Operator(Operator::Multiply),
            ..
        } => {
            return AssignmentIdentifier::Dereference(Box::new(parse_assignment_identifier(
                &mut tokens,
            )));
        }
        TokenWithDebugInfo {
            internal_tok: Token::Identifier(s),
            ..
        } => {
            let next_tok = tokens.clone().next().unwrap();
            if let TokenWithDebugInfo {
                internal_tok: Token::LBracket,
                ..
            } = next_tok
            {
                // Array access
                tokens.next();
                let mut args = Vec::new();
                loop {
                    let next_tok = tokens.clone().next().unwrap();
                    if let TokenWithDebugInfo {
                        internal_tok: Token::RBracket,
                        ..
                    } = next_tok
                    {
                        tokens.next();
                        break;
                    } else if let TokenWithDebugInfo {
                        internal_tok: Token::Comma,
                        ..
                    } = next_tok
                    {
                        tokens.next();
                    } else {
                        args.push(parse_expression(&mut tokens));
                    }
                }
                return AssignmentIdentifier::Array(s.to_string(), args);
            }
            return AssignmentIdentifier::Variable(s.to_string());
        }
        _ => error_unexpected_token("valid assignment identifier", tok),
    }
}

fn find_arr_dims(array: &Atom) -> Option<Vec<i64>> {
    match array {
        Atom::Array(sub_elements, dim) => {
            let mut dims: Vec<i64> = Vec::new();

            for expr in sub_elements {
                let elem = match expr {
                    Expression::Atom(atom) => atom,
                    _ => continue,
                };

                if !matches!(elem, Atom::Array(_, _)) {
                    continue;
                }

                let sub_dims = find_arr_dims(elem).unwrap();
				for (i, sub_dim) in sub_dims.iter().enumerate() {
					if i >= dims.len() {
						dims.push(*sub_dim);
					} else if dims[i] < *sub_dim {
						dims[i] = *sub_dim;
					}
				}
            }
			dims.insert(0, *dim);
            return Some(dims);
        }
        _ => None,
    }
}

fn rectangularize_array(array: &mut Vec<Expression>, depth: usize, max_dims: &Vec<i64>) {
	let max_size = max_dims[depth] as usize;
	if depth+1 < max_dims.len() {
		for elem in array.iter_mut() {
			if let Expression::Atom(Atom::Array(ref mut sub_array, _)) = elem {
				rectangularize_array(sub_array, depth + 1, max_dims);
			} else {
				let mut new_elem = vec![elem.clone()];
				rectangularize_array(&mut new_elem, depth + 1, max_dims);
				*elem = Expression::Atom(Atom::Array(new_elem.clone(), new_elem.len() as i64));
			}
		}
	}

	while array.len() < max_size {
		if depth == max_dims.len() - 1 {
			array.push(Expression::Atom(Atom::Literal(Literal::Int(0))));
		} else {
			let mut new_elem = vec![];
			rectangularize_array(&mut new_elem, depth + 1, max_dims);
			array.push(Expression::Atom(Atom::Array(new_elem.clone(), new_elem.len() as i64)));
		}
	}
}

fn flatten(array: &Vec<Expression>) -> Vec<Expression> {
	let mut flat_arr = Vec::new();
	for elem in array.iter() {
		if let Expression::Atom(Atom::Array(ref sub_array, _)) = elem {
			flatten(sub_array);
			for sub_elem in sub_array.iter() {
				flat_arr.push(sub_elem.clone());
			}
		} else {
			flat_arr.push(elem.clone());
		}
	}
	return flat_arr;
}

fn parse_array(tokens: &mut Iter<TokenWithDebugInfo>) -> Atom {
    let mut elements = Vec::new();
    loop {
        let next_tok = tokens.clone().next().unwrap();
        if let TokenWithDebugInfo {
            internal_tok: Token::RBracket,
            ..
        } = next_tok
        {
            tokens.next();
            break;
        } else if let TokenWithDebugInfo {
            internal_tok: Token::Comma,
            ..
        } = next_tok
        {
            tokens.next();
        } else {
            elements.push(parse_expression(tokens));
        }
    }

    // Find the max size in each dimension
	let array = Atom::Array(elements.clone(), elements.len() as i64);
    let max_dims = find_arr_dims(&array).unwrap();
	rectangularize_array(&mut elements, 0, &max_dims);
	let flat_elements = flatten(&elements);

    return Atom::Array(flat_elements, max_dims[0]);
}

/// Recursively parses an expression, taking into account operator precedence.
fn parse_expression_with_precedence(
    mut tokens: &mut Iter<TokenWithDebugInfo>,
    precedence_level: usize,
    precedence_table: &Vec<PrecedenceLevel>,
) -> Expression {
    if precedence_level == 0 {
        // Parse the lowest precedence, like literals or atoms
        return Expression::Atom(parse_atom(&mut tokens));
    }

    // In case we are actually in an assignment expression the left hand side
    // has some special rules, so it is parsed separately in parse_assignment_identifier.
    // We start by assuming we are not in an assignment expression, and if it turns out
    // we are, we will change the expr variable to an Assignment expression.
    // In the meantime we keep a copy of the tokens iterator to be able to backtrack
    let mut next_if_assignment = tokens.clone();

    // Check if the current token is a unary operator for this precedence level
    let mut next = tokens.clone().next().unwrap();
    let mut expr;

    if precedence_table[precedence_level]
        .unary_ops
        .contains(&get_un_operator_from_token(&next))
    {
        let tok = tokens.next().unwrap();
        let op = get_un_operator_from_token(tok); // Get the unary operator
        let operand =
            parse_expression_with_precedence(&mut tokens, precedence_level, precedence_table); // Parse operand
        expr = Expression::UnaryOp(Box::new(operand), op); // Apply unary operator
    } else {
        // No unary operator, so parse the next lower precedence level
        expr =
            parse_expression_with_precedence(&mut tokens, precedence_level - 1, precedence_table);
    }

    // Now handle binary and assignment operators for the current precedence level
    next = tokens.clone().next().unwrap();
    while precedence_table[precedence_level]
        .binary_ops
        .contains(&get_bin_operator_from_token(&next))
        || precedence_table[precedence_level]
            .assignment_ops
            .contains(&get_assign_operator_from_token(&next))
    {
        let tok = tokens.next().unwrap();
        let op = get_bin_operator_from_token(tok); // Get the binary operator

        if op == BinOp::NotABinaryOp {
            // If it's not a binary operator, it must be an assignment operator
            let op = get_assign_operator_from_token(tok); // Get the assignment operator
            let next_term = parse_expression_with_precedence(
                &mut tokens,
                precedence_level - 1,
                precedence_table,
            ); // Parse next term

            // Re-parse the left hand side of the assignment expression
            let assignment_identifier = parse_assignment_identifier(&mut next_if_assignment);
            expr = Expression::Assignment(assignment_identifier, Box::new(next_term), op);
        } else {
            let next_term = parse_expression_with_precedence(
                &mut tokens,
                precedence_level - 1,
                precedence_table,
            ); // Parse next term
            expr = Expression::BinaryOp(Box::new(expr), Box::new(next_term), op);
        }
        next = tokens.clone().next().unwrap();
    }

	// Type cast
	if let TokenWithDebugInfo {
		internal_tok: Token::Colon,
		..
	} = next {
		tokens.next();
		let type_casted = parse_type(&mut tokens);
		expr = Expression::TypeCast(Box::new(expr), type_casted);
	}

    expr
}

/// Builds the precedence table for the parser.
///
/// The precedence table is used to determine the order of operations in an expression.
/// It is a list of precedence levels, where each level contains a list of binary operators, unary operators, and assignment operators.
/// The levels are ordered from highest to lowest precedence.
///
/// # Precedence levels
/// 1. Member access (.)
/// 2. Pre increment (++a) Pre decrement (--a) Unary plus (+a) Unary minus (-a) Logical not (!a) Bitwise not (~a) Dereference (*a) Address of (&a)
/// 3. Exponentiation (a ** b)
/// 4. Multiplication (a * b) Division (a / b) Modulus (a % b)
/// 5. Addition (a + b) Subtraction (a - b)
/// 6. Bitwise left shift (a << b) Bitwise right shift (a >> b)
/// 7. Less than (a < b) Greater than (a > b) Less than or equal to (a <= b) Greater than or equal to (a >= b)
/// 8. Equal to (a == b) Not equal to (a != b)
/// 9. Bitwise and (a & b)
/// 10. Bitwise xor (a ^ b)
/// 11. Bitwise or (a | b)
/// 12. Logical and (a && b)
/// 13. Logical xor (a ^^ b)
/// 14. Logical or (a || b)
/// 15. Assignment (a = b) Add assignment (a += b) Subtract assignment (a -= b) Multiply assignment (a *= b) Divide assignment (a /= b) Modulus assignment (a %= b) Left shift assignment (a <<= b) Right shift assignment (a >>= b) Bitwise and assignment (a &= b) Bitwise xor assignment (a ^= b) Bitwise or assignment (a |= b)
fn build_precedence_table() -> Vec<PrecedenceLevel> {
    vec![
        PrecedenceLevel {
            binary_ops: vec![BinOp::MemberAccess],
            unary_ops: vec![],
            assignment_ops: vec![],
        },
        PrecedenceLevel {
            binary_ops: vec![],
            unary_ops: vec![
                UnOp::PreIncrement,
                UnOp::PreDecrement,
                UnOp::UnaryPlus,
                UnOp::UnaryMinus,
                UnOp::LogicalNot,
                UnOp::BitwiseNot,
                UnOp::Dereference,
                UnOp::AddressOf,
            ],
            assignment_ops: vec![],
        },
        PrecedenceLevel {
            binary_ops: vec![BinOp::Multiply, BinOp::Divide, BinOp::Modulus],
            unary_ops: vec![],
            assignment_ops: vec![],
        },
        PrecedenceLevel {
            binary_ops: vec![BinOp::Add, BinOp::Subtract],
            unary_ops: vec![],
            assignment_ops: vec![],
        },
        PrecedenceLevel {
            binary_ops: vec![BinOp::LeftShift, BinOp::RightShift],
            unary_ops: vec![],
            assignment_ops: vec![],
        },
        PrecedenceLevel {
            binary_ops: vec![
                BinOp::LessThan,
                BinOp::GreaterThan,
                BinOp::LessOrEqualThan,
                BinOp::GreaterOrEqualThan,
            ],
            unary_ops: vec![],
            assignment_ops: vec![],
        },
        PrecedenceLevel {
            binary_ops: vec![BinOp::Equal, BinOp::NotEqual],
            unary_ops: vec![],
            assignment_ops: vec![],
        },
        PrecedenceLevel {
            binary_ops: vec![BinOp::BitwiseAnd],
            unary_ops: vec![],
            assignment_ops: vec![],
        },
        PrecedenceLevel {
            binary_ops: vec![BinOp::BitwiseXor],
            unary_ops: vec![],
            assignment_ops: vec![],
        },
        PrecedenceLevel {
            binary_ops: vec![BinOp::BitwiseOr],
            unary_ops: vec![],
            assignment_ops: vec![],
        },
        PrecedenceLevel {
            binary_ops: vec![BinOp::LogicalAnd],
            unary_ops: vec![],
            assignment_ops: vec![],
        },
        PrecedenceLevel {
            binary_ops: vec![BinOp::LogicalXor],
            unary_ops: vec![],
            assignment_ops: vec![],
        },
        PrecedenceLevel {
            binary_ops: vec![BinOp::LogicalOr],
            unary_ops: vec![],
            assignment_ops: vec![],
        },
        PrecedenceLevel {
            binary_ops: vec![],
            unary_ops: vec![],
            assignment_ops: vec![
                AssignmentOp::Assign,
                AssignmentOp::AddAssign,
                AssignmentOp::SubtractAssign,
                AssignmentOp::MultiplyAssign,
                AssignmentOp::DivideAssign,
                AssignmentOp::ModulusAssign,
                AssignmentOp::LeftShiftAssign,
                AssignmentOp::RightShiftAssign,
                AssignmentOp::BitwiseAndAssign,
                AssignmentOp::BitwiseXorAssign,
                AssignmentOp::BitwiseOrAssign,
            ],
        },
    ]
}

fn parse_type(mut tokens: &mut Iter<TokenWithDebugInfo>) -> Type {
    let tok = tokens.next().unwrap();

    match tok {
        TokenWithDebugInfo {
            internal_tok: Token::PrimitiveType(k),
            ..
        } => {
            if k == "int" {
                return Type::Int;
            } else if k == "float" {
                return Type::Float;
            } else if k == "bool" {
                return Type::Int;
            } else if k == "void" {
                return Type::Void;
            } else if k == "char" {
				return Type::Char;
			} else if k == "str" {
				return Type::Array(Box::new(Type::Char));
			} else if k == "array" {
                let next_tok = tokens.next().unwrap();
                if !matches!(
                    next_tok,
                    TokenWithDebugInfo {
                        internal_tok: Token::LBracket,
                        ..
                    }
                ) {
                    error_unexpected_token("opening bracket", next_tok);
                }

                let inner_type = parse_type(&mut tokens);
                let next_tok = tokens.next().unwrap();

                if !matches!(
                    next_tok,
                    TokenWithDebugInfo {
                        internal_tok: Token::RBracket,
                        ..
                    }
                ) {
                    error_unexpected_token("closing bracket", next_tok);
                }
                return Type::Array(Box::new(inner_type));
            } else {
                error_unexpected_token("valid type keyword", tok);
            }
        }
        TokenWithDebugInfo {
            internal_tok: Token::Operator(Operator::Multiply),
            ..
        } => {
            let inner_type = parse_type(&mut tokens);
            return Type::Pointer(Box::new(inner_type));
        }
        TokenWithDebugInfo {
            internal_tok: Token::LParen,
            ..
        } => {
            let return_type = parse_type(&mut tokens);
            let mut params = Vec::new();
            loop {
                let next_tok = tokens.clone().next().unwrap();
                if let TokenWithDebugInfo {
                    internal_tok: Token::RParen,
                    ..
                } = next_tok
                {
                    tokens.next();
                    break;
                } else if let TokenWithDebugInfo {
                    internal_tok: Token::Comma,
                    ..
                } = next_tok
                {
                    tokens.next();
                } else {
                    params.push(parse_type(&mut tokens));
                }
            }
            return Type::Function(Box::new(return_type), params);
        }
        _ => error_unexpected_token("valid type token", tok),
    }
}

/// Parses an expression from a list of tokens.
///
/// This function is a wrapper around parse_expression_with_precedence that uses the highest precedence level.
/// It is used to parse the top-level expression.
///
/// An expression is a combination of atoms and operators that evaluates to a value.
fn parse_expression(mut tokens: &mut Iter<TokenWithDebugInfo>) -> Expression {
    let precedence_table = build_precedence_table();
    let max_precedence = precedence_table.len() - 1;

    parse_expression_with_precedence(&mut tokens, max_precedence, &precedence_table)
}

/// Parses a statement from a list of tokens.
///
/// A statement is a single instruction in the program.
/// Statements can be assignments, let bindings, if statements, while loops, loops, do-while loops, for loops, return statements, expressions, compound statements, break statements, or continue statements.
fn parse_statement(tokens: &mut Iter<TokenWithDebugInfo>) -> Statement {
    let tok = tokens.clone().next().unwrap();
    let statement;

    match tok {
        TokenWithDebugInfo {
            internal_tok: Token::Keyword(k),
            ..
        } => {
            tokens.next();
            if k == "return" {
                // return exp;
                let exp = parse_expression(tokens);
                statement = Statement::Return(exp);
            } else if k == "let" {
                // let id: type = exp;

                let id = parse_assignment_identifier(tokens);

                let next_tok = tokens.next().unwrap();
                if &Token::Colon != next_tok {
                    error_unexpected_token("colon", next_tok);
                };

                let var_type = parse_type(tokens); // TODO: Do something with this

                let next_tok = tokens.clone().next().unwrap();
                if let TokenWithDebugInfo {
                    internal_tok: Token::Operator(Operator::Assign),
                    ..
                } = next_tok
                {
                    tokens.next();
                    let exp = parse_expression(tokens);
                    statement = Statement::Let(id, Some(exp), var_type);
                } else if let TokenWithDebugInfo {
                    internal_tok: Token::Semicolon,
                    ..
                } = next_tok
                {
                    statement = Statement::Let(id, None, var_type);
                } else {
                    error_unexpected_token("semicolon or assignment operator", next_tok);
                }
            } else if k == "if" {
                // if (exp) statement [else statement]
                let exp = parse_expression(tokens);
                let mut if_stmt = parse_statement(tokens);

                if !matches!(if_stmt, Statement::Compound(_)) {
                    if_stmt = Statement::Compound(vec![if_stmt]);
                }

                let next_tok = tokens.clone().next().unwrap();
                if let TokenWithDebugInfo {
                    internal_tok: Token::Keyword(k),
                    ..
                } = next_tok
                {
                    if k == "else" {
                        tokens.next();
                        let mut else_stmt = parse_statement(tokens);

                        if !matches!(else_stmt, Statement::Compound(_)) {
                            else_stmt = Statement::Compound(vec![else_stmt]);
                        }

                        statement =
                            Statement::If(exp, Box::new(if_stmt), Some(Box::new(else_stmt)));
                    } else {
                        statement = Statement::If(exp, Box::new(if_stmt), None);
                    }
                } else {
                    statement = Statement::If(exp, Box::new(if_stmt), None);
                }
            } else if k == "while" {
                // while (exp) statement
                let exp = parse_expression(tokens);
                let mut while_stmt = parse_statement(tokens);

                if !matches!(while_stmt, Statement::Compound(_)) {
                    while_stmt = Statement::Compound(vec![while_stmt]);
                }

                statement = Statement::While(exp, Box::new(while_stmt));
            } else if k == "loop" {
                // loop statement
                let mut loop_stmt = parse_statement(tokens);

                if let Statement::Compound(_) = loop_stmt {
                } else {
                    loop_stmt = Statement::Compound(vec![loop_stmt]);
                }

                statement = Statement::Loop(Box::new(loop_stmt));
            } else if k == "for" {
                // for (exp; exp; exp) statement
                let next_tok = tokens.next().unwrap(); // Skip opening parenthesis
                if !matches!(
                    next_tok,
                    TokenWithDebugInfo {
                        internal_tok: Token::LParen,
                        ..
                    }
                ) {
                    error_unexpected_token("opening parenthesis", next_tok);
                }
                let init = parse_expression(tokens);
                let next_tok = tokens.next().unwrap(); // Skip semicolon
                if !matches!(
                    next_tok,
                    TokenWithDebugInfo {
                        internal_tok: Token::Semicolon,
                        ..
                    }
                ) {
                    error_unexpected_token("semicolon", next_tok);
                }
                let cond = parse_expression(tokens);
                let next_tok = tokens.next().unwrap(); // Skip semicolon
                if !matches!(
                    next_tok,
                    TokenWithDebugInfo {
                        internal_tok: Token::Semicolon,
                        ..
                    }
                ) {
                    error_unexpected_token("semicolon", next_tok);
                }
                let step = parse_expression(tokens);
                let next_tok = tokens.next().unwrap(); // Skip closing parenthesis
                if !matches!(
                    next_tok,
                    TokenWithDebugInfo {
                        internal_tok: Token::RParen,
                        ..
                    }
                ) {
                    error_unexpected_token("closing parenthesis", next_tok);
                }
                let mut for_stmt = parse_statement(tokens);

                if !matches!(for_stmt, Statement::Compound(_)) {
                    for_stmt = Statement::Compound(vec![for_stmt]);
                }

                statement = Statement::For(init, cond, step, Box::new(for_stmt));
            } else if k == "do" {
                // do statement while (exp);
                let mut do_stmt = parse_statement(tokens);

                if !matches!(do_stmt, Statement::Compound(_)) {
                    do_stmt = Statement::Compound(vec![do_stmt]);
                }

                let next_tok = tokens.next().unwrap();
                if let TokenWithDebugInfo {
                    internal_tok: Token::Keyword(k),
                    ..
                } = next_tok
                {
                    if k == "while" {
                        let exp = parse_expression(tokens);
                        statement = Statement::Dowhile(exp, Box::new(do_stmt));
                    } else {
                        error_unexpected_token("while keyword", next_tok);
                    }
                } else {
                    error_unexpected_token("while keyword", next_tok);
                }
            } else if k == "break" {
                statement = Statement::Break;
            } else if k == "continue" {
                statement = Statement::Continue;
            } else {
                error_unexpected_token("valid keyword token", tok);
            }
        }
        TokenWithDebugInfo {
            internal_tok: Token::LBrace,
            ..
        } => {
            tokens.next();
            let mut statements = Vec::new();

            loop {
                let next = tokens.clone().next().unwrap();

                if let TokenWithDebugInfo {
                    internal_tok: Token::RBrace,
                    ..
                } = next
                {
                    tokens.next();
                    break;
                } else {
                    statements.push(parse_statement(tokens));
                }
            }

            statement = Statement::Compound(statements);
        }
        _ => {
            let exp = parse_expression(tokens);
            statement = Statement::Expression(exp);
        }
    }

    // Compound, If statements and loops don't need a semicolon
    if let Statement::Compound(_) = statement {
        return statement;
    } else if let Statement::If(_, _, _) = statement {
        return statement;
    } else if let Statement::While(_, _) = statement {
        return statement;
    } else if let Statement::Loop(_) = statement {
        return statement;
    } else if let Statement::For(_, _, _, _) = statement {
        return statement;
    }

    let tok = tokens.next().unwrap();
    if !matches!(
        tok,
        TokenWithDebugInfo {
            internal_tok: Token::Semicolon,
            ..
        }
    ) {
        error_unexpected_token("semicolon", tok);
    }

    return statement;
}

fn parse_const(tokens: &mut Iter<TokenWithDebugInfo>) -> Constant {
    // const id: type = exp;

    let next_tok = tokens.next().unwrap();

    if &Token::Keyword("const".to_string()) != next_tok {
        error_unexpected_token("const keyword", next_tok);
    }

    let next_tok = tokens.next().unwrap();
    let id = match next_tok {
        TokenWithDebugInfo {
            internal_tok: Token::Identifier(id),
            ..
        } => id.clone(),
        _ => error_unexpected_token("identifier", next_tok),
    };

    let next_tok = tokens.next().unwrap();
    if &Token::Colon != next_tok {
        error_unexpected_token("colon", next_tok);
    };

    let var_type = parse_type(tokens); // TODO: Do something with this

    let lit;
    let next_tok = tokens.clone().next().unwrap();
    if let TokenWithDebugInfo {
        internal_tok: Token::Operator(Operator::Assign),
        ..
    } = next_tok
    {
        tokens.next();
        lit = parse_literal(tokens.next().unwrap());
    } else {
        error_unexpected_token(
            "assignment operator (constants must be assigned on declaration)",
            next_tok,
        );
    }

    let next_tok = tokens.next().unwrap();
    if &Token::Semicolon != next_tok {
        error_unexpected_token("semicolon", next_tok);
    }

    return Constant::Constant(id.to_string(), lit, var_type);
}

/// Parses a function from a list of tokens.
/// fn id(params): type statement
fn parse_function(mut tokens: &mut Iter<TokenWithDebugInfo>) -> Function {
    let tok = tokens.next().unwrap();

    match tok {
        TokenWithDebugInfo {
            internal_tok: Token::Keyword(ref k),
            ..
        } => {
            if k != "fn" {
                error_unexpected_token("fn keyword", tok)
            }
        }
        TokenWithDebugInfo { .. } => error_unexpected_token("fn keyword", tok),
    };

    let tok = tokens.next().unwrap();
    let id = match tok {
        TokenWithDebugInfo {
            internal_tok: Token::Identifier(id),
            ..
        } => id.clone(),
        TokenWithDebugInfo { .. } => error_unexpected_token("identifier", tok),
    };

    let tok = tokens.next().unwrap();
    if !matches!(
        tok,
        TokenWithDebugInfo {
            internal_tok: Token::LParen,
            ..
        }
    ) {
        error_unexpected_token("opening parenthesis", tok);
    };

    let mut params = Vec::new();

    loop {
        let tok = tokens.next().unwrap();

        if let TokenWithDebugInfo {
            internal_tok: Token::RParen,
            ..
        } = tok
        {
            break;
        } else if let TokenWithDebugInfo {
            internal_tok: Token::Comma,
            ..
        } = tok
        {
            continue;
        } else if let TokenWithDebugInfo {
            internal_tok: Token::Identifier(id),
            ..
        } = tok
        {
            let tok = tokens.next().unwrap();
            if !matches!(
                tok,
                TokenWithDebugInfo {
                    internal_tok: Token::Colon,
                    ..
                }
            ) {
				dbg!(tok);
                error_unexpected_token("colon", tok);
            }
			params.push((id.clone(), parse_type(&mut tokens)));
        } else {
            error_unexpected_token("identifier or closing parenthesis", tok);
        }
    }

	let tok = tokens.next().unwrap();
	if !matches!(
		tok,
		TokenWithDebugInfo {
			internal_tok: Token::Colon,
			..
		}
	) {
		error_unexpected_token("colon", tok);
	}

	let return_type = parse_type(&mut tokens);

    let mut statement = parse_statement(&mut tokens);

    // If the statement is not a compound statement, wrap it in one
    // This is to allow okay-ish scope handling
    if let Statement::Compound(_) = statement {
    } else {
        statement = Statement::Compound(vec![statement]);
    }

    return Function::Function(id, params, statement, return_type);
}

/// Parses an abstract syntax tree (AST) from a list of tokens.
pub fn parse(tokens: &Vec<TokenWithDebugInfo>) -> Ast {
    let mut tokens = tokens.iter();
    let mut functions = Vec::new();
    let mut constants = Vec::new();

    // Parse functions until there are no more tokens
    loop {
        let next_tok = tokens.clone().next().unwrap();
        match next_tok {
            TokenWithDebugInfo {
                internal_tok: Token::EOF,
                ..
            } => break,
            TokenWithDebugInfo {
                internal_tok: Token::Keyword(ref k),
                ..
            } if k == "fn" => functions.push(parse_function(&mut tokens)),
            TokenWithDebugInfo {
                internal_tok: Token::Keyword(ref k),
                ..
            } if k == "const" => constants.push(parse_const(&mut tokens)),
            TokenWithDebugInfo { .. } => {
                error_unexpected_token("function or constant declaration", next_tok)
            }
        }
    }

    let ast = Ast {
        program: Program::Program(functions, constants),
    };
    return ast;
}
