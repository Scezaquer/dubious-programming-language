use crate::lexer::Token;

enum Constant {
    Int(i64),
    Float(f64),
    Bool(bool),
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
    NotEqual,
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
            } else { // May be unnecessary if lexer is correct
                panic!("Invalid constant keyword token: {:?}", token)
            }
        }
        _ => panic!("Invalid constant token: {:?}", token),
    }
}

fn parse_expression(tokens: &Vec<Token>) -> Expression {
    // TODO
    return Expression::Constant(Constant::Int(0));
}

fn parse_statement(tokens: &Vec<Token>) -> Statement {
    // TODO
    return Statement::Expression(Expression::Constant(Constant::Int(0)));
}

fn parse_function(tokens: &Vec<Token>) -> Function {
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

