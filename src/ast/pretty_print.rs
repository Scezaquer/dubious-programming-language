use std::fmt;

use super::build_ast::{Ast, BinOp, UnOp, Constant, Expression, Atom, Function, Program, Statement, AssignmentOp};

impl fmt::Display for Constant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Constant::Int(i) => write!(f, "{}", i),
            Constant::Float(fl) => write!(f, "{}", fl),
            Constant::Bool(b) => write!(f, "{}", b),
        }
    }
}

impl fmt::Display for UnOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnOp::PreIncrement => write!(f, "++"),
            UnOp::PreDecrement => write!(f, "--"),
            UnOp::UnaryPlus => write!(f, "+"),
            UnOp::UnaryMinus => write!(f, "-"),
            UnOp::LogicalNot => write!(f, "!"),
            UnOp::BitwiseNot => write!(f, "~"),
            UnOp::Dereference => write!(f, "*"),
            UnOp::AddressOf => write!(f, "&"),
            _ => write!(f, "Unknown"),
        }
    }
}

impl fmt::Display for BinOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinOp::MemberAccess => write!(f, "."),
            BinOp::Exponent => write!(f, "**"),
            BinOp::Multiply => write!(f, "*"),
            BinOp::Divide => write!(f, "/"),
            BinOp::Modulus => write!(f, "%"),
            BinOp::Add => write!(f, "+"),
            BinOp::Subtract => write!(f, "-"),
            BinOp::LeftShift => write!(f, "<<"),
            BinOp::RightShift => write!(f, ">>"),
            BinOp::LessThan => write!(f, "<"),
            BinOp::GreaterThan => write!(f, ">"),
            BinOp::LessOrEqualThan => write!(f, "<="),
            BinOp::GreaterOrEqualThan => write!(f, ">="),
            BinOp::Equal => write!(f, "=="),
            BinOp::NotEqual => write!(f, "!="),
            BinOp::BitwiseAnd => write!(f, "&"),
            BinOp::BitwiseXor => write!(f, "^"),
            BinOp::BitwiseOr => write!(f, "|"),
            BinOp::LogicalAnd => write!(f, "&&"),
            BinOp::LogicalXor => write!(f, "^^"),
            BinOp::LogicalOr => write!(f, "||"),
            _ => write!(f, "Unknown"),
        }
    }
}

impl fmt::Display for AssignmentOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AssignmentOp::Assign => write!(f, "="),
            AssignmentOp::AddAssign => write!(f, "+="),
            AssignmentOp::SubtractAssign => write!(f, "-="),
            AssignmentOp::MultiplyAssign => write!(f, "*="),
            AssignmentOp::DivideAssign => write!(f, "/="),
            AssignmentOp::ModulusAssign => write!(f, "%="),
            AssignmentOp::LeftShiftAssign => write!(f, "<<="),
            AssignmentOp::RightShiftAssign => write!(f, ">>="),
            AssignmentOp::BitwiseAndAssign => write!(f, "&="),
            AssignmentOp::BitwiseXorAssign => write!(f, "^="),
            AssignmentOp::BitwiseOrAssign => write!(f, "|="),
            _ => write!(f, "Unknown"),
        }
    }
}

impl Atom {
    fn pretty_print(&self, indent: usize) -> String {
        match self {
            Atom::Constant(c) => format!("{}", c),
            Atom::Expression(e) => format!("({})", e.pretty_print(indent)),
            Atom::Variable(v) => v.clone(),
        }
    }
}

impl Expression {
    fn pretty_print(&self, indent: usize) -> String {
        match self {
            Expression::Atom(a) => a.pretty_print(indent),
            Expression::UnaryOp(expr, op) => format!("{}{}", op, expr.pretty_print(indent)),
            Expression::BinaryOp(left, right, op) => format!(
                "{} {} {}",
                left.pretty_print(indent),
                op,
                right.pretty_print(indent)
            ),
            Expression::Assignment(var, expr, op) => format!(
                "{} {} {}",
                var,
                op,
                expr.pretty_print(indent)
            ),
        }
    }
}

impl Statement {
    fn pretty_print(&self, indent: usize) -> String {
        let indent_str = " ".repeat(indent);
        match self {
            Statement::Assignment(var, expr) => format!(
                "{}{} = {}",
                indent_str,
                var,
                expr.pretty_print(indent + 2)
            ),
            Statement::Let(var, expr_opt) => {
                if let Some(expr) = expr_opt {
                    format!(
                        "{}let {} = {}",
                        indent_str,
                        var,
                        expr.pretty_print(indent + 2)
                    )
                } else {
                    format!("{}let {}", indent_str, var)
                }
            },
            Statement::If(cond, body) => {
                let mut result = format!("{}if {}\n", indent_str, cond.pretty_print(indent));
                for stmt in body {
                    result.push_str(&format!(
                        "{}\n",
                        stmt.pretty_print(indent + 2)
                    ));
                }
                result
            },
            Statement::While(cond, body) => {
                let mut result = format!("{}while {}\n", indent_str, cond.pretty_print(indent));
                for stmt in body {
                    result.push_str(&format!(
                        "{}\n",
                        stmt.pretty_print(indent + 2)
                    ));
                }
                result
            },
            Statement::Return(expr) => format!(
                "{}return {}",
                indent_str,
                expr.pretty_print(indent)
            ),
            Statement::Expression(expr) => format!(
                "{}{}",
                indent_str,
                expr.pretty_print(indent)
            ),
        }
    }
}

impl Function {
    fn pretty_print(&self, indent: usize) -> String {
        let indent_str = " ".repeat(indent);
        match self {
            Function::Function(name, params, body) => {
                let mut result = format!("{}fn {}({})\n", indent_str, name, params.join(", "));
                result.push_str(&format!("{}{{\n", indent_str));
                for stmt in body {
                    result.push_str(&format!(
                        "{}\n",
                        stmt.pretty_print(indent + 2)
                    ));
                }
                result.push_str(&format!("{}}}", indent_str));
                result
            }
        }
    }
}

impl Program {
    fn pretty_print(&self, indent: usize) -> String {
        match self {
            Program::Program(functions) => {
                functions
                    .iter()
                    .map(|f| f.pretty_print(indent))
                    .collect::<Vec<String>>()
                    .join("\n\n")
            }
        }
    }
}

impl Ast {
    pub fn pretty_print(&self) -> String {
        self.program.pretty_print(0)
    }
}