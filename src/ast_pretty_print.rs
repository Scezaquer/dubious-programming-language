use crate::ast_build::{
	Ast, Atom, BinOp, Constant, Expression, Function, Program, Statement, UnOp, AssignmentOp,
};

impl std::fmt::Display for Ast {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.program)
	}
}

impl std::fmt::Display for Program {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Program::Program(functions) => {
				for function in functions {
					writeln!(f, "{}", function)?;
				}
				Ok(())
			}
		}
	}
}

impl std::fmt::Display for Function {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Function::Function(name, params, body) => {
				write!(f, "fn {}(", name)?;
				for (i, param) in params.iter().enumerate() {
					write!(f, "{}", param)?;
					if i < params.len() - 1 {
						write!(f, ", ")?;
					}
				}
				writeln!(f, ") {{")?;
				writeln!(f, "{}", body)?;
				writeln!(f, "}}")
			}
		}
	}
}

impl std::fmt::Display for Statement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Statement::Assignment(var, expr, op) => write!(f, "{} {} {};", var, op, expr),
			Statement::Let(var, Some(expr)) => write!(f, "let {} = {};", var, expr),
			Statement::Let(var, None) => write!(f, "let {};", var),
			Statement::If(cond, then_stmt, Some(else_stmt)) => {
				write!(f, "if ({}) {{\n{}\n}} else {{\n{}\n}}", cond, then_stmt, else_stmt)
			}
			Statement::If(cond, then_stmt, None) => {
				write!(f, "if ({}) {{\n{}\n}}", cond, then_stmt)
			}
			Statement::While(cond, body) => write!(f, "while ({}) {{\n{}\n}}", cond, body),
			Statement::Loop(body) => write!(f, "loop {{\n{}\n}}", body),
			Statement::Dowhile(cond, body) => write!(f, "do {{\n{}\n}} while ({});", body, cond),
			Statement::For(init, cond, step, body) => {
				write!(f, "for ({}; {}; {}) {{\n{}\n}}", init, cond, step, body)
			}
			Statement::Return(expr) => write!(f, "return {};", expr),
			Statement::Expression(expr) => write!(f, "{};", expr),
			Statement::Compound(statements) => {
				for stmt in statements {
					writeln!(f, "{}", stmt)?;
				}
				Ok(())
			}
			Statement::Break => write!(f, "break;"),
			Statement::Continue => write!(f, "continue;"),
		}
	}
}

impl std::fmt::Display for Expression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Expression::Atom(atom) => write!(f, "{}", atom),
			Expression::UnaryOp(expr, op) => write!(f, "({}{})", op, expr),
			Expression::BinaryOp(lhs, rhs, op) => write!(f, "({} {} {})", lhs, rhs, op),
			Expression::Assignment(var, expr, op) => write!(f, "{} {} {}", var, op, expr),
		}
	}
}

impl std::fmt::Display for Atom {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Atom::Constant(constant) => write!(f, "{}", constant),
			Atom::Expression(expr) => write!(f, "({})", expr),
			Atom::Variable(var) => write!(f, "{}", var),
			Atom::FunctionCall(name, args) => {
				write!(f, "{}(", name)?;
				for (i, arg) in args.iter().enumerate() {
					write!(f, "{}", arg)?;
					if i < args.len() - 1 {
						write!(f, ", ")?;
					}
				}
				write!(f, ")")
			}
		}
	}
}

impl std::fmt::Display for UnOp {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let op_str = match self {
			UnOp::PreIncrement => "++",
			UnOp::PreDecrement => "--",
			UnOp::UnaryPlus => "+",
			UnOp::UnaryMinus => "-",
			UnOp::LogicalNot => "!",
			UnOp::BitwiseNot => "~",
			UnOp::Dereference => "*",
			UnOp::AddressOf => "&",
			UnOp::NotAUnaryOp => "",
		};
		write!(f, "{}", op_str)
	}
}

impl std::fmt::Display for BinOp {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let op_str = match self {
			BinOp::MemberAccess => ".",
			BinOp::Exponent => "**",
			BinOp::Multiply => "*",
			BinOp::Divide => "/",
			BinOp::Modulus => "%",
			BinOp::Add => "+",
			BinOp::Subtract => "-",
			BinOp::LeftShift => "<<",
			BinOp::RightShift => ">>",
			BinOp::LessThan => "<",
			BinOp::GreaterThan => ">",
			BinOp::LessOrEqualThan => "<=",
			BinOp::GreaterOrEqualThan => ">=",
			BinOp::Equal => "==",
			BinOp::NotEqual => "!=",
			BinOp::BitwiseAnd => "&",
			BinOp::BitwiseXor => "^",
			BinOp::BitwiseOr => "|",
			BinOp::LogicalAnd => "&&",
			BinOp::LogicalXor => "^^",
			BinOp::LogicalOr => "||",
			BinOp::NotABinaryOp => "",
		};
		write!(f, "{}", op_str)
	}
}

impl std::fmt::Display for AssignmentOp {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let op_str = match self {
			AssignmentOp::Assign => "=",
			AssignmentOp::AddAssign => "+=",
			AssignmentOp::SubtractAssign => "-=",
			AssignmentOp::MultiplyAssign => "*=",
			AssignmentOp::DivideAssign => "/=",
			AssignmentOp::ModulusAssign => "%=",
			AssignmentOp::LeftShiftAssign => "<<=",
			AssignmentOp::RightShiftAssign => ">>=",
			AssignmentOp::BitwiseAndAssign => "&=",
			AssignmentOp::BitwiseXorAssign => "^=",
			AssignmentOp::BitwiseOrAssign => "|=",
			AssignmentOp::NotAnAssignmentOp => "",
		};
		write!(f, "{}", op_str)
	}
}