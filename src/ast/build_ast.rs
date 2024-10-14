use crate::lexer::lex::Operator;
use crate::lexer::lex::Token;
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
pub enum UnOp { // TODO: We only support prefix unary operators for now
    PreIncrement,
    PreDecrement,
    UnaryPlus,
    UnaryMinus,
    LogicalNot,
    BitwiseNot,
    Dereference,
    AddressOf,
    NotAUnaryOp,    // Not pretty but it makes the code nicer
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
    NotABinaryOp,   // Not pretty but it makes the code nicer
}

#[derive(Debug)]
pub struct PrecedenceLevel {
    binary_ops: Vec<BinOp>,
    unary_ops: Vec<UnOp>,
}

#[derive(Debug)]
pub enum Expression {
    Atom(Atom),
    UnaryOp(Box<Expression>, UnOp),
    BinaryOp(Box<Expression>, Box<Expression>, BinOp),
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
            Operator::Assign => BinOp::Assign,
            Operator::AddAssign => BinOp::AddAssign,
            Operator::SubtractAssign => BinOp::SubtractAssign,
            Operator::MultiplyAssign => BinOp::MultiplyAssign,
            Operator::DivideAssign => BinOp::DivideAssign,
            Operator::ModulusAssign => BinOp::ModulusAssign,
            Operator::LeftShiftAssign => BinOp::LeftShiftAssign,
            Operator::RightShiftAssign => BinOp::RightShiftAssign,
            Operator::BitwiseAndAssign => BinOp::BitwiseAndAssign,
            Operator::BitwiseXorAssign => BinOp::BitwiseXorAssign,
            Operator::BitwiseOrAssign => BinOp::BitwiseOrAssign,
            _ => panic!("Invalid binary operator token: {:?}", token),
        },
        _ => BinOp::NotABinaryOp,
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
    If(Expression, Vec<Statement>),
    While(Expression, Vec<Statement>),
    Return(Expression),
    Expression(Expression),
}

#[derive(Debug)]
pub enum Function {
    Function(String, Vec<String>, Vec<Statement>),
}

#[derive(Debug)]
pub enum Program {
    Program(Vec<Function>),
}

#[derive(Debug)]
pub struct Ast {
    pub program: Program,
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
        Token::Identifier(_) => {
            // TODO
            panic!("Not implemented");
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

    // Now handle binary operators for the current precedence level
    next = tokens.clone().next().unwrap();
    while precedence_table[precedence_level]
        .binary_ops
        .contains(&get_bin_operator_from_token(&next))
    {
        let tok = tokens.next().unwrap();
        let op = get_bin_operator_from_token(tok); // Get the binary operator
        let next_term =
            parse_expression_with_precedence(&mut tokens, precedence_level - 1, precedence_table); // Parse next term
        expr = Expression::BinaryOp(Box::new(expr), Box::new(next_term), op);
        next = tokens.clone().next().unwrap();
    }

    expr
}

fn build_precedence_table() -> Vec<PrecedenceLevel> {
    vec![
        PrecedenceLevel {
            binary_ops: vec![BinOp::MemberAccess],
            unary_ops: vec![],
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
        },
        PrecedenceLevel {
            binary_ops: vec![BinOp::Exponent],
            unary_ops: vec![],
        },
        PrecedenceLevel {
            binary_ops: vec![BinOp::Multiply, BinOp::Divide, BinOp::Modulus],
            unary_ops: vec![],
        },
        PrecedenceLevel {
            binary_ops: vec![BinOp::Add, BinOp::Subtract],
            unary_ops: vec![],
        },
        PrecedenceLevel {
            binary_ops: vec![BinOp::LeftShift, BinOp::RightShift],
            unary_ops: vec![],
        },
        PrecedenceLevel {
            binary_ops: vec![
                BinOp::LessThan,
                BinOp::GreaterThan,
                BinOp::LessOrEqualThan,
                BinOp::GreaterOrEqualThan,
            ],
            unary_ops: vec![],
        },
        PrecedenceLevel {
            binary_ops: vec![BinOp::Equal, BinOp::NotEqual],
            unary_ops: vec![],
        },
        PrecedenceLevel {
            binary_ops: vec![BinOp::BitwiseAnd],
            unary_ops: vec![],
        },
        PrecedenceLevel {
            binary_ops: vec![BinOp::BitwiseXor],
            unary_ops: vec![],
        },
        PrecedenceLevel {
            binary_ops: vec![BinOp::BitwiseOr],
            unary_ops: vec![],
        },
        PrecedenceLevel {
            binary_ops: vec![BinOp::LogicalAnd],
            unary_ops: vec![],
        },
        PrecedenceLevel {
            binary_ops: vec![BinOp::LogicalXor],
            unary_ops: vec![],
        },
        PrecedenceLevel {
            binary_ops: vec![BinOp::LogicalOr],
            unary_ops: vec![],
        },
        PrecedenceLevel {
            binary_ops: vec![
                BinOp::Assign,
                BinOp::AddAssign,
                BinOp::SubtractAssign,
                BinOp::MultiplyAssign,
                BinOp::DivideAssign,
                BinOp::ModulusAssign,
                BinOp::LeftShiftAssign,
                BinOp::RightShiftAssign,
                BinOp::BitwiseAndAssign,
                BinOp::BitwiseXorAssign,
                BinOp::BitwiseOrAssign,
            ],
            unary_ops: vec![],
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
                let exp = parse_expression(tokens);
                statement = Statement::Return(exp);
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
        _ => {
            let exp = parse_expression(tokens);
            statement = Statement::Expression(exp);
        }
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

    let id = match tokens.next().unwrap() {
        Token::Identifier(id) => id.clone(),
        _ => panic!("Expected identifier, found: {:?}", tok),
    };

    if &Token::LParen != tokens.next().unwrap() {
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

    if &Token::LBrace != tokens.next().unwrap() {
        panic!("Expected opening brace, found: {:?}", tok);
    }

    let mut statements = Vec::new();

    loop {
        let next = tokens.clone().next().unwrap();

        if let &Token::RBrace = next {
            tokens.next();
            break;
        } else {
            statements.push(parse_statement(&mut tokens));
        }
    }

    return Function::Function(id, params, statements);
}

pub fn parse(tokens: &Vec<Token>) -> Ast {
    let ast = Ast {
        program: Program::Program(vec![parse_function(&mut tokens.iter())]),
    };

    return ast;
}
