// TODO: Typechecking
// TODO: Check uninitialized variables aren't called
// TODO: Complain if a variable is declared but never used
// TODO: Check that the return type of a function matches the type of the return statement in every branch
// TODO: is there ambiguity between function and variable names?

use crate::ast_build::{AssignmentIdentifier, Ast, Atom, BinOp, Expression, Function, Literal, Program, Statement, Type, UnOp};
use std::{collections::{HashMap, HashSet}, mem::transmute};
use crate::ast_build::Constant;

#[derive(Debug, Clone)]
struct Context {
	variables: HashMap<String, Type>,
	functions: HashMap<String, Type>,
}

fn type_expression(expr: &Expression, context: &Context) -> Type {
	match expr {
		Expression::Atom(atom) => {
			match atom {
				Atom::Literal(Literal::Int(_)) => Type::Int,
				Atom::Literal(Literal::Float(_)) => Type::Float,
				Atom::Literal(Literal::Char(_)) => Type::Char,
				Atom::Literal(Literal::Hex(_)) => Type::Int,
				Atom::Literal(Literal::Binary(_)) => Type::Int,
				Atom::Literal(Literal::String(_)) => Type::Array(Box::new(Type::Char)),
				Atom::Variable(v) => {
					// Check that the variable is in scope
					if let Some(var_type) = context.variables.get(v) {
						var_type.clone()
					} else {
						panic!("Variable '{}' not in scope", v)
					}
				},	// TODO: variable of different types
				Atom::Array(expressions, _) => {
					// Check all expressions are of the same type
					let mut array_type = None;
					for expr in expressions {
						let expr_type = type_expression(expr, context);
						if let Some(t) = &array_type {
							if *t != expr_type {
								panic!("Array elements are not of the same type");
							}
						} else {
							array_type = Some(expr_type);
						}
					}
					Type::Array(Box::new(array_type.unwrap()))
				},
				Atom::FunctionCall(_, _) => Type::Int,	// TODO: function return type
				Atom::ArrayAccess(_, _) => Type::Int,	// TODO: array type
				Atom::Expression(expr) => type_expression(expr, context),
			}
		},
		Expression::BinaryOp(lhs, rhs, op) => {
			let lhs_type = type_expression(lhs, context);
			let rhs_type = type_expression(rhs, context);

			match op {
				BinOp::Add | BinOp::Subtract | BinOp::Multiply | BinOp::Divide => {
					if lhs_type == Type::Int && rhs_type == Type::Int {
						Type::Int
					} else if lhs_type == Type::Float && rhs_type == Type::Float {
						Type::Float
					} else {
						panic!("Invalid types for arithmetic operation");
					}
				},
				BinOp::Equal | BinOp::NotEqual | BinOp::LessThan | BinOp::LessOrEqualThan | BinOp::GreaterThan | BinOp::GreaterOrEqualThan => {
					if lhs_type == rhs_type {
						Type::Int
					} else {
						panic!("Invalid types for comparison operation")
					}
				},
				BinOp::LogicalAnd | BinOp::LogicalOr | BinOp::LogicalXor => {
					if lhs_type == Type::Int && rhs_type == Type::Int {
						Type::Int
					} else {
						panic!("Invalid types for logical operation")
					}
				},
				BinOp::Modulus | BinOp::LeftShift | BinOp::RightShift |
				BinOp::BitwiseAnd | BinOp::BitwiseOr | BinOp::BitwiseXor => {
					if lhs_type == Type::Int && rhs_type == Type::Int {
						Type::Int
					} else {
						panic!("Invalid types for bitwise operation")
					}
				},
				BinOp::MemberAccess => {
					panic!("Member access is not implemented") // TODO: implement
				},
				BinOp::NotABinaryOp => {
					panic!("Invalid binary operation")
				}
			}
		},
		Expression::UnaryOp(expr, op) => {
			let expr_type = type_expression(expr, context);

			match op {
				UnOp::UnaryMinus | UnOp::UnaryPlus | UnOp::BitwiseNot |
				UnOp::PreIncrement | UnOp::PreDecrement => {
					if expr_type == Type::Int {
						Type::Int
					} else if expr_type == Type::Float {
						Type::Float
					} else {
						panic!("Invalid type for unary operation")
					}
				},
				UnOp::LogicalNot => {
					if expr_type == Type::Int {
						Type::Int
					} else {
						panic!("Invalid type for not operation")
					}
				},
				UnOp::Dereference => {
					if let Type::Pointer(ptr_type) = expr_type {
						*ptr_type
					} else {
						panic!("Invalid type for dereference operation")
					}
				},
				UnOp::AddressOf => {
					Type::Pointer(Box::new(expr_type))
				},
				UnOp::NotAUnaryOp => {
					panic!("Invalid unary operation")
				}
			}
		},
		Expression::Assignment(var, expr, op) => {
			match var {
				AssignmentIdentifier::Variable(v) => {
					if let Some(var_type) = context.variables.get(v){
						let expr_type = type_expression(expr, context);

						if *var_type == expr_type {
							expr_type
						} else {
							panic!("Invalid types for assignment")
						}
					}
					else {
						panic!("Variable '{}' not in scope", v)
					}
				},
				AssignmentIdentifier::Array(_, _) => {
					panic!("Array assignment is not implemented") // TODO: implement
				},
				AssignmentIdentifier::Dereference(_) => {
					panic!("Dereference assignment is not implemented")
				},
			}
		},
	}
}

fn type_statement(statement: &Statement, context: &Context) -> Type {
	match statement{
		Statement::Expression(expr) => {
			return type_expression(expr, context);
		},
		Statement::Return(expr) => {
			return type_expression(expr, context);
		},
		Statement::If(condition, if_body, else_body) => {
			let condition_type = type_expression(condition, context);
			if condition_type != Type::Int {
				panic!("Condition in if statement is not a boolean");
			}
			let if_body_type = type_statement(if_body, context);
			if let Some(else_body) = else_body {
				let else_body_type =  type_statement(else_body, context);
				if if_body_type != else_body_type {
					panic!("If and else branches have different types");
				}
			}
			return if_body_type;
		},
		Statement::While(condition, body) => {
			let condition_type = type_expression(condition, context);
			if condition_type != Type::Int {
				panic!("Condition in while statement is not a boolean");
			}
			return type_statement(body, context);
		},
		Statement::For(init, condition, increment, body) => {
			type_expression(init, context);

			let condition_type = type_expression(condition, context);
			if condition_type != Type::Int {
				panic!("Condition in for statement is not a boolean");
			}
			type_expression(increment, context);
			return type_statement(body, context);
		},
		Statement::Compound(statements) => {
			let mut new_context = context.clone();
			let mut last_type = Type::Void;
			for statement in statements {
				last_type = type_statement(statement, &new_context);
				if let Statement::Let(name, _, var_type) = statement {

					let mut flag = true;
					let mut name = name;
					
					while flag {
						if let AssignmentIdentifier::Dereference(inner) = name {
							name = inner;
						} else {
							flag = false;
						}
					}

					new_context.variables.insert(name.to_string(), var_type.clone());
				}
			}
			return last_type;
		},
		Statement::Let(id, expr, var_type) => {
			let t = var_type.clone();
			if let Some(expr) = expr {
				let expr_type = type_expression(expr, context);
				let mut id = id;
				let mut var_type = var_type;

				let mut flag = true;

				while flag {
					if let AssignmentIdentifier::Dereference(inner) = id {
						if let Type::Pointer(t) = var_type {
							var_type = t;
							id = inner;
						} else {
							panic!("Dereferencing a non-pointer type");
						}
					} else {
						flag = false;
					}
				}

				if *var_type != expr_type {
					panic!("Variable type does not match expression type");
				}
			}
			return t;
		},
		Statement::Break => {
			return Type::Void
		},
		Statement::Continue => {
			return Type::Void
		},
		Statement::Loop(body) => {
			return type_statement(body, context);
		},
		Statement::Dowhile(expr, body) => {
			let expr_type = type_expression(expr, context);
			if expr_type != Type::Int {
				panic!("Condition in do while statement is not a boolean");
			}
			return type_statement(body, context);
		},
	}
}

fn typechecking(ast: &Ast) {
	let Program::Program(functions, constants) = &ast.program;
	let mut context = Context { 
		variables: HashMap::new(),
		functions: HashMap::new()
	};

	for constant in constants {
		let Constant::Constant(name, lit, var_type) = constant;
		let expr_type = type_expression(&Expression::Atom(Atom::Literal(lit.clone())), &context);
		if *var_type != expr_type {
			panic!("Constant type does not match expression type");
		} else {
			context.variables.insert(name.clone(), var_type.clone());
		}
	}

	for function in functions {
		let Function::Function(name, params, body, return_type) = function;
		context.functions.insert(name.clone(), return_type.clone());
		for (param_name, param_type) in params {
			context.variables.insert(param_name.clone(), param_type.clone());
		}
		let body_type = type_statement(body, &context);
		if body_type != *return_type {
			panic!("Function '{}' return type ({}) does not match body type ({})", name, return_type, body_type);
		}
	}
	return;
}

pub fn check_program(ast: &Ast) {

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
			panic!("Function cannot be called '_start'");
		} else if !function_names.insert(name) {
			panic!("A function was declared twice: {}", name);
		}
	}

	if !main_found {
		panic!("No main function found");
	}

	typechecking(ast);
}