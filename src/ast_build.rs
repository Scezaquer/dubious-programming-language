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

/// Monad to attach types to elements of the AST
#[derive(Debug, Clone)]
pub struct Typed<T> {
	pub expr: T,
	pub type_: Type,
}

impl<T> Typed<T> {
	/// Creates a new Typed instance with a void type.
	pub fn new(expr: T) -> Self {
		Typed {
			expr,
			type_: Type::Void,
		}
	}

	pub fn new_with_type(expr: T, type_: Type) -> Self {
		Typed {
			expr,
			type_,
		}
	}

	pub fn get_type(&self) -> &Type {
		&self.type_
	}
}


/// Represents a literal value in the AST.
#[derive(Debug, Clone)]
pub enum Literal {
	Bool(bool),
    Int(i64),
    Float(f64),
    Hex(i64),
    Binary(i64),
    Char(String),
}

/// Represents an atom in the AST.
///
/// An atom is the smallest unit of an expression. It can be a constant, an expression, a variable, a function call or an array access.
#[derive(Debug, Clone)]
pub enum Atom {
    Literal(Typed<Literal>),
    Expression(Box<Typed<Expression>>),
    Variable(String),
    FunctionCall(String, Vec<Typed<Expression>>),
    Array(Vec<Typed<Expression>>, i64), // Array literal with a given number of dimensions
    StructInstance(String, Vec<Typed<Expression>>), // Struct instance with a given number of fields
}

// In let bindings, the left hand side of the assignment
#[derive(Debug, Clone)]
pub enum AssignmentIdentifier {
    Variable(String),
    Dereference(Box<Typed<AssignmentIdentifier>>),
    Array(String, Vec<Typed<Expression>>), // identifier[dim1, dim2, ...]
}

// In assignments, the left hand side of the assignment. This is NOT the same
// as AssignmentIdentifier, as it can be more complex.
// struct.member = ... is a ReassignmentIdentifier, but not an AssignmentIdentifier
#[derive(Debug, Clone)]
pub enum ReassignmentIdentifier {
    Variable(String),
    Dereference(Box<Typed<Expression>>),
    Array(Box<Typed<Expression>>, Vec<Typed<Expression>>),
    MemberAccess(Box<Typed<Expression>>, Box<Typed<Expression>>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Int,
    Float,
    Void,
    Char,
	Bool,
    Pointer(Box<Type>),
    Array(Box<Type>), // array[type]. Strings are array[char]
    Struct(String),
	Enum(String),
	Namespace(String, Box<Type>)
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
	NamespaceAccess,
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
    Atom(Typed<Atom>),
    UnaryOp(Box<Typed<Expression>>, UnOp),
    BinaryOp(Box<Typed<Expression>>, Box<Typed<Expression>>, BinOp),
    Assignment(Typed<ReassignmentIdentifier>, Box<Typed<Expression>>, AssignmentOp),
    TypeCast(Box<Typed<Expression>>, Type),
    ArrayAccess(Box<Typed<Expression>>, Vec<Typed<Expression>>),
}

/// Gets the binary operator corresponding to the token.
fn get_bin_operator_from_token(token: &TokenWithDebugInfo) -> BinOp {
    match token {
        TokenWithDebugInfo {
            internal_tok: Token::Operator(op),
            ..
        } => match op {
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
            Operator::MemberAccess => BinOp::MemberAccess,
			Operator::DoubleColon => BinOp::NamespaceAccess,
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
    Let(AssignmentIdentifier, Option<Typed<Expression>>, Type),
    If(Typed<Expression>, Box<Typed<Statement>>, Option<Box<Typed<Statement>>>),
    While(Typed<Expression>, Box<Typed<Statement>>),
    Loop(Box<Typed<Statement>>),
    Dowhile(Typed<Expression>, Box<Typed<Statement>>),
    For(Typed<Expression>, Typed<Expression>, Typed<Expression>, Box<Typed<Statement>>),
    Return(Typed<Expression>),
    Expression(Typed<Expression>),
    Compound(Vec<Typed<Statement>>),
    Break,
    Continue,
	Asm(String, Type),
}

/// Represents a function in the AST.
#[derive(Debug, Clone)]
pub enum Function {
	//TODO: should be struct instead of enum
    Function(String, Vec<(String, Type)>, Typed<Statement>, Type),
}

/// Represents a constant in the AST.
/// Constants can only be assigned on declaration, and can only be assigned a literal,
/// so they're not terribly useful as of now. They're basically static globals.
#[derive(Debug, Clone)]
pub enum Constant {
    //TODO: should be a struct instead of enum
    Constant(String, Typed<Literal>, Type),
}

#[derive(Debug, Clone)]
pub struct Namespace{
	pub id: String,
	pub functions: Vec<Typed<Function>>,
	pub constants: Vec<Typed<Constant>>,
	pub structs: Vec<Struct>,
	pub enums: Vec<Enum>,
	pub sub_namespaces: Vec<Namespace>,
}

/// Represents the abstract syntax tree (AST) of a program.
#[derive(Debug, Clone)]
pub struct Ast {
    pub program: Namespace,
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
			Literal::Bool(b) => write!(f, "{}", if *b {"0xFFFFFFFFFFFFFFFF"} else {"0"}),
            Literal::Int(i) => write!(f, "{}", i),
            Literal::Float(fl) => write!(f, "{}", fl),
            Literal::Hex(h) => write!(f, "0x{:x}", h),
            Literal::Binary(b) => write!(f, "0b{:b}", b),
            Literal::Char(c) => write!(f, "'{}'", c),
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
fn parse_literal(token: &TokenWithDebugInfo) -> Typed<Literal> {
    match token {
        TokenWithDebugInfo {
            internal_tok: Token::IntLiteral(i),
            ..
        } => Typed::new_with_type(Literal::Int(*i), Type::Int),
        TokenWithDebugInfo {
            internal_tok: Token::FloatLiteral(f),
            ..
        } => Typed::new_with_type(Literal::Float(*f), Type::Float),
        TokenWithDebugInfo {
            internal_tok: Token::HexLiteral(h),
            ..
        } => Typed::new_with_type(Literal::Hex(*h), Type::Int),
        TokenWithDebugInfo {
            internal_tok: Token::BinLiteral(b),
            ..
        } => Typed::new_with_type(Literal::Binary(*b), Type::Int),
        TokenWithDebugInfo {
            internal_tok: Token::BoolLiteral(b),
            ..
        } => Typed::new_with_type(Literal::Bool(*b), Type::Bool),
        TokenWithDebugInfo {
            internal_tok: Token::CharLiteral(c),
            ..
        } => Typed::new_with_type(Literal::Char(c.to_string()), Type::Char),
        _ => error_unexpected_token("constant", token),
    }
}

/// Parses an atom from a list of tokens.
fn parse_atom(mut tokens: &mut Iter<TokenWithDebugInfo>) -> Typed<Atom> {
    let tok = tokens.next().unwrap();

    // Check if what we're parsing is an array literal
    if let TokenWithDebugInfo {
        internal_tok: Token::LBracket,
        ..
    } = tok
    {
        return Typed::new(parse_array(&mut tokens));
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
                return Typed::new(Atom::Expression(Box::new(Typed::new(inner_exp))));
            } else {
                error_unexpected_token("closing parenthesis", &tok);
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
			let lit = parse_literal(&tok);
			let t = lit.get_type().clone();
            return Typed::new_with_type(Atom::Literal(lit), t);
        }
        TokenWithDebugInfo {
            internal_tok: Token::StringLiteral(s),
            ..
        } => {
            return Typed::new(Atom::Array(
                s.chars()
                    .collect::<Vec<_>>()
                    .chunks(8)
                    .map(|chunk| {
                        Typed::new(Expression::Atom(Typed::new(Atom::Literal(Typed::new(Literal::Char(chunk.iter().collect()))))))
                    })
                    .collect(),
                (s.len() as i64 + 3) / 8,
            ));
        }
        TokenWithDebugInfo {
            internal_tok: Token::Identifier(s),
            ..
        } => {
            let next_tok = tokens.clone().next().unwrap();
            // TODO: Make this if-else chain into match statement
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
                        args.push(Typed::new(parse_expression(&mut tokens)));
                    }
                }
                return Typed::new(Atom::FunctionCall(s.to_string(), args));
            }
            else if let TokenWithDebugInfo {
                internal_tok: Token::LBrace,
                ..
            } = next_tok
            {
                // Struct instance
                tokens.next();
                let mut fields = Vec::new();
                loop {
                    let next_tok = tokens.clone().next().unwrap();
                    if let TokenWithDebugInfo {
                        internal_tok: Token::RBrace,
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
                        fields.push(Typed::new(parse_expression(&mut tokens)));
                    }
                }
                return Typed::new(Atom::StructInstance(s.to_string(), fields));
            }

            // Variable
            return Typed::new(Atom::Variable(s.to_string()));
        }
        _ => error_unexpected_token("valid atom token", &tok),
    }
}

fn parse_assignment_identifier(mut tokens: &mut Iter<TokenWithDebugInfo>) -> AssignmentIdentifier {
    let tok = tokens.next().unwrap();

    match tok {
        TokenWithDebugInfo {
            internal_tok: Token::Operator(Operator::Multiply),
            ..
        } => {
            return AssignmentIdentifier::Dereference(Box::new(Typed::new(parse_assignment_identifier(
                &mut tokens,
            ))));
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
                        args.push(Typed::new(parse_expression(&mut tokens)));
                    }
                }
                return AssignmentIdentifier::Array(s.to_string(), args);
            }
            return AssignmentIdentifier::Variable(s.to_string());
        }
        _ => error_unexpected_token("valid assignment identifier", &tok),
    }
}

fn parse_reassignment_identifier(expr: Expression) -> ReassignmentIdentifier {
    // pub enum ReassignmentIdentifier {
    // 	Variable(String),
    // 	Dereference(Box<Expression>),
    // 	Array(Box<Expression>, Vec<Expression>),
    // 	MemberAccess(Box<Expression>, Box<Expression>),
    // }

    match expr {
        Expression::Atom(Typed { expr: Atom::Variable(s), .. }) => ReassignmentIdentifier::Variable(s),
        Expression::UnaryOp(expr, UnOp::Dereference) => ReassignmentIdentifier::Dereference(expr),
        Expression::ArrayAccess(expr, args) => ReassignmentIdentifier::Array(expr, args),
        Expression::BinaryOp(expr1, expr2, BinOp::MemberAccess) => {
            ReassignmentIdentifier::MemberAccess(expr1, expr2)
        }
        _ => panic!("Invalid reassignment identifier. Only modifiable lvalues are allowed."),
    }
}

fn find_arr_dims(array: &Atom) -> Option<Vec<i64>> {
    match array {
        Atom::Array(sub_elements, dim) => {
            let mut dims: Vec<i64> = Vec::new();

            for Typed{expr, ..} in sub_elements {
                let Typed{expr: elem, ..} = match expr {
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

fn rectangularize_array(array: &mut Vec<Typed<Expression>>, depth: usize, max_dims: &Vec<i64>) {
    let max_size = max_dims[depth] as usize;
    if depth + 1 < max_dims.len() {
        for Typed{expr: elem , ..} in array.iter_mut() {
            if let Expression::Atom(Typed{expr: Atom::Array(ref mut sub_array, _), ..}) = elem {
                rectangularize_array(sub_array, depth + 1, max_dims);
            } else {
                let mut new_elem = vec![Typed::new(elem.clone())];
                rectangularize_array(&mut new_elem, depth + 1, max_dims);
                *elem = Expression::Atom(Typed::new(Atom::Array(new_elem.clone(), new_elem.len() as i64)));
            }
        }
    }

    while array.len() < max_size {
        if depth == max_dims.len() - 1 {
            array.push(Typed::new(Expression::Atom(Typed::new(Atom::Literal(Typed::new(Literal::Int(0)))))));
        } else {
            let mut new_elem = vec![];
            rectangularize_array(&mut new_elem, depth + 1, max_dims);
            array.push(Typed::new(Expression::Atom(Typed::new(Atom::Array(
                new_elem.clone(),
                new_elem.len() as i64,
            )))));
        }
    }
}

fn flatten(array: &Vec<Typed<Expression>>) -> Vec<Typed<Expression>> {
    let mut flat_arr = Vec::new();
    for Typed{expr: elem, ..} in array.iter() {
        if let Expression::Atom(Typed{expr: Atom::Array(ref sub_array, _), ..}) = elem {
            flatten(sub_array);
            for sub_elem in sub_array.iter() {
                flat_arr.push(sub_elem.clone());
            }
        } else {
            flat_arr.push(Typed::new(elem.clone()));
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
            elements.push(Typed::new(parse_expression(tokens)));
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

    // Check if the current token is a unary operator for this precedence level
    let next = tokens.clone().next().unwrap();
    let mut expr;

    if precedence_table[precedence_level]
        .unary_ops
        .contains(&get_un_operator_from_token(&next))
    {
        let tok = tokens.next().unwrap();
        let op = get_un_operator_from_token(tok); // Get the unary operator
        let operand =
            parse_expression_with_precedence(&mut tokens, precedence_level, precedence_table); // Parse operand
        expr = Expression::UnaryOp(Box::new(Typed::new(operand)), op); // Apply unary operator
    } else {
        // No unary operator, so parse the next lower precedence level
        expr =
            parse_expression_with_precedence(&mut tokens, precedence_level - 1, precedence_table);
    }

    let mut next = tokens.clone().next().unwrap();
    // Array access
    if let TokenWithDebugInfo {
        internal_tok: Token::LBracket,
        ..
    } = next
    {
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
                args.push(Typed::new(parse_expression(&mut tokens)));
            }
        }
        expr = Expression::ArrayAccess(Box::new(Typed::new(expr)), args);
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
            let assignment_identifier = parse_reassignment_identifier(expr);
            expr = Expression::Assignment(Typed::new(assignment_identifier), Box::new(Typed::new(next_term)), op);
        } else {
            let next_term = parse_expression_with_precedence(
                &mut tokens,
                precedence_level - 1,
                precedence_table,
            ); // Parse next term
            expr = Expression::BinaryOp(Box::new(Typed::new(expr)), Box::new(Typed::new(next_term)), op);
        }
        next = tokens.clone().next().unwrap();
    }

    // Type cast
    if let TokenWithDebugInfo {
        internal_tok: Token::Colon,
        ..
    } = next
    {
        tokens.next();
        let type_casted = parse_type(&mut tokens);
        expr = Expression::TypeCast(Box::new(Typed::new(expr)), type_casted);
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
/// 1. Member access (.) Namespace access (::)
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
            binary_ops: vec![],
            unary_ops: vec![],
            assignment_ops: vec![],
        },
        PrecedenceLevel {
            binary_ops: vec![BinOp::MemberAccess, BinOp::NamespaceAccess],
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
                error_unexpected_token("valid type keyword", &tok);
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
            internal_tok: Token::Identifier(id),
            ..
        } => {
			// Here we assume that the identifier is a struct name, but it could also be an enum, or just not exist
			// in case there is no such struct/enum. We check for this in logic_checker.rs (check_if_struct_or_enum)
			let next_tok = tokens.clone().next().unwrap();
			if let TokenWithDebugInfo {
				internal_tok: Token::Operator(Operator::DoubleColon),
				..
			} = next_tok
			{
				tokens.next();
				Type::Namespace(id.to_string(), Box::new(parse_type(tokens)))
			} else {
				Type::Struct(id.to_string())
			}
		}
        _ => error_unexpected_token("valid type token", &tok),
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
                statement = Statement::Return(Typed::new(exp));
            } else if k == "let" {
                // let id: type = exp;

                let id = parse_assignment_identifier(tokens);

                let next_tok = tokens.next().unwrap();
                if &Token::Colon != next_tok {
                    error_unexpected_token("colon", next_tok);
                };

                let var_type = parse_type(tokens);

                let next_tok = tokens.clone().next().unwrap();
                if let TokenWithDebugInfo {
                    internal_tok: Token::Operator(Operator::Assign),
                    ..
                } = next_tok
                {
                    tokens.next();
                    let exp = parse_expression(tokens);
                    statement = Statement::Let(id, Some(Typed::new(exp)), var_type);
                } else if let TokenWithDebugInfo {
                    internal_tok: Token::Semicolon,
                    ..
                } = next_tok
                {
                    statement = Statement::Let(id, None, var_type);
                } else {
                    error_unexpected_token("semicolon or assignment operator", &next_tok);
                }
            } else if k == "if" {
                // if (exp) statement [else statement]
                let exp = parse_expression(tokens);
                let mut if_stmt = parse_statement(tokens);

                if !matches!(if_stmt, Statement::Compound(_)) {
                    if_stmt = Statement::Compound(vec![Typed::new(if_stmt)]);
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
                            else_stmt = Statement::Compound(vec![Typed::new(else_stmt)]);
                        }

                        statement =
                            Statement::If(Typed::new(exp), Box::new(Typed::new(if_stmt)), Some(Box::new(Typed::new(else_stmt))));
                    } else {
                        statement = Statement::If(Typed::new(exp), Box::new(Typed::new(if_stmt)), None);
                    }
                } else {
                    statement = Statement::If(Typed::new(exp), Box::new(Typed::new(if_stmt)), None);
                }
            } else if k == "while" {
                // while (exp) statement
                let exp = parse_expression(tokens);
                let mut while_stmt = parse_statement(tokens);

                if !matches!(while_stmt, Statement::Compound(_)) {
                    while_stmt = Statement::Compound(vec![Typed::new(while_stmt)]);
                }

                statement = Statement::While(Typed::new(exp), Box::new(Typed::new(while_stmt)));
            } else if k == "loop" {
                // loop statement
                let mut loop_stmt = parse_statement(tokens);

                if let Statement::Compound(_) = loop_stmt {
                } else {
                    loop_stmt = Statement::Compound(vec![Typed::new(loop_stmt)]);
                }

                statement = Statement::Loop(Box::new(Typed::new(loop_stmt)));
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
                    for_stmt = Statement::Compound(vec![Typed::new(for_stmt)]);
                }

                statement = Statement::For(Typed::new(init), Typed::new(cond), Typed::new(step), Box::new(Typed::new(for_stmt)));
            } else if k == "do" {
                // do statement while (exp);
                let mut do_stmt = parse_statement(tokens);

                if !matches!(do_stmt, Statement::Compound(_)) {
                    do_stmt = Statement::Compound(vec![Typed::new(do_stmt)]);
                }

                let next_tok = tokens.next().unwrap();
                if let TokenWithDebugInfo {
                    internal_tok: Token::Keyword(k),
                    ..
                } = next_tok
                {
                    if k == "while" {
                        let exp = parse_expression(tokens);
                        statement = Statement::Dowhile(Typed::new(exp), Box::new(Typed::new(do_stmt)));
                    } else {
                        error_unexpected_token("while keyword", &next_tok);
                    }
                } else {
                    error_unexpected_token("while keyword", &next_tok);
                }
            } else if k == "break" {
                statement = Statement::Break;
            } else if k == "continue" {
                statement = Statement::Continue;
            } else if k == "asm" {
				let mut asm = String::new();
				let next_tok = tokens.next().unwrap();
				if let TokenWithDebugInfo { internal_tok: Token::StringLiteral(s), .. } = next_tok {
					asm.push_str(s);
				} else {
					error_unexpected_token("string literal", &next_tok);
				}

				// Can type asm statements
				let next_tok = tokens.clone().next().unwrap();
				if let TokenWithDebugInfo {
					internal_tok: Token::Colon,
					..
				} = next_tok
				{
					tokens.next();
					let return_type = parse_type(tokens);
					statement = Statement::Asm(asm, return_type);
				} else {
					statement = Statement::Asm(asm, Type::Void);
				}

			} else {
                error_unexpected_token("valid keyword token", &tok);
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
                    statements.push(Typed::new(parse_statement(tokens)));
                }
            }

            statement = Statement::Compound(statements);
        }
        _ => {
            let exp = parse_expression(tokens);
            statement = Statement::Expression(Typed::new(exp));
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
        _ => error_unexpected_token("identifier", &next_tok),
    };

    let next_tok = tokens.next().unwrap();
    if &Token::Colon != next_tok {
        error_unexpected_token("colon", next_tok);
    };

    let var_type = parse_type(tokens);

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
            &next_tok,
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
                error_unexpected_token("fn keyword", &tok)
            }
        }
        TokenWithDebugInfo { .. } => error_unexpected_token("fn keyword", &tok),
    };

    let tok = tokens.next().unwrap();
    let id = match tok {
        TokenWithDebugInfo {
            internal_tok: Token::Identifier(id),
            ..
        } => id.clone(),
        TokenWithDebugInfo { .. } => error_unexpected_token("identifier", &tok),
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
                error_unexpected_token("colon", tok);
            }
            params.push((id.clone(), parse_type(&mut tokens)));
        } else {
            error_unexpected_token("identifier or closing parenthesis", &tok);
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
        statement = Statement::Compound(vec![Typed::new(statement)]);
    }

    return Function::Function(id, params, Typed::new(statement), return_type);
}

#[derive(Debug, Clone)]
pub struct Struct {
    pub id: String,
    pub members: Vec<(String, Type)>,
}

fn parse_struct(mut tokens: &mut Iter<TokenWithDebugInfo>) -> Struct {
    let tok = tokens.next().unwrap();

    if let TokenWithDebugInfo {
        internal_tok: Token::Keyword(ref k),
        ..
    } = tok
    {
        if k != "struct" {
            error_unexpected_token("struct keyword", &tok)
        }
    } else {
        error_unexpected_token("struct keyword", &tok)
    }

    let tok = tokens.next().unwrap();
    let id = match tok {
        TokenWithDebugInfo {
            internal_tok: Token::Identifier(id),
            ..
        } => id.clone(),
        TokenWithDebugInfo { .. } => error_unexpected_token("identifier", &tok),
    };

    let tok = tokens.next().unwrap();
    if !matches!(
        tok,
        TokenWithDebugInfo {
            internal_tok: Token::LBrace,
            ..
        }
    ) {
        error_unexpected_token("opening brace", tok);
    }

    let mut members = Vec::new();

    loop {
        let tok = tokens.next().unwrap();

        if let TokenWithDebugInfo {
            internal_tok: Token::RBrace,
            ..
        } = tok
        {
            break;
        } else if let TokenWithDebugInfo {
            internal_tok: Token::Semicolon,
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
                error_unexpected_token("colon", tok);
            }
            members.push((id.clone(), parse_type(&mut tokens)));
        } else {
            error_unexpected_token("identifier or closing brace", &tok);
        }
    }

    return Struct { id, members };
}

#[derive(Debug, Clone)]
pub struct Enum {
	pub id: String,
	pub variants: Vec<String>,
}

fn parse_enum(tokens: &mut Iter<TokenWithDebugInfo>) -> Enum {
	let tok = tokens.next().unwrap();

	if let TokenWithDebugInfo {
		internal_tok: Token::Keyword(ref k),
		..
	} = tok {
		if k != "enum" {
			error_unexpected_token("enum keyword", &tok)
		}
	} else {
		error_unexpected_token("enum keyword", &tok)
	}

	let tok = tokens.next().unwrap();
	let id = match tok {
		TokenWithDebugInfo {
			internal_tok: Token::Identifier(id),
			..
		} => id.clone(),
		TokenWithDebugInfo { .. } => error_unexpected_token("identifier", &tok),
	};

	let tok = tokens.next().unwrap();
	if !matches!(
		tok,
		TokenWithDebugInfo {
			internal_tok: Token::LBrace,
			..
		}
	) {
		error_unexpected_token("opening brace", tok);
	}

	let mut variants = Vec::new();

	loop {
		let tok = tokens.next().unwrap();

		if let TokenWithDebugInfo {
			internal_tok: Token::Identifier(id),
			..
		} = tok {
			variants.push(id.clone());
			let next_tok = tokens.next().unwrap();

			if let TokenWithDebugInfo {
				internal_tok: Token::Comma,
				..
			} = next_tok {
				continue;
			} else if let TokenWithDebugInfo {
				internal_tok: Token::RBrace,
				..
			} = next_tok {
				break;
			} else {
				error_unexpected_token("comma or closing brace", &next_tok);
			}

		} else {
			error_unexpected_token("identifier", &tok);
		}
	}

	return Enum { id, variants };
}


/// Parses an abstract syntax tree (AST) from a list of tokens.
pub fn parse_namespace(tokens: &mut Iter<TokenWithDebugInfo>, is_toplevel: bool) -> Namespace {

	let id;
	if !is_toplevel {
		let tok = tokens.next().unwrap();
		if let TokenWithDebugInfo {
			internal_tok: Token::Keyword(ref k),
			..
		} = tok {
			if k != "namespace" {
				error_unexpected_token("namespace keyword", &tok)
			}
		} else {
			error_unexpected_token("namespace keyword", &tok)
		}
		let tok = tokens.next().unwrap();
		id = match tok {
			TokenWithDebugInfo {
				internal_tok: Token::Identifier(id),
				..
			} => id.clone(),
			TokenWithDebugInfo { .. } => error_unexpected_token("identifier", &tok),
		};
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
	} else {
		id = "toplevel".to_string();
	}

    let mut functions = Vec::new();
    let mut constants = Vec::new();
    let mut structs = Vec::new();
	let mut enums = Vec::new();
	let mut sub_namespaces = Vec::new();

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
            } => {
				if k == "fn" {functions.push(Typed::new(parse_function(tokens)))}
				else if k == "const" {constants.push(Typed::new(parse_const(tokens)))}
				else if k == "struct" {structs.push(parse_struct(tokens))}
				else if k == "enum" {enums.push(parse_enum(tokens))}
				else if k == "union" {todo!("union")}
				else if k == "namespace" {sub_namespaces.push(parse_namespace(tokens, false));}
				else if k == "spacename" {
					if is_toplevel {
						error("cannot use 'spacename' outside of a namespace", &next_tok);
					} else {
						tokens.next();
						let tok = tokens.next().unwrap();
						if !matches!(
							tok,
							TokenWithDebugInfo {
								internal_tok: Token::Semicolon,
								..
							}
						) {
							error_unexpected_token("semicolon", &tok);
						}
						break
					}
				}
				else {error_unexpected_token("function, constant, struct, enum or union declaration", &next_tok)}
			}
			_ => error_unexpected_token("function, constant, struct, enum or union declaration", &next_tok),
        }
    }

	Namespace { id, functions, constants, structs, enums, sub_namespaces }
}

pub fn parse(tokens: &Vec<TokenWithDebugInfo>) -> Ast {
	let mut tokens = tokens.iter();
	let program = parse_namespace(&mut tokens, true);

	Ast { program }
}
