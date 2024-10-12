use crate::lexer::lex::Operator;
use crate::lexer::lex::Token;
use std::slice::Iter;

#[derive(Debug)]
pub enum Constant {
    Int(i64),
    Float(f64),
    Bool(bool),
}

#[derive(Debug)]
pub enum Factor {
    Constant(Constant),
    Expression(Box<Expression>),
    UnaryOp(Box<Factor>, UnaryOp),
    Variable(String),
}

#[derive(Debug)]
pub enum Term {
    Factor(Factor),
    BinaryOp(Box<Term>, Factor, TermBinaryOp),
}

#[derive(Debug)]
pub enum Expression {
    Term(Term),
    BinaryOp(Box<Expression>, Term, ExpressionBinaryOp),
}

#[derive(Debug)]
pub enum ExpressionBinaryOp {
    Add,
    Sub,
}

#[derive(Debug)]
pub enum TermBinaryOp {
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

#[derive(Debug)]
pub enum UnaryOp {
    Neg,
    Not,
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

fn get_expression_binary_op(token: &Token) -> ExpressionBinaryOp {
    match token {
        Token::Operator(op) => match op {
            Operator::Add => ExpressionBinaryOp::Add,
            Operator::Subtract => ExpressionBinaryOp::Sub,
            _ => panic!("Invalid binary operator token: {:?}", token),
        },
        _ => panic!("Invalid binary operator token: {:?}", token),
    }
}

fn get_term_binary_op(token: &Token) -> TermBinaryOp {
    match token {
        Token::Operator(op) => match op {
            Operator::Multiply => TermBinaryOp::Mul,
            Operator::Divide => TermBinaryOp::Div,
            Operator::Modulus => TermBinaryOp::Mod,
            Operator::Exponent => TermBinaryOp::Pow,
            Operator::BitwiseAnd => TermBinaryOp::And,
            Operator::BitwiseOr => TermBinaryOp::Or,
            Operator::LessThan => TermBinaryOp::Less,
            Operator::GreaterThan => TermBinaryOp::Greater,
            Operator::Assign => TermBinaryOp::Equal,
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
                return Factor::Expression(Box::new(inner_exp));
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
    let mut next= tokens.clone().next().unwrap();

    while next == &Token::Operator(Operator::Multiply)
        || next == &Token::Operator(Operator::Divide)
        || next == &Token::Operator(Operator::Modulus)
        || next == &Token::Operator(Operator::Exponent)
        || next == &Token::Operator(Operator::BitwiseAnd)
        || next == &Token::Operator(Operator::BitwiseOr)
    {
        let tok = tokens.next().unwrap();
        let op = get_term_binary_op(tok);
        let next_term = parse_factor(&mut tokens);
        term = Term::BinaryOp(Box::new(term), next_term, op);
        next = tokens.clone().next().unwrap();
    }
    return term;
}

fn parse_expression(mut tokens: &mut Iter<Token>) -> Expression {
    let mut expression = Expression::Term(parse_term(&mut tokens));

    let mut next= tokens.clone().next().unwrap();

    while next == &Token::Operator(Operator::Add) || next == &Token::Operator(Operator::Subtract) {
        let tok = tokens.next().unwrap();
        let op = get_expression_binary_op(tok);
        let next_term = parse_term(&mut tokens);
        expression = Expression::BinaryOp(Box::new(expression), next_term, op);
        next = tokens.clone().next().unwrap();
    }

    return expression;
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
            panic!("Expected identifier or closing parenthesis, found: {:?}", tok);
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
