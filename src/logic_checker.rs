// TODO: Typechecking
// TODO: Check uninitialized variables aren't called
// TODO: Complain if a variable is declared but never used
// TODO: Check that the return type of a function matches the type of the return statement in every branch
// TODO: is there ambiguity between function and variable names?

use crate::ast_build::{
    AssignmentIdentifier, AssignmentOp, Ast, Atom, BinOp, Expression, Function, Literal, Program,
    Statement, Type, Typed, UnOp, Enum, Struct
};
use crate::ast_build::{Constant, ReassignmentIdentifier};
use core::panic;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
struct Context {
    variables: HashMap<String, Type>,
    functions: HashMap<String, (Type, Vec<Type>)>,
    structs: HashMap<String, (Vec<(String, Type)>, HashMap<String, Type>)>,
    enums: HashMap<String, Vec<String>>,
    array_dims: HashMap<String, Vec<Expression>>,
}

fn type_atom(expr: &Expression, atom: &Atom, context: &Context) -> Typed<Expression> {
    match atom {
		Atom::Literal(Typed {
			expr: Literal::Bool(_),
			..
		}) => Typed {
			expr: expr.clone(),
			type_: Type::Bool,
		},
        Atom::Literal(Typed {
            expr: Literal::Int(_),
            ..
        }) => Typed {
            expr: expr.clone(),
            type_: Type::Int,
        },
        Atom::Literal(Typed {
            expr: Literal::Float(_),
            ..
        }) => Typed {
            expr: expr.clone(),
            type_: Type::Float,
        },
        Atom::Literal(Typed {
            expr: Literal::Char(_),
            ..
        }) => Typed {
            expr: expr.clone(),
            type_: Type::Char,
        },
        Atom::Literal(Typed {
            expr: Literal::Hex(_),
            ..
        }) => Typed {
            expr: expr.clone(),
            type_: Type::Int,
        },
        Atom::Literal(Typed {
            expr: Literal::Binary(_),
            ..
        }) => Typed {
            expr: expr.clone(),
            type_: Type::Int,
        },
        Atom::Variable(v) => {
            // Check that the variable is in scope
            if let Some(var_type) = context.variables.get(v) {
                Typed {
                    expr: Expression::Atom(Typed::new_with_type(Atom::Variable(v.clone()), var_type.clone())),
                    type_: var_type.clone(),
                }
            } else if let Some(_) = context.enums.get(v) {
                Typed {
                    expr: Expression::Atom(Typed::new_with_type(Atom::Variable(v.clone()), Type::Enum(v.clone()))),
                    type_: Type::Enum(v.clone()),
                }
            } else {
                panic!("Variable '{}' not in scope", v)
            }
        }
        Atom::Array(expressions, i) => {
            // Check all expressions are of the same type
            let mut array_type = None;
            let mut new_expressions = Vec::new();
            for Typed { expr, .. } in expressions {
                let Typed {
                    expr: new_expr,
                    type_: expr_type,
                } = type_expression(expr, context);
                new_expressions.push(Typed {
                    expr: new_expr,
                    type_: expr_type.clone(),
                });
                if let Some(t) = &array_type {
                    if *t != expr_type {
                        panic!("Array elements are not of the same type");
                    }
                } else {
                    array_type = Some(expr_type);
                }
            }
            Typed {
                expr: Expression::Atom(Typed {
                    expr: Atom::Array(new_expressions, *i),
                    type_: Type::Array(Box::new(array_type.clone().unwrap())),
                }),
                type_: Type::Array(Box::new(array_type.unwrap())),
            }
        }
        Atom::FunctionCall(name, args) => {
            if let Some((function_type, args_types)) = context.functions.get(name) {
                let mut new_args = Vec::new();
                
                if args.len() != args_types.len() {
                    panic!("Function '{}' expects {} arguments, but {} were provided", name, args_types.len(), args.len());
                }
                
                for (i, (arg, expected_type)) in args.iter().zip(args_types.iter()).enumerate() {
                    let Typed {
                        expr: new_arg,
                        type_: arg_type,
                    } = type_expression(&arg.expr, context);
                    
                    if arg_type != *expected_type {
                        panic!("Argument {} of function '{}' has type '{}', but '{}' was expected", i, name, arg_type, expected_type);
                    }
                    
                    new_args.push(Typed {
                        expr: new_arg,
                        type_: arg_type.clone(),
                    });
                }
                
                Typed {
                    expr: Expression::Atom(Typed {
                        expr: Atom::FunctionCall(name.clone(), new_args),
                        type_: function_type.clone(),
                    }),
                    type_: function_type.clone(),
                }
            } else {
                panic!("Function '{}' not in scope", name)
            }
        }
        Atom::Expression(expr) => type_expression(&expr.expr, context),
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
                    let Typed {
                        expr: new_expr,
                        type_: expr_type,
                    } = type_expression(&expr.expr, context);
                    new_exprs.push(Typed {
                        expr: new_expr,
                        type_: expr_type.clone(),
                    });
                    if *t != expr_type {
                        panic!(
                            "Struct member type '{}' does not match expression type '{}'",
                            t, expr_type
                        );
                    }
                }
                Typed {
                    expr: Expression::Atom(Typed {
                        expr: Atom::StructInstance(id.to_string(), new_exprs),
                        type_: Type::Struct(id.clone()),
                    }),
                    type_: Type::Struct(id.clone()),
                }
            } else {
                panic!("Struct '{}' not in scope", id);
            }
        }
    }
}

fn type_binaryop(
    lhs: &Expression,
    rhs: &Expression,
    op: &BinOp,
    context: &Context,
) -> Typed<Expression> {
    let Typed {
        expr: lhs_expr,
        type_: lhs_type,
    } = type_expression(lhs, context);

    // Treat member access separately from the other operators, otherwise
    // we try to typecheck both sides, but the rhs isn't defined as a
    // separate variable so it crashes
    if let BinOp::MemberAccess = op {
        if let Type::Struct(s) = lhs_type.clone() {
            if let Expression::Atom(Typed {
                expr: Atom::Variable(attribute),
                ..
            }) = rhs
            {
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
                    return Typed {
                        expr: Expression::BinaryOp(
                            Box::new(Typed {
                                expr: lhs_expr,
                                type_: lhs_type,
                            }),
                            Box::new(Typed {
                                expr: Expression::Atom(Typed {
                                    expr: Atom::Literal(Typed {
                                        expr: Literal::Int(index),
                                        type_: Type::Int,
                                    }),
                                    type_: Type::Int,
                                }),
                                type_: Type::Int,
                            }),
                            op.clone(),
                        ),
                        type_: t.clone(),
                    };
                } else {
                    panic!("Struct '{}' does not have member '{}'", lhs, rhs);
                }
            } else {
                panic!("Member access must be on a variable");
            }
        } else if let Type::Enum(e) = lhs_type.clone() {
            if let Expression::Atom(Typed {
                expr: Atom::Variable(attribute),
                ..
            }) = rhs
            {
                if let Some(variants) = context.enums.get(&e) {
                    if variants.contains(attribute) {
                        return Typed {
                            expr: Expression::Atom(Typed {
                                expr: Atom::Literal(Typed {
                                    expr: Literal::Int(
                                        variants.iter().position(|v| v == attribute).unwrap()
                                            as i64,
                                    ),
                                    type_: Type::Int,
                                }),
                                type_: Type::Int,
                            }),
                            type_: Type::Enum(e),
                        };
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

    let Typed {
        expr: rhs_expr,
        type_: rhs_type,
    } = type_expression(rhs, context);

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
        | BinOp::NotEqual => {
            if lhs_type == rhs_type {
                Type::Bool
            } else {
                panic!("Invalid types for comparison operation")
            }
        }
        BinOp::LessThan
        | BinOp::LessOrEqualThan
        | BinOp::GreaterThan
        | BinOp::GreaterOrEqualThan => {
            if lhs_type == rhs_type && (lhs_type == Type::Int || lhs_type == Type::Float) {
                Type::Bool
            } else {
                panic!("Invalid types for comparison operation")
            }
        }
        BinOp::LogicalAnd | BinOp::LogicalOr | BinOp::LogicalXor => {
            if lhs_type == Type::Bool && rhs_type == Type::Bool {
                Type::Bool
            } else {
                panic!("Invalid types for logical operation")
            }
        }
        BinOp::LeftShift
        | BinOp::RightShift
        | BinOp::BitwiseAnd
        | BinOp::BitwiseOr
        | BinOp::BitwiseXor
		| BinOp::Modulus => {
            if lhs_type == Type::Int && rhs_type == Type::Int {
                Type::Int
            } else {
                panic!("Invalid types for bitwise operation")
            }
        }
        BinOp::MemberAccess => {
            panic!("Unreachable code, member access is turned into array-like access earlier")
        }
        BinOp::NotABinaryOp => {
            panic!("Invalid binary operation")
        }
    };
    Typed {
        expr: Expression::BinaryOp(
            Box::new(Typed {
                expr: lhs_expr,
                type_: lhs_type,
            }),
            Box::new(Typed {
                expr: rhs_expr,
                type_: rhs_type,
            }),
            op.clone(),
        ),
        type_: binop_type,
    }
}

fn type_unaryop(expr: &Expression, op: &UnOp, context: &Context) -> Typed<Expression> {
    let Typed {
        expr: new_expr,
        type_: expr_type,
    } = type_expression(expr, context);
    let expr_type_clone = expr_type.clone();

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
            if expr_type == Type::Bool {
                Type::Bool
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
    Typed {
        expr: Expression::UnaryOp(
            Box::new(Typed {
                expr: new_expr,
                type_: expr_type_clone,
            }),
            op.clone(),
        ),
        type_: unop_type,
    }
}

fn type_assignment(
    var: &ReassignmentIdentifier,
    expr: &Expression,
    op: &AssignmentOp,
    context: &Context,
) -> Typed<Expression> {
    let Typed {
        expr: new_rhs,
        type_: rhs_type,
    } = type_expression(expr, context);

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
            Typed {
                expr: lhs_expr,
                type_: lhs_type,
            } = type_expression(&Expression::ArrayAccess(arr.clone(), idxs.clone()), context);
            if let Expression::ArrayAccess(e1, e2) = lhs_expr {
                new_lhs = ReassignmentIdentifier::Array(e1, e2);
            } else {
                panic!("Unreachable code")
            }
        }
        ReassignmentIdentifier::Dereference(expr) => {
            let Typed {
                expr: new_expr,
                type_: expr_type,
            } = type_expression(&expr.expr, context);
            if let Type::Pointer(t) = expr_type.clone() {
                lhs_type = *t;
                new_lhs = ReassignmentIdentifier::Dereference(Box::new(Typed {
                    expr: new_expr,
                    type_: expr_type,
                }));
            } else {
                panic!("Dereferencing a non-pointer type");
            }
        }
        ReassignmentIdentifier::MemberAccess(obj, mbr) => {
            let lhs_expr;
            Typed {
                expr: lhs_expr,
                type_: lhs_type,
            } = type_expression(
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

	if lhs_type != rhs_type {
		panic!("Assignment types do not match")
	}

	match op {
		AssignmentOp::Assign => {
			// Do nothing
		}
		AssignmentOp::AddAssign
		| AssignmentOp::SubtractAssign
		| AssignmentOp::MultiplyAssign
		| AssignmentOp::DivideAssign => {
			if lhs_type == Type::Int && rhs_type == Type::Int {
				// Do nothing
			} else if lhs_type == Type::Float && rhs_type == Type::Float {
				// Do nothing
			} else {
				panic!("This assignment operation is only supported for integers and floats")
			}
		}
		AssignmentOp::BitwiseAndAssign
		| AssignmentOp::BitwiseOrAssign
		| AssignmentOp::BitwiseXorAssign
		| AssignmentOp::LeftShiftAssign
		| AssignmentOp::RightShiftAssign
		| AssignmentOp::ModulusAssign => {
			if lhs_type == Type::Int && rhs_type == Type::Int {
				// Do nothing
			} else {
				panic!("This assignment operation is only supported for integers")
			}
		}
		AssignmentOp::NotAnAssignmentOp => {
			panic!("Invalid assignment operation")
		}
	}

	Typed {
		expr: Expression::Assignment(
			Typed {
				expr: new_lhs,
				type_: lhs_type.clone(),
			},
			Box::new(Typed {
				expr: new_rhs,
				type_: rhs_type,
			}),
			op.clone(),
		),
		type_: lhs_type,
	}
}

fn type_arrayaccess(
    expr: &Expression,
    indices: &Vec<Typed<Expression>>,
    context: &Context,
) -> Typed<Expression> {
    let Typed {
        expr: new_expr,
        type_: expr_type,
    } = type_expression(expr, context);

    if let Type::Array(element_type) = expr_type {
        let mut new_indices = Vec::new();
        for index in indices {
            let Typed {
                expr: new_index,
                type_: index_type,
            } = type_expression(&index.expr, context);
            if index_type != Type::Int {
                panic!("Array index is not an integer");
            }
            new_indices.push(new_index);
        }

        // if array i has dimensions [dim1, dim2, dim3, ...] and we want to access element (i, j, k, ...)
        // ((i * dim1 + j) * dim2 + k) * dim3 + ...

        let new_index;

        if indices.len() > 1 {
            let array_name = if let Expression::Atom(Typed {
                expr: Atom::Variable(ref name),
                ..
            }) = new_expr
            {
                name.clone()
            } else {
                panic!("Multidimensional array access must be on a variable");
            };
            let dims = context
                .array_dims
                .get(&array_name)
                .expect("Array dimensions not found");
            new_index = dims.iter().zip(new_indices.iter()).fold(
                Expression::Atom(Typed {
                    expr: Atom::Literal(Typed {
                        expr: Literal::Int(0),
                        type_: Type::Int,
                    }),
                    type_: Type::Int,
                }),
                |acc, (dim, idx)| {
                    Expression::BinaryOp(
                        Box::new(Typed {
                            expr: Expression::BinaryOp(
                                Box::new(Typed {
                                    expr: acc,
                                    type_: Type::Int,
                                }),
                                Box::new(Typed {
                                    expr: dim.clone(),
                                    type_: Type::Int,
                                }),
                                BinOp::Multiply,
                            ),
                            type_: Type::Int,
                        }),
                        Box::new(Typed {
                            expr: idx.clone(),
                            type_: Type::Int,
                        }),
                        BinOp::Add,
                    )
                },
            );
        } else {
            new_index = new_indices[0].clone();
        }

        Typed {
            expr: Expression::ArrayAccess(
                Box::new(Typed {
                    expr: new_expr,
                    type_: Type::Int,
                }),
                vec![Typed {
                    expr: new_index,
                    type_: Type::Int,
                }],
            ),
            type_: element_type.as_ref().clone(),
        }
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
            let new_args = args
                .iter()
                .map(|t| check_if_struct_or_enum(context, t))
                .collect();
            Type::Function(Box::new(check_if_struct_or_enum(context, ret)), new_args)
        }
        Type::Bool | Type::Int | Type::Float | Type::Char | Type::Void => t.clone(),
    }
}

fn type_expression(expr: &Expression, context: &Context) -> Typed<Expression> {
    match expr {
        Expression::Atom(atom) => type_atom(expr, &atom.expr, context),
        Expression::BinaryOp(lhs, rhs, op) => type_binaryop(&lhs.expr, &rhs.expr, op, context),
        Expression::UnaryOp(expr, op) => type_unaryop(&expr.expr, op, context),
        Expression::Assignment(var, expr, op) => {
            type_assignment(&var.expr, &expr.expr, op, context)
        }
        Expression::TypeCast(expr, t) => {
            let Typed { expr: new_expr, .. } = type_expression(&expr.expr, context);
            Typed {
                expr: new_expr,
                type_: check_if_struct_or_enum(context, t),
            }
        }
        Expression::ArrayAccess(expr, indices) => type_arrayaccess(&expr.expr, indices, context),
    }
}

fn type_statement(statement: &Statement, context: &Context) -> Typed<Statement> {
    match statement {
        Statement::Expression(expr) => {
            let Typed {
                expr: new_expr,
                type_: expr_type,
            } = type_expression(&expr.expr, context);
            return Typed {
                expr: Statement::Expression(Typed {
                    expr: new_expr,
                    type_: expr_type.clone(),
                }),
                type_: expr_type,
            };
        }
        Statement::Return(expr) => {
            let Typed {
                expr: new_expr,
                type_: expr_type,
            } = type_expression(&expr.expr, context);
            return Typed {
                expr: Statement::Return(Typed {
                    expr: new_expr,
                    type_: expr_type.clone(),
                }),
                type_: expr_type,
            };
        }
        Statement::If(condition, if_body, else_body) => {
            let Typed {
                expr: new_condition,
                type_: condition_type,
            } = type_expression(&condition.expr, context);
            if condition_type != Type::Bool {
                panic!("Condition in if statement is not a boolean");
            }
            let Typed {
                expr: new_if_body,
                type_: if_body_type,
            } = type_statement(&if_body.expr, context);
            if let Some(else_body) = else_body {
                let Typed {
                    expr: new_else_body,
                    type_: else_body_type,
                } = type_statement(&else_body.expr, context);
                if if_body_type != else_body_type {
                    panic!("If and else branches have different types");
                }
                return Typed {
                    expr: Statement::If(
                        Typed {
                            expr: new_condition,
                            type_: condition_type,
                        },
                        Box::new(Typed {
                            expr: new_if_body,
                            type_: if_body_type.clone(),
                        }),
                        Some(Box::new(Typed {
                            expr: new_else_body,
                            type_: else_body_type,
                        })),
                    ),
                    type_: if_body_type,
                };
            }
            return Typed {
                expr: Statement::If(
                    Typed {
                        expr: new_condition,
                        type_: condition_type,
                    },
                    Box::new(Typed {
                        expr: new_if_body,
                        type_: if_body_type.clone(),
                    }),
                    None,
                ),
                type_: if_body_type,
            };
        }
        Statement::While(condition, body) => {
            let Typed {
                expr: new_condition,
                type_: condition_type,
            } = type_expression(&condition.expr, context);
            if condition_type != Type::Bool {
                panic!("Condition in while statement is not a boolean");
            }
            let Typed {
                expr: new_body,
                type_: body_type,
            } = type_statement(&body.expr, context);
            return Typed {
                expr: Statement::While(
                    Typed {
                        expr: new_condition,
                        type_: condition_type,
                    },
                    Box::new(Typed {
                        expr: new_body,
                        type_: body_type.clone(),
                    }),
                ),
                type_: body_type,
            };
        }
        Statement::For(init, condition, increment, body) => {
            let Typed {
                expr: new_init,
                type_: init_type,
            } = type_expression(&init.expr, context);

            let Typed {
                expr: new_condition,
                type_: condition_type,
            } = type_expression(&condition.expr, context);
            if condition_type != Type::Bool {
                panic!("Condition in for statement is not a boolean");
            }
            let Typed {
                expr: new_increment,
                type_: increment_type,
            } = type_expression(&increment.expr, context);
            let Typed {
                expr: new_body,
                type_: body_type,
            } = type_statement(&body.expr, context);
            return Typed {
                expr: Statement::For(
                    Typed {
                        expr: new_init,
                        type_: init_type,
                    },
                    Typed {
                        expr: new_condition,
                        type_: condition_type,
                    },
                    Typed {
                        expr: new_increment,
                        type_: increment_type,
                    },
                    Box::new(Typed {
                        expr: new_body,
                        type_: body_type.clone(),
                    }),
                ),
                type_: body_type,
            };
        }
        Statement::Compound(statements) => {
            let mut new_context = context.clone();
            let mut last_type = Type::Void;
            let mut new_statements = Vec::new();
            for statement in statements {
                let Typed {
                    expr: new_statement,
                    type_: statement_type,
                } = type_statement(&statement.expr, &new_context);
                new_statements.push(Typed {
                    expr: new_statement,
                    type_: statement_type.clone(),
                });
                if statement_type != Type::Void {
                    last_type = statement_type // The type of the compound statement is the type of the last non void statement it contains
                }
                if let Typed {
                    expr: Statement::Let(name, _, var_type),
                    ..
                } = statement
                {
                    let var_type = &check_if_struct_or_enum(context, var_type);
                    let mut flag = true;
                    let mut name = name;

                    while flag {
                        if let AssignmentIdentifier::Dereference(inner) = name {
                            name = &inner.expr;
                        } else {
                            flag = false;
                        }
                    }

                    if let AssignmentIdentifier::Array(var_name, dimensions) = name {
                        new_context
                            .variables
                            .insert(var_name.to_string(), var_type.clone());

                        // Extract the inner Expression from each Typed<Expression>
                        let unwrapped_dimensions: Vec<Expression> = dimensions
                            .iter()
                            .map(|typed_expr| typed_expr.expr.clone())
                            .collect();

                        new_context
                            .array_dims
                            .insert(var_name.to_string(), unwrapped_dimensions);
                    } else {
                        new_context
                            .variables
                            .insert(name.to_string(), var_type.clone());
                    }
                }
            }
            return Typed {
                expr: Statement::Compound(new_statements),
                type_: last_type,
            };
        }
        Statement::Let(id, expr, var_type) => {
            let var_type = &check_if_struct_or_enum(context, var_type);
            let t = var_type.clone();
            if let Some(expr) = expr {
                let Typed {
                    expr: new_expr,
                    type_: expr_type,
                } = type_expression(&expr.expr, context);
                let mut id = id;
                let mut var_type = var_type;

                let mut flag = true;
                let original_id = id.clone();
                while flag {
                    if let AssignmentIdentifier::Dereference(inner) = id {
                        if let Type::Pointer(t) = var_type {
                            var_type = t;
                            id = &inner.expr;
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
                return Typed {
                    expr: Statement::Let(
                        original_id,
                        Some(Typed {
                            expr: new_expr,
                            type_: expr_type,
                        }),
                        var_type.clone(),
                    ),
                    type_: t,
                };
            }
            return Typed {
                expr: Statement::Let(id.clone(), None, var_type.clone()),
                type_: t,
            };
        }
        Statement::Break => {
            return Typed {
                expr: statement.clone(),
                type_: Type::Void,
            }
        }
        Statement::Continue => {
            return Typed {
                expr: statement.clone(),
                type_: Type::Void,
            }
        }
        Statement::Loop(body) => {
            let Typed {
                expr: new_body,
                type_: body_type,
            } = type_statement(&body.expr, context);
            return Typed {
                expr: Statement::Loop(Box::new(Typed {
                    expr: new_body,
                    type_: body_type.clone(),
                })),
                type_: body_type,
            };
        }
        Statement::Dowhile(condition, body) => {
            let Typed {
                expr: new_expr,
                type_: expr_type,
            } = type_expression(&condition.expr, context);
            if expr_type != Type::Bool {
                panic!("Condition in do while statement is not a boolean");
            }
            let Typed {
                expr: new_body,
                type_: body_type,
            } = type_statement(&body.expr, context);
            return Typed {
                expr: Statement::Dowhile(
                    Typed {
                        expr: new_expr,
                        type_: expr_type,
                    },
                    Box::new(Typed {
                        expr: new_body,
                        type_: body_type.clone(),
                    }),
                ),
                type_: body_type,
            };
        }
        Statement::Asm(_, type_) => {
            return Typed {
                expr: statement.clone(),
                type_: type_.clone(),
            }
        }
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


	// Functions, structs and enums are defined everywhere, even before declaration
	// so we need to add them to the context before typechecking them

    for function in functions {
        // Functions are defined everywhere
        let Function::Function(name, args, _, return_type) = &function.expr;
        // Extract only the types from args, discarding the parameter names
        let arg_types: Vec<Type> = args.iter().map(|(_, t)| t.clone()).collect();
        context.functions.insert(
            name.clone(),
            (check_if_struct_or_enum(&context, &return_type), arg_types),
        );
    }

	for enum_ in enums {
		let Enum {id, variants} = enum_;

		if context.enums.contains_key(id) {
            panic!("Enum '{}' is declared more than once", id);
        }
	
		context.enums.insert(id.clone(), variants.clone());
	}

	for struct_ in structs {
		let Struct {id, members} = struct_;

		if context.structs.contains_key(id) {
            panic!("Struct '{}' is declared more than once", id);
        } else if context.enums.contains_key(id) {
            panic!("Struct '{}' has the same name as an enum", id);
        }

		context.structs.insert(id.clone(), (members.clone(), HashMap::new()));
	}

	// Now we can do proper typechecking

    for enum_ in enums {
        let Enum { id, variants } = enum_;

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

    let mut typed_constants = Vec::new();
    for constant in constants {
        let Constant::Constant(name, lit, var_type) = &constant.expr;

        if context.variables.contains_key(name) {
            panic!("Constant '{}' is declared more than once", name);
        }

        let Typed {
            type_: expr_type, ..
        } = type_expression(
            &Expression::Atom(Typed::new(Atom::Literal(lit.clone()))),
            &context,
        );
        if *var_type != expr_type {
            panic!("Constant type does not match expression type");
        } else {
            context.variables.insert(name.clone(), var_type.clone());
        }

        typed_constants.push(Typed {
            expr: Constant::Constant(name.clone(), lit.clone(), var_type.clone()),
            type_: var_type.clone(),
        });
    }

    for struct_ in structs {
        let Struct { id, members } = struct_;

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

    let mut new_functions = Vec::new();

    for function in functions {
        let Function::Function(name, params, body, return_type) = &function.expr;
        let mut local_context = context.clone();
        // Function parameters are only in scope in the function
        for (param_name, param_type) in params.clone() {
            local_context
                .variables
                .insert(param_name.clone(), param_type.clone());
        }
        let Typed {
            expr: new_body,
            type_: body_type,
        } = type_statement(&body.expr, &local_context);
        if body_type != *return_type {
            panic!(
                "Function '{}' return type ({}) does not match body type ({})",
                name, return_type, body_type
            );
        }
        new_functions.push(Typed {
            expr: Function::Function(
                name.clone(),
                params.clone(),
                Typed {
                    expr: new_body,
                    type_: body_type,
                },
                return_type.clone(),
            ),
            type_: return_type.clone(),
        });
    }

    let new_ast = Ast {
        program: Program::Program(
            new_functions,
            typed_constants,
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
        let Function::Function(name, _, _, ret_type) = &function.expr;
        if name == "main" {
            main_found = true;
            if *ret_type != Type::Int {
                panic!("Main function must return an integer");
            }
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
