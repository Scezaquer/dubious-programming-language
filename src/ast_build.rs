use crate::lexer::Operator;
use crate::lexer::Token;
use crate::shared::TokenWithDebugInfo;
use crate::shared::{
	Type, Typed, error, error_unexpected_token,
};
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
    Literal(Typed<TokenWithDebugInfo<Literal>>),
    Expression(Box<Typed<TokenWithDebugInfo<Expression>>>),
    Variable(TokenWithDebugInfo<String>),
    FunctionCall(
        TokenWithDebugInfo<String>,
        Vec<Typed<TokenWithDebugInfo<Expression>>>,
        Vec<TokenWithDebugInfo<Type>>,
    ), // id<T1, T2, ...>(args)
    Array(Vec<Typed<TokenWithDebugInfo<Expression>>>, i64), // Array literal with a given number of dimensions
    StructInstance(
        TokenWithDebugInfo<String>,
        Vec<Typed<TokenWithDebugInfo<Expression>>>,
        Vec<TokenWithDebugInfo<Type>>,
    ), // Struct instance with a given number of fields
}

// In let bindings, the left hand side of the assignment
#[derive(Debug, Clone)]
pub enum AssignmentIdentifier {
    Variable(TokenWithDebugInfo<String>),
    Dereference(Box<Typed<TokenWithDebugInfo<AssignmentIdentifier>>>),
    Array(
        TokenWithDebugInfo<String>,
        Vec<Typed<TokenWithDebugInfo<Expression>>>,
    ), // identifier[dim1, dim2, ...]
}

// In assignments, the left hand side of the assignment. This is NOT the same
// as AssignmentIdentifier, as it can be more complex.
// struct.member = ... is a ReassignmentIdentifier, but not an AssignmentIdentifier
#[derive(Debug, Clone)]
pub enum ReassignmentIdentifier {
    Variable(TokenWithDebugInfo<String>),
    Dereference(Box<Typed<TokenWithDebugInfo<Expression>>>),
    Array(
        Box<Typed<TokenWithDebugInfo<Expression>>>,
        Vec<Typed<TokenWithDebugInfo<Expression>>>,
    ),
    MemberAccess(
        Box<Typed<TokenWithDebugInfo<Expression>>>,
        Box<Typed<TokenWithDebugInfo<Expression>>>,
    ),
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
    Atom(Typed<TokenWithDebugInfo<Atom>>),
    UnaryOp(Box<Typed<TokenWithDebugInfo<Expression>>>, UnOp),
    BinaryOp(
        Box<Typed<TokenWithDebugInfo<Expression>>>,
        Box<Typed<TokenWithDebugInfo<Expression>>>,
        BinOp,
    ),
    Assignment(
        Typed<TokenWithDebugInfo<ReassignmentIdentifier>>,
        Box<Typed<TokenWithDebugInfo<Expression>>>,
        AssignmentOp,
    ),
    TypeCast(Box<Typed<TokenWithDebugInfo<Expression>>>, TokenWithDebugInfo<Type>),
    ArrayAccess(
        Box<Typed<TokenWithDebugInfo<Expression>>>,
        Vec<Typed<TokenWithDebugInfo<Expression>>>,
    ),
}

/// Gets the binary operator corresponding to the token.
fn get_bin_operator_from_tokens(
    token: &Iter<TokenWithDebugInfo<Token>>,
) -> TokenWithDebugInfo<BinOp> {
    let mut cloned_tokens = token.clone();
    let token = cloned_tokens.next().unwrap();

    let (line, file) = (token.line.clone(), token.file.clone());

    let op = match token {
        TokenWithDebugInfo {
            internal_tok: Token::Operator(op),
            ..
        } => match op {
            Operator::Multiply => BinOp::Multiply,
            Operator::Divide => BinOp::Divide,
            Operator::Modulus => BinOp::Modulus,
            Operator::Add => BinOp::Add,
            Operator::Subtract => BinOp::Subtract,
                // Similar to  Operator::GreaterThan
                Operator::LeftShift => {
                let next = cloned_tokens.next().unwrap();
                if let TokenWithDebugInfo {
                    internal_tok: Token::Operator(Operator::Assign),
                    ..
                } = next
                {
                    BinOp::NotABinaryOp // <<=
                } else {
                    BinOp::LeftShift // <<
                }
            }
            Operator::LessThan => BinOp::LessThan,
            Operator::LessOrEqualThan => BinOp::LessOrEqualThan,
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
            Operator::GreaterThan => {
                // We need special treatment for the greater than operator.
                // Since there is ambiguity at the tokenization level between
                // generics syntax and oprators that start with '>', we don't use
                // the tokenization level to determine if it's a generic or an operator.

                // e.g in 'let a: S<T>= ..;' we don't want to parse the '>=' as
                // a 'greater than' operator, but as a generic binding followed
                // by an assignment operator.

                // Similarly for '>>' in the function call 'func:<S<T>>(args);'

                let next = cloned_tokens.next().unwrap();

                if let TokenWithDebugInfo {
                    internal_tok: Token::Operator(Operator::Assign),
                    ..
                } = next
                {
                    BinOp::GreaterOrEqualThan // >=
                } else if let TokenWithDebugInfo {
                    internal_tok: Token::Operator(Operator::GreaterThan),
                    ..
                } = next
                {
                    let next = cloned_tokens.next().unwrap();
                    if let TokenWithDebugInfo {
                        internal_tok: Token::Operator(Operator::Assign),
                        ..
                    } = next
                    {
                        BinOp::NotABinaryOp // >>=
                    } else {
                        BinOp::RightShift // >>
                    }
                } else {
                    BinOp::GreaterThan // >
                }
            }
            _ => BinOp::NotABinaryOp,
        },
        _ => BinOp::NotABinaryOp,
    };

    TokenWithDebugInfo {
        internal_tok: op,
        line,
        file,
    }
}

/// Gets the assignment operator corresponding to the token.
fn get_assign_operator_from_tokens(
    token: &Iter<TokenWithDebugInfo<Token>>,
) -> TokenWithDebugInfo<AssignmentOp> {
    let mut cloned_tokens = token.clone();
    let token = cloned_tokens.next().unwrap();
    let (line, file) = (token.line.clone(), token.file.clone());

    let op = match token {
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
            Operator::LeftShift => {
                let next = cloned_tokens.next().unwrap();
                if let TokenWithDebugInfo {
                    internal_tok: Token::Operator(Operator::Assign),
                    ..
                } = next
                {
                    AssignmentOp::LeftShiftAssign // <<=
                } else {
                    AssignmentOp::NotAnAssignmentOp // <<
                }
            }
            Operator::GreaterThan => {
                // For the same reason as in get_bin_operator_from_tokens, we
                // need to make sure the '>>=' in 'let a: S<T<U>>= ..;' is not
                // parsed as a 'rightshift assignment' operator.

                let next = cloned_tokens.next().unwrap();
                if let TokenWithDebugInfo {
                    internal_tok: Token::Operator(Operator::GreaterThan),
                    ..
                } = next
                {
                    let next = cloned_tokens.next().unwrap();
                    if let TokenWithDebugInfo {
                        internal_tok: Token::Operator(Operator::Assign),
                        ..
                    } = next
                    {
                        AssignmentOp::RightShiftAssign // >>=
                    } else {
                        AssignmentOp::NotAnAssignmentOp // >>
                    }
                } else {
                    AssignmentOp::NotAnAssignmentOp // >
                }
            }
            Operator::BitwiseAndAssign => AssignmentOp::BitwiseAndAssign,
            Operator::BitwiseXorAssign => AssignmentOp::BitwiseXorAssign,
            Operator::BitwiseOrAssign => AssignmentOp::BitwiseOrAssign,
            _ => AssignmentOp::NotAnAssignmentOp,
        },
        _ => AssignmentOp::NotAnAssignmentOp,
    };

    TokenWithDebugInfo {
        internal_tok: op,
        line,
        file,
    }
}

/// Gets the unary operator corresponding to the token.
fn get_un_operator_from_token(token: &TokenWithDebugInfo<Token>) -> TokenWithDebugInfo<UnOp> {
    let (line, file) = (token.line.clone(), token.file.clone());

    let op = match token {
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
    };

    TokenWithDebugInfo {
        internal_tok: op,
        line,
        file,
    }
}

/// Represents a statement in the AST.
///
/// A statement is a single instruction in the program.
/// Statements can be assignments, let bindings, if statements, while loops, loops, do-while loops, for loops, return statements, expressions, compound statements, break statements, or continue statements.
#[derive(Debug, Clone)]
pub enum Statement {
    Let(
        TokenWithDebugInfo<AssignmentIdentifier>,
        Option<Typed<TokenWithDebugInfo<Expression>>>,
        TokenWithDebugInfo<Type>,
    ),
    If(
        Typed<TokenWithDebugInfo<Expression>>,
        Box<Typed<TokenWithDebugInfo<Statement>>>,
        Option<Box<Typed<TokenWithDebugInfo<Statement>>>>,
    ),
    While(
        Typed<TokenWithDebugInfo<Expression>>,
        Box<Typed<TokenWithDebugInfo<Statement>>>,
    ),
    Loop(Box<Typed<TokenWithDebugInfo<Statement>>>),
    Dowhile(
        Typed<TokenWithDebugInfo<Expression>>,
        Box<Typed<TokenWithDebugInfo<Statement>>>,
    ),
    For(
        Box<Typed<TokenWithDebugInfo<Statement>>>,
        Typed<TokenWithDebugInfo<Expression>>,
        Typed<TokenWithDebugInfo<Expression>>,
        Box<Typed<TokenWithDebugInfo<Statement>>>,
    ),
    Return(Typed<TokenWithDebugInfo<Expression>>),
    Expression(Typed<TokenWithDebugInfo<Expression>>),
    Compound(Vec<Typed<TokenWithDebugInfo<Statement>>>),
    Break,
    Continue,
    Asm(TokenWithDebugInfo<String>, TokenWithDebugInfo<Type>),
}

/// Represents a function in the AST.
#[derive(Debug, Clone)]
pub struct Function {
    pub id: TokenWithDebugInfo<String>,
    pub args: Vec<(TokenWithDebugInfo<String>, TokenWithDebugInfo<Type>)>,
    pub body: Typed<TokenWithDebugInfo<Statement>>,
    pub return_type: TokenWithDebugInfo<Type>,
    pub generics: Vec<TokenWithDebugInfo<String>>,
}

/// Represents a constant in the AST.
/// Constants can only be assigned on declaration, and can only be assigned a literal,
/// so they're not terribly useful as of now. They're basically static globals.
#[derive(Debug, Clone)]
pub enum Constant {
    //TODO: should be a struct instead of enum
    Constant(
        TokenWithDebugInfo<String>,
        Typed<TokenWithDebugInfo<Literal>>,
        TokenWithDebugInfo<Type>,
    ),
}

#[derive(Debug, Clone)]
pub struct Namespace {
    pub id: String,
    pub functions: Vec<Typed<TokenWithDebugInfo<Function>>>,
    pub constants: Vec<Typed<TokenWithDebugInfo<Constant>>>,
    pub structs: Vec<TokenWithDebugInfo<Struct>>,
    pub enums: Vec<TokenWithDebugInfo<Enum>>,
    pub sub_namespaces: Vec<TokenWithDebugInfo<Namespace>>,
}

/// Represents the abstract syntax tree (AST) of a program.
#[derive(Debug, Clone)]
pub struct Ast {
    pub program: TokenWithDebugInfo<Namespace>,
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Bool(b) => write!(f, "{}", if *b { "0xFFFFFFFFFFFFFFFF" } else { "0" }),
            Literal::Int(i) => write!(f, "{}", i),
            Literal::Float(fl) => write!(f, "{}", fl),
            Literal::Hex(h) => write!(f, "0x{:x}", h),
            Literal::Binary(b) => write!(f, "0b{:b}", b),
            Literal::Char(c) => write!(f, "'{}'", c),
        }
    }
}

/// Parses a constant from a token.
fn parse_literal(token: &TokenWithDebugInfo<Token>) -> Typed<TokenWithDebugInfo<Literal>> {
    match token {
        TokenWithDebugInfo {
            internal_tok: Token::IntLiteral(i),
            line,
            file,
        } => Typed::new_with_type(
            TokenWithDebugInfo {
                internal_tok: Literal::Int(*i),
                line: *line,
                file: file.clone(),
            },
            Type::Int,
        ),
        TokenWithDebugInfo {
            internal_tok: Token::FloatLiteral(f),
            line,
            file,
        } => Typed::new_with_type(
            TokenWithDebugInfo {
                internal_tok: Literal::Float(*f),
                line: *line,
                file: file.clone(),
            },
            Type::Float,
        ),
        TokenWithDebugInfo {
            internal_tok: Token::HexLiteral(h),
            line,
            file,
        } => Typed::new_with_type(
            TokenWithDebugInfo {
                internal_tok: Literal::Hex(*h),
                line: *line,
                file: file.clone(),
            },
            Type::Int,
        ),
        TokenWithDebugInfo {
            internal_tok: Token::BinLiteral(b),
            line,
            file,
        } => Typed::new_with_type(
            TokenWithDebugInfo {
                internal_tok: Literal::Binary(*b),
                line: *line,
                file: file.clone(),
            },
            Type::Int,
        ),
        TokenWithDebugInfo {
            internal_tok: Token::BoolLiteral(b),
            line,
            file,
        } => Typed::new_with_type(
            TokenWithDebugInfo {
                internal_tok: Literal::Bool(*b),
                line: *line,
                file: file.clone(),
            },
            Type::Bool,
        ),
        TokenWithDebugInfo {
            internal_tok: Token::CharLiteral(c),
            line,
            file,
        } => Typed::new_with_type(
            TokenWithDebugInfo {
                internal_tok: Literal::Char(c.to_string()),
                line: *line,
                file: file.clone(),
            },
            Type::Char,
        ),
        _ => error_unexpected_token("constant", token),
    }
}

/// Parses an atom from a list of tokens.
fn parse_atom(mut tokens: &mut Iter<TokenWithDebugInfo<Token>>) -> Typed<TokenWithDebugInfo<Atom>> {
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
            line,
            file,
        } => {
            let inner_exp = parse_expression(&mut tokens);

            if let TokenWithDebugInfo {
                internal_tok: Token::RParen,
                ..
            } = tokens.next().unwrap()
            {
                return Typed::new(TokenWithDebugInfo {
                    internal_tok: Atom::Expression(Box::new(Typed::new(inner_exp))),
                    line: line.clone(),
                    file: file.clone(),
                });
            } else {
                error_unexpected_token("closing parenthesis", &tok);
            }
        }
        TokenWithDebugInfo {
            internal_tok: Token::IntLiteral(_),
            line,
            file,
        }
        | TokenWithDebugInfo {
            internal_tok: Token::FloatLiteral(_),
            line,
            file,
        }
        | TokenWithDebugInfo {
            internal_tok: Token::BinLiteral(_),
            line,
            file,
        }
        | TokenWithDebugInfo {
            internal_tok: Token::HexLiteral(_),
            line,
            file,
        }
        | TokenWithDebugInfo {
            internal_tok: Token::BoolLiteral(_),
            line,
            file,
        }
        | TokenWithDebugInfo {
            internal_tok: Token::CharLiteral(_),
            line,
            file,
        } => {
            let lit = parse_literal(&tok);
            let t = lit.get_type().clone();
            return Typed::new_with_type(
                TokenWithDebugInfo {
                    internal_tok: Atom::Literal(lit),
                    line: line.clone(),
                    file: file.clone(),
                },
                t,
            );
        }
        TokenWithDebugInfo {
            internal_tok: Token::StringLiteral(s),
            line,
            file,
        } => {
            return Typed::new(TokenWithDebugInfo {
                internal_tok: Atom::Array(
                    s.chars()
                        .collect::<Vec<_>>()
                        .chunks(8)
                        .map(|chunk| {
                            Typed::new(TokenWithDebugInfo {
                                internal_tok: Expression::Atom(Typed::new(TokenWithDebugInfo {
                                    internal_tok: Atom::Literal(Typed::new(TokenWithDebugInfo {
                                        internal_tok: Literal::Char(chunk.iter().collect()),
                                        line: line.clone(),
                                        file: file.clone(),
                                    })),
                                    line: line.clone(),
                                    file: file.clone(),
                                })),
                                line: line.clone(),
                                file: file.clone(),
                            })
                        })
                        .collect(),
                    (s.len() as i64 + 3) / 8,
                ),
                line: line.clone(),
                file: file.clone(),
            });
        }
        TokenWithDebugInfo {
            internal_tok: Token::Identifier(s),
            line: id_line,
            file: id_file,
        } => {
            let mut next_tok = tokens.clone().next().unwrap();

            // Function call / struct instance with generic binding
            // id:<T1, T2, ...>(args)
            // The colon is necessary or else there is ambiguity with the
            // "less than" and "greater than" operators.

            let mut generics = Vec::new();

            if let TokenWithDebugInfo {
                internal_tok: Token::BeginGeneric,
                ..
            } = next_tok
            {
                tokens.next();
                loop {
                    generics.push(parse_type(&mut tokens));
                    let next_tok = tokens.next().unwrap();
                    if let TokenWithDebugInfo {
                        internal_tok: Token::Operator(Operator::GreaterThan),
                        ..
                    } = next_tok
                    {
                        break;
                    } else if let TokenWithDebugInfo {
                        internal_tok: Token::Comma,
                        ..
                    } = next_tok
                    {
                        continue;
                    } else {
                        error_unexpected_token("comma or '>'", &next_tok);
                    }
                }

                next_tok = tokens.clone().next().unwrap();

                if !matches!(next_tok.internal_tok, Token::LParen)
                    & !matches!(next_tok.internal_tok, Token::LBrace)
                {
                    error_unexpected_token(
                        "function call '(' ( fn_id:<T1, T2, ..>(args) )",
                        &next_tok,
                    );
                }
            }

            if let TokenWithDebugInfo {
                internal_tok: Token::LParen,
                line,
                file,
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
                return Typed::new(TokenWithDebugInfo {
                    internal_tok: Atom::FunctionCall(
                        TokenWithDebugInfo {
                            internal_tok: s.to_string(),
                            line: *id_line,
                            file: id_file.clone(),
                        },
                        args,
                        generics,
                    ),
                    line: line.clone(),
                    file: file.clone(),
                });
            } else if let TokenWithDebugInfo {
                internal_tok: Token::LBrace,
                line,
                file,
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
                return Typed::new(TokenWithDebugInfo {
                    internal_tok: Atom::StructInstance(
                        TokenWithDebugInfo {
                            internal_tok: s.to_string(),
                            line: *id_line,
                            file: id_file.clone(),
                        },
                        fields,
                        generics,
                    ),
                    line: line.clone(),
                    file: file.clone(),
                });
            }

            // Variable
            return Typed::new(TokenWithDebugInfo {
                internal_tok: Atom::Variable(TokenWithDebugInfo {
                    internal_tok: s.to_string(),
                    line: *id_line,
                    file: id_file.clone(),
                }),
                line: id_line.clone(),
                file: id_file.clone(),
            });
        }
        _ => error_unexpected_token("valid atom token", &tok),
    }
}

fn parse_assignment_identifier(
    mut tokens: &mut Iter<TokenWithDebugInfo<Token>>,
) -> TokenWithDebugInfo<AssignmentIdentifier> {
    let tok = tokens.next().unwrap();

    match tok {
        TokenWithDebugInfo {
            internal_tok: Token::Operator(Operator::Multiply),
            line,
            file,
        } => {
            return TokenWithDebugInfo {
                internal_tok: AssignmentIdentifier::Dereference(Box::new(Typed::new(
                    parse_assignment_identifier(&mut tokens),
                ))),
                line: line.clone(),
                file: file.clone(),
            };
        }
        TokenWithDebugInfo {
            internal_tok: Token::Identifier(s),
            line: id_line,
            file: id_file,
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
                return TokenWithDebugInfo {
                    internal_tok: AssignmentIdentifier::Array(
                        TokenWithDebugInfo {
                            internal_tok: s.to_string(),
                            line: id_line.clone(),
                            file: id_file.clone(),
                        },
                        args,
                    ),
                    line: id_line.clone(),
                    file: id_file.clone(),
                };
            }
            return TokenWithDebugInfo {
                internal_tok: AssignmentIdentifier::Variable(TokenWithDebugInfo {
                    internal_tok: s.to_string(),
                    line: id_line.clone(),
                    file: id_file.clone(),
                }),
                line: id_line.clone(),
                file: id_file.clone(),
            };
        }
        _ => error_unexpected_token("valid assignment identifier", &tok),
    }
}

fn parse_reassignment_identifier(expr: TokenWithDebugInfo<Expression>) -> TokenWithDebugInfo<ReassignmentIdentifier> {
    // pub enum ReassignmentIdentifier {
    // 	Variable(String),
    // 	Dereference(Box<Expression>),
    // 	Array(Box<Expression>, Vec<Expression>),
    // 	MemberAccess(Box<Expression>, Box<Expression>),
    // }

    match expr.internal_tok {
        Expression::Atom(Typed {
            expr:
                TokenWithDebugInfo {
                    internal_tok:
                        Atom::Variable(TokenWithDebugInfo {
                            internal_tok: s,
                            line: line1,
                            file: file1,
                        }),
                    line,
                    file,
                },
            ..
        }) => TokenWithDebugInfo {
            internal_tok: ReassignmentIdentifier::Variable(TokenWithDebugInfo {
                internal_tok: s,
                line: line1.clone(),
                file: file1.clone(),
            }),
            line: line.clone(),
            file: file.clone(),
        },
        Expression::UnaryOp(exprs, UnOp::Dereference) => TokenWithDebugInfo {
            internal_tok: ReassignmentIdentifier::Dereference(exprs.clone()),
            line: exprs.clone().expr.line,
            file: exprs.clone().expr.file,
        },
        Expression::ArrayAccess(expr, args) => TokenWithDebugInfo {
            internal_tok: ReassignmentIdentifier::Array(expr.clone(), args),
            line: expr.clone().expr.line,
            file: expr.clone().expr.file,
        },
        Expression::BinaryOp(expr1, expr2, BinOp::MemberAccess) => TokenWithDebugInfo {
            internal_tok: ReassignmentIdentifier::MemberAccess(expr1.clone(), expr2),
            line: expr1.clone().expr.line,
            file: expr1.clone().expr.file,
        },
        _ => error("Invalid reassignment identifier. Only modifiable lvalues are allowed.", &expr),
    }
}

fn find_arr_dims(array: &Atom) -> Option<Vec<i64>> {
    match array {
        Atom::Array(sub_elements, dim) => {
            let mut dims: Vec<i64> = Vec::new();

            for Typed {
                expr:
                    TokenWithDebugInfo {
                        internal_tok: expr, ..
                    },
                ..
            } in sub_elements
            {
                let Typed {
                    expr:
                        TokenWithDebugInfo {
                            internal_tok: elem, ..
                        },
                    ..
                } = match expr {
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

fn rectangularize_array(
    array: &mut Vec<Typed<TokenWithDebugInfo<Expression>>>,
    depth: usize,
    max_dims: &Vec<i64>,
) {
    let max_size = max_dims[depth] as usize;
    let (line, file) = if let Some(first_elem) = array.first() {
        (first_elem.expr.line.clone(), first_elem.expr.file.clone())
    } else {
        (0, String::new())
    };
    if depth + 1 < max_dims.len() {
        for Typed { expr: elem, .. } in array.iter_mut() {
            if let Expression::Atom(Typed {
                expr:
                    TokenWithDebugInfo {
                        internal_tok: Atom::Array(ref mut sub_array, _),
                        ..
                    },
                ..
            }) = elem.internal_tok
            {
                rectangularize_array(sub_array, depth + 1, max_dims);
            } else {
                let mut new_elem = vec![Typed::new(elem.clone())];
                let (line, file) = (elem.line.clone(), elem.file.clone());
                rectangularize_array(&mut new_elem, depth + 1, max_dims);
                *elem = TokenWithDebugInfo {
                    internal_tok: Expression::Atom(Typed::new(TokenWithDebugInfo {
                        internal_tok: Atom::Array(new_elem.clone(), new_elem.len() as i64),
                        line,
                        file: file.clone(),
                    })),
                    line,
                    file,
                };
            }
        }
    }

    while array.len() < max_size {
        if depth == max_dims.len() - 1 {
            array.push(Typed::new(TokenWithDebugInfo {
                internal_tok: Expression::Atom(Typed::new(TokenWithDebugInfo {
                    internal_tok: Atom::Literal(Typed::new(TokenWithDebugInfo {
                        internal_tok: Literal::Int(0),
                        line: line.clone(),
                        file: file.clone(),
                    })),
                    line,
                    file: file.clone(),
                })),
                line,
                file: file.clone(),
            }));
        } else {
            let mut new_elem = vec![];
            rectangularize_array(&mut new_elem, depth + 1, max_dims);
            let (line, file) = if let Some(first_elem) = new_elem.first() {
                (first_elem.expr.line.clone(), first_elem.expr.file.clone())
            } else {
                (0, String::new())
            };
            array.push(Typed::new(TokenWithDebugInfo {
                internal_tok: Expression::Atom(Typed::new(TokenWithDebugInfo {
                    internal_tok: Atom::Array(new_elem.clone(), new_elem.len() as i64),
                    line,
                    file: file.clone(),
                })),
                line,
                file,
            }));
        }
    }
}

fn flatten(
    array: &Vec<Typed<TokenWithDebugInfo<Expression>>>,
) -> Vec<Typed<TokenWithDebugInfo<Expression>>> {
    let mut flat_arr = Vec::new();
    for Typed { expr: elem, .. } in array.iter() {
        if let Expression::Atom(Typed {
            expr:
                TokenWithDebugInfo {
                    internal_tok: Atom::Array(ref sub_array, _),
                    ..
                },
            ..
        }) = elem.internal_tok
        {
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

fn parse_array(tokens: &mut Iter<TokenWithDebugInfo<Token>>) -> TokenWithDebugInfo<Atom> {
    let mut elements = Vec::new();
    let tmp_clone = tokens.clone().next().unwrap();
    let (line, file) = (tmp_clone.line.clone(), tmp_clone.file.clone());
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

    TokenWithDebugInfo {
        internal_tok: Atom::Array(flat_elements, max_dims[0]),
        line,
        file,
    }
}

/// Recursively parses an expression, taking into account operator precedence.
fn parse_expression_with_precedence(
    mut tokens: &mut Iter<TokenWithDebugInfo<Token>>,
    precedence_level: usize,
    precedence_table: &Vec<PrecedenceLevel>,
) -> TokenWithDebugInfo<Expression> {
    if precedence_level == 0 {
        // Parse the lowest precedence, like literals or atoms
        let atom = parse_atom(&mut tokens);
        let (line, file) = (atom.expr.line.clone(), atom.expr.file.clone());
        return TokenWithDebugInfo {
            internal_tok: Expression::Atom(atom),
            line,
            file,
        };
    }

    // Check if the current token is a unary operator for this precedence level
    let next = tokens.clone().next().unwrap();
    let mut expr;

    if precedence_table[precedence_level]
        .unary_ops
        .contains(&get_un_operator_from_token(&next).internal_tok)
    {
        let tok = tokens.next().unwrap();
        let (line, file) = (tok.line.clone(), tok.file.clone());
        let op = get_un_operator_from_token(tok); // Get the unary operator
        let operand =
            parse_expression_with_precedence(&mut tokens, precedence_level, precedence_table); // Parse operand
        expr = TokenWithDebugInfo {
            internal_tok: Expression::UnaryOp(Box::new(Typed::new(operand)), op.internal_tok),
            line,
            file,
        }; // Apply unary operator
    } else {
        // No unary operator, so parse the next lower precedence level
        expr =
            parse_expression_with_precedence(&mut tokens, precedence_level - 1, precedence_table);
    }

    let mut next = tokens.clone().next().unwrap();
    // Array access
    if let TokenWithDebugInfo {
        internal_tok: Token::LBracket,
        line,
        file,
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
        expr = TokenWithDebugInfo {
            internal_tok: Expression::ArrayAccess(Box::new(Typed::new(expr)), args),
            line: line.clone(),
            file: file.clone(),
        };
    }

    // Now handle binary and assignment operators for the current precedence level
    while precedence_table[precedence_level]
        .binary_ops
        .contains(&get_bin_operator_from_tokens(tokens).internal_tok)
        || precedence_table[precedence_level]
            .assignment_ops
            .contains(&get_assign_operator_from_tokens(tokens).internal_tok)
    {
        let op = get_bin_operator_from_tokens(tokens); // Get the binary operator
        let (line, file) = (op.line.clone(), op.file.clone());

        if op.internal_tok == BinOp::NotABinaryOp {
            // If it's not a binary operator, it must be an assignment operator
            let op = get_assign_operator_from_tokens(tokens); // Get the assignment operator

            // Skip forward the appropriate number of tokens
            match op.internal_tok {
                AssignmentOp::RightShiftAssign => {
                    // This specific operator is made of three tokens
                    tokens.next();
                    tokens.next();
                    tokens.next();
                }
                AssignmentOp::LeftShiftAssign => {
                    // This specific operator is made of two tokens
                    tokens.next();
                    tokens.next();
                }
                _ => {
                    tokens.next();
                }
            }

            let next_term = parse_expression_with_precedence(
                &mut tokens,
                precedence_level - 1,
                precedence_table,
            ); // Parse next term

            // Re-parse the left hand side of the assignment expression
            let assignment_identifier = parse_reassignment_identifier(expr);
            expr = TokenWithDebugInfo {
                internal_tok: Expression::Assignment(
                    Typed::new(assignment_identifier),
                    Box::new(Typed::new(next_term)),
                    op.internal_tok,
                ),
                line,
                file,
            };
        } else {
            // Skip forward the appropriate number of tokens
            match op.internal_tok {
                BinOp::GreaterOrEqualThan | BinOp::RightShift => {
                    // There specific operators are made of two tokens
                    tokens.next();
                    tokens.next();
                }
                _ => {
                    tokens.next();
                }
            }

            let next_term = parse_expression_with_precedence(
                &mut tokens,
                precedence_level - 1,
                precedence_table,
            ); // Parse next term
            expr = TokenWithDebugInfo {
                internal_tok: Expression::BinaryOp(
                    Box::new(Typed::new(expr)),
                    Box::new(Typed::new(next_term)),
                    op.internal_tok,
                ),
                line,
                file,
            };
        }
        next = tokens.clone().next().unwrap();
    }

    // Type cast
    if let TokenWithDebugInfo {
        internal_tok: Token::Colon,
        line,
        file,
    } = next
    {
        tokens.next();
        let type_casted = parse_type(&mut tokens);
        expr = TokenWithDebugInfo {
            internal_tok: Expression::TypeCast(
                Box::new(Typed::new(expr)),
                type_casted,
            ),
            line: line.clone(),
            file: file.clone(),
        };
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

fn parse_type(mut tokens: &mut Iter<TokenWithDebugInfo<Token>>) -> TokenWithDebugInfo<Type> {
    let tok = tokens.next().unwrap();
    let (line, file) = (tok.line.clone(), tok.file.clone());

    let t = match tok {
        TokenWithDebugInfo {
            internal_tok: Token::PrimitiveType(k),
            ..
        } => match k.as_str() {
            "int" => Type::Int,
            "float" => Type::Float,
            "bool" => Type::Bool,
            "void" => Type::Void,
            "char" => Type::Char,
            "str" => Type::Array(Box::new(TokenWithDebugInfo {
                internal_tok: Type::Char,
                line: line.clone(),
                file: file.clone(),
            })),
            "array" => {
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
                Type::Array(Box::new(inner_type))
            }
            _ => error_unexpected_token("valid type keyword", &tok),
        },
        TokenWithDebugInfo {
            internal_tok: Token::Operator(Operator::Multiply),
            ..
        } => {
            let inner_type = parse_type(&mut tokens);
            Type::Pointer(Box::new(inner_type))
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
            } else if let TokenWithDebugInfo {
                internal_tok: Token::BeginGeneric,
                ..
            } = next_tok
            {
                // Generics bindings type:<T1, T2, ...>
                tokens.next();
                let mut generics = Vec::new();
                loop {
                    generics.push(parse_type(&mut tokens));
                    let next_tok = tokens.next().unwrap();
                    if let TokenWithDebugInfo {
                        internal_tok: Token::Operator(Operator::GreaterThan),
                        ..
                    } = next_tok
                    {
                        break;
                    } else if let TokenWithDebugInfo {
                        internal_tok: Token::Comma,
                        ..
                    } = next_tok
                    {
                        continue;
                    } else {
                        error_unexpected_token("comma or '>'", &next_tok);
                    }
                }
                Type::GenericBinding(id.to_string(), generics)
            } else {
                Type::Struct(id.to_string())
            }
        }
        _ => error_unexpected_token("valid type token", &tok),
    };

    TokenWithDebugInfo {
        internal_tok: t,
        line,
        file,
    }
}

/// Parses an expression from a list of tokens.
///
/// This function is a wrapper around parse_expression_with_precedence that uses the highest precedence level.
/// It is used to parse the top-level expression.
///
/// An expression is a combination of atoms and operators that evaluates to a value.
fn parse_expression(
    mut tokens: &mut Iter<TokenWithDebugInfo<Token>>,
) -> TokenWithDebugInfo<Expression> {
    let precedence_table = build_precedence_table();
    let max_precedence = precedence_table.len() - 1;

    parse_expression_with_precedence(&mut tokens, max_precedence, &precedence_table)
}

/// Parses a statement from a list of tokens.
///
/// A statement is a single instruction in the program.
/// Statements can be assignments, let bindings, if statements, while loops, loops, do-while loops, for loops, return statements, expressions, compound statements, break statements, or continue statements.
fn parse_statement(tokens: &mut Iter<TokenWithDebugInfo<Token>>) -> TokenWithDebugInfo<Statement> {
    let tok = tokens.clone().next().unwrap();
    let (line, file) = (tok.line.clone(), tok.file.clone());
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
                let (line, file) = (if_stmt.line.clone(), if_stmt.file.clone());

                if !matches!(
                    if_stmt,
                    TokenWithDebugInfo {
                        internal_tok: Statement::Compound(_),
                        ..
                    }
                ) {
                    if_stmt = TokenWithDebugInfo {
                        internal_tok: Statement::Compound(vec![Typed::new(if_stmt)]),
                        line: line.clone(),
                        file: file.clone(),
                    };
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
                        let (line, file) = (else_stmt.line.clone(), else_stmt.file.clone());

                        if !matches!(
                            else_stmt,
                            TokenWithDebugInfo {
                                internal_tok: Statement::Compound(_),
                                ..
                            }
                        ) {
                            else_stmt = TokenWithDebugInfo {
                                internal_tok: Statement::Compound(vec![Typed::new(else_stmt)]),
                                line: line.clone(),
                                file: file.clone(),
                            };
                        }

                        statement = Statement::If(
                            Typed::new(exp),
                            Box::new(Typed::new(if_stmt)),
                            Some(Box::new(Typed::new(else_stmt))),
                        );
                    } else {
                        statement =
                            Statement::If(Typed::new(exp), Box::new(Typed::new(if_stmt)), None);
                    }
                } else {
                    statement = Statement::If(Typed::new(exp), Box::new(Typed::new(if_stmt)), None);
                }
            } else if k == "while" {
                // while (exp) statement
                let exp = parse_expression(tokens);
                let mut while_stmt = parse_statement(tokens);
                let (line, file) = (while_stmt.line.clone(), while_stmt.file.clone());

                if !matches!(
                    while_stmt,
                    TokenWithDebugInfo {
                        internal_tok: Statement::Compound(_),
                        ..
                    }
                ) {
                    while_stmt = TokenWithDebugInfo {
                        internal_tok: Statement::Compound(vec![Typed::new(while_stmt)]),
                        line: line.clone(),
                        file: file.clone(),
                    };
                }

                statement = Statement::While(Typed::new(exp), Box::new(Typed::new(while_stmt)));
            } else if k == "loop" {
                // loop statement
                let mut loop_stmt = parse_statement(tokens);
                let (line, file) = (loop_stmt.line.clone(), loop_stmt.file.clone());

                if let TokenWithDebugInfo {
                    internal_tok: Statement::Compound(_),
                    ..
                } = loop_stmt
                {
                } else {
                    loop_stmt = TokenWithDebugInfo {
                        internal_tok: Statement::Compound(vec![Typed::new(loop_stmt)]),
                        line: line.clone(),
                        file: file.clone(),
                    };
                }

                statement = Statement::Loop(Box::new(Typed::new(loop_stmt)));
            } else if k == "for" {
                // for (statement; exp; exp) statement
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
                let init = parse_statement(tokens);
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
                let (line, file) = (for_stmt.line.clone(), for_stmt.file.clone());

                if !matches!(
                    for_stmt,
                    TokenWithDebugInfo {
                        internal_tok: Statement::Compound(_),
                        ..
                    }
                ) {
                    for_stmt = TokenWithDebugInfo {
                        internal_tok: Statement::Compound(vec![Typed::new(for_stmt)]),
                        line: line.clone(),
                        file: file.clone(),
                    };
                }

                statement = Statement::For(
                    Box::new(Typed::new(init)),
                    Typed::new(cond),
                    Typed::new(step),
                    Box::new(Typed::new(for_stmt)),
                );
            } else if k == "do" {
                // do statement while (exp);
                let mut do_stmt = parse_statement(tokens);
                let (line, file) = (do_stmt.line.clone(), do_stmt.file.clone());

                if !matches!(
                    do_stmt,
                    TokenWithDebugInfo {
                        internal_tok: Statement::Compound(_),
                        ..
                    }
                ) {
                    do_stmt = TokenWithDebugInfo {
                        internal_tok: Statement::Compound(vec![Typed::new(do_stmt)]),
                        line: line.clone(),
                        file: file.clone(),
                    };
                }

                let next_tok = tokens.next().unwrap();
                if let TokenWithDebugInfo {
                    internal_tok: Token::Keyword(k),
                    ..
                } = next_tok
                {
                    if k == "while" {
                        let exp = parse_expression(tokens);
                        statement =
                            Statement::Dowhile(Typed::new(exp), Box::new(Typed::new(do_stmt)));
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
                let (line, file) = (next_tok.line.clone(), next_tok.file.clone());
                if let TokenWithDebugInfo {
                    internal_tok: Token::StringLiteral(s),
                    ..
                } = next_tok
                {
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
                    statement = Statement::Asm(
                        TokenWithDebugInfo {
                            internal_tok: asm,
                            line,
                            file,
                        },
                        return_type,
                    );
                } else {
                    statement = Statement::Asm(
                        TokenWithDebugInfo {
                            internal_tok: asm,
                            line,
                            file: file.clone(),
                        },
                        TokenWithDebugInfo {
                            internal_tok: Type::Void,
                            line,
                            file,
                        },
                    );
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
					let new_statement = parse_statement(tokens);
					if let TokenWithDebugInfo{ internal_tok: Statement::For(init, ..), ..} = &new_statement {
						statements.push(*init.clone());
					}
                    statements.push(Typed::new(new_statement));
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
    match statement {
        Statement::Compound(_)
        | Statement::If(_, _, _)
        | Statement::While(_, _)
        | Statement::Loop(_)
        | Statement::For(_, _, _, _) => {
            return TokenWithDebugInfo {
                internal_tok: statement,
                line,
                file,
            };
        }
        _ => {}
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

    return TokenWithDebugInfo {
        internal_tok: statement,
        line,
        file,
    };
}

fn parse_const(tokens: &mut Iter<TokenWithDebugInfo<Token>>) -> TokenWithDebugInfo<Constant> {
    // const id: type = exp;

    let next_tok = tokens.next().unwrap();
    let (line, file) = (next_tok.line.clone(), next_tok.file.clone());

    if &Token::Keyword("const".to_string()) != next_tok {
        error_unexpected_token("const keyword", next_tok);
    }

    let next_tok = tokens.next().unwrap();
    let (id, id_line, id_file) = match next_tok {
        TokenWithDebugInfo {
            internal_tok: Token::Identifier(id),
            line: id_line,
            file: id_file,
        } => (id.clone(), id_line, id_file),
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

    return TokenWithDebugInfo {
        internal_tok: Constant::Constant(
            TokenWithDebugInfo {
                internal_tok: id,
                line: id_line.clone(),
                file: id_file.clone(),
            },
            lit,
            var_type,
        ),
        line,
        file,
    };
}

/// Parses a function from a list of tokens.
/// fn id(params): type statement
fn parse_function(
    mut tokens: &mut Iter<TokenWithDebugInfo<Token>>,
) -> TokenWithDebugInfo<Function> {
    let tok = tokens.next().unwrap();
    let (line, file) = (tok.line.clone(), tok.file.clone());

    match tok {
        TokenWithDebugInfo {
            internal_tok: Token::Keyword(k),
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
            line,
            file,
        } => TokenWithDebugInfo {
            internal_tok: id.clone(),
            line: line.clone(),
            file: file.clone(),
        },
        TokenWithDebugInfo { .. } => error_unexpected_token("identifier", &tok),
    };

    let mut tok = tokens.next().unwrap();
    // Check if any abstract types are defined
    // fn id:<type1, type2, ..>(params): type statement
    let mut abstract_types = Vec::new();
    if let TokenWithDebugInfo {
        internal_tok: Token::BeginGeneric,
        ..
    } = tok
    {
        loop {
            let abstract_type_tok = tokens.next().unwrap();
            if let TokenWithDebugInfo {
                internal_tok: Token::Identifier(id),
                line,
                file,
            } = abstract_type_tok
            {
                abstract_types.push(TokenWithDebugInfo {
                    internal_tok: id.clone(),
                    line: line.clone(),
                    file: file.clone(),
                });

                let tok = tokens.next().unwrap();
                if let TokenWithDebugInfo {
                    internal_tok: Token::Comma,
                    ..
                } = tok
                {
                    continue;
                } else if let TokenWithDebugInfo {
                    internal_tok: Token::Operator(Operator::GreaterThan),
                    ..
                } = tok
                {
                    break;
                } else {
                    error_unexpected_token("comma or closing angle bracket", &tok);
                }
            }
        }
        tok = tokens.next().unwrap();
    }

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
            line,
            file,
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
            params.push((
                TokenWithDebugInfo {
                    internal_tok: id.clone(),
                    line: line.clone(),
                    file: file.clone(),
                },
                parse_type(&mut tokens),
            ));
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
    let (stm_line, stm_file) = (statement.line.clone(), statement.file.clone());

    // If the statement is not a compound statement, wrap it in one
    // This is to allow okay-ish scope handling
    if let Statement::Compound(_) = statement.internal_tok {
    } else {
        statement = TokenWithDebugInfo {
            internal_tok: Statement::Compound(vec![Typed::new(statement)]),
            line: stm_line,
            file: stm_file,
        };
    }

    return TokenWithDebugInfo {
        internal_tok: Function {
            id: id,
            args: params,
            body: Typed::new(statement),
            return_type: return_type,
            generics: abstract_types,
        },
        line,
        file,
    };
}

#[derive(Debug, Clone)]
pub struct Struct {
    pub id: TokenWithDebugInfo<String>,
    pub members: Vec<(TokenWithDebugInfo<String>, TokenWithDebugInfo<Type>)>,
    pub generics: Vec<TokenWithDebugInfo<String>>,
}

fn parse_struct(mut tokens: &mut Iter<TokenWithDebugInfo<Token>>) -> TokenWithDebugInfo<Struct> {
    let tok = tokens.next().unwrap();
    let (line, file) = (tok.line.clone(), tok.file.clone());

    if let TokenWithDebugInfo {
        internal_tok: Token::Keyword(k),
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
            line,
            file,
        } => TokenWithDebugInfo {
            internal_tok: id.clone(),
            line: line.clone(),
            file: file.clone(),
        },
        TokenWithDebugInfo { .. } => error_unexpected_token("identifier", &tok),
    };

    let mut tok = tokens.next().unwrap();

    // Check if any abstract types are defined
    // struct id:<type1, type2, ..> { members }
    let mut abstract_types = Vec::new();
    if let TokenWithDebugInfo {
        internal_tok: Token::BeginGeneric,
        ..
    } = tok
    {
        loop {
            let abstract_type_tok = tokens.next().unwrap();
            if let TokenWithDebugInfo {
                internal_tok: Token::Identifier(id),
                line,
                file,
            } = abstract_type_tok
            {
                abstract_types.push(TokenWithDebugInfo {
                    internal_tok: id.clone(),
                    line: line.clone(),
                    file: file.clone(),
                });
                let tok = tokens.next().unwrap();
                if let TokenWithDebugInfo {
                    internal_tok: Token::Comma,
                    ..
                } = tok
                {
                    continue;
                } else if let TokenWithDebugInfo {
                    internal_tok: Token::Operator(Operator::GreaterThan),
                    ..
                } = tok
                {
                    break;
                } else {
                    error_unexpected_token("comma or closing angle bracket", &tok);
                }
            }
        }
        tok = tokens.next().unwrap();
    }

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
            line,
            file,
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
            members.push((
                TokenWithDebugInfo {
                    internal_tok: id.clone(),
                    line: line.clone(),
                    file: file.clone(),
                },
                parse_type(&mut tokens),
            ));
        } else {
            error_unexpected_token("identifier or closing brace", &tok);
        }
    }

    return TokenWithDebugInfo {
        internal_tok: Struct {
            id,
            members,
            generics: abstract_types,
        },
        line,
        file,
    };
}

#[derive(Debug, Clone)]
pub struct Enum {
    pub id: TokenWithDebugInfo<String>,
    pub variants: Vec<TokenWithDebugInfo<String>>,
}

fn parse_enum(tokens: &mut Iter<TokenWithDebugInfo<Token>>) -> TokenWithDebugInfo<Enum> {
    let tok = tokens.next().unwrap();
    let (line, file) = (tok.line.clone(), tok.file.clone());

    if let TokenWithDebugInfo {
        internal_tok: Token::Keyword(k),
        ..
    } = tok
    {
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
            line,
            file,
        } => TokenWithDebugInfo {
            internal_tok: id.clone(),
            line: line.clone(),
            file: file.clone(),
        },
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
            line,
            file,
        } = tok
        {
            variants.push(TokenWithDebugInfo {
                internal_tok: id.clone(),
                line: line.clone(),
                file: file.clone(),
            });
            let next_tok = tokens.next().unwrap();

            if let TokenWithDebugInfo {
                internal_tok: Token::Comma,
                ..
            } = next_tok
            {
                continue;
            } else if let TokenWithDebugInfo {
                internal_tok: Token::RBrace,
                ..
            } = next_tok
            {
                break;
            } else {
                error_unexpected_token("comma or closing brace", &next_tok);
            }
        } else {
            error_unexpected_token("identifier", &tok);
        }
    }

    TokenWithDebugInfo {
        internal_tok: Enum { id, variants },
        line,
        file,
    }
}

/// Parses an abstract syntax tree (AST) from a list of tokens.
pub fn parse_namespace(
    tokens: &mut Iter<TokenWithDebugInfo<Token>>,
    is_toplevel: bool,
) -> TokenWithDebugInfo<Namespace> {
    let id;

    let tmp_clone = tokens.clone().next().unwrap();
    let (line, file) = (tmp_clone.line.clone(), tmp_clone.file.clone());

    if !is_toplevel {
        let tok = tokens.next().unwrap();
        if let TokenWithDebugInfo {
            internal_tok: Token::Keyword(k),
            ..
        } = tok
        {
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
            } => {
                if id.eq("toplevel") {
                    error("cannot use 'toplevel' as a namespace name", &tok);
                }
                id.clone()
            }
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
            } => {
                if !is_toplevel {
                    error("unexpected EOF: Missing 'spacename' keyword", &next_tok);
                }
                break;
            }
            TokenWithDebugInfo {
                internal_tok: Token::Keyword(k),
                ..
            } => {
                if k == "fn" {
                    functions.push(Typed::new(parse_function(tokens)))
                } else if k == "const" {
                    constants.push(Typed::new(parse_const(tokens)))
                } else if k == "struct" {
                    structs.push(parse_struct(tokens))
                } else if k == "enum" {
                    enums.push(parse_enum(tokens))
                } else if k == "union" {
                    todo!("union")
                } else if k == "namespace" {
                    sub_namespaces.push(parse_namespace(tokens, false));
                } else if k == "spacename" {
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
                        break;
                    }
                } else {
                    error_unexpected_token(
                        "function, constant, struct, enum or union declaration",
                        &next_tok,
                    )
                }
            }
            _ => error_unexpected_token(
                "function, constant, struct, enum or union declaration",
                &next_tok,
            ),
        }
    }

    TokenWithDebugInfo {
        internal_tok: Namespace {
            id,
            functions,
            constants,
            structs,
            enums,
            sub_namespaces,
        },
        line,
        file,
    }
}

pub fn parse(tokens: &Vec<TokenWithDebugInfo<Token>>) -> Ast {
    let mut tokens = tokens.iter();
    let program = parse_namespace(&mut tokens, true);

    Ast { program }
}
