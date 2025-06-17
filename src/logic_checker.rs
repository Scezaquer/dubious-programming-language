// TODO: Check uninitialized variables aren't called
// TODO: Complain if a variable is declared but never used
// TODO: Check that the return type of a function matches the type of the return statement in every branch
// TODO: is there ambiguity between function and variable names?

use crate::ast_build::{
    AssignmentIdentifier, AssignmentOp, Ast, Atom, BinOp, Enum, Expression, Function, Literal,
    Namespace, Statement, Struct, Type, Typed, UnOp,
};
use crate::ast_build::{Constant, ReassignmentIdentifier};
use core::panic;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
struct Context {
    namespace: String,
	namespace_root: String,
    variables: HashMap<String, Type>,
    concrete_functions: HashMap<String, (Type, Vec<Type>)>, // Hashmap<id, (return_type, Vec<arg_types>)>
    generic_functions: HashMap<String, Function>, // Hashmap<id, (return_type, Vec<arg_types>, Vec<generic>)>
    concrete_structs: HashMap<String, (Vec<(String, Type)>, HashMap<String, Type>)>, // Hashmap<id, (ordered_list_of_members, unordered_list_of_members)>
    generic_structs: HashMap<String, (Vec<(String, Type)>, HashMap<String, Type>, Vec<String>)>, // Hashmap<id, (ordered_list_of_members, unordered_list_of_members, Vec<generic>)>
    enums: HashMap<String, Vec<String>>, // Hashmap<id, Vec<variants>>
    array_dims: HashMap<String, Vec<Expression>>,
    generics_bindings: HashMap<String, Type>,
}

fn type_atom(
    fn_array: &mut Vec<Typed<Function>>,
    expr: &Expression,
    atom: &Atom,
    context: &mut Context,
) -> Typed<Expression> {
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
                    expr: Expression::Atom(Typed::new_with_type(
                        Atom::Variable(v.clone()),
                        var_type.clone(),
                    )),
                    type_: var_type.clone(),
                }
            } else if let Some(var_type) = context
                .variables
                .get(&format!("{}{}", context.namespace, v))
            {
                Typed {
                    expr: Expression::Atom(Typed::new_with_type(
                        Atom::Variable(format!("{}{}", context.namespace, v)),
                        var_type.clone(),
                    )),
                    type_: var_type.clone(),
                }
            } else if let Some(_) = context.enums.get(&format!("{}{}", context.namespace, v)) {
                Typed {
                    expr: Expression::Atom(Typed::new_with_type(
                        Atom::Variable(format!("{}{}", context.namespace, v)),
                        Type::Enum(format!("{}{}", context.namespace, v)),
                    )),
                    type_: Type::Enum(format!("{}{}", context.namespace, v)),
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
                } = type_expression(fn_array, expr, context);
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
        Atom::FunctionCall(name, args, generics_bindings) => {
            let mut name = if name.contains("::") | name.eq("toplevel") {
                name.clone()
            } else {
                format!("{}{}", context.namespace, name)
            };

            // Every time a generic function is called, we need to check if it has been
            // instantiated with the same generics before. If it has, we can use the
            // concrete function instead of creating a new one.
            // If it hasn't, we need to create a new concrete function and add it to the
            // concrete_functions map.
            if let Some(func) = context.generic_functions.clone().get(&name) {
                if func.generics.len() != generics_bindings.len() {
                    panic!(
                        "Function '{}' expects {} generics, but {} were provided",
                        name,
                        func.generics.len(),
                        generics_bindings.len()
                    );
                }

                let generics_bindings = generics_bindings
                    .iter()
                    .map(|binding| check_if_struct_or_enum(context, binding))
                    .collect::<Vec<_>>();

                // name = toplevel::namespace::fn_id..binding1..binding2
                name = format!(
                    "{}{}",
                    name,
                    generics_bindings
                        .iter()
                        .map(|binding| {
                            let binding_str = format!("{}", binding);
                            let sanitized = binding_str
                                .chars()
                                .map(|c| if c.is_alphanumeric() { c } else { '.' })
                                .collect::<String>();
                            format!("..{}", sanitized)
                        })
                        .collect::<String>()
                );

                if !context.concrete_functions.contains_key(&name) {
                    let mut local_context = context.clone();
                    local_context.generics_bindings = HashMap::new();
                    for (generic, binding) in func.generics.iter().zip(generics_bindings.iter()) {
                        local_context
                            .generics_bindings
                            .insert(generic.clone(), check_if_struct_or_enum(context, binding));
                    }
                    local_context.concrete_functions.insert(
                        name.to_string(),
                        (
                            func.return_type.clone(),
                            func.args.iter().map(|(_, t)| t.clone()).collect(),
                        ),
                    );

                    let func = Function {
                        id: name.to_string(),
                        return_type: func.return_type.clone(),
                        args: func.args.clone(),
                        generics: vec![],
                        body: func.body.clone(),
                    };

                    let binded_function =
                        type_function(fn_array, func.clone(), &mut local_context, "".to_string());
                    fn_array.push(binded_function.clone());
                    context.concrete_functions = local_context.concrete_functions;
					context.concrete_functions.insert(name.clone(), (
						binded_function.expr.return_type.clone(),
						binded_function.expr.args.iter().map(|(_, t)| t.clone()).collect(),
					));
                }
            }

            if let Some((return_type, args_types)) = context.concrete_functions.clone().get(&name) {
                let mut new_args = Vec::new();

                if args.len() != args_types.len() {
                    panic!(
                        "Function '{}' expects {} arguments, but {} were provided",
                        name,
                        args_types.len(),
                        args.len()
                    );
                }

				let mut tmp_ctx = context.clone();
				tmp_ctx.namespace = context.namespace_root.clone();
                for (i, (arg, expected_type)) in args.iter().zip(args_types.iter()).enumerate() {
					let Typed {
                        expr: new_arg,
                        type_: arg_type,
                    } = type_expression(fn_array, &arg.expr, &mut tmp_ctx);

                    if arg_type != *expected_type {
                        panic!(
                            "Argument {} of function '{}' has type '{}', but '{}' was expected",
                            i, name, arg_type, expected_type
                        );
                    }

                    new_args.push(Typed {
                        expr: new_arg,
                        type_: arg_type.clone(),
                    });
                }

                let return_type = check_if_struct_or_enum(context, return_type);
                Typed {
                    expr: Expression::Atom(Typed {
                        expr: Atom::FunctionCall(name.to_string(), new_args, vec![]),
                        type_: return_type.clone(),
                    }),
                    type_: return_type.clone(),
                }
            } else {
                panic!("Function '{}' not in scope", name);
            }
        }
        Atom::Expression(expr) => type_expression(fn_array, &expr.expr, context),
        Atom::StructInstance(id, exprs, generics_bindings) => {
            // id = toplevel::namespace::struct_id..binding1..binding2
			let id = if id.contains("::") | id.eq("toplevel") {
                id
            } else {
                &format!("{}{}", context.namespace, id)
            };

			let generics_bindings = generics_bindings.iter()
				.map(|binding| check_if_struct_or_enum(context, binding))
				.collect::<Vec<_>>();

			// Add bindings to struct id
			let id = format!(
				"{}{}",
				id,
				generics_bindings
					.iter()
					.map(|binding| {
						let binding_str = format!("{}", binding);
						let sanitized = binding_str
							.chars()
							.map(|c| if c.is_alphanumeric() { c } else { '.' })
							.collect::<String>();
						format!("..{}", sanitized)
					})
					.collect::<String>()
			);

            if let Some((ordered_list, _)) = context.concrete_structs.clone().get(&id) {
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
                    } = type_expression(fn_array, &expr.expr, context);
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
                        expr: Atom::StructInstance(id.clone(), new_exprs, vec![]),
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

fn type_member_access(
    lhs: &Expression,
    rhs: &Expression,
    op: &BinOp,
    context: &Context,
    lhs_expr: Expression,
    lhs_type: Type,
) -> Typed<Expression> {
    // Treat member access separately from the other operators, otherwise
    // we try to typecheck both sides, but the rhs isn't defined as a
    // separate variable so it crashes
    if let Type::Struct(s) = lhs_type.clone() {
        if let Expression::Atom(Typed {
            expr: Atom::Variable(attribute),
            ..
        }) = rhs
        {
            let s = if s.contains("::") | s.eq("toplevel") {
                s
            } else {
                format!("{}{}", context.namespace, s)
            };
            let (ordered_list, unordered_list) = context
                .concrete_structs
                .get(&s)
                .expect(format!("Undefined struct {}", lhs_type).as_str());

			// Slow but whatever. Don't make structs with a bajillion members,
			// or if you do change this to a hashmap

			let index;
			let t;
			if attribute.eq("len") { // reserved member name for struct length
				index = -1;
				t = Type::Int;
			} else {
				if let Some(t_) = unordered_list.get(attribute) {
					t = t_.clone();
				} else {
					panic!("Struct '{}' does not have member '{}'", lhs, rhs);
				}
				index = ordered_list
					.iter()
					.position(|(attr, _)| attr == attribute)
					.expect("Attribute not found in ordered list")
					as i64;
			}

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
				type_: t,
			};
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
                                    variants.iter().position(|v| v == attribute).unwrap() as i64,
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

fn unroll_namespace_access(lhs: &Expression, context: &Context) -> String {
    // The namespace operator is left-associative, so we need a bit of machinery
    // to unroll it. This turns
    // (((a b ::) c ::) d ::)
    // into
    // "a::b::c::d"

    match lhs {
        Expression::Atom(Typed {
            expr: Atom::Variable(namespace),
            ..
        }) => {
            let namespace_path = if namespace.contains("::") {
                namespace.to_string()
            } else if namespace.eq("toplevel") {
                format!("{}::", namespace.to_string())
            } else {
                format!("{}{}::", context.namespace, namespace)
            };
            namespace_path
        }
        Expression::BinaryOp(l, r, BinOp::NamespaceAccess) => {
            if let Typed {
                expr:
                    Expression::Atom(Typed {
                        expr: Atom::Variable(id),
                        ..
                    }),
                ..
            } = &**r
            {
                let namespace_path = unroll_namespace_access(&l.expr, context);
                format!("{}{}::", namespace_path, id)
            } else {
                panic!("Namespace access must be on a variable");
            }
        }
        _ => {
            panic!("Namespace access must be on a variable");
        }
    }
}

fn type_namespace_access(
    fn_array: &mut Vec<Typed<Function>>,
    lhs: &Expression,
    rhs: &Expression,
    context: &mut Context,
) -> Typed<Expression> {
    let namespace_path = unroll_namespace_access(lhs, context);
    let mut local_context = context.clone();
    local_context.namespace = namespace_path;

    let typed_expr = type_expression(fn_array, rhs, &mut local_context);

    context.concrete_functions = local_context.concrete_functions;
    return typed_expr;
}

fn type_binaryop(
    fn_array: &mut Vec<Typed<Function>>,
    lhs: &Expression,
    rhs: &Expression,
    op: &BinOp,
    context: &mut Context,
) -> Typed<Expression> {
    if let BinOp::NamespaceAccess = op {
        return type_namespace_access(fn_array, lhs, rhs, context);
    }

    let Typed {
        expr: lhs_expr,
        type_: lhs_type,
    } = type_expression(fn_array, lhs, context);

    if let BinOp::MemberAccess = op {
        return type_member_access(lhs, rhs, op, context, lhs_expr, lhs_type);
    }

    let Typed {
        expr: rhs_expr,
        type_: rhs_type,
    } = type_expression(fn_array, rhs, context);

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
        BinOp::Equal | BinOp::NotEqual => {
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
            unreachable!()
        }
        BinOp::NamespaceAccess => {
            unreachable!()
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

fn type_unaryop(
    fn_array: &mut Vec<Typed<Function>>,
    expr: &Expression,
    op: &UnOp,
    context: &mut Context,
) -> Typed<Expression> {
    let Typed {
        expr: new_expr,
        type_: expr_type,
    } = type_expression(fn_array, expr, context);
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
    fn_array: &mut Vec<Typed<Function>>,
    var: &ReassignmentIdentifier,
    expr: &Expression,
    op: &AssignmentOp,
    context: &mut Context,
) -> Typed<Expression> {
    let Typed {
        expr: new_rhs,
        type_: rhs_type,
    } = type_expression(fn_array, expr, context);

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
            } = type_expression(
                fn_array,
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
            let Typed {
                expr: new_expr,
                type_: expr_type,
            } = type_expression(fn_array, &expr.expr, context);
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
                fn_array,
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
    fn_array: &mut Vec<Typed<Function>>,
    expr: &Expression,
    indices: &Vec<Typed<Expression>>,
    context: &mut Context,
) -> Typed<Expression> {
    let Typed {
        expr: new_expr,
        type_: expr_type,
    } = type_expression(fn_array, expr, context);

    if let Type::Array(element_type) = expr_type {
        let mut new_indices = Vec::new();
        for index in indices {
            let Typed {
                expr: new_index,
                type_: index_type,
            } = type_expression(fn_array, &index.expr, context);
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

fn check_if_struct_or_enum(context: &mut Context, t: &Type) -> Type {
    match t {
        Type::Struct(id) => {
            if context.generics_bindings.contains_key(id) {
                return context.generics_bindings.get(id).unwrap().clone();
            }
            let id = if id.contains("::") | id.eq("toplevel") {
                id
            } else {
                &format!("{}{}", context.namespace, id)
            };
            if let Some(_) = context.concrete_structs.get(id) {
                Type::Struct(id.clone())
            } else if let Some(_) = context.enums.get(id) {
                Type::Enum(id.clone())
            } else {
                panic!("Type '{}' not in scope", id);
            }
        }
        Type::Enum(id) => {
            let id = if id.contains("::") | id.eq("toplevel") {
                id
            } else {
                &format!("{}{}", context.namespace, id)
            };
            if let Some(_) = context.concrete_structs.get(id) {
                Type::Struct(id.clone())
            } else if let Some(_) = context.enums.get(id) {
                Type::Enum(id.clone())
            } else {
                panic!("Type '{}' not in scope", id);
            }
        }
        Type::Array(t) => Type::Array(Box::new(check_if_struct_or_enum(context, t))),
        Type::Pointer(t) => Type::Pointer(Box::new(check_if_struct_or_enum(context, t))),
        Type::Bool | Type::Int | Type::Float | Type::Char | Type::Void => t.clone(),
        Type::Namespace(id, t) => match &**t {
            Type::Struct(sub_id) => {
                let id = if id.starts_with("toplevel") {
                    format!("{}::{}", id, sub_id)
                } else {
                    format!("{}{}::{}", context.namespace, id, sub_id)
                };
                check_if_struct_or_enum(context, &Type::Struct(id))
            }
            Type::Enum(sub_id) => {
                let id = if id.starts_with("toplevel") {
                    format!("{}::{}", id, sub_id)
                } else {
                    format!("{}{}::{}", context.namespace, id, sub_id)
                };
                check_if_struct_or_enum(context, &Type::Enum(id))
            }
            Type::Namespace(sub_id, sub_t) => {
                let id = if id.starts_with("toplevel") {
                    format!("{}::{}", id, sub_id)
                } else {
                    format!("{}{}::{}", context.namespace, id, sub_id)
                };
                check_if_struct_or_enum(context, &Type::Namespace(id, sub_t.clone()))
            }
            _ => panic!(
                "Type '{}' cannot be namespaced (only struct or enums may be)",
                id
            ),
        },
        Type::GenericBinding(id, bindings) => {
            // let i: S<T1, T2, ...> = ...
            // First check that S is a generic struct, then that the number of
            // generic bindings match and that the bindings are valid

            let mut id = if id.contains("::") | id.eq("toplevel") {
                id.clone()
            } else {
                format!("{}{}", context.namespace, id)
            };

            if let Some((ordered_list, _, generics)) = context.generic_structs.clone().get(&id)
            {
                if generics.len() != bindings.len() {
                    panic!(
                        "Generic struct '{}' expects {} generics, but {} were provided",
                        id,
                        generics.len(),
                        bindings.len()
                    );
                }
                let mut new_bindings = Vec::new();
                for binding in bindings.iter() {
                    new_bindings.push(check_if_struct_or_enum(context, binding));
                }

                // id = toplevel::namespace::struct_id..binding1..binding2
                id = format!(
                    "{}{}",
                    id,
                    new_bindings
                        .iter()
                        .map(|binding| {
                            let binding_str = format!("{}", binding);
                            let sanitized = binding_str
                                .chars()
                                .map(|c| if c.is_alphanumeric() { c } else { '.' })
                                .collect::<String>();
                            format!("..{}", sanitized)
                        })
                        .collect::<String>()
                );

                if !context.concrete_structs.contains_key(&id) {
                    let mut local_context = context.clone();
                    local_context
                        .concrete_structs
                        .insert(id.clone(), (vec![], HashMap::new()));
					local_context.generics_bindings = HashMap::new();
					for (generic, binding) in generics.iter().zip(new_bindings.iter()) {
						local_context
							.generics_bindings
							.insert(generic.clone(), binding.clone());
					}

                    let (id, ordered_members, unordered_lookup_table) =
                        type_struct(id.clone(), ordered_list.clone(), "".to_string(), &mut local_context);

                    context
                        .concrete_structs
                        .insert(id, (ordered_members, unordered_lookup_table));
                }
                Type::Struct(id.clone())
            } else {
                panic!("Type '{}' not in scope", id);
            }
        }
    }
}

fn type_expression(
    fn_array: &mut Vec<Typed<Function>>,
    expr: &Expression,
    context: &mut Context,
) -> Typed<Expression> {
    match expr {
        Expression::Atom(atom) => type_atom(fn_array, expr, &atom.expr, context),
        Expression::BinaryOp(lhs, rhs, op) => {
            type_binaryop(fn_array, &lhs.expr, &rhs.expr, op, context)
        }
        Expression::UnaryOp(expr, op) => type_unaryop(fn_array, &expr.expr, op, context),
        Expression::Assignment(var, expr, op) => {
            type_assignment(fn_array, &var.expr, &expr.expr, op, context)
        }
        Expression::TypeCast(expr, t) => {
            let Typed { expr: new_expr, .. } = type_expression(fn_array, &expr.expr, context);
            Typed {
                expr: new_expr,
                type_: check_if_struct_or_enum(context, t),
            }
        }
        Expression::ArrayAccess(expr, indices) => {
            type_arrayaccess(fn_array, &expr.expr, indices, context)
        }
    }
}

fn type_statement(
    fn_array: &mut Vec<Typed<Function>>,
    statement: &Statement,
    context: &mut Context,
) -> Typed<Statement> {
    match statement {
        Statement::Expression(expr) => {
            let Typed {
                expr: new_expr,
                type_: expr_type,
            } = type_expression(fn_array, &expr.expr, context);
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
            } = type_expression(fn_array, &expr.expr, context);
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
            } = type_expression(fn_array, &condition.expr, context);
            if condition_type != Type::Bool {
                panic!("Condition in if statement is not a boolean");
            }
            let Typed {
                expr: new_if_body,
                type_: if_body_type,
            } = type_statement(fn_array, &if_body.expr, context);
            if let Some(else_body) = else_body {
                let Typed {
                    expr: new_else_body,
                    type_: else_body_type,
                } = type_statement(fn_array, &else_body.expr, context);
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
            } = type_expression(fn_array, &condition.expr, context);
            if condition_type != Type::Bool {
                panic!("Condition in while statement is not a boolean");
            }
            let Typed {
                expr: new_body,
                type_: body_type,
            } = type_statement(fn_array, &body.expr, context);
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
            } = type_expression(fn_array, &init.expr, context);

            let Typed {
                expr: new_condition,
                type_: condition_type,
            } = type_expression(fn_array, &condition.expr, context);
            if condition_type != Type::Bool {
                panic!("Condition in for statement is not a boolean");
            }
            let Typed {
                expr: new_increment,
                type_: increment_type,
            } = type_expression(fn_array, &increment.expr, context);
            let Typed {
                expr: new_body,
                type_: body_type,
            } = type_statement(fn_array, &body.expr, context);
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
            // TODO: Check all return statements in a compound statement have the same type
            // The return statements in inner-blocks should be checked recursively too
            let mut local_context = context.clone();
            let mut last_type = Type::Void;
            let mut new_statements = Vec::new();
            for statement in statements {
                let Typed {
                    expr: new_statement,
                    type_: statement_type,
                } = type_statement(fn_array, &statement.expr, &mut local_context);
                context.concrete_functions = local_context.concrete_functions.clone();
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
                        local_context
                            .variables
                            .insert(var_name.to_string(), var_type.clone());

                        // Extract the inner Expression from each Typed<Expression>
                        let unwrapped_dimensions: Vec<Expression> = dimensions
                            .iter()
                            .map(|typed_expr| typed_expr.expr.clone())
                            .collect();

                        local_context
                            .array_dims
                            .insert(var_name.to_string(), unwrapped_dimensions);
                    } else {
                        local_context
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
                } = type_expression(fn_array, &expr.expr, context);
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
            } = type_statement(fn_array, &body.expr, context);
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
            } = type_expression(fn_array, &condition.expr, context);
            if expr_type != Type::Bool {
                panic!("Condition in do while statement is not a boolean");
            }
            let Typed {
                expr: new_body,
                type_: body_type,
            } = type_statement(fn_array, &body.expr, context);
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

fn type_struct(
    id: String,
    members: Vec<(String, Type)>,
    namespace_path: String,
    context: &mut Context,
) -> (String, Vec<(String, Type)>, HashMap<String, Type>) {
    let id = format!("{}{}", namespace_path, id);

    let mut member_names = HashSet::new();
    for (name, _) in members.clone() {
        if !member_names.insert(name.clone()) {
            panic!("Struct '{}' has duplicate member '{}'", id, name);
        } else if name == "len" {
			panic!("Struct '{}' has a member named 'len': 'len' is a reserved member name", id);
		}
    }

    let mut unordered_lookup_table = HashMap::new();
    let mut ordered_members = Vec::new();
    for (name, t) in members {
        let t = check_if_struct_or_enum(context, &t);
        unordered_lookup_table.insert(name.clone(), t.clone());
        ordered_members.push((name.clone(), t));
    }

    (id, ordered_members, unordered_lookup_table)
}

fn type_function(
    fn_array: &mut Vec<Typed<Function>>,
    function: Function,
    context: &mut Context,
    namespace_path: String,
) -> Typed<Function> {
    let Function {
        id: name,
        args: params,
        body,
        return_type,
        ..
    } = function;

    let name = format!("{}{}", namespace_path, name);
    let mut local_context = context.clone();
	let mut new_params = Vec::new();

    // Function parameters are only in scope in the function
    for (param_name, param_type) in params.clone() {
        let param_type = check_if_struct_or_enum(&mut local_context, &param_type);
        local_context
            .variables
            .insert(param_name.clone(), param_type.clone());
		new_params.push((param_name.clone(), param_type.clone()));
    }

    let Typed {
        expr: new_body,
        type_: body_type,
    } = type_statement(fn_array, &body.expr, &mut local_context);

    let return_type = check_if_struct_or_enum(&mut local_context, &return_type);
    context.concrete_functions = local_context.concrete_functions.clone();

    if body_type != return_type {
        panic!(
            "Function '{}' return type ({}) does not match body type ({})",
            name, return_type, body_type
        );
    }

    Typed {
        expr: Function {
            id: name.clone(),
            args: new_params,
            body: Typed {
                expr: new_body,
                type_: body_type,
            },
            return_type: return_type.clone(),
            generics: vec![],
        },
        type_: return_type.clone(),
    }
}

fn get_all_functions_structs_enums_consts(
    namespace: &Namespace,
    namespace_path: &str,
) -> (
    HashMap<String, (Type, Vec<Type>)>,
    HashMap<String, Function>,
    HashMap<String, (Vec<(String, Type)>, HashMap<String, Type>)>,
    HashMap<String, (Vec<(String, Type)>, HashMap<String, Type>, Vec<String>)>,
    HashMap<String, Vec<String>>,
    HashMap<String, Type>,
) {
    // Functions, structs and enums are defined everywhere, even before declaration
    // so we need to add them to the context before typechecking them. This
    // allows recursion

    let namespace_path = format!("{}{}::", namespace_path, namespace.id);

    let mut concrete_functions = HashMap::new();
    let mut generic_functions = HashMap::new();
    let mut concrete_structs = HashMap::new();
    let mut generic_structs = HashMap::new();
    let mut enums = HashMap::new();
    let mut constants = HashMap::new();

    for function in &namespace.functions {
        let Function {
            id: name,
            args,
            return_type,
            generics,
            ..
        } = &function.expr;
        let name = format!("{}{}", namespace_path, name);

        if concrete_functions.contains_key(&name) | generic_functions.contains_key(&name) {
            panic!("Function '{}' is declared more than once", name);
        }

        let arg_types: Vec<Type> = args.iter().map(|(_, t)| t.clone()).collect();
        if generics.len() > 0 {
            generic_functions.insert(name.clone(), function.expr.clone());
        } else {
            concrete_functions.insert(name.clone(), (return_type.clone(), arg_types));
        }
    }

    for struct_ in &namespace.structs {
        let Struct {
            id,
            members,
            generics,
        } = struct_;
        let id = format!("{}{}", namespace_path, id);
        if concrete_structs.contains_key(&id) | generic_structs.contains_key(&id) {
            panic!("Struct '{}' is declared more than once", id);
        } else if enums.contains_key(&id) {
            panic!("Struct '{}' has the same name as an enum", id);
        }
        let unordered_list = members
            .iter()
            .map(|(name, t)| (name.clone(), t.clone()))
            .collect::<HashMap<_, _>>();
        if generics.len() > 0 {
            generic_structs.insert(
                id.clone(),
                (members.clone(), unordered_list, generics.clone()),
            );
        } else {
            concrete_structs.insert(id.clone(), (members.clone(), unordered_list));
        }
    }

    for enum_ in &namespace.enums {
        let Enum { id, .. } = enum_;
        let id = format!("{}{}", namespace_path, id);
        if enums.contains_key(&id) {
            panic!("Enum '{}' is declared more than once", id);
        } else if concrete_structs.contains_key(&id) {
            panic!("Enum '{}' has the same name as a struct", id);
        }
        let variants = enum_
            .variants
            .iter()
            .map(|variant| variant.clone())
            .collect::<Vec<_>>();
        enums.insert(id.clone(), variants);
    }

    for constant in &namespace.constants {
        let Constant::Constant(name, _, var_type) = &constant.expr;
        let name = format!("{}{}", namespace_path, name);

        if constants.contains_key(&name) {
            panic!("Constant '{}' is declared more than once", name);
        }

        constants.insert(name.clone(), var_type.clone());
    }

    for sub_namespace in &namespace.sub_namespaces {
        let (
            sub_concrete_functions,
            sub_generic_functions,
            sub_concrete_structs,
            sub_generic_structs,
            sub_enums,
            sub_constants,
        ) = get_all_functions_structs_enums_consts(&sub_namespace, &namespace_path);
        concrete_functions.extend(sub_concrete_functions);
        generic_functions.extend(sub_generic_functions);
        concrete_structs.extend(sub_concrete_structs);
        generic_structs.extend(sub_generic_structs);
        enums.extend(sub_enums);
        constants.extend(sub_constants);
    }

    (
        concrete_functions,
        generic_functions,
        concrete_structs,
        generic_structs,
        enums,
        constants,
    )
}

fn typechecking(
    namespace: &Namespace,
    is_toplevel: bool,
    namespace_path: &str,
    all_concrete_functions: HashMap<String, (Type, Vec<Type>)>,
    all_generic_functions: HashMap<String, Function>,
    all_concrete_structs: HashMap<String, (Vec<(String, Type)>, HashMap<String, Type>)>,
    all_generic_structs: HashMap<String, (Vec<(String, Type)>, HashMap<String, Type>, Vec<String>)>,
    all_enums: HashMap<String, Vec<String>>,
    all_constants: HashMap<String, Type>,
) -> Namespace {
    let Namespace {
        id,
        functions,
        constants,
        structs,
        enums,
        sub_namespaces,
    } = namespace;

    if id == "toplevel" && !is_toplevel {
        panic!("'toplevel' namespace is reserved, use a different name");
    }

    let namespace_path = format!("{}{}::", namespace_path, id);

    let mut context = Context {
        namespace: namespace_path.clone(),
		namespace_root: namespace_path.clone(),
        variables: all_constants.clone(),
        concrete_functions: all_concrete_functions.clone(),
        generic_functions: all_generic_functions.clone(),
        concrete_structs: all_concrete_structs.clone(),
        generic_structs: all_generic_structs.clone(),
        enums: all_enums.clone(),
        array_dims: HashMap::new(),
        generics_bindings: HashMap::new(),
    };

    // Now we can do proper typechecking

    let mut new_functions = Vec::new();

    for enum_ in enums {
        let Enum { id, variants } = enum_;
        let id = format!("{}{}", namespace_path, id);

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
        let name = format!("{}{}", namespace_path, name);

        let Typed {
            type_: expr_type, ..
        } = type_expression(
            &mut new_functions,
            &Expression::Atom(Typed::new(Atom::Literal(lit.clone()))),
            &mut context,
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
        let Struct {
            id,
            members,
            generics,
        } = struct_;

        if generics.len() > 0 {
            continue;
        }

        let (id, ordered_members, unordered_lookup_table) = type_struct(
            id.clone(),
            members.clone(),
            namespace_path.clone(),
            &mut context,
        );

        context
            .concrete_structs
            .insert(id, (ordered_members, unordered_lookup_table));
    }

    for function in functions {
        let Function { generics, .. } = &function.expr;
        if generics.len() > 0 {
            continue;
        }

        let new_function = type_function(
            &mut new_functions,
            function.expr.clone(),
            &mut context,
            namespace_path.clone(),
        );

        new_functions.push(new_function);
    }

    let mut new_enums = enums.clone();
    let mut new_structs = structs.clone();

    for sub_namespace in sub_namespaces.iter() {
        let Namespace {
            functions: sub_functions,
            constants: sub_constants,
            structs: sub_structs,
            enums: sub_enums,
            ..
        } = typechecking(
            sub_namespace,
            false,
            namespace_path.as_str(),
            all_concrete_functions.clone(),
            all_generic_functions.clone(),
            all_concrete_structs.clone(),
            all_generic_structs.clone(),
            all_enums.clone(),
            all_constants.clone(),
        );

        new_functions.extend(sub_functions);
        typed_constants.extend(sub_constants);
        new_structs.extend(sub_structs);
        new_enums.extend(sub_enums);
    }

    Namespace {
        id: id.to_string(),
        functions: new_functions,
        constants: typed_constants,
        structs: new_structs,
        enums: new_enums,
        sub_namespaces: vec![],
    }
}

pub fn check_program(ast: &Ast) -> Ast {
    // Check that there is a main function, no function is called _start and no
    // function is declared twice
    let mut main_found = false;
    let mut function_names = HashSet::new();

    let Namespace { functions, .. } = &ast.program;
    for function in functions {
        let Function {
            id: name,
            return_type: ret_type,
            generics,
            ..
        } = &function.expr;
        if name == "main" {
            main_found = true;
            if generics.len() > 0 {
                panic!("Main function cannot have generics");
            }
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

    // We do a pre-parsing pass to get all functions, structs, enums and constants
    // This lets us use functions, structs and enums before they are declared
    // and allows recursivity
    let (
        concrete_functions,
        generic_functions,
        concrete_structs,
        generic_structs,
        enums,
        constants,
    ) = get_all_functions_structs_enums_consts(&ast.program, "");

    Ast {
        program: typechecking(
            &ast.program,
            true,
            "",
            concrete_functions,
            generic_functions,
            concrete_structs,
            generic_structs,
            enums,
            constants,
        ),
    }
}
