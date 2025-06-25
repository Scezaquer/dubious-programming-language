// TODO: Check uninitialized variables aren't called
// TODO: Complain if a variable is declared but never used
// TODO: Check that the return type of a function matches the type of the return statement in every branch
// TODO: is there ambiguity between function and variable names?

use crate::ast_build::{
    AssignmentIdentifier, AssignmentOp, Ast, Atom, BinOp, Enum, Expression, Function, Literal,
    Namespace, Statement, Struct, UnOp,
};
use crate::ast_build::{Constant, ReassignmentIdentifier};
use crate::shared::{error, TokenWithDebugInfo, Type, Typed};
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
    fn_array: &mut Vec<Typed<TokenWithDebugInfo<Function>>>,
    expr: TokenWithDebugInfo<Expression>,
    atom: TokenWithDebugInfo<Atom>,
    context: &mut Context,
) -> Typed<TokenWithDebugInfo<Expression>> {
    match atom.internal_tok.clone() {
        Atom::Literal(Typed {
            expr:
                TokenWithDebugInfo {
                    internal_tok: Literal::Bool(_),
                    ..
                },
            ..
        }) => Typed {
            expr: expr.clone(),
            type_: Type::Bool,
        },
        Atom::Literal(Typed {
            expr:
                TokenWithDebugInfo {
                    internal_tok: Literal::Int(_),
                    ..
                },
            ..
        }) => Typed {
            expr: expr.clone(),
            type_: Type::Int,
        },
        Atom::Literal(Typed {
            expr:
                TokenWithDebugInfo {
                    internal_tok: Literal::Float(_),
                    ..
                },
            ..
        }) => Typed {
            expr: expr.clone(),
            type_: Type::Float,
        },
        Atom::Literal(Typed {
            expr:
                TokenWithDebugInfo {
                    internal_tok: Literal::Char(_),
                    ..
                },
            ..
        }) => Typed {
            expr: expr.clone(),
            type_: Type::Char,
        },
        Atom::Literal(Typed {
            expr:
                TokenWithDebugInfo {
                    internal_tok: Literal::Hex(_),
                    ..
                },
            ..
        }) => Typed {
            expr: expr.clone(),
            type_: Type::Int,
        },
        Atom::Literal(Typed {
            expr:
                TokenWithDebugInfo {
                    internal_tok: Literal::Binary(_),
                    ..
                },
            ..
        }) => Typed {
            expr: expr.clone(),
            type_: Type::Int,
        },
        Atom::Variable(v) => {
            // Check that the variable is in scope
            let (line, file) = (v.line, v.file.clone());
            if let Some(var_type) = context.variables.get(&v.internal_tok) {
                Typed {
                    expr: TokenWithDebugInfo {
                        internal_tok: Expression::Atom(Typed::new_with_type(
                            TokenWithDebugInfo {
                                internal_tok: Atom::Variable(v.clone()),
                                line: line,
                                file: file.clone(),
                            },
                            var_type.clone(),
                        )),
                        line,
                        file,
                    },
                    type_: var_type.clone(),
                }
            } else if let Some(var_type) = context
                .variables
                .get(&format!("{}{}", context.namespace, v))
            {
                Typed {
                    expr: TokenWithDebugInfo {
                        internal_tok: Expression::Atom(Typed::new_with_type(
                            TokenWithDebugInfo {
                                internal_tok: Atom::Variable(TokenWithDebugInfo {
                                    internal_tok: format!("{}{}", context.namespace, v),
                                    line: line,
                                    file: file.clone(),
                                }),
                                line: line,
                                file: file.clone(),
                            },
                            var_type.clone(),
                        )),
                        line,
                        file,
                    },
                    type_: var_type.clone(),
                }
            } else if let Some(_) = context.enums.get(&format!("{}{}", context.namespace, v)) {
                Typed {
                    expr: TokenWithDebugInfo {
                        internal_tok: Expression::Atom(Typed::new_with_type(
                            TokenWithDebugInfo {
                                internal_tok: Atom::Variable(TokenWithDebugInfo {
                                    internal_tok: format!("{}{}", context.namespace, v),
                                    line: line,
                                    file: file.clone(),
                                }),
                                line: line,
                                file: file.clone(),
                            },
                            Type::Enum(format!("{}{}", context.namespace, v)),
                        )),
                        line: line,
                        file: file.clone(),
                    },
                    type_: Type::Enum(format!("{}{}", context.namespace, v)),
                }
            } else {
                error(format!("Variable '{}' not in scope", v).as_str(), &v);
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
                } = type_expression(fn_array, expr.clone(), context);
                new_expressions.push(Typed {
                    expr: new_expr,
                    type_: expr_type.clone(),
                });
                if let Some(t) = &array_type {
                    if *t != expr_type {
                        error(
                            format!(
								"Array elements are not of the same type: expected '{}', found '{}'",
								t, expr_type
							)
                            .as_str(),
                            &expr,
                        );
                    }
                } else {
                    array_type = Some(expr_type);
                }
            }
            Typed {
                expr: TokenWithDebugInfo {
                    internal_tok: Expression::Atom(Typed {
                        expr: TokenWithDebugInfo {
                            internal_tok: Atom::Array(new_expressions, i),
                            line: atom.line,
                            file: atom.file.clone(),
                        },
                        type_: Type::Array(Box::new(TokenWithDebugInfo {
                            internal_tok: array_type.clone().unwrap(),
                            line: atom.line,
                            file: atom.file.clone(),
                        })),
                    }),
                    line: atom.line,
                    file: atom.file.clone(),
                },
                type_: Type::Array(Box::new(TokenWithDebugInfo {
                    internal_tok: array_type.unwrap(),
                    line: atom.line,
                    file: atom.file.clone(),
                })),
            }
        }
        Atom::FunctionCall(name, args, generics_bindings) => {
            let (line_name, file_name) = (name.line, name.file.clone());
            let mut name = if name.internal_tok.contains("::") | name.internal_tok.eq("toplevel") {
                name.internal_tok.clone()
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
                    error(
                        format!(
                            "Function '{}' expects {} generics, but {} were provided",
                            name,
                            func.generics.len(),
                            generics_bindings.len()
                        )
                        .as_str(),
                        &TokenWithDebugInfo {
                            internal_tok: name,
                            line: line_name,
                            file: file_name,
                        },
                    );
                }

                let generics_bindings = generics_bindings
                    .iter()
                    .map(|binding| check_if_struct_or_enum(context, binding.clone()))
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
                        local_context.generics_bindings.insert(
                            generic.internal_tok.clone(),
                            check_if_struct_or_enum(context, binding.clone()).internal_tok,
                        );
                    }
                    local_context.concrete_functions.insert(
                        name.to_string(),
                        (
                            func.return_type.internal_tok.clone(),
                            func.args
                                .iter()
                                .map(|(_, t)| t.internal_tok.clone())
                                .collect(),
                        ),
                    );

                    let func = TokenWithDebugInfo {
                        internal_tok: Function {
                            id: TokenWithDebugInfo {
                                internal_tok: name.clone(),
                                line: line_name,
                                file: file_name.clone(),
                            },
                            return_type: func.return_type.clone(),
                            args: func.args.clone(),
                            generics: vec![],
                            body: func.body.clone(),
                        },
                        line: atom.line,
                        file: atom.file.clone(),
                    };

                    let binded_function =
                        type_function(fn_array, func.clone(), &mut local_context, "".to_string());
                    fn_array.push(binded_function.clone());
                    context.concrete_functions = local_context.concrete_functions;
                    context.concrete_functions.insert(
                        name.clone(),
                        (
                            binded_function
                                .expr
                                .internal_tok
                                .return_type
                                .internal_tok
                                .clone(),
                            binded_function
                                .expr
                                .internal_tok
                                .args
                                .iter()
                                .map(|(_, t)| t.internal_tok.clone())
                                .collect(),
                        ),
                    );
                }
            }

            if let Some((return_type, args_types)) = context.concrete_functions.clone().get(&name) {
                let mut new_args = Vec::new();

                if args.len() != args_types.len() {
                    error(
                        format!(
                            "Function '{}' expects {} arguments, but {} were provided",
                            name,
                            args_types.len(),
                            args.len()
                        )
                        .as_str(),
                        &atom,
                    );
                }

                let mut tmp_ctx = context.clone();
                tmp_ctx.namespace = context.namespace_root.clone();
                for (i, (arg, expected_type)) in args.iter().zip(args_types.iter()).enumerate() {
                    let Typed {
                        expr: new_arg,
                        type_: arg_type,
                    } = type_expression(fn_array, arg.expr.clone(), &mut tmp_ctx);

                    if arg_type != *expected_type {
                        error(
                            format!(
                                "Argument {} of function '{}' has type '{}', but '{}' was expected",
                                i, name, arg_type, expected_type
                            )
                            .as_str(),
                            &arg.expr,
                        );
                    }

                    new_args.push(Typed {
                        expr: new_arg,
                        type_: arg_type.clone(),
                    });
                }

                let return_type = check_if_struct_or_enum(
                    context,
                    TokenWithDebugInfo {
                        internal_tok: return_type.clone(),
                        line: atom.line,
                        file: atom.file.clone(),
                    },
                );
                Typed {
                    expr: TokenWithDebugInfo {
                        internal_tok: Expression::Atom(Typed {
                            expr: TokenWithDebugInfo {
                                internal_tok: Atom::FunctionCall(
                                    TokenWithDebugInfo {
                                        internal_tok: name.to_string(),
                                        line: line_name,
                                        file: file_name,
                                    },
                                    new_args,
                                    vec![],
                                ),
                                line: atom.line,
                                file: atom.file.clone(),
                            },
                            type_: return_type.internal_tok.clone(),
                        }),
                        line: atom.line,
                        file: atom.file.clone(),
                    },
                    type_: return_type.internal_tok.clone(),
                }
            } else {
                error(format!("Function '{}' not in scope", name).as_str(), &atom);
            }
        }
        Atom::Expression(expr) => type_expression(fn_array, expr.expr, context),
        Atom::StructInstance(id, exprs, generics_bindings) => {
            // id = toplevel::namespace::struct_id..binding1..binding2
            let id = if id.internal_tok.contains("::") | id.internal_tok.eq("toplevel") {
                id.internal_tok
            } else {
                format!("{}{}", context.namespace, id)
            };

            let generics_bindings = generics_bindings
                .iter()
                .map(|binding| check_if_struct_or_enum(context, binding.clone()))
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
                    error(
                        format!(
                            "Struct '{}' has {} members, but {} were provided",
                            id,
                            ordered_list.len(),
                            exprs.len()
                        )
                        .as_str(),
                        &atom,
                    );
                }

                let mut new_exprs = Vec::new();
                for (expr, (_, t)) in exprs.iter().zip(ordered_list.iter()) {
                    let Typed {
                        expr: new_expr,
                        type_: expr_type,
                    } = type_expression(fn_array, expr.expr.clone(), context);
                    new_exprs.push(Typed {
                        expr: new_expr,
                        type_: expr_type.clone(),
                    });
                    if *t != expr_type {
                        error(
                            format!(
                                "Struct member type '{}' does not match expression type '{}'",
                                t, expr_type
                            )
                            .as_str(),
                            &expr.expr,
                        );
                    }
                }
                Typed {
                    expr: TokenWithDebugInfo {
                        internal_tok: Expression::Atom(Typed {
                            expr: TokenWithDebugInfo {
                                internal_tok: Atom::StructInstance(
                                    TokenWithDebugInfo {
                                        internal_tok: id.clone(),
                                        line: atom.line,
                                        file: atom.file.clone(),
                                    },
                                    new_exprs,
                                    vec![],
                                ),
                                line: atom.line,
                                file: atom.file.clone(),
                            },
                            type_: Type::Struct(id.clone()),
                        }),
                        line: atom.line,
                        file: atom.file.clone(),
                    },
                    type_: Type::Struct(id.clone()),
                }
            } else {
                error(format!("Struct '{}' not in scope", id).as_str(), &atom);
            }
        }
    }
}

fn type_member_access(
    lhs: TokenWithDebugInfo<Expression>,
    rhs: TokenWithDebugInfo<Expression>,
    op: &BinOp,
    context: &Context,
    lhs_expr: TokenWithDebugInfo<Expression>,
    lhs_type: Type,
) -> Typed<TokenWithDebugInfo<Expression>> {
    // Treat member access separately from the other operators, otherwise
    // we try to typecheck both sides, but the rhs isn't defined as a
    // separate variable so it crashes
    if let Type::Struct(s) = lhs_type.clone() {
        if let Expression::Atom(Typed {
            expr:
                TokenWithDebugInfo {
                    internal_tok: Atom::Variable(attribute),
                    line,
                    file,
                },
            ..
        }) = rhs.internal_tok.clone()
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
            if attribute.internal_tok.eq("len") {
                // reserved member name for struct length
                index = -1;
                t = Type::Int;
            } else {
                if let Some(t_) = unordered_list.get(&attribute.internal_tok) {
                    t = t_.clone();
                } else {
                    error(
                        format!("Struct '{}' does not have member '{}'", lhs, rhs).as_str(),
                        &rhs,
                    );
                }
                index = ordered_list
                    .iter()
                    .position(|(attr, _)| *attr == attribute.internal_tok)
                    .expect("Attribute not found in ordered list") as i64;
            }

            return Typed {
                expr: TokenWithDebugInfo {
                    internal_tok: Expression::BinaryOp(
                        Box::new(Typed {
                            expr: lhs_expr,
                            type_: lhs_type,
                        }),
                        Box::new(Typed {
                            expr: TokenWithDebugInfo {
                                internal_tok: Expression::Atom(Typed {
                                    expr: TokenWithDebugInfo {
                                        internal_tok: Atom::Literal(Typed {
                                            expr: TokenWithDebugInfo {
                                                internal_tok: Literal::Int(index),
                                                line: line,
                                                file: file.clone(),
                                            },
                                            type_: Type::Int,
                                        }),
                                        line: line,
                                        file: file.clone(),
                                    },
                                    type_: Type::Int,
                                }),
                                line: line,
                                file: file.clone(),
                            },
                            type_: Type::Int,
                        }),
                        op.clone(),
                    ),
                    line: line,
                    file: file.clone(),
                },
                type_: t,
            };
        } else {
            error("Member access must be on a variable", &rhs);
        }
    } else if let Type::Enum(e) = lhs_type.clone() {
        if let Expression::Atom(Typed {
            expr:
                TokenWithDebugInfo {
                    internal_tok: Atom::Variable(attribute),
                    line,
                    file,
                },
            ..
        }) = rhs.internal_tok.clone()
        {
            if let Some(variants) = context.enums.get(&e) {
                if variants.contains(&attribute.internal_tok) {
                    return Typed {
                        expr: TokenWithDebugInfo {
                            internal_tok: Expression::Atom(Typed {
                                expr: TokenWithDebugInfo {
                                    internal_tok: Atom::Literal(Typed {
                                        expr: TokenWithDebugInfo {
                                            internal_tok: Literal::Int(
                                                variants
                                                    .iter()
                                                    .position(|v| *v == attribute.internal_tok)
                                                    .unwrap()
                                                    as i64,
                                            ),
                                            line: line,
                                            file: file.clone(),
                                        },
                                        type_: Type::Int,
                                    }),
                                    line: line,
                                    file: file.clone(),
                                },
                                type_: Type::Int,
                            }),
                            line: line,
                            file: file.clone(),
                        },
                        type_: Type::Enum(e),
                    };
                } else {
                    error(
                        format!("Enum '{}' does not have variant '{}'", e, attribute).as_str(),
                        &rhs,
                    );
                }
            } else {
                error(format!("Enum '{}' not in scope", e).as_str(), &rhs);
            }
        } else {
            error("Member access must be on a variable", &rhs);
        }
    } else {
        error(
            format!(
                "Type '{}' is not a struct or an enum, members access is undefined",
                lhs_type
            )
            .as_str(),
            &lhs,
        );
    }
}

fn unroll_namespace_access(
    lhs: TokenWithDebugInfo<Expression>,
    context: &Context,
) -> TokenWithDebugInfo<String> {
    // The namespace operator is left-associative, so we need a bit of machinery
    // to unroll it. This turns
    // (((a b ::) c ::) d ::)
    // into
    // "a::b::c::d"

    let (line, file) = (lhs.line, lhs.file.clone());
    match lhs.internal_tok.clone() {
        Expression::Atom(Typed {
            expr:
                TokenWithDebugInfo {
                    internal_tok: Atom::Variable(namespace),
                    ..
                },
            ..
        }) => {
            let namespace_path = if namespace.internal_tok.contains("::") {
                namespace.to_string()
            } else if namespace.internal_tok.eq("toplevel") {
                format!("{}::", namespace.to_string())
            } else {
                format!("{}{}::", context.namespace, namespace)
            };
            TokenWithDebugInfo {
                internal_tok: namespace_path,
                line,
                file,
            }
        }
        Expression::BinaryOp(l, r, BinOp::NamespaceAccess) => {
            if let Typed {
                expr:
                    TokenWithDebugInfo {
                        internal_tok:
                            Expression::Atom(Typed {
                                expr:
                                    TokenWithDebugInfo {
                                        internal_tok: Atom::Variable(id),
                                        ..
                                    },
                                ..
                            }),
                        line,
                        file,
                    },
                ..
            } = *r
            {
                let namespace_path = unroll_namespace_access(l.expr, context);
                TokenWithDebugInfo {
                    internal_tok: format!("{}{}::", namespace_path, id),
                    line,
                    file,
                }
            } else {
                error("Namespace access must be on a variable", &lhs);
            }
        }
        _ => {
            error("Namespace access must be on a variable", &lhs);
        }
    }
}

fn type_namespace_access(
    fn_array: &mut Vec<Typed<TokenWithDebugInfo<Function>>>,
    lhs: TokenWithDebugInfo<Expression>,
    rhs: TokenWithDebugInfo<Expression>,
    context: &mut Context,
) -> Typed<TokenWithDebugInfo<Expression>> {
    let namespace_path = unroll_namespace_access(lhs, context);
    let mut local_context = context.clone();
    local_context.namespace = namespace_path.internal_tok;

    let typed_expr = type_expression(fn_array, rhs, &mut local_context);

    context.concrete_functions = local_context.concrete_functions;
    return typed_expr;
}

fn type_binaryop(
    fn_array: &mut Vec<Typed<TokenWithDebugInfo<Function>>>,
    lhs: TokenWithDebugInfo<Expression>,
    rhs: TokenWithDebugInfo<Expression>,
    op: BinOp,
    context: &mut Context,
) -> Typed<TokenWithDebugInfo<Expression>> {
    let (line, file) = (lhs.line, lhs.file.clone());
    if let BinOp::NamespaceAccess = op {
        return type_namespace_access(fn_array, lhs, rhs, context);
    }

    let Typed {
        expr: lhs_expr,
        type_: lhs_type,
    } = type_expression(fn_array, lhs.clone(), context);

    if let BinOp::MemberAccess = op {
        return type_member_access(lhs, rhs, &op, context, lhs_expr, lhs_type);
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
                error(
                    format!(
                        "Invalid types for arithmetic operation: '{}' and '{}'",
                        lhs_type, rhs_type
                    )
                    .as_str(),
                    &lhs,
                );
            }
        }
        BinOp::Equal | BinOp::NotEqual => {
            if lhs_type == rhs_type {
                Type::Bool
            } else {
                error(
                    format!(
                        "Invalid types for comparison operation: '{}' and '{}'",
                        lhs_type, rhs_type
                    )
                    .as_str(),
                    &lhs,
                );
            }
        }
        BinOp::LessThan
        | BinOp::LessOrEqualThan
        | BinOp::GreaterThan
        | BinOp::GreaterOrEqualThan => {
            if lhs_type == rhs_type && (lhs_type == Type::Int || lhs_type == Type::Float) {
                Type::Bool
            } else {
                error(
                    format!(
                        "Invalid types for comparison operation: '{}' and '{}'",
                        lhs_type, rhs_type
                    )
                    .as_str(),
                    &lhs,
                );
            }
        }
        BinOp::LogicalAnd | BinOp::LogicalOr | BinOp::LogicalXor => {
            if lhs_type == Type::Bool && rhs_type == Type::Bool {
                Type::Bool
            } else {
                error(
                    format!(
                        "Invalid types for logical operation: '{}' and '{}'",
                        lhs_type, rhs_type
                    )
                    .as_str(),
                    &lhs,
                );
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
                error(
                    format!(
                        "Invalid types for bitwise operation: '{}' and '{}'",
                        lhs_type, rhs_type
                    )
                    .as_str(),
                    &lhs,
                );
            }
        }
        BinOp::MemberAccess => {
            unreachable!()
        }
        BinOp::NamespaceAccess => {
            unreachable!()
        }
        BinOp::NotABinaryOp => {
            error("Invalid binary operation", &lhs);
        }
    };
    Typed {
        expr: TokenWithDebugInfo {
            internal_tok: Expression::BinaryOp(
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
            line,
            file,
        },
        type_: binop_type,
    }
}

fn type_unaryop(
    fn_array: &mut Vec<Typed<TokenWithDebugInfo<Function>>>,
    expr: TokenWithDebugInfo<Expression>,
    op: UnOp,
    context: &mut Context,
) -> Typed<TokenWithDebugInfo<Expression>> {
    let Typed {
        expr: new_expr,
        type_: expr_type,
    } = type_expression(fn_array, expr.clone(), context);
    let expr_type_clone = expr_type.clone();

    let (line, file) = (new_expr.line, new_expr.file.clone());

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
                error(
                    format!("Invalid type for unary operation: '{}'", expr_type).as_str(),
                    &expr,
                );
            }
        }
        UnOp::LogicalNot => {
            if expr_type == Type::Bool {
                Type::Bool
            } else {
                error(
                    format!("Invalid type for logical not operation: '{}'", expr_type).as_str(),
                    &expr,
                );
            }
        }
        UnOp::Dereference => {
            if let Type::Pointer(ptr_type) = expr_type {
                (*ptr_type).internal_tok
            } else {
                error(
                    format!("Dereferencing a non-pointer type: '{}'", expr_type).as_str(),
                    &expr,
                );
            }
        }
        UnOp::AddressOf => Type::Pointer(Box::new(TokenWithDebugInfo {
            internal_tok: expr_type,
            line,
            file: file.clone(),
        })),
        UnOp::NotAUnaryOp => {
            error("Invalid unary operation", &expr);
        }
    };
    Typed {
        expr: TokenWithDebugInfo {
            internal_tok: Expression::UnaryOp(
                Box::new(Typed {
                    expr: new_expr,
                    type_: expr_type_clone,
                }),
                op.clone(),
            ),
            line,
            file,
        },
        type_: unop_type,
    }
}

fn type_assignment(
    fn_array: &mut Vec<Typed<TokenWithDebugInfo<Function>>>,
    var: TokenWithDebugInfo<ReassignmentIdentifier>,
    expr: TokenWithDebugInfo<Expression>,
    op: AssignmentOp,
    context: &mut Context,
) -> Typed<TokenWithDebugInfo<Expression>> {
    let Typed {
        expr: new_rhs,
        type_: rhs_type,
    } = type_expression(fn_array, expr, context);

    let (line, file) = (var.line, var.file.clone());
    let (new_lhs, lhs_type);
    match var.internal_tok.clone() {
        ReassignmentIdentifier::Variable(v) => {
            if let Some(var_type) = context.variables.get(&v.internal_tok) {
                lhs_type = var_type.clone();
                new_lhs = ReassignmentIdentifier::Variable(v.clone());
            } else {
                error(
                    format!("Variable '{}' not in scope", v.internal_tok).as_str(),
                    &v,
                );
            }
        }
        ReassignmentIdentifier::Array(arr, idxs) => {
            let lhs_expr;
            Typed {
                expr: lhs_expr,
                type_: lhs_type,
            } = type_expression(
                fn_array,
                TokenWithDebugInfo {
                    internal_tok: Expression::ArrayAccess(arr.clone(), idxs.clone()),
                    line,
                    file: file.clone(),
                },
                context,
            );
            if let Expression::ArrayAccess(e1, e2) = lhs_expr.internal_tok {
                new_lhs = ReassignmentIdentifier::Array(e1, e2);
            } else {
                error("Unreachable code in array access", &var);
            }
        }
        ReassignmentIdentifier::Dereference(expr) => {
            let Typed {
                expr: new_expr,
                type_: expr_type,
            } = type_expression(fn_array, expr.expr.clone(), context);
            if let Type::Pointer(t) = expr_type.clone() {
                lhs_type = (*t).internal_tok;
                new_lhs = ReassignmentIdentifier::Dereference(Box::new(Typed {
                    expr: new_expr,
                    type_: expr_type,
                }));
            } else {
                error(
                    format!("Dereferencing a non-pointer type: '{}'", expr_type).as_str(),
                    &(*expr).expr,
                );
            }
        }
        ReassignmentIdentifier::MemberAccess(obj, mbr) => {
            let lhs_expr;
            Typed {
                expr: lhs_expr,
                type_: lhs_type,
            } = type_expression(
                fn_array,
                TokenWithDebugInfo {
                    internal_tok: Expression::BinaryOp(
                        Box::new(*obj.clone()),
                        Box::new(*mbr.clone()),
                        BinOp::MemberAccess,
                    ),
                    line,
                    file: file.clone(),
                },
                context,
            );
            if let Expression::BinaryOp(e1, e2, BinOp::MemberAccess) = lhs_expr.internal_tok {
                new_lhs = ReassignmentIdentifier::Array(e1, vec![*e2]);
            } else {
                error("Unreachable code in member access", &var);
            }
        }
    }

    if lhs_type != rhs_type {
        error(
            format!(
                "Assignment types do not match: '{}' on the left, '{}' on the right",
                lhs_type, rhs_type
            )
            .as_str(),
            &var,
        );
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
                error(
                    format!(
					"This assignment operation is only supported for integers and floats, but got '{}' and '{}'",
					lhs_type, rhs_type)
                    .as_str(),
                    &var,
                );
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
                error(
                    format!(
						"This assignment operation is only supported for integers, but got '{}' and '{}'",
						lhs_type, rhs_type
					)
                    .as_str(),
                    &var,
                );
            }
        }
        AssignmentOp::NotAnAssignmentOp => {
            error("Invalid assignment operation", &var);
        }
    }

    Typed {
        expr: TokenWithDebugInfo {
            internal_tok: Expression::Assignment(
                Typed {
                    expr: TokenWithDebugInfo {
                        internal_tok: new_lhs,
                        line,
                        file: file.clone(),
                    },
                    type_: lhs_type.clone(),
                },
                Box::new(Typed {
                    expr: new_rhs,
                    type_: rhs_type,
                }),
                op.clone(),
            ),
            line,
            file,
        },
        type_: lhs_type,
    }
}

fn type_arrayaccess(
    fn_array: &mut Vec<Typed<TokenWithDebugInfo<Function>>>,
    expr: TokenWithDebugInfo<Expression>,
    indices: Vec<Typed<TokenWithDebugInfo<Expression>>>,
    context: &mut Context,
) -> Typed<TokenWithDebugInfo<Expression>> {
    let (line, file) = (expr.line, expr.file.clone());

    let Typed {
        expr: new_expr,
        type_: expr_type,
    } = type_expression(fn_array, expr.clone(), context);

    if let Type::Array(element_type) = expr_type {
        let mut new_indices = Vec::new();
        for index in indices.clone() {
            let Typed {
                expr: new_index,
                type_: index_type,
            } = type_expression(fn_array, index.expr.clone(), context);
            if index_type != Type::Int {
                error(
                    format!("Array index must be an integer, but got '{}'", index_type).as_str(),
                    &index.expr,
                );
            }
            new_indices.push(new_index);
        }

        // if array i has dimensions [dim1, dim2, dim3, ...] and we want to access element (i, j, k, ...)
        // ((i * dim1 + j) * dim2 + k) * dim3 + ...

        let new_index;

        if indices.len() > 1 {
            let array_name = if let Expression::Atom(Typed {
                expr:
                    TokenWithDebugInfo {
                        internal_tok: Atom::Variable(ref name),
                        ..
                    },
                ..
            }) = new_expr.internal_tok
            {
                name.clone()
            } else {
                error("Multidimensional array access must be on a variable", &expr);
            };
            let dims = context
                .array_dims
                .get(&array_name.internal_tok)
                .expect("Array dimensions not found");
            new_index = dims.iter().zip(new_indices.iter()).fold(
                Expression::Atom(Typed {
                    expr: TokenWithDebugInfo {
                        internal_tok: Atom::Literal(Typed {
                            expr: TokenWithDebugInfo {
                                internal_tok: Literal::Int(0),
                                line,
                                file: file.clone(),
                            },
                            type_: Type::Int,
                        }),
                        line,
                        file: file.clone(),
                    },
                    type_: Type::Int,
                }),
                |acc, (dim, idx)| {
                    Expression::BinaryOp(
                        Box::new(Typed {
                            expr: TokenWithDebugInfo {
                                internal_tok: Expression::BinaryOp(
                                    Box::new(Typed {
                                        expr: TokenWithDebugInfo {
                                            internal_tok: acc,
                                            line,
                                            file: file.clone(),
                                        },
                                        type_: Type::Int,
                                    }),
                                    Box::new(Typed {
                                        expr: TokenWithDebugInfo {
                                            internal_tok: dim.clone(),
                                            line,
                                            file: file.clone(),
                                        },
                                        type_: Type::Int,
                                    }),
                                    BinOp::Multiply,
                                ),
                                line,
                                file: file.clone(),
                            },
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
            new_index = new_indices[0].clone().internal_tok;
        }

        Typed {
            expr: TokenWithDebugInfo {
                internal_tok: Expression::ArrayAccess(
                    Box::new(Typed {
                        expr: new_expr,
                        type_: Type::Int,
                    }),
                    vec![Typed {
                        expr: TokenWithDebugInfo {
                            internal_tok: new_index,
                            line,
                            file: file.clone(),
                        },
                        type_: Type::Int,
                    }],
                ),
                line,
                file,
            },
            type_: element_type.as_ref().clone().internal_tok,
        }
    } else {
        error(
            format!("Array access on non-array type: '{}'", expr_type).as_str(),
            &expr,
        );
    }
}

fn check_if_struct_or_enum(
    context: &mut Context,
    t: TokenWithDebugInfo<Type>,
) -> TokenWithDebugInfo<Type> {
    let (line, file) = (t.line, t.file.clone());
    match t.internal_tok.clone() {
        Type::Struct(id) => {
            if context.generics_bindings.contains_key(&id) {
                return TokenWithDebugInfo {
                    internal_tok: context.generics_bindings.get(&id).unwrap().clone(),
                    line,
                    file,
                };
            }
            let id = if id.contains("::") | id.eq("toplevel") {
                id
            } else {
                format!("{}{}", context.namespace, id)
            };
            if let Some(_) = context.concrete_structs.get(&id) {
                TokenWithDebugInfo {
                    internal_tok: Type::Struct(id.clone()),
                    line,
                    file,
                }
            } else if let Some(_) = context.enums.get(&id) {
                TokenWithDebugInfo {
                    internal_tok: Type::Enum(id.clone()),
                    line,
                    file,
                }
            } else {
                error(format!("Type '{}' not in scope", id).as_str(), &t);
            }
        }
        Type::Enum(id) => {
            let id = if id.contains("::") | id.eq("toplevel") {
                id
            } else {
                format!("{}{}", context.namespace, id)
            };
            if let Some(_) = context.concrete_structs.get(&id) {
                TokenWithDebugInfo {
                    internal_tok: Type::Struct(id.clone()),
                    line,
                    file,
                }
            } else if let Some(_) = context.enums.get(&id) {
                TokenWithDebugInfo {
                    internal_tok: Type::Enum(id.clone()),
                    line,
                    file,
                }
            } else {
                error(format!("Type '{}' not in scope", id).as_str(), &t);
            }
        }
        Type::Array(t) => TokenWithDebugInfo {
            internal_tok: Type::Array(Box::new(check_if_struct_or_enum(context, *t))),
            line,
            file,
        },
        Type::Pointer(t) => TokenWithDebugInfo {
            internal_tok: Type::Pointer(Box::new(check_if_struct_or_enum(context, *t))),
            line,
            file,
        },
        Type::Bool | Type::Int | Type::Float | Type::Char | Type::Void => t.clone(),
        Type::Namespace(id, t) => match (*t).internal_tok {
            Type::Struct(sub_id) => {
                let id = if id.starts_with("toplevel") {
                    format!("{}::{}", id, sub_id)
                } else {
                    format!("{}{}::{}", context.namespace, id, sub_id)
                };
                check_if_struct_or_enum(
                    context,
                    TokenWithDebugInfo {
                        internal_tok: Type::Struct(id),
                        line,
                        file,
                    },
                )
            }
            Type::Enum(sub_id) => {
                let id = if id.starts_with("toplevel") {
                    format!("{}::{}", id, sub_id)
                } else {
                    format!("{}{}::{}", context.namespace, id, sub_id)
                };
                check_if_struct_or_enum(
                    context,
                    TokenWithDebugInfo {
                        internal_tok: Type::Enum(id),
                        line,
                        file,
                    },
                )
            }
            Type::Namespace(sub_id, sub_t) => {
                let id = if id.starts_with("toplevel") {
                    format!("{}::{}", id, sub_id)
                } else {
                    format!("{}{}::{}", context.namespace, id, sub_id)
                };
                check_if_struct_or_enum(
                    context,
                    TokenWithDebugInfo {
                        internal_tok: Type::Namespace(id, sub_t.clone()),
                        line,
                        file,
                    },
                )
            }
            _ => error(
                format!(
                    "Type '{}' cannot be namespaced (only struct or enums may be)",
                    id
                )
                .as_str(),
                &t,
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

            if let Some((ordered_list, _, generics)) = context.generic_structs.clone().get(&id) {
                if generics.len() != bindings.len() {
                    error(
                        format!(
                            "Generic struct '{}' expects {} generics, but {} were provided",
                            id,
                            generics.len(),
                            bindings.len()
                        )
                        .as_str(),
                        &t,
                    );
                }
                let mut new_bindings = Vec::new();
                for binding in bindings.iter() {
                    new_bindings.push(check_if_struct_or_enum(context, binding.clone()));
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
                            .insert(generic.clone(), binding.internal_tok.clone());
                    }

                    let (id, ordered_members, unordered_lookup_table) = type_struct(
                        TokenWithDebugInfo {
                            internal_tok: id.clone(),
                            line,
                            file: file.clone(),
                        },
                        ordered_list
                            .iter()
                            .map(|(name, t)| {
                                (
                                    TokenWithDebugInfo {
                                        internal_tok: name.clone(),
                                        line,
                                        file: file.clone(),
                                    },
                                    TokenWithDebugInfo {
                                        internal_tok: t.clone(),
                                        line,
                                        file: file.clone(),
                                    },
                                )
                            })
                            .collect(),
                        "".to_string(),
                        &mut local_context,
                    );

                    context
                        .concrete_structs
                        .insert(id, (ordered_members, unordered_lookup_table));
                }
                TokenWithDebugInfo {
                    internal_tok: Type::Struct(id.clone()),
                    line,
                    file,
                }
            } else {
				error(
					format!("Generic struct '{}' not in scope", id).as_str(),
					&t,
				);
            }
        }
    }
}

fn type_expression(
    fn_array: &mut Vec<Typed<TokenWithDebugInfo<Function>>>,
    expr: TokenWithDebugInfo<Expression>,
    context: &mut Context,
) -> Typed<TokenWithDebugInfo<Expression>> {
    match expr.internal_tok.clone() {
        Expression::Atom(atom) => type_atom(fn_array, expr, atom.expr, context),
        Expression::BinaryOp(lhs, rhs, op) => {
            type_binaryop(fn_array, lhs.expr, rhs.expr, op, context)
        }
        Expression::UnaryOp(expr, op) => type_unaryop(fn_array, expr.expr, op, context),
        Expression::Assignment(var, expr, op) => {
            type_assignment(fn_array, var.expr, expr.expr, op, context)
        }
        Expression::TypeCast(expr, t) => {
            let Typed { expr: new_expr, .. } = type_expression(fn_array, expr.expr, context);
            Typed {
                expr: new_expr,
                type_: check_if_struct_or_enum(context, t).internal_tok,
            }
        }
        Expression::ArrayAccess(expr, indices) => {
            type_arrayaccess(fn_array, expr.expr, indices, context)
        }
    }
}

fn type_statement(
    fn_array: &mut Vec<Typed<TokenWithDebugInfo<Function>>>,
    statement: &TokenWithDebugInfo<Statement>,
    context: &mut Context,
) -> Typed<TokenWithDebugInfo<Statement>> {
    match statement.internal_tok.clone() {
        Statement::Expression(expr) => {
            let Typed {
                expr: new_expr,
                type_: expr_type,
            } = type_expression(fn_array, expr.expr, context);
            return Typed {
                expr: TokenWithDebugInfo {
                    internal_tok: Statement::Expression(Typed {
                        expr: new_expr,
                        type_: expr_type.clone(),
                    }),
                    line: statement.line,
                    file: statement.file.clone(),
                },
                type_: expr_type,
            };
        }
        Statement::Return(expr) => {
            let Typed {
                expr: new_expr,
                type_: expr_type,
            } = type_expression(fn_array, expr.expr, context);
            return Typed {
                expr: TokenWithDebugInfo {
                    internal_tok: Statement::Return(Typed {
                        expr: new_expr,
                        type_: expr_type.clone(),
                    }),
                    line: statement.line,
                    file: statement.file.clone(),
                },
                type_: expr_type,
            };
        }
        Statement::If(condition, if_body, else_body) => {
            let Typed {
                expr: new_condition,
                type_: condition_type,
            } = type_expression(fn_array, condition.expr.clone(), context);
            if condition_type != Type::Bool {
				error(format!("Condition in if statement is not a boolean: '{}'", condition_type).as_str(), &condition.expr);
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
					error(format!("If and else branches have different types: '{}' and '{}'", if_body_type, else_body_type).as_str(), &if_body.expr);
                }
                return Typed {
                    expr: TokenWithDebugInfo {
                        internal_tok: Statement::If(
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
                        line: statement.line,
                        file: statement.file.clone(),
                    },
                    type_: if_body_type,
                };
            }
            return Typed {
                expr: TokenWithDebugInfo {
                    internal_tok: Statement::If(
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
                    line: statement.line,
                    file: statement.file.clone(),
                },
                type_: if_body_type,
            };
        }
        Statement::While(condition, body) => {
            let Typed {
                expr: new_condition,
                type_: condition_type,
            } = type_expression(fn_array, condition.expr.clone(), context);
            if condition_type != Type::Bool {
				error(
					format!("Condition in while statement is not a boolean: '{}'", condition_type).as_str(),
					&condition.expr,
				);
            }
            let Typed {
                expr: new_body,
                type_: body_type,
            } = type_statement(fn_array, &body.expr, context);
            return Typed {
                expr: TokenWithDebugInfo {
                    internal_tok: Statement::While(
                        Typed {
                            expr: new_condition,
                            type_: condition_type,
                        },
                        Box::new(Typed {
                            expr: new_body,
                            type_: body_type.clone(),
                        }),
                    ),
                    line: statement.line,
                    file: statement.file.clone(),
                },
                type_: body_type,
            };
        }
        Statement::For(init, condition, increment, body) => {
            let Typed {
                expr: new_init,
                type_: init_type,
            } = type_expression(fn_array, init.expr, context);

            let Typed {
                expr: new_condition,
                type_: condition_type,
            } = type_expression(fn_array, condition.expr.clone(), context);
            if condition_type != Type::Bool {
				error(format!("Condition in for statement is not a boolean: '{}'", condition_type).as_str(), &condition.expr);
            }
            let Typed {
                expr: new_increment,
                type_: increment_type,
            } = type_expression(fn_array, increment.expr, context);
            let Typed {
                expr: new_body,
                type_: body_type,
            } = type_statement(fn_array, &body.expr, context);
            return Typed {
                expr: TokenWithDebugInfo {
                    internal_tok: Statement::For(
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
                    line: statement.line,
                    file: statement.file.clone(),
                },
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
                if let Statement::Let(name, _, var_type) = statement.expr.internal_tok {
                    let var_type = &check_if_struct_or_enum(context, var_type);
                    let mut flag = true;
                    let mut name = name;

                    while flag {
                        if let AssignmentIdentifier::Dereference(inner) = name.internal_tok {
                            name = inner.expr;
                        } else {
                            flag = false;
                        }
                    }

                    if let AssignmentIdentifier::Array(var_name, dimensions) = name.internal_tok {
                        local_context.variables.insert(
                            var_name.internal_tok.to_string(),
                            var_type.internal_tok.clone(),
                        );

                        // Extract the inner Expression from each Typed<Expression>
                        let unwrapped_dimensions: Vec<Expression> = dimensions
                            .iter()
                            .map(|typed_expr| typed_expr.expr.internal_tok.clone())
                            .collect();

                        local_context
                            .array_dims
                            .insert(var_name.to_string(), unwrapped_dimensions);
                    } else {
                        local_context
                            .variables
                            .insert(name.internal_tok.to_string(), var_type.internal_tok.clone());
                    }
                }
            }
            return Typed {
                expr: TokenWithDebugInfo {
                    internal_tok: Statement::Compound(new_statements),
                    line: statement.line,
                    file: statement.file.clone(),
                },
                type_: last_type,
            };
        }
        Statement::Let(id, expr, var_type) => {
            let var_type = check_if_struct_or_enum(context, var_type);
            let t = var_type.clone();
            if let Some(expr) = expr {
                let Typed {
                    expr: new_expr,
                    type_: expr_type,
                } = type_expression(fn_array, expr.expr.clone(), context);
                let mut id = id;
                let mut var_type = var_type;

                let mut flag = true;
                let original_id = id.clone();
                while flag {
                    if let AssignmentIdentifier::Dereference(inner) = id.internal_tok.clone() {
                        if let Type::Pointer(t) = var_type.internal_tok {
                            var_type = *t;
                            id = inner.expr;
                        } else {
							error(format!("Dereferencing a non-pointer type: '{}'", var_type.internal_tok).as_str(), &id);
                        }
                    } else {
                        flag = false;
                    }
                }

                let var_type = &check_if_struct_or_enum(context, var_type);

                if var_type.internal_tok != expr_type {
					error(format!(
						"Variable type ('{}') does not match expression type ('{}')",
						var_type.internal_tok, expr_type
					).as_str(), &expr.expr);
                }
                return Typed {
                    expr: TokenWithDebugInfo {
                        internal_tok: Statement::Let(
                            original_id,
                            Some(Typed {
                                expr: new_expr,
                                type_: expr_type,
                            }),
                            var_type.clone(),
                        ),
                        line: statement.line,
                        file: statement.file.clone(),
                    },
                    type_: t.internal_tok,
                };
            }
            return Typed {
                expr: TokenWithDebugInfo {
                    internal_tok: Statement::Let(id.clone(), None, var_type.clone()),
                    line: statement.line,
                    file: statement.file.clone(),
                },
                type_: t.internal_tok,
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
                expr: TokenWithDebugInfo {
                    internal_tok: Statement::Loop(Box::new(Typed {
                        expr: new_body,
                        type_: body_type.clone(),
                    })),
                    line: statement.line,
                    file: statement.file.clone(),
                },
                type_: body_type,
            };
        }
        Statement::Dowhile(condition, body) => {
            let Typed {
                expr: new_expr,
                type_: expr_type,
            } = type_expression(fn_array, condition.expr.clone(), context);
            if expr_type != Type::Bool {
				error(format!("Condition in do while statement is not a boolean: '{}'", expr_type).as_str(), &condition.expr);
            }
            let Typed {
                expr: new_body,
                type_: body_type,
            } = type_statement(fn_array, &body.expr, context);
            return Typed {
                expr: TokenWithDebugInfo {
                    internal_tok: Statement::Dowhile(
                        Typed {
                            expr: new_expr,
                            type_: expr_type,
                        },
                        Box::new(Typed {
                            expr: new_body,
                            type_: body_type.clone(),
                        }),
                    ),
                    line: statement.line,
                    file: statement.file.clone(),
                },
                type_: body_type,
            };
        }
        Statement::Asm(_, type_) => {
            return Typed {
                expr: statement.clone(),
                type_: type_.internal_tok.clone(),
            }
        }
    }
}

fn type_struct(
    id: TokenWithDebugInfo<String>,
    members: Vec<(TokenWithDebugInfo<String>, TokenWithDebugInfo<Type>)>,
    namespace_path: String,
    context: &mut Context,
) -> (String, Vec<(String, Type)>, HashMap<String, Type>) {
    let id = format!("{}{}", namespace_path, id);

    let mut member_names = HashSet::new();
    for (name, _) in members.clone() {
        if !member_names.insert(name.internal_tok.clone()) {
            error(
                &format!(
                    "Struct '{}' has duplicate member '{}'",
                    id, name.internal_tok
                ),
                &name,
            );
        } else if name.internal_tok == "len" {
            error(
                &format!(
                    "Struct '{}' has a member named 'len': 'len' is a reserved member name",
                    id
                ),
                &name,
            );
        }
    }

    let mut unordered_lookup_table = HashMap::new();
    let mut ordered_members = Vec::new();
    for (name, t) in members {
        let t = check_if_struct_or_enum(context, t);
        unordered_lookup_table.insert(name.internal_tok.clone(), t.internal_tok.clone());
        ordered_members.push((name.internal_tok.clone(), t.internal_tok));
    }

    (id, ordered_members, unordered_lookup_table)
}

fn type_function(
    fn_array: &mut Vec<Typed<TokenWithDebugInfo<Function>>>,
    function: TokenWithDebugInfo<Function>,
    context: &mut Context,
    namespace_path: String,
) -> Typed<TokenWithDebugInfo<Function>> {
    let Function {
        id: name,
        args: params,
        body,
        return_type,
        ..
    } = function.internal_tok;
    let (line, file) = (function.line, function.file);

    let name = TokenWithDebugInfo {
        internal_tok: format!("{}{}", namespace_path, name),
        line,
        file: file.clone(),
    };
    let mut local_context = context.clone();
    let mut new_params = Vec::new();

    // Function parameters are only in scope in the function
    for (param_name, param_type) in params.clone() {
        let param_type = check_if_struct_or_enum(&mut local_context, param_type);
        local_context.variables.insert(
            param_name.internal_tok.clone(),
            param_type.internal_tok.clone(),
        );
        new_params.push((param_name.clone(), param_type.clone()));
    }

    let Typed {
        expr: new_body,
        type_: body_type,
    } = type_statement(fn_array, &body.expr, &mut local_context);

    let return_type = check_if_struct_or_enum(&mut local_context, return_type);
    context.concrete_functions = local_context.concrete_functions.clone();

    if body_type != return_type.internal_tok {
        error(
            &format!(
                "Function '{}' body type ({}) does not match return type ({})",
                name.internal_tok, body_type, return_type.internal_tok
            ),
            &body.expr,
        );
    }

    Typed {
        expr: TokenWithDebugInfo {
            internal_tok: Function {
                id: name.clone(),
                args: new_params,
                body: Typed {
                    expr: new_body,
                    type_: body_type,
                },
                return_type: return_type.clone(),
                generics: vec![],
            },
            line: line,
            file: file.clone(),
        },
        type_: return_type.internal_tok.clone(),
    }
}

fn get_all_functions_structs_enums_consts(
    namespace: &TokenWithDebugInfo<Namespace>,
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

    let namespace_path = format!("{}{}::", namespace_path, namespace.internal_tok.id);

    let mut concrete_functions = HashMap::new();
    let mut generic_functions = HashMap::new();
    let mut concrete_structs = HashMap::new();
    let mut generic_structs = HashMap::new();
    let mut enums = HashMap::new();
    let mut constants = HashMap::new();

    for function in &namespace.internal_tok.functions {
        let Function {
            id: name,
            args,
            return_type,
            generics,
            ..
        } = &function.expr.internal_tok;
        let name = format!("{}{}", namespace_path, name.internal_tok);

        if concrete_functions.contains_key(&name) | generic_functions.contains_key(&name) {
            error(
                &format!("Function '{}' is declared more than once", name),
                &function.expr,
            );
        }

        let arg_types: Vec<Type> = args.iter().map(|(_, t)| t.internal_tok.clone()).collect();
        if generics.len() > 0 {
            generic_functions.insert(name.clone(), function.expr.internal_tok.clone());
        } else {
            concrete_functions.insert(name.clone(), (return_type.internal_tok.clone(), arg_types));
        }
    }

    for struct_ in &namespace.internal_tok.structs {
        let Struct {
            id,
            members,
            generics,
        } = struct_.internal_tok.clone();
        let id = format!("{}{}", namespace_path, id.internal_tok);
        if concrete_structs.contains_key(&id) | generic_structs.contains_key(&id) {
            error(
                &format!("Struct '{}' is declared more than once", id),
                &struct_,
            );
        } else if enums.contains_key(&id) {
            error(
                &format!("Struct '{}' has the same name as an enum", id),
                &struct_,
            );
        }
        let unordered_list = members
            .iter()
            .map(|(name, t)| (name.internal_tok.clone(), t.internal_tok.clone()))
            .collect::<HashMap<_, _>>();
        if generics.len() > 0 {
            generic_structs.insert(
                id.clone(),
                (
                    members
                        .iter()
                        .map(|(name, t)| (name.internal_tok.clone(), t.internal_tok.clone()))
                        .collect(),
                    unordered_list,
                    generics.iter().map(|g| g.internal_tok.clone()).collect(),
                ),
            );
        } else {
            concrete_structs.insert(
                id.clone(),
                (
                    members
                        .iter()
                        .map(|(name, t)| (name.internal_tok.clone(), t.internal_tok.clone()))
                        .collect::<Vec<_>>(),
                    unordered_list,
                ),
            );
        }
    }

    for enum_ in &namespace.internal_tok.enums {
        let Enum { id, .. } = enum_.internal_tok.clone();
        let id = format!("{}{}", namespace_path, id);
        if enums.contains_key(&id) {
            error(&format!("Enum '{}' is declared more than once", id), &enum_);
        } else if concrete_structs.contains_key(&id) {
            error(
                &format!("Enum '{}' has the same name as a struct", id),
                &enum_,
            );
        }
        let variants = enum_
            .internal_tok
            .variants
            .iter()
            .map(|variant| variant.internal_tok.clone())
            .collect::<Vec<_>>();
        enums.insert(id.clone(), variants);
    }

    for constant in &namespace.internal_tok.constants {
        let Constant::Constant(name, _, var_type) = &constant.expr.internal_tok;
        let name = format!("{}{}", namespace_path, name);

        if constants.contains_key(&name) {
            error(
                &format!("Constant '{}' is declared more than once", name),
                &constant.expr,
            );
        }

        constants.insert(name.clone(), var_type.internal_tok.clone());
    }

    for sub_namespace in &namespace.internal_tok.sub_namespaces {
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
    namespace: &TokenWithDebugInfo<Namespace>,
    is_toplevel: bool,
    namespace_path: &str,
    all_concrete_functions: HashMap<String, (Type, Vec<Type>)>,
    all_generic_functions: HashMap<String, Function>,
    all_concrete_structs: HashMap<String, (Vec<(String, Type)>, HashMap<String, Type>)>,
    all_generic_structs: HashMap<String, (Vec<(String, Type)>, HashMap<String, Type>, Vec<String>)>,
    all_enums: HashMap<String, Vec<String>>,
    all_constants: HashMap<String, Type>,
) -> TokenWithDebugInfo<Namespace> {
    let TokenWithDebugInfo {
        internal_tok:
            Namespace {
                id,
                functions,
                constants,
                structs,
                enums,
                sub_namespaces,
            },
        line,
        file,
    } = namespace;

    if id == "toplevel" && !is_toplevel {
        error(
            "'toplevel' namespace is reserved for the top-level namespace, use a different name",
            &namespace,
        );
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
        let Enum { id, variants } = enum_.internal_tok.clone();
        let id = format!("{}{}", namespace_path, id.internal_tok);

        let mut variant_set = HashSet::new();
        let mut variant_list = Vec::new();
        for variant in variants {
            if !variant_set.insert(variant.internal_tok.clone()) {
                error(
                    &format!(
                        "Enum '{}' has duplicate variant '{}'",
                        id, variant.internal_tok
                    ),
                    &enum_,
                );
            }
            variant_list.push(variant.internal_tok.clone());
        }
        context.enums.insert(id.clone(), variant_list);
    }

    let mut typed_constants = Vec::new();
    for constant in constants {
        let Constant::Constant(name, lit, var_type) = &constant.expr.internal_tok;
        let (line, file) = (constant.expr.line, constant.expr.file.clone());
        let name = format!("{}{}", namespace_path, name.internal_tok);

        let Typed {
            type_: expr_type, ..
        } = type_expression(
            &mut new_functions,
            TokenWithDebugInfo {
                internal_tok: Expression::Atom(Typed::new(TokenWithDebugInfo {
                    internal_tok: Atom::Literal(lit.clone()),
                    line,
                    file: file.clone(),
                })),
                line,
                file: file.clone(),
            },
            &mut context,
        );
        if var_type.internal_tok != expr_type {
            error(
                &format!(
                    "Constant '{}' type ({:?}) does not match expression type ({:?})",
                    name, var_type.internal_tok, expr_type
                ),
                &constant.expr,
            );
        } else {
            context
                .variables
                .insert(name.clone(), var_type.internal_tok.clone());
        }

        typed_constants.push(Typed {
            expr: TokenWithDebugInfo {
                internal_tok: Constant::Constant(
                    TokenWithDebugInfo {
                        internal_tok: name.clone(),
                        line: line,
                        file: file.clone(),
                    },
                    lit.clone(),
                    var_type.clone(),
                ),
                line: line,
                file: file.clone(),
            },
            type_: var_type.internal_tok.clone(),
        });
    }

    for struct_ in structs {
        let Struct {
            id,
            members,
            generics,
        } = struct_.internal_tok.clone();

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
        let Function { generics, .. } = &function.expr.internal_tok;
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
        )
        .internal_tok;

        new_functions.extend(sub_functions);
        typed_constants.extend(sub_constants);
        new_structs.extend(sub_structs);
        new_enums.extend(sub_enums);
    }

    TokenWithDebugInfo {
        internal_tok: Namespace {
            id: id.to_string(),
            functions: new_functions,
            constants: typed_constants,
            structs: new_structs,
            enums: new_enums,
            sub_namespaces: vec![],
        },
        line: *line,
        file: file.clone(),
    }
}

pub fn check_program(ast: &Ast) -> Ast {
    // Check that there is a main function, no function is called _start and no
    // function is declared twice
    let mut main_found = false;
    let mut function_names = HashSet::new();

    let Namespace { functions, .. } = &ast.program.internal_tok;
    for function in functions {
        let Function {
            id: name,
            return_type: ret_type,
            generics,
            ..
        } = &function.expr.internal_tok;
        if name.internal_tok == "main" {
            main_found = true;
            if generics.len() > 0 {
                error("Main function cannot have generics", &function.expr);
            }
            if ret_type.internal_tok != Type::Int {
                error("Main function must return an integer", &function.expr);
            }
        } else if name.internal_tok == "_start" {
            error(
				"Function cannot be called '_start'. Function '_start' is reserved for the runtime, use 'main' instead",
				&function.expr,
			);
        } else if !function_names.insert(name.internal_tok.clone()) {
            error(
                &format!(
                    "Function '{}' is declared more than once",
                    name.internal_tok
                ),
                &function.expr,
            );
        }
    }

    if !main_found {
		error("Main function not found. Please define a function named 'main' that returns an integer.", &ast.program);
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
