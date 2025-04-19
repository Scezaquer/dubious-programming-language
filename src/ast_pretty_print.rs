use crate::ast_build::{
	AssignmentIdentifier, AssignmentOp, Ast, Atom, BinOp, Constant, Expression, Function, Namespace, ReassignmentIdentifier, Statement, Type, UnOp, Typed
};

impl std::fmt::Display for Ast {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.program)
	}
}

impl<T: std::fmt::Display> std::fmt::Display for Typed<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.expr)
	}
}

impl std::fmt::Display for Namespace {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match &self {
			&Namespace{ id, functions, structs, enums, constants, sub_namespaces } => {
				write!(f, "namespace {} {{\n", id)?;
				for function in functions {
					write!(f, "{}\n", function)?;
				}
				for struct_ in structs {
					write!(f, "{}\n", struct_)?;
				}
				for enum_ in enums {
					write!(f, "{}\n", enum_)?;
				}
				for constant in constants {
					write!(f, "{}\n", constant)?;
				}
				for sub_namespace in sub_namespaces {
					write!(f, "{}\n", sub_namespace)?;
				}
				write!(f, "}}")
			}
		}
	}
}

impl std::fmt::Display for crate::ast_build::Enum {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match &self {
			&crate::ast_build::Enum{ id, variants } => {
				write!(f, "enum {} {{\n", id)?;
				for variant in variants {
					write!(f, "{},\n", variant)?;
				}
				write!(f, "}}")
			}
		}
	}
}

impl std::fmt::Display for crate::ast_build::Struct {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match &self {
			&crate::ast_build::Struct{ id, members, generics } => {
				if generics.is_empty() {
					write!(f, "struct {} {{\n", id)?;
				} else {
					write!(f, "struct {}<", id)?;
					for (i, generic) in generics.iter().enumerate() {
						write!(f, "{}", generic)?;
						if i < generics.len() - 1 {
							write!(f, ", ")?;
						}
					}
					write!(f, "> {{\n")?;
				}
				for member in members {
					write!(f, "{}: {},\n", member.0, member.1)?;
				}
				write!(f, "}}")
			}
		}
	}
}

impl std::fmt::Display for Constant {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Constant::Constant(name, value, t) => write!(f, "const {}: {} = {};", name, t, value)
		}
	}
}

impl std::fmt::Display for Function {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Function{ id, args, body, return_type, generics} => {
				if generics.is_empty() {
					write!(f, "fn {}(", id)?;
				} else {
					write!(f, "fn {}<", id)?;
					for (i, generic) in generics.iter().enumerate() {
						write!(f, "{}", generic)?;
						if i < generics.len() - 1 {
							write!(f, ", ")?;
						}
					}
					write!(f, ">(")?;
				}
				for (i, param) in args.iter().enumerate() {
					write!(f, "{}: {}", param.0, param.1)?;
					if i < args.len() - 1 {
						write!(f, ", ")?;
					}
				}
				writeln!(f, "): {} {{", return_type)?;
				writeln!(f, "{}", body)?;
				writeln!(f, "}}")
			}
		}
	}
}

impl std::fmt::Display for Type {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Type::Bool => write!(f, "bool"),
			Type::Int => write!(f, "int"),
			Type::Float => write!(f, "float"),
			Type::Char => write!(f, "char"),
			Type::Void => write!(f, "void"),
			Type::Pointer(t) => write!(f, "*{}", t),
			Type::Array(t) => write!(f, "[{}]", t),
			Type::Struct(id) => write!(f, "{}", id),
			Type::Enum(id) => write!(f, "{}", id),
			Type::Namespace(id, t) => write!(f, "{}::{}", id, t),
			Type::GenericBinding(id, types) => {
				write!(f, "{}<", id)?;
				for (i, t) in types.iter().enumerate() {
					write!(f, "{}", t)?;
					if i < types.len() - 1 {
						write!(f, ", ")?;
					}
				}
				write!(f, ">")
			}
		}
	}
}

impl std::fmt::Display for Statement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Statement::Let(var, Some(expr), t) => write!(f, "let {}: {} = {};", var, t, expr),
			Statement::Let(var, _, t) => write!(f, "let {}: {};", var, t),
			Statement::If(cond, then_stmt, Some(else_stmt)) => {
				write!(f, "if ({}) {{\n{}\n}} else {{\n{}\n}}", cond, then_stmt, else_stmt)
			}
			Statement::If(cond, then_stmt, _) => {
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
			Statement::Asm(asm, _) => write!(f, "{}", asm),
		}
	}
}

impl std::fmt::Display for AssignmentIdentifier {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			AssignmentIdentifier::Variable(var) => write!(f, "{}", var),
			AssignmentIdentifier::Dereference(var) => write!(f, "*{}", var),
			AssignmentIdentifier::Array(var, index) => write!(f, "{}[{:?}]", var, index),
		}
	}
}

impl std::fmt::Display for ReassignmentIdentifier {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			ReassignmentIdentifier::Variable(var) => write!(f, "{}", var),
			ReassignmentIdentifier::Dereference(var) => write!(f, "*{}", var),
			ReassignmentIdentifier::Array(var, index) => write!(f, "{}[{:?}]", var, index),
			ReassignmentIdentifier::MemberAccess(var, member) => write!(f, "{}.{}", var, member),
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
			Expression::TypeCast(expr, t) => write!(f, "({} : {})", expr, t),
			Expression::ArrayAccess(var, indices) => {
				write!(f, "{}[", var)?;
				for (i, index) in indices.iter().enumerate() {
					write!(f, "{}", index)?;
					if i < indices.len() - 1 {
						write!(f, ", ")?;
					}
				}
				write!(f, "]")
			}
		}
	}
}

impl std::fmt::Display for Atom {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Atom::Literal(constant) => write!(f, "{}", constant),
			Atom::Expression(expr) => write!(f, "({})", expr),
			Atom::Variable(var) => write!(f, "{}", var),
			Atom::FunctionCall(name, args, generics) => {
				if generics.is_empty() {
					write!(f, "{}(", name)?;
				} else {
					write!(f, "{}:<", name)?;
					for (i, generic) in generics.iter().enumerate() {
						write!(f, "{}", generic)?;
						if i < generics.len() - 1 {
							write!(f, ", ")?;
						}
					}
					write!(f, ">(" )?;
				}
				for (i, arg) in args.iter().enumerate() {
					write!(f, "{}", arg)?;
					if i < args.len() - 1 {
						write!(f, ", ")?;
					}
				}
				write!(f, ")")
			},
			Atom::Array(elements, _) => {
				write!(f, "[")?;
				for (i, element) in elements.iter().enumerate() {
					write!(f, "{}", element)?;
					if i < elements.len() - 1 {
						write!(f, ", ")?;
					}
				}
				write!(f, "]")
			}
			Atom::StructInstance(id, members, generics) => {
				if generics.is_empty() {
					write!(f, "{} {{", id)?;
				} else {
					write!(f, "{}:<", id)?;
					for (i, generic) in generics.iter().enumerate() {
						write!(f, "{}", generic)?;
						if i < generics.len() - 1 {
							write!(f, ", ")?;
						}
					}
					write!(f, "> {{")?;
				}
				for (i, member) in members.iter().enumerate() {
					write!(f, "{}", member)?;
					if i < members.len() - 1 {
						write!(f, ", ")?;
					}
				}
				write!(f, "}}")
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
			BinOp::NamespaceAccess => "::",
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