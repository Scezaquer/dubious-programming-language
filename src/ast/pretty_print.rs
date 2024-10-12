use std::fmt;

use super::build_ast::{Ast, TermBinaryOp, ExpressionBinaryOp, Constant, Expression, Factor, Function, Program, Statement, Term, UnaryOp};

impl fmt::Display for Constant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Constant::Int(i) => write!(f, "{}", i),
            Constant::Float(fl) => write!(f, "{}", fl),
            Constant::Bool(b) => write!(f, "{}", b),
        }
    }
}

impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnaryOp::Neg => write!(f, "-"),
            UnaryOp::Not => write!(f, "!"),
        }
    }
}

impl fmt::Display for ExpressionBinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExpressionBinaryOp::Add => write!(f, "+"),
            ExpressionBinaryOp::Sub => write!(f, "-"),
        }
    }
}

impl fmt::Display for TermBinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TermBinaryOp::Mul => write!(f, "*"),
            TermBinaryOp::Div => write!(f, "/"),
            TermBinaryOp::Mod => write!(f, "%"),
            TermBinaryOp::Pow => write!(f, "^"),
            TermBinaryOp::And => write!(f, "&"),
            TermBinaryOp::Or => write!(f, "|"),
            TermBinaryOp::Less => write!(f, "<"),
            TermBinaryOp::Greater => write!(f, ">"),
            TermBinaryOp::Equal => write!(f, "=="),
        }
    }
}

impl Factor {
    fn pretty_print(&self, indent: usize) -> String {
        match self {
            Factor::Constant(c) => format!("{}", c),
            Factor::Expression(e) => format!("({})", e.pretty_print(indent)),
            Factor::UnaryOp(f, op) => format!("{}({})", op, f.pretty_print(indent)),
            Factor::Variable(v) => v.clone(),
        }
    }
}

impl Term {
    fn pretty_print(&self, indent: usize) -> String {
        match self {
            Term::Factor(f) => f.pretty_print(indent),
            Term::BinaryOp(left, right, op) => format!(
                "{} {} {}",
                left.pretty_print(indent),
                op,
                right.pretty_print(indent)
            ),
        }
    }
}

impl Expression {
    fn pretty_print(&self, indent: usize) -> String {
        match self {
            Expression::Term(t) => t.pretty_print(indent),
            Expression::BinaryOp(left, right, op) => format!(
                "{} {} {}",
                left.pretty_print(indent),
                op,
                right.pretty_print(indent)
            ),
        }
    }
}

impl Statement {
    fn pretty_print(&self, indent: usize) -> String {
        let indent_str = " ".repeat(indent);
        match self {
            Statement::Assignment(var, expr) => format!(
                "{}{}= {}",
                indent_str,
                var,
                expr.pretty_print(indent + 2)
            ),
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