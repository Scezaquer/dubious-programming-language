// TODO: Typechecking
// TODO: Check uninitialized variables aren't called
// TODO: Complain if a variable is declared but never used
// TODO: Check that the return type of a function matches the type of the return statement in every branch
// TODO: is there ambiguity between function and variable names?

use crate::ast_build::{Ast, Program, Function, Statement, Expression, Atom, BinOp, UnOp, Type, Literal};
use std::collections::{HashMap, HashSet};

fn type_expression(expr: &Expression) -> Result<Type, String> {
	match expr {
		Expression::Atom(atom) => {
			match atom {
				Atom::Literal(Literal::Int(_)) => Ok(Type::Int),
				Atom::Literal(Literal::Float(_)) => Ok(Type::Float),
				Atom::Literal(Literal::Bool(_)) => Ok(Type::Bool),
				Atom::Literal(Literal::Char(_)) => Ok(Type::Char),
				Atom::Literal(Literal::Hex(_)) => Ok(Type::Int),
				Atom::Literal(Literal::Binary(_)) => Ok(Type::Int),
				Atom::Literal(Literal::String(_)) => Ok(Type::Pointer(Box::new(Type::Char))),
				Atom::Variable(_) => Ok(Type::Int),	// TODO: variable of different types
				Atom::Array(expressions, _) => {
					// Check all expressions are of the same type
					let mut array_type = None;
					for expr in expressions {
						let expr_type = type_expression(expr)?;
						if let Some(t) = &array_type {
							if *t != expr_type {
								return Err("Array elements are not of the same type".to_string());
							}
						} else {
							array_type = Some(expr_type);
						}
					}
					Ok(Type::Array(Box::new(array_type.unwrap())))
				},
				Atom::FunctionCall(_, _) => Ok(Type::Int),	// TODO: function return type
				Atom::ArrayAccess(_, _) => Ok(Type::Int),	// TODO: array type
				Atom::Expression(expr) => type_expression(expr),
			}
		},
		Expression::BinaryOp(lhs, rhs, op) => {
			let lhs_type = type_expression(lhs)?;
			let rhs_type = type_expression(rhs)?;

			match op {
				BinOp::Add | BinOp::Subtract | BinOp::Multiply | BinOp::Divide => {
					if lhs_type == Type::Int && rhs_type == Type::Int {
						Ok(Type::Int)
					} else if lhs_type == Type::Float && rhs_type == Type::Float {
						Ok(Type::Float)
					} else {
						Err("Invalid types for arithmetic operation".to_string())
					}
				},
				BinOp::Equal | BinOp::NotEqual | BinOp::LessThan | BinOp::LessOrEqualThan | BinOp::GreaterThan | BinOp::GreaterOrEqualThan => {
					if lhs_type == rhs_type {
						Ok(Type::Bool)
					} else {
						Err("Invalid types for comparison operation".to_string())
					}
				},
				BinOp::LogicalAnd | BinOp::LogicalOr | BinOp::LogicalXor => {
					if lhs_type == Type::Bool && rhs_type == Type::Bool {
						Ok(Type::Bool)
					} else {
						Err("Invalid types for logical operation".to_string())
					}
				},
				BinOp::Modulus | BinOp::LeftShift | BinOp::RightShift |
				BinOp::BitwiseAnd | BinOp::BitwiseOr | BinOp::BitwiseXor => {
					if lhs_type == Type::Int && rhs_type == Type::Int {
						Ok(Type::Int)
					} else {
						Err("Invalid types for bitwise operation".to_string())
					}
				},
				BinOp::MemberAccess => {
					Err("Member access is not implemented".to_string())
				},
				BinOp::NotABinaryOp => {
					Err("Invalid binary operation".to_string())
				}
			}
		},
		Expression::UnaryOp(expr, op) => {
			let expr_type = type_expression(expr)?;

			match op {
				UnOp::UnaryMinus | UnOp::UnaryPlus | UnOp::BitwiseNot |
				UnOp::PreIncrement | UnOp::PreDecrement => {
					if expr_type == Type::Int {
						Ok(Type::Int)
					} else if expr_type == Type::Float {
						Ok(Type::Float)
					} else {
						Err("Invalid type for unary operation".to_string())
					}
				},
				UnOp::LogicalNot => {
					if expr_type == Type::Bool {
						Ok(Type::Bool)
					} else {
						Err("Invalid type for not operation".to_string())
					}
				},
				UnOp::Dereference => {
					if let Type::Pointer(ptr_type) = expr_type {
						Ok(*ptr_type)
					} else {
						Err("Invalid type for dereference operation".to_string())
					}
				},
				UnOp::AddressOf => {
					Ok(Type::Pointer(Box::new(expr_type)))
				},
				UnOp::NotAUnaryOp => {
					Err("Invalid unary operation".to_string())
				}
			}
		},
		Expression::Assignment(var, expr, op) => {
			let var_type = Type::Int;	// TODO: get variable type
			let expr_type = type_expression(expr)?;

			if var_type == expr_type {
				Ok(var_type)
			} else {
				Err("Invalid types for assignment".to_string())
			}
		},
	}
}

fn type_statement(statement: &Statement) -> Result<Type, String> {
	match statement{
		Statement::Expression(expr) => {
			return type_expression(expr);
		},
		Statement::Return(expr) => {
			return type_expression(expr);
		},
		Statement::If(condition, if_body, else_body) => {
			let condition_type = type_expression(condition)?;
			if condition_type != Type::Bool {
				return Err("Condition in if statement is not a boolean".to_string());
			}
			let if_body_type = type_statement(if_body)?;
			if let Some(else_body) = else_body {
				let else_body_type =  type_statement(else_body)?;
				if if_body_type != else_body_type {
					return Err("If and else branches have different types".to_string());
				}
			}
			return Ok(if_body_type);
		},
		Statement::While(condition, body) => {
			let condition_type = type_expression(condition)?;
			if condition_type != Type::Bool {
				return Err("Condition in while statement is not a boolean".to_string());
			}
			return type_statement(body);
		},
		Statement::For(init, condition, increment, body) => {
			type_expression(init)?;

			let condition_type = type_expression(condition)?;
			if condition_type != Type::Bool {
				return Err("Condition in for statement is not a boolean".to_string());
			}
			type_expression(increment)?;
			return type_statement(body);
		},
		Statement::Compound(statements) => {
			let mut last_type = Type::Void;
			for statement in statements {
				last_type = type_statement(statement)?;
			}
			return Ok(last_type);
		},
		Statement::Let(_, expr, var_type) => {
			let t = var_type.clone();
			if let Some(expr) = expr {
				let expr_type = type_expression(expr)?;
				if *var_type != expr_type {
					return Err("Variable type does not match expression type".to_string());
				}
			}
			return Ok(t);
		},
		Statement::Break => {
			return Ok(Type::Void)
		},
		Statement::Continue => {
			return Ok(Type::Void)
		},
		Statement::Loop(body) => {
			return type_statement(body);
		},
		Statement::Dowhile(expr, body) => {
			let expr_type = type_expression(expr)?;
			if expr_type != Type::Bool {
				return Err("Condition in do while statement is not a boolean".to_string());
			}
			return type_statement(body);
		},
	}
}

fn typechecking(ast: &Ast) -> Result<(), String> {
	let Program::Program(functions, _) = &ast.program;
	for function in functions {
		let Function::Function(name, params, body) = function;
		type_statement(body);
	}
	return Ok(());
}

pub fn check_program(ast: &Ast) -> Result<(), String> {

	// Check that there is a main function, no function is called _start and no
	// function is declared twice
	let mut main_found = false;
	let mut function_names = HashSet::new();

	let Program::Program(functions, _) = &ast.program;
	for function in functions {
		let Function::Function(name, ..) = function;
		if name == "main" {
			main_found = true;
		} else if name == "_start" {
			return Err("Function cannot be called '_start'".to_string());
		} else if !function_names.insert(name) {
			return Err(format!("A function was declared twice: {}", name));
		}
	}

	if !main_found {
		return Err("No main function found".to_string());
	}

	return typechecking(ast);
}