// TODO: Typechecking
// TODO: Check uninitialized variables aren't called
// TODO: Complain if a variable is declared but never used
// TODO: Check that the return type of a function matches the type of the return statement in every branch
// TODO: is there ambiguity between function and variable names?

use crate::ast_build::{
    AssignmentIdentifier, AssignmentOp, Ast, Atom, BinOp, Expression, Function, Literal, Program, Statement, Type, UnOp
};
use crate::ast_build::{Constant, ReassignmentIdentifier};
use core::panic;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
struct Context {
    variables: HashMap<String, Type>,
    functions: HashMap<String, Type>,
    structs: HashMap<String, (Vec<(String, Type)>, HashMap<String, Type>)>,
    enums: HashMap<String, Vec<String>>,
    array_dims: HashMap<String, Vec<Expression>>,
}

fn type_atom(expr: &Expression, atom: &Atom, context: &Context) -> (Expression, Type) {
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
			} else if let Some(_) = context.enums.get(v) {
				(expr.clone(), Type::Enum(v.clone()))
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
			(
				Expression::Atom(Atom::Array(new_expressions, *i)),
				Type::Array(Box::new(array_type.unwrap())),
			)
		}
		Atom::FunctionCall(_, _) => (expr.clone(), Type::Int), // TODO: function return type, check arguments
		Atom::Expression(expr) => {
			let sub_expr = type_expression(expr, context);
			(
				Expression::Atom(Atom::Expression(Box::new(sub_expr.0))),
				sub_expr.1,
			)
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
				(
					Expression::Atom(Atom::StructInstance(id.to_string(), new_exprs)),
					Type::Struct(id.clone()),
				)
			} else {
				panic!("Struct '{}' not in scope", id);
			}
		}
	}
}

fn type_binaryop(lhs: &Box<Expression>, rhs: &Box<Expression>, op:&BinOp, context: &Context) -> (Expression, Type) {
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
					let index = ordered_list
						.iter()
						.position(|(attr, _)| attr == attribute)
						.expect("Attribute not found in ordered list")
						as i64;
					return (
						Expression::BinaryOp(
							Box::new(lhs_expr),
							Box::new(Expression::Atom(Atom::Literal(Literal::Int(index)))),
							op.clone(),
						),
						t.clone(),
					);
				} else {
					panic!("Struct '{}' does not have member '{}'", lhs, rhs);
				}
			} else {
				panic!("Member access must be on a variable");
			}
		} else if let Type::Enum(e) = lhs_type.clone() {
			if let Expression::Atom(Atom::Variable(attribute)) = rhs.as_ref() {
				if let Some(variants) = context.enums.get(&e) {
					if variants.contains(attribute) {
						return (
							Expression::Atom(Atom::Literal(Literal::Int(
								variants.iter().position(|v| v == attribute).unwrap()
									as i64,
							))),
							Type::Enum(e),
						);
					} else {
						panic!("Enum '{}' does not have variant '{}'", e, attribute);
					}
				} else {
					panic!("Enum '{}' not in scope", e);
				}
			} else {
				panic!("Member access must be on a variable");
			}
		} else {
			panic!(
				"Type {} is not a struct or an enum, members access is undefined",
				lhs_type
			)
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
			panic!(
				"Unreachable code, member access is turned into array-like access earlier"
			)
		}
		BinOp::NotABinaryOp => {
			panic!("Invalid binary operation")
		}
	};
	(
		Expression::BinaryOp(Box::new(lhs_expr), Box::new(rhs_expr), op.clone()),
		binop_type,
	)
}

fn type_unaryop(expr: &Expression, op: &UnOp, context: &Context) -> (Expression, Type) {
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
	(
		Expression::UnaryOp(Box::new(new_expr), op.clone()),
		unop_type,
	)
}

fn type_assignment(var: &ReassignmentIdentifier, expr: &Box<Expression>, op: &AssignmentOp, context: &Context) -> (Expression, Type) {
	let (new_rhs, rhs_type) = type_expression(expr, context);
	let (new_lhs, lhs_type);
	match var {
		ReassignmentIdentifier::Variable(v) => {
			if let Some(var_type) = context.variables.get(v) {
				lhs_type = var_type.clone();
				new_lhs = ReassignmentIdentifier::Variable(v.clone());
			} else {
				panic!("Variable '{}' not in scope", v)
			}
		}
		ReassignmentIdentifier::Array(arr, idxs) => {
			let lhs_expr;
			(lhs_expr, lhs_type) = type_expression(
				&Expression::ArrayAccess(arr.clone(), idxs.clone()),
				context,
			);
			if let Expression::ArrayAccess(e1, e2) = lhs_expr {
				new_lhs = ReassignmentIdentifier::Array(e1, e2);
			} else {
				panic!("Unreachable code")
			}
		}
		ReassignmentIdentifier::Dereference(expr) => {
			let (new_expr, expr_type) = type_expression(expr, context);
			if let Type::Pointer(t) = expr_type {
				lhs_type = *t;
				new_lhs = ReassignmentIdentifier::Dereference(Box::new(new_expr));
			} else {
				panic!("Dereferencing a non-pointer type");
			}
		}
		ReassignmentIdentifier::MemberAccess(obj, mbr) => {
			let lhs_expr;
			(lhs_expr, lhs_type) = type_expression(
				&Expression::BinaryOp(
					Box::new(*obj.clone()),
					Box::new(*mbr.clone()),
					BinOp::MemberAccess,
				),
				context,
			);
			if let Expression::BinaryOp(e1, e2, BinOp::MemberAccess) = lhs_expr {
				new_lhs = ReassignmentIdentifier::Array(e1, vec![*e2]);
			} else {
				panic!("Unreachable code")
			}
		}
	}

	if lhs_type == rhs_type {
		(
			Expression::Assignment(new_lhs, Box::new(new_rhs), op.clone()),
			lhs_type,
		)
	} else {
		panic!("Assignment types do not match")
	}
}

fn type_arrayaccess(expr: &Box<Expression>, indices: &Vec<Expression>, context: &Context) -> (Expression, Type) {
	let (new_expr, expr_type) = type_expression(expr, context);

	if let Type::Array(element_type) = expr_type {
		let mut new_indices = Vec::new();
		for index in indices {
			let (new_index, index_type) = type_expression(index, context);
			if index_type != Type::Int {
				panic!("Array index is not an integer");
			}
			new_indices.push(new_index);
		}

		// if array i has dimensions [dim1, dim2, dim3, ...] and we want to access element (i, j, k, ...)
		// ((i * dim1 + j) * dim2 + k) * dim3 + ...

		let new_index;

		if indices.len() > 1 {
			let array_name = if let Expression::Atom(Atom::Variable(ref name)) = new_expr {
				name.clone()
			} else {
				panic!("Multidimensional array access must be on a variable");
			};
			let dims = context
				.array_dims
				.get(&array_name)
				.expect("Array dimensions not found");
			new_index = dims.iter().zip(new_indices.iter()).fold(
				Expression::Atom(Atom::Literal(Literal::Int(0))),
				|acc, (dim, idx)| {
					Expression::BinaryOp(
						Box::new(Expression::BinaryOp(
							Box::new(acc),
							Box::new(dim.clone()),
							BinOp::Multiply,
						)),
						Box::new(idx.clone()),
						BinOp::Add,
					)
				},
			);
		} else {
			new_index = new_indices[0].clone();
		}

		(
			Expression::ArrayAccess(Box::new(new_expr), vec![new_index]),
			element_type.as_ref().clone(),
		)
	} else {
		panic!("Array access on non-array type");
	}
}

fn check_if_struct_or_enum(context: &Context, t: &Type) -> Type {
	match t {
		Type::Struct(id) => {
			if let Some(_) = context.structs.get(id) {
				t.clone()
			} else if let Some(_) = context.enums.get(id) {
				Type::Enum(id.clone())
			} else {
				panic!("Type '{}' not in scope", id);
			}
		}
		Type::Enum(id) => {
			if let Some(_) = context.structs.get(id) {
				Type::Struct(id.clone())
			} else if let Some(_) = context.enums.get(id) {
				t.clone()
			} else {
				panic!("Type '{}' not in scope", id);
			}
		}
		Type::Array(t) => Type::Array(Box::new(check_if_struct_or_enum(context, t))),
		Type::Pointer(t) => Type::Pointer(Box::new(check_if_struct_or_enum(context, t))),
		Type::Function(ret, args) => {
			let new_args = args.iter().map(|t| check_if_struct_or_enum(context, t)).collect();
			Type::Function(Box::new(check_if_struct_or_enum(context, ret)), new_args)
		}
 		Type::Int | Type::Float | Type::Char | Type::Void => t.clone(),
	}
}

fn type_expression(expr: &Expression, context: &Context) -> (Expression, Type) {
    match expr {
        Expression::Atom(atom) => type_atom(expr, atom, context),
        Expression::BinaryOp(lhs, rhs, op) => type_binaryop(lhs, rhs, op, context),
        Expression::UnaryOp(expr, op) => type_unaryop(expr, op, context),
        Expression::Assignment(var, expr, op) => type_assignment(var, expr, op, context),
        Expression::TypeCast(expr, t) => {
            let (new_expr, _) = type_expression(expr, context);
			(new_expr, check_if_struct_or_enum(context, t))
        }
        Expression::ArrayAccess(expr, indices) => type_arrayaccess(expr, indices, context),
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
                return (
                    Statement::If(
                        new_condition,
                        Box::new(new_if_body),
                        Some(Box::new(new_else_body)),
                    ),
                    if_body_type,
                );
            }
            return (
                Statement::If(new_condition, Box::new(new_if_body), None),
                if_body_type,
            );
        }
        Statement::While(condition, body) => {
            let (new_condition, condition_type) = type_expression(condition, context);
            if condition_type != Type::Int {
                panic!("Condition in while statement is not a boolean");
            }
            let (new_body, body_type) = type_statement(body, context);
            return (
                Statement::While(new_condition, Box::new(new_body)),
                body_type,
            );
        }
        Statement::For(init, condition, increment, body) => {
            let (new_init, _) = type_expression(init, context);

            let (new_condition, condition_type) = type_expression(condition, context);
            if condition_type != Type::Int {
                panic!("Condition in for statement is not a boolean");
            }
            let (new_increment, _) = type_expression(increment, context);
            let (new_body, body_type) = type_statement(body, context);
            return (
                Statement::For(new_init, new_condition, new_increment, Box::new(new_body)),
                body_type,
            );
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
					let var_type = &check_if_struct_or_enum(context, var_type);
                    let mut flag = true;
                    let mut name = name;

                    while flag {
                        if let AssignmentIdentifier::Dereference(inner) = name {
                            name = inner;
                        } else {
                            flag = false;
                        }
                    }

                    if let AssignmentIdentifier::Array(var_name, dimensions) = name {
                        new_context
                            .variables
                            .insert(var_name.to_string(), var_type.clone());
                        new_context
                            .array_dims
                            .insert(var_name.to_string(), dimensions.clone());
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
			let var_type = &check_if_struct_or_enum(context, var_type);
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

				let var_type = &check_if_struct_or_enum(context, var_type);

                if *var_type != expr_type {
                    panic!(
                        "Variable type ({}) does not match expression type ({})",
                        var_type, expr_type
                    );
                }
                return (
                    Statement::Let(original_id, Some(new_expr), var_type.clone()),
                    t,
                );
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
		Statement::Asm(_) => return (statement.clone(), Type::Void),
    }
}

fn typechecking(ast: &Ast) -> Ast {
    let Program::Program(functions, constants, structs, enums) = &ast.program;
    let mut context = Context {
        variables: HashMap::new(),
        functions: HashMap::new(),
        structs: HashMap::new(),
        enums: HashMap::new(),
        array_dims: HashMap::new(),
    };

    for enum_ in enums {
        let crate::ast_build::Enum { id, variants } = enum_;

        if context.enums.contains_key(id) {
            panic!("Enum '{}' is declared more than once", id);
        }

        let mut variant_set = HashSet::new();
        let mut variant_list = Vec::new();
        for variant in variants {
            if !variant_set.insert(variant.clone()) {
                panic!("Enum '{}' has duplicate variant '{}'", id, variant);
            }
            variant_list.push(variant.clone());
        }
        context.enums.insert(id.clone(), variant_list);
    }

    for constant in constants {
        let Constant::Constant(name, lit, var_type) = constant;

        if context.variables.contains_key(name) {
            panic!("Constant '{}' is declared more than once", name);
        }

        let (_, expr_type) =
            type_expression(&Expression::Atom(Atom::Literal(lit.clone())), &context);
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
        } else if context.enums.contains_key(id) {
            panic!("Struct '{}' has the same name as an enum", id);
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
			let t = check_if_struct_or_enum(&context, &t);
            unordered_lookup_table.insert(name.clone(), t.clone());
            ordered_members.push((name.clone(), t));
        }
        context
            .structs
            .insert(id.clone(), (ordered_members, unordered_lookup_table));
    }

    for function in functions {
        // Functions are defined everywhere
        let Function::Function(name, _, _, return_type) = function;
        context.functions.insert(name.clone(), check_if_struct_or_enum(&context, return_type));
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
        new_functions.push(Function::Function(
            name.clone(),
            params.clone(),
            new_body,
            return_type.clone(),
        ));
    }

    let new_ast = Ast {
        program: Program::Program(
            new_functions,
            constants.clone(),
            structs.clone(),
            enums.clone(),
        ),
    };
    return new_ast;
}

pub fn check_program(ast: &Ast) -> Ast {
    // Check that there is a main function, no function is called _start and no
    // function is declared twice
    let mut main_found = false;
    let mut function_names = HashSet::new();

    let Program::Program(functions, _, _, _) = &ast.program;
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
