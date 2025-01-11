// TODO: Typechecking
// TODO: Check uninitialized variables aren't called
// TODO: Complain if a variable is declared but never used
// TODO: Check that the return type of a function matches the type of the return statement in every branch
// TODO: is there ambiguity between function and variable names?

use crate::ast_build::Constant;
use crate::ast_build::{
    AssignmentIdentifier, Ast, Atom, BinOp, Expression, Function, Literal, Program, Statement,
    Type, UnOp,
};
use core::net;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
struct Context {
    variables: HashMap<String, Type>,
    functions: HashMap<String, Type>,
    structs: HashMap<String, (Vec<(String, Type)>, HashMap<String, Type>)>,
}

fn type_expression(expr: &Expression, context: &Context) -> (Expression, Type) {
    match expr {
        Expression::Atom(atom) => {
            match atom {
                Atom::Literal(Literal::Int(_)) => (expr.clone(), Type::Int),
                Atom::Literal(Literal::Float(_)) => (expr.clone(), Type::Float),
                Atom::Literal(Literal::Char(_)) => (expr.clone(), Type::Char),
                Atom::Literal(Literal::Hex(_)) => (expr.clone(), Type::Int),
                Atom::Literal(Literal::Binary(_)) => (expr.clone(), Type::Int),
                Atom::Variable(v) => {
                    // Check that the variable is in scope
                    if let Some(var_type) = context.variables.get(v) {
                        (expr.clone(), var_type.clone())
                    } else {
                        panic!("Variable '{}' not in scope", v)
                    }
                }
                Atom::Array(expressions, i) => {
                    // Check all expressions are of the same type
                    let mut array_type = None;
					let mut new_expressions = Vec::new();
                    for expr in expressions {
                        let (new_expr, expr_type) = type_expression(expr, context);
						new_expressions.push(new_expr);
                        if let Some(t) = &array_type {
                            if *t != expr_type {
                                panic!("Array elements are not of the same type");
                            }
                        } else {
                            array_type = Some(expr_type);
                        }
                    }
                    (Expression::Atom(Atom::Array(new_expressions, *i)), Type::Array(Box::new(array_type.unwrap())))
                }
                Atom::FunctionCall(_, _) => (expr.clone(), Type::Int), // TODO: function return type, check arguments
                Atom::ArrayAccess(_, _) => (expr.clone(), Type::Int),  // TODO: array type
                Atom::Expression(expr) => {
					let sub_expr = type_expression(expr, context);
					(Expression::Atom(Atom::Expression(Box::new(sub_expr.0))), sub_expr.1)
				}
                Atom::StructInstance(id, exprs) => {
                    if let Some((ordered_list, _)) = context.structs.get(id) {
                        if exprs.len() != ordered_list.len() {
                            panic!(
                                "Struct '{}' has {} members, but {} were provided",
                                id,
                                ordered_list.len(),
                                exprs.len()
                            );
                        }

						let mut new_exprs = Vec::new();
                        for (expr, (_, t)) in exprs.iter().zip(ordered_list.iter()) {
                            let (new_expr, expr_type) = type_expression(expr, context);
							new_exprs.push(new_expr);
                            if *t != expr_type {
                                panic!(
                                    "Struct member type '{}' does not match expression type '{}'",
                                    t, expr_type
                                );
                            }
                        }
                        (Expression::Atom(Atom::StructInstance(id.to_string(), new_exprs)), Type::Struct(id.clone()))
                    } else {
                        panic!("Struct '{}' not in scope", id);
                    }
                }
            }
        }
        Expression::BinaryOp(lhs, rhs, op) => {
            let (lhs_expr, lhs_type) = type_expression(lhs, context);

			// Treat member access separately from the other operators, otherwise
            // we try to typecheck both sides, but the rhs isn't defined as a
            // separate variable so it crashes
			if let BinOp::MemberAccess = op {
				if let Type::Struct(s) = lhs_type.clone() {
					if let Expression::Atom(Atom::Variable(attribute)) = rhs.as_ref() {
						let (ordered_list, unordered_list) = context
							.structs
							.get(&s)
							.expect(format!("Undefined struct {}", lhs_type).as_str());

						if let Some(t) = unordered_list.get(attribute) {
							// Slow but whatever. Don't make structs with a bajillion members,
							// or if you do change this to a hashmap
							let index = ordered_list.iter().position(|(attr, _)| attr == attribute).expect("Attribute not found in ordered list") as i64;
							return (Expression::BinaryOp(Box::new(lhs_expr), Box::new(Expression::Atom(Atom::Literal(Literal::Int(index)))), op.clone()), t.clone());
						} else {
							panic!("Struct '{}' does not have member '{}'", lhs, rhs);
						}
					} else {
						panic!("Member access must be on a variable");
					}
				} else {
					panic!("Type {} is not a struct, members access is undefined", lhs_type)
				}
			}

            let (rhs_expr, rhs_type) = type_expression(rhs, context);

            let binop_type = match op {
                BinOp::Add | BinOp::Subtract | BinOp::Multiply | BinOp::Divide => {
                    if lhs_type == Type::Int && rhs_type == Type::Int {
                        Type::Int
                    } else if lhs_type == Type::Float && rhs_type == Type::Float {
                        Type::Float
                    } else {
                        panic!("Invalid types for arithmetic operation");
                    }
                }
                BinOp::Equal
                | BinOp::NotEqual
                | BinOp::LessThan
                | BinOp::LessOrEqualThan
                | BinOp::GreaterThan
                | BinOp::GreaterOrEqualThan => {
                    if lhs_type == rhs_type {
                        Type::Int
                    } else {
                        panic!("Invalid types for comparison operation")
                    }
                }
                BinOp::LogicalAnd | BinOp::LogicalOr | BinOp::LogicalXor => {
                    if lhs_type == Type::Int && rhs_type == Type::Int {
                        Type::Int
                    } else {
                        panic!("Invalid types for logical operation")
                    }
                }
                BinOp::Modulus
                | BinOp::LeftShift
                | BinOp::RightShift
                | BinOp::BitwiseAnd
                | BinOp::BitwiseOr
                | BinOp::BitwiseXor => {
                    if lhs_type == Type::Int && rhs_type == Type::Int {
                        Type::Int
                    } else {
                        panic!("Invalid types for bitwise operation")
                    }
                }
                BinOp::MemberAccess => {
                    panic!("Unreachable code")
                }
                BinOp::NotABinaryOp => {
                    panic!("Invalid binary operation")
                }
            };
			(Expression::BinaryOp(Box::new(lhs_expr), Box::new(rhs_expr), op.clone()), binop_type)
        }
        Expression::UnaryOp(expr, op) => {
            let (new_expr, expr_type) = type_expression(expr, context);

            let unop_type = match op {
                UnOp::UnaryMinus
                | UnOp::UnaryPlus
                | UnOp::BitwiseNot
                | UnOp::PreIncrement
                | UnOp::PreDecrement => {
                    if expr_type == Type::Int {
                        Type::Int
                    } else if expr_type == Type::Float {
                        Type::Float
                    } else {
                        panic!("Invalid type for unary operation")
                    }
                }
                UnOp::LogicalNot => {
                    if expr_type == Type::Int {
                        Type::Int
                    } else {
                        panic!("Invalid type for not operation")
                    }
                }
                UnOp::Dereference => {
                    if let Type::Pointer(ptr_type) = expr_type {
                        *ptr_type
                    } else {
                        panic!("Invalid type for dereference operation")
                    }
                }
                UnOp::AddressOf => Type::Pointer(Box::new(expr_type)),
                UnOp::NotAUnaryOp => {
                    panic!("Invalid unary operation")
                }
            };
			(Expression::UnaryOp(Box::new(new_expr), op.clone()), unop_type)
        }
        Expression::Assignment(var, expr, op) => {
            match var {
                AssignmentIdentifier::Variable(v) => {
                    if let Some(var_type) = context.variables.get(v) {
                        let (new_expr, expr_type) = type_expression(expr, context);

                        if *var_type == expr_type {
                            (Expression::Assignment(var.clone(), Box::new(new_expr), op.clone()), expr_type)
                        } else {
                            panic!("Invalid types for assignment")
                        }
                    } else {
                        panic!("Variable '{}' not in scope", v)
                    }
                }
                AssignmentIdentifier::Array(_, _) => {
                    panic!("Array assignment is not implemented") // TODO
                }
                AssignmentIdentifier::Dereference(_) => {
                    panic!("Dereference assignment is not implemented") // TODO
                }
            }
        }
        Expression::TypeCast(expr, t) => {
            let (new_expr, _) = type_expression(expr, context);
            (new_expr, t.clone())
        }
    }
}

fn type_statement(statement: &Statement, context: &Context) -> (Statement, Type) {
    match statement {
        Statement::Expression(expr) => {
			let (new_expr, expr_type) = type_expression(expr, context);
            return (Statement::Expression(new_expr), expr_type);
        }
        Statement::Return(expr) => {
			let (new_expr, expr_type) = type_expression(expr, context);
            return (Statement::Return(new_expr), expr_type);
        }
        Statement::If(condition, if_body, else_body) => {
            let (new_condition, condition_type) = type_expression(condition, context);
            if condition_type != Type::Int {
                panic!("Condition in if statement is not a boolean");
            }
            let (new_if_body, if_body_type) = type_statement(if_body, context);
            if let Some(else_body) = else_body {
                let (new_else_body, else_body_type) = type_statement(else_body, context);
                if if_body_type != else_body_type {
                    panic!("If and else branches have different types");
                }
				return (Statement::If(new_condition, Box::new(new_if_body), Some(Box::new(new_else_body))), if_body_type);
            }
            return (Statement::If(new_condition, Box::new(new_if_body), None), if_body_type);
        }
        Statement::While(condition, body) => {
            let (new_condition, condition_type) = type_expression(condition, context);
            if condition_type != Type::Int {
                panic!("Condition in while statement is not a boolean");
            }
			let (new_body, body_type) = type_statement(body, context);
            return (Statement::While(new_condition, Box::new(new_body)), body_type);
        }
        Statement::For(init, condition, increment, body) => {
            let (new_init, _) = type_expression(init, context);

            let (new_condition, condition_type) = type_expression(condition, context);
            if condition_type != Type::Int {
                panic!("Condition in for statement is not a boolean");
            }
            let (new_increment, _) = type_expression(increment, context);
			let (new_body, body_type) = type_statement(body, context);
			return (Statement::For(new_init, new_condition, new_increment, Box::new(new_body)), body_type);
        }
        Statement::Compound(statements) => {
            let mut new_context = context.clone();
            let mut last_type = Type::Void;
			let mut new_statements = Vec::new();
            for statement in statements {
                let (new_statement, statement_type) = type_statement(statement, &new_context);
				new_statements.push(new_statement);
				last_type = statement_type;
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

					if let AssignmentIdentifier::Array(var_name, _) = name {
						new_context
                        .variables
                        .insert(var_name.to_string(), var_type.clone());
					} else {
						new_context
                        .variables
                        .insert(name.to_string(), var_type.clone());
					}
                }
            }
			return (Statement::Compound(new_statements), last_type);
        }
        Statement::Let(id, expr, var_type) => {
            let t = var_type.clone();
            if let Some(expr) = expr {
                let (new_expr, expr_type) = type_expression(expr, context);
                let mut id = id;
                let mut var_type = var_type;

                let mut flag = true;
				let original_id = id.clone();
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
                    panic!(
                        "Variable type ({}) does not match expression type ({})",
                        var_type, expr_type
                    );
                }
				return (Statement::Let(original_id, Some(new_expr), var_type.clone()), t);
            }
            return (Statement::Let(id.clone(), None, var_type.clone()), t);
        }
        Statement::Break => return (statement.clone(), Type::Void),
        Statement::Continue => return (statement.clone(), Type::Void),
        Statement::Loop(body) => {
			let (new_body, body_type) = type_statement(body, context);
			return (Statement::Loop(Box::new(new_body)), body_type);
        }
        Statement::Dowhile(expr, body) => {
            let (new_expr, expr_type) = type_expression(expr, context);
            if expr_type != Type::Int {
                panic!("Condition in do while statement is not a boolean");
            }
			let (new_body, body_type) = type_statement(body, context);
            return (Statement::Dowhile(new_expr, Box::new(new_body)), body_type);
        }
    }
}

fn typechecking(ast: &Ast) -> Ast {
    let Program::Program(functions, constants, structs) = &ast.program;
    let mut context = Context {
        variables: HashMap::new(),
        functions: HashMap::new(),
        structs: HashMap::new(),
    };

    for constant in constants {
        let Constant::Constant(name, lit, var_type) = constant;

        if context.variables.contains_key(name) {
            panic!("Constant '{}' is declared more than once", name);
        }

        let (_, expr_type) = type_expression(&Expression::Atom(Atom::Literal(lit.clone())), &context);
        if *var_type != expr_type {
            panic!("Constant type does not match expression type");
        } else {
            context.variables.insert(name.clone(), var_type.clone());
        }
    }

    for struct_ in structs {
        let crate::ast_build::Struct { id, members } = struct_;

        if context.structs.contains_key(id) {
            panic!("Struct '{}' is declared more than once", id);
        }

		let mut member_names = HashSet::new();
		for (name, _) in members {
			if !member_names.insert(name) {
			panic!("Struct '{}' has duplicate member '{}'", id, name);
			}
		}

        let mut unordered_lookup_table = HashMap::new();
        let mut ordered_members = Vec::new();
        for (name, t) in members {
            unordered_lookup_table.insert(name.clone(), t.clone());
            ordered_members.push((name.clone(), t.clone()));
        }
        context
            .structs
            .insert(id.clone(), (ordered_members, unordered_lookup_table));
    }

    for function in functions {
        // Functions are defined everywhere
        let Function::Function(name, _, _, return_type) = function;
        context.functions.insert(name.clone(), return_type.clone());
    }

	let mut new_functions = Vec::new();

    for function in functions {
        let Function::Function(name, params, body, return_type) = function;
        let mut local_context = context.clone();
        // Function parameters are only in scope in the function
        for (param_name, param_type) in params {
            local_context
                .variables
                .insert(param_name.clone(), param_type.clone());
        }
        let (new_body, body_type) = type_statement(&body, &local_context);
        if body_type != *return_type {
            panic!(
                "Function '{}' return type ({}) does not match body type ({})",
                name, return_type, body_type
            );
        }
		new_functions.push(Function::Function(name.clone(), params.clone(), new_body, return_type.clone()));
    }

	let new_ast = Ast{ program: Program::Program(new_functions, constants.clone(), structs.clone()) };
	return new_ast;
}

pub fn check_program(ast: &Ast) -> Ast {
    // Check that there is a main function, no function is called _start and no
    // function is declared twice
    let mut main_found = false;
    let mut function_names = HashSet::new();

    let Program::Program(functions, _, _) = &ast.program;
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

    return typechecking(ast);
}
