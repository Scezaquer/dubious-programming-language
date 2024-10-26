use crate::lexer::lex::Operator;
use crate::lexer::lex::Token;
use core::panic;
use std::slice::Iter;

// OPERATOR PRECEDENCE TABLE:
// 1. Member access (.)
// 2. Pre increment (++a) Pre decrement (--a) Unary plus (+a) Unary minus (-a) Logical not (!a) Bitwise not (~a) Dereference (*a) Address of (&a)
// 3. Exponentiation (a ** b)
// 4. Multiplication (a * b) Division (a / b) Modulus (a % b)
// 5. Addition (a + b) Subtraction (a - b)
// 6. Bitwise left shift (a << b) Bitwise right shift (a >> b)
// 7. Less than (a < b) Greater than (a > b) Less than or equal to (a <= b) Greater than or equal to (a >= b)
// 8. Equal to (a == b) Not equal to (a != b)
// 9. Bitwise and (a & b)
// 10. Bitwise xor (a ^ b)
// 11. Bitwise or (a | b)
// 12. Logical and (a && b)
// 13. Logical xor (a ^^ b)
// 14. Logical or (a || b)
// 15. Assignment (a = b) Add assignment (a += b) Subtract assignment (a -= b) Multiply assignment (a *= b) Divide assignment (a /= b) Modulus assignment (a %= b) Left shift assignment (a <<= b) Right shift assignment (a >>= b) Bitwise and assignment (a &= b) Bitwise xor assignment (a ^= b) Bitwise or assignment (a |= b)

#[derive(Debug)]
pub enum Constant {
    Int(i64),
    Float(f64),
    Bool(bool),
}

#[derive(Debug)]
pub enum Atom {
    Constant(Constant),
    Expression(Box<Expression>),
    Variable(String),
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum BinOp {
    MemberAccess,
    Exponent,
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

#[derive(Debug, PartialEq)]
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

#[derive(Debug)]
pub struct PrecedenceLevel {
    binary_ops: Vec<BinOp>,
    unary_ops: Vec<UnOp>,
    assignment_ops: Vec<AssignmentOp>,
}

#[derive(Debug)]
pub enum Expression {
    Atom(Atom),
    UnaryOp(Box<Expression>, UnOp),
    BinaryOp(Box<Expression>, Box<Expression>, BinOp),
    Assignment(String, Box<Expression>, AssignmentOp),
}

fn get_bin_operator_from_token(token: &Token) -> BinOp {
    match token {
        Token::Operator(op) => match op {
            Operator::MemberAccess => BinOp::MemberAccess,
            Operator::Exponent => BinOp::Exponent,
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

fn get_assign_operator_from_token(token: &Token) -> AssignmentOp {
    match token {
        Token::Operator(op) => match op {
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

fn get_un_operator_from_token(token: &Token) -> UnOp {
    match token {
        Token::Operator(op) => match op {
            Operator::Increment => UnOp::PreIncrement,
            Operator::Decrement => UnOp::PreDecrement,
            Operator::Add => UnOp::UnaryPlus,
            Operator::Subtract => UnOp::UnaryMinus,
            Operator::LogicalNot => UnOp::LogicalNot,
            Operator::BitwiseNot => UnOp::BitwiseNot,
            Operator::Multiply => UnOp::Dereference,
            Operator::BitwiseAnd => UnOp::AddressOf,
            _ => panic!("Invalid unary operator token: {:?}", token),
        },
        _ => UnOp::NotAUnaryOp,
    }
}

#[derive(Debug)]
pub enum Statement {
    Assignment(String, Expression),
    Let(String, Option<Expression>),
    If(Expression, Box<Statement>, Option<Box<Statement>>),
    While(Expression, Box<Statement>),
    Return(Expression),
    Expression(Expression),
    Compound(Vec<Statement>),
}

#[derive(Debug)]
pub enum Function {
    Function(String, Vec<String>, Statement),
}

#[derive(Debug)]
pub enum Program {
    Program(Vec<Function>),
}

#[derive(Debug)]
pub struct Ast {
    pub program: Program,
}

impl std::fmt::Display for Constant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Constant::Int(i) => write!(f, "{}", i),
            Constant::Float(fl) => write!(f, "{}", fl),
            Constant::Bool(b) => write!(f, "{}", b),
        }
    }
}

fn parse_constant(token: &Token) -> Constant {
    match token {
        Token::IntLiteral(i) => Constant::Int(*i),
        Token::FloatLiteral(f) => Constant::Float(*f),
        Token::Keyword(k) => {
            if k == "true" {
                Constant::Bool(true)
            } else if k == "false" {
                Constant::Bool(false)
            } else {
                // May be unnecessary if lexer is correct
                panic!("Invalid constant keyword token: {:?}", token)
            }
        }
        _ => panic!("Invalid constant token: {:?}", token),
    }
}

fn parse_atom(mut tokens: &mut Iter<Token>) -> Atom {
    let tok = tokens.next().unwrap();

    match tok {
        Token::LParen => {
            let inner_exp = parse_expression(&mut tokens);

            if let Token::RParen = tokens.next().unwrap() {
                return Atom::Expression(Box::new(inner_exp));
            } else {
                panic!("Expected closing parenthesis, found: {:?}", tok);
            }
        }
        Token::IntLiteral(_) | Token::FloatLiteral(_) | Token::Keyword(_) => {
            return Atom::Constant(parse_constant(tok));
        }
        Token::Identifier(s) => {
            return Atom::Variable(s.to_string());
        }
        _ => panic!("Invalid atom token: {:?}", tok),
    }
}

fn parse_expression_with_precedence(
    mut tokens: &mut Iter<Token>,
    precedence_level: usize,
    precedence_table: &Vec<PrecedenceLevel>,
) -> Expression {
    if precedence_level == 0 {
        // Parse the lowest precedence, like literals or atoms
        return Expression::Atom(parse_atom(&mut tokens));
    }

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

            if let Expression::Atom(Atom::Variable(var)) = expr {
                expr = Expression::Assignment(var, Box::new(next_term), op);
            } else {
                panic!(
                    "Invalid assignment target (can only assign to variables): {:?}",
                    expr
                );
            }
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

    expr
}

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
            binary_ops: vec![BinOp::Exponent],
            unary_ops: vec![],
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

fn parse_expression(mut tokens: &mut Iter<Token>) -> Expression {
    let precedence_table = build_precedence_table();
    let max_precedence = precedence_table.len() - 1;

    parse_expression_with_precedence(&mut tokens, max_precedence, &precedence_table)
}

fn parse_statement(tokens: &mut Iter<Token>) -> Statement {
    let tok = tokens.next().unwrap();
    let statement;

    match tok {
        Token::Keyword(k) => {
            if k == "return" {
                // return exp;
                let exp = parse_expression(tokens);
                statement = Statement::Return(exp);
            } else if k == "let" {
                // let id: type = exp;

                let next_tok = tokens.next().unwrap();
                let id = match next_tok {
                    Token::Identifier(id) => id.clone(),
                    _ => panic!("Expected identifier, found: {:?}", next_tok),
                };

                let next_tok = tokens.next().unwrap();
                if &Token::Colon != next_tok {
                    panic!("Expected colon, found: {:?}", next_tok);
                };

                let _ = tokens.next().unwrap(); // Skip type for now. // TODO: Implement types

                let next_tok = tokens.clone().next().unwrap();
                if let Token::Operator(Operator::Assign) = next_tok {
					tokens.next();
                    let exp = parse_expression(tokens);
                    statement = Statement::Let(id.to_string(), Some(exp));
                } else if let Token::Semicolon = next_tok {
                    statement = Statement::Let(id.to_string(), None);
                } else {
                    panic!(
                        "Expected semicolon or assignment operator, found: {:?}",
                        next_tok
                    );
                }
            } else if k == "if" {
				// if (exp) statement [else statement]
				let exp = parse_expression(tokens);
				let mut if_stmt = parse_statement(tokens);

				if let Statement::Compound(_) = if_stmt {
				} else {
					if_stmt = Statement::Compound(vec![if_stmt]);
				}

				let next_tok = tokens.clone().next().unwrap();
				if let Token::Keyword(k) = next_tok {
					if k == "else" {
						tokens.next();
						let mut else_stmt = parse_statement(tokens);

						if let Statement::Compound(_) = else_stmt {
						} else {
							else_stmt = Statement::Compound(vec![else_stmt]);
						}

						statement = Statement::If(exp, Box::new(if_stmt), Some(Box::new(else_stmt)));
					} else {
						statement = Statement::If(exp, Box::new(if_stmt), None);
					}
				} else {
					statement = Statement::If(exp, Box::new(if_stmt), None);
				}
			} else {
                panic!("Invalid keyword token: {:?}", tok);
            }
        }
        Token::Identifier(_) => {
            let id = match tok {
                Token::Identifier(id) => id,
                _ => panic!("Invalid identifier token: {:?}", tok),
            };

            if let Token::Operator(Operator::Assign) = tokens.next().unwrap() {
                let exp = parse_expression(tokens);
                statement = Statement::Assignment(id.to_string(), exp);
            } else {
                panic!("Expected assignment operator, found: {:?}", tok);
            }
        }
        &Token::LBrace => {
            let mut statements = Vec::new();

            loop {
                let next = tokens.clone().next().unwrap();

                if let &Token::RBrace = next {
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

    // Compound and If statements don't need a semicolon
    if let Statement::Compound(_) = statement {
        return statement;
    } else if let Statement::If(_, _, _) = statement {
		return statement;
	}

    let tok = tokens.next().unwrap();
    if tok != &Token::Semicolon {
        panic!("Expected semicolon, found: {:?}", tok);
    }

    return statement;
}

fn parse_function(mut tokens: &mut Iter<Token>) -> Function {
    let tok = tokens.next().unwrap();

    let k = match tok {
        Token::Keyword(k) => k,
        _ => panic!("Expected keyword, found: {:?}", tok),
    };

    if k != "fn" {
        panic!("Expected function keyword, found: {:?}", tok);
    }

    let tok = tokens.next().unwrap();
    let id = match tok {
        Token::Identifier(id) => id.clone(),
        _ => panic!("Expected identifier, found: {:?}", tok),
    };

    let tok = tokens.next().unwrap();
    if &Token::LParen != tok {
        panic!("Expected opening parenthesis, found: {:?}", tok);
    }

    let mut params = Vec::new();

    loop {
        let tok = tokens.next().unwrap();

        if let Token::RParen = tok {
            break;
        } else if let Token::Identifier(id) = tok {
            params.push(id.clone());
        } else {
            panic!(
                "Expected identifier or closing parenthesis, found: {:?}",
                tok
            );
        }
    }

    let mut statement = parse_statement(&mut tokens);

	// If the statement is not a compound statement, wrap it in one
	// This is to allow okay-ish scope handling
	if let Statement::Compound(_) = statement {
	} else {
		statement = Statement::Compound(vec![statement]);
	}

    return Function::Function(id, params, statement);
}

pub fn parse(tokens: &Vec<Token>) -> Ast {
    let ast = Ast {
        program: Program::Program(vec![parse_function(&mut tokens.iter())]),
    };

    return ast;
}
