use crate::lexer::lex::Operator;
use crate::lexer::lex::Token;
use std::slice::Iter;

enum Constant {
    Int(i64),
    Float(f64),
    Bool(bool),
}

enum Factor {
    Constant(Constant),
    Expression(Expression),
    UnaryOp(Box<Factor>, UnaryOp),
    Variable(String),
}

enum Term {
    Factor(Factor),
    BinaryOp(Box<Term>, Box<Term>, BinaryOp),
}

enum Expression {
    Constant(Constant),
    BinaryOp(Box<Expression>, Box<Expression>, BinaryOp),
    UnaryOp(Box<Expression>, UnaryOp),
    Variable(String),
}

enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    And,
    Or,
    Less,
    Greater,
    Equal,
}

enum UnaryOp {
    Neg,
    Not,
}

enum Statement {
    Assignment(String, Expression),
    If(Expression, Vec<Statement>),
    While(Expression, Vec<Statement>),
    Return(Expression),
    Expression(Expression),
}

enum Function {
    Function(String, Vec<String>, Vec<Statement>),
}

enum Program {
    Program(Vec<Function>),
}

struct Ast {
    program: Program,
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

fn get_unary_op(token: &Token) -> UnaryOp {
    match token {
        Token::Operator(op) => match op {
            Operator::Subtract => UnaryOp::Neg,
            Operator::Not => UnaryOp::Not,
            _ => panic!("Invalid unary operator token: {:?}", token),
        },
        _ => panic!("Invalid unary operator token: {:?}", token),
    }
}

fn get_binary_op(token: &Token) -> BinaryOp {
    match token {
        Token::Operator(op) => match op {
            Operator::Add => BinaryOp::Add,
            Operator::Subtract => BinaryOp::Sub,
            Operator::Multiply => BinaryOp::Mul,
            Operator::Divide => BinaryOp::Div,
            Operator::Modulus => BinaryOp::Mod,
            Operator::Exponent => BinaryOp::Pow,
            Operator::BitwiseAnd => BinaryOp::And,
            Operator::BitwiseOr => BinaryOp::Or,
            Operator::LessThan => BinaryOp::Less,
            Operator::GreaterThan => BinaryOp::Greater,
            Operator::Equal => BinaryOp::Equal,
            _ => panic!("Invalid binary operator token: {:?}", token),
        },
        _ => panic!("Invalid binary operator token: {:?}", token),
    }
}

fn parse_factor(mut tokens: &mut Iter<Token>) -> Factor {
    let tok = tokens.next().unwrap();

    match tok {
        Token::LParen => {
            let inner_exp = parse_expression(&mut tokens);

            if let Token::RParen = tokens.next().unwrap() {
                return Factor::Expression(inner_exp);
            } else {
                panic!("Expected closing parenthesis, found: {:?}", tok);
            }
        }
        Token::Operator(_op) => {
            let op = get_unary_op(tok);
            let factor = parse_factor(&mut tokens);
            return Factor::UnaryOp(Box::new(factor), op);
        }
        Token::IntLiteral(_) | Token::FloatLiteral(_) | Token::Keyword(_) => {
            return Factor::Constant(parse_constant(tok));
        }
        Token::Identifier(_) => {
            // TODO
            panic!("Not implemented");
        }
        _ => panic!("Invalid factor token: {:?}", tok),
    }
}

fn parse_term(mut tokens: &mut Iter<Token>) -> Term {
    let mut term = Term::Factor(parse_factor(&mut tokens));
    let &&(mut next) = tokens.peekable().peek().unwrap();

    while next == Token::Operator(Operator::Multiply)
        || next == Token::Operator(Operator::Divide)
        || next == Token::Operator(Operator::Modulus)
        || next == Token::Operator(Operator::Exponent)
        || next == Token::Operator(Operator::BitwiseAnd)
        || next == Token::Operator(Operator::BitwiseOr)
    {
        let tok = tokens.next().unwrap();
        let op = get_binary_op(tok);
        let next_term = Term::Factor(parse_factor(&mut tokens));
        term = Term::BinaryOp(Box::new(term), Box::new(next_term), op);
        next = **tokens.peekable().peek().unwrap()
    }
    return term;
}

fn parse_expression(tokens: &mut Iter<Token>) -> Expression {
    let term = parse_term(&mut tokens);

    let &&(mut next) = tokens.peekable().peek().unwrap();

    while next == Token::Operator(Operator::Add) || next == Token::Operator(Operator::Subtract) {
        let tok = tokens.next().unwrap();
        let op = get_binary_op(tok);
        let next_term = parse_term(&mut tokens);
        term = Expression::BinaryOp(Box::new(term), Box::new(next_term), op);
        next = **tokens.peekable().peek().unwrap()
    }

    return term;
}

fn parse_statement(tokens: &mut Iter<Token>) -> Statement {
    // TODO
    return Statement::Expression(Expression::Constant(Constant::Int(0)));
}

fn parse_function(tokens: &mut Iter<Token>) -> Function {
    // TODO
    return Function::Function("".to_string(), Vec::new(), Vec::new());
}

pub fn parse(tokens: &Vec<Token>) -> Ast {
    let mut ast = Ast {
        program: Program::Program(Vec::new()),
    };

    // TODO

    return ast;
}
