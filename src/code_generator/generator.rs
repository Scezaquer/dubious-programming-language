use crate::ast::build_ast::{
    AssignmentOp, Ast, Atom, BinOp, Expression, Function, Program, Statement, UnOp,
};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Write;
use std::path;

// TODO: test all of these one by one (pain)

fn generate_atom(file: &mut File, atom: &Atom, var_map: &HashMap<&String, i64>) {
    match atom {
        Atom::Constant(constant) => {
            // NOTE: change the std::fmt::Display trait for Constant in build_ast.rs in case it doesn't print the asm correctly
            writeln!(file, "    mov rax, {}", constant).unwrap(); // TODO: Doesn't work for floats
        }
        Atom::Variable(variable) => {
			let var_address = var_map
				.get(&variable)
				.expect(format!("Undeclared variable {}", variable).as_str());
            writeln!(file, "    mov rax, [rbp-{}]", var_address).unwrap();
        }
        Atom::Expression(expression) => {
            generate_expression(file, expression, var_map);
        }
        /*Atom::FunctionCall(name, args) => {
            for arg in args.iter() {
                generate_expression(file, arg);
                writeln!(file, "push rax").unwrap();
            }
            writeln!(file, "call _{}", name).unwrap();
            writeln!(file, "add rsp, {}", 8 * args.len()).unwrap();
        },*/
        _ => unimplemented!(),
    }
}

fn generate_expression(file: &mut File, expression: &Expression, var_map: &HashMap<&String, i64>) {
    match expression {
        Expression::Atom(atom) => {
            generate_atom(file, atom, var_map);
        }
        Expression::UnaryOp(expr, unop) => {
            generate_expression(file, expr, var_map);
            match unop {
                UnOp::UnaryMinus => {
                    writeln!(file, "    neg rax").unwrap();
                }
                UnOp::BitwiseNot => {
                    writeln!(file, "    not rax").unwrap();
                }
                UnOp::LogicalNot => {
                    writeln!(file, "    not rax").unwrap();
                }
                UnOp::PreIncrement => {
                    writeln!(file, "    inc rax").unwrap();
                }
                UnOp::PreDecrement => {
                    writeln!(file, "    dec rax").unwrap();
                }
                UnOp::UnaryPlus => {
                    // Do nothing
                }
                UnOp::Dereference => {
                    writeln!(file, "    mov rax, [rax]").unwrap();
                }
                UnOp::AddressOf => {
                    // Do nothing
                }
                _ => unimplemented!(),
            }
        }
        Expression::BinaryOp(left, right, bin_op) => {
            generate_expression(file, left, var_map);
            writeln!(file, "    push rax").unwrap();
            generate_expression(file, right, var_map);
            writeln!(file, "    pop rcx").unwrap();
            writeln!(file, "    xchg rax, rcx").unwrap();
            match bin_op {
                BinOp::Add => {
                    writeln!(file, "    add rax, rcx").unwrap();
                }
                BinOp::Subtract => {
                    writeln!(file, "    sub rax, rcx").unwrap();
                }
                BinOp::Multiply => {
                    writeln!(file, "    imul rax, rcx").unwrap();
                }
                BinOp::Divide => {
                    writeln!(file, "    cqo").unwrap();
                    writeln!(file, "    idiv rcx").unwrap();
                }
                BinOp::Modulus => {
                    writeln!(file, "    cqo").unwrap();
                    writeln!(file, "    idiv rcx").unwrap();
                    writeln!(file, "    mov rax, rdx").unwrap();
                }
                BinOp::Exponent => {
                    writeln!(file, "    mov rbx, rax").unwrap();
                    writeln!(file, "    mov rcx, rax").unwrap();
                    writeln!(file, "    mov rdx, rcx").unwrap();
                    writeln!(file, "    mov rax, rbx").unwrap();
                    writeln!(file, "    mov rsi, rcx").unwrap();
                    writeln!(file, "    mov rdi, rdx").unwrap();
                    writeln!(file, "    call pow").unwrap(); //TODO
                }
                BinOp::LessThan => {
                    writeln!(file, "    cmp rax, rcx").unwrap();
                    writeln!(file, "    setl al").unwrap();
                    writeln!(file, "    movzx rax, al").unwrap();
                }
                BinOp::GreaterThan => {
                    writeln!(file, "    cmp rax, rcx").unwrap();
                    writeln!(file, "    setg al").unwrap();
                    writeln!(file, "    movzx rax, al").unwrap();
                }
                BinOp::Equal => {
                    writeln!(file, "    cmp rax, rcx").unwrap();
                    writeln!(file, "    sete al").unwrap();
                    writeln!(file, "    movzx rax, al").unwrap();
                }
                BinOp::NotEqual => {
                    writeln!(file, "    cmp rax, rcx").unwrap();
                    writeln!(file, "    setne al").unwrap();
                    writeln!(file, "    movzx rax, al").unwrap();
                }
                BinOp::LessOrEqualThan => {
                    writeln!(file, "    cmp rax, rcx").unwrap();
                    writeln!(file, "    setle al").unwrap();
                    writeln!(file, "    movzx rax, al").unwrap();
                }
                BinOp::GreaterOrEqualThan => {
                    writeln!(file, "    cmp rax, rcx").unwrap();
                    writeln!(file, "    setge al").unwrap();
                    writeln!(file, "    movzx rax, al").unwrap();
                }
                BinOp::BitwiseAnd => {
                    writeln!(file, "    and rax, rcx").unwrap();
                }
                BinOp::BitwiseXor => {
                    writeln!(file, "    xor rax, rcx").unwrap();
                }
                BinOp::BitwiseOr => {
                    writeln!(file, "    or rax, rcx").unwrap();
                }
                BinOp::LogicalAnd => {
                    writeln!(file, "    and rax, rcx").unwrap();
                }
                BinOp::LogicalOr => {
                    writeln!(file, "    or rax, rcx").unwrap();
                }
                BinOp::LogicalXor => {
                    writeln!(file, "    xor rax, rcx").unwrap();
                }
                BinOp::LeftShift => {
                    writeln!(file, "    shl rax, cl").unwrap();
                }
                BinOp::RightShift => {
                    writeln!(file, "    shr rax, cl").unwrap();
                }
                _ => unimplemented!(),
            }
        }
        Expression::Assignment(variable, expr, op) => {
            generate_expression(file, expr, var_map);
			let var_address = var_map
				.get(&variable)
				.expect(format!("Undeclared variable {}", variable).as_str());
            match op {
                AssignmentOp::Assign => {
                    writeln!(file, "    mov [rbp-{}], rax", var_address).unwrap();
                }
                AssignmentOp::AddAssign => {
                    writeln!(file, "    add [rbp-{}], rax", var_address).unwrap();
                }
                AssignmentOp::SubtractAssign => {
                    writeln!(file, "    sub [rbp-{}], rax", var_address).unwrap();
                }
                AssignmentOp::MultiplyAssign => {
                    writeln!(file, "    imul [rbp-{}], rax", var_address).unwrap();
                }
                AssignmentOp::DivideAssign => {
                    writeln!(file, "    cqo").unwrap();
                    writeln!(file, "    idiv [rbp-{}]", var_address).unwrap();
                }
                AssignmentOp::ModulusAssign => {
                    writeln!(file, "    cqo").unwrap();
                    writeln!(file, "    idiv [rbp-{}]", var_address).unwrap();
                    writeln!(file, "    mov rax, rdx").unwrap();
                }
                AssignmentOp::LeftShiftAssign => {
                    writeln!(file, "    mov rcx, rax").unwrap();
                    writeln!(file, "    shl [rbp-{}], cl", var_address).unwrap();
                }
                AssignmentOp::RightShiftAssign => {
                    writeln!(file, "    mov rcx, rax").unwrap();
                    writeln!(file, "    shr [rbp-{}], cl", var_address).unwrap();
                }
                AssignmentOp::BitwiseAndAssign => {
                    writeln!(file, "    and [rbp-{}], rax", var_address).unwrap();
                }
                AssignmentOp::BitwiseXorAssign => {
                    writeln!(file, "    xor [rbp-{}], rax", var_address).unwrap();
                }
                AssignmentOp::BitwiseOrAssign => {
                    writeln!(file, "    or [rbp-{}], rax", var_address).unwrap();
                }
                _ => unimplemented!(),
            }
        }
    }
}

fn generate_compound_statement(
    file: &mut File,
    cmp_statement: &Statement,
    var_map: &HashMap<&String, i64>,
    stack_index: &i64,
) {
    let mut var_map = var_map.clone();
    let mut stack_index = stack_index.clone();
	let mut context = HashSet::new();

	if let Statement::Compound(statements) = cmp_statement{
		for statement in statements.iter() {
			match statement {
				Statement::Return(expression) => {
					generate_expression(file, expression, &var_map);
					// Pop all the variables from the stack
					writeln!(file, "    add rsp, {}		;pop local variables before return", context.len() * 8).unwrap();
					writeln!(file, "    pop rbx		;restore rbx for caller function").unwrap();
					writeln!(file, "    pop rbp		;restore base pointer").unwrap();
					writeln!(file, "    ret").unwrap();
				}
				Statement::Expression(expression) => {
					generate_expression(file, expression, &var_map);
				}
				Statement::Assignment(variable, expr) => {
					generate_expression(file, expr, &var_map);
					let var_address = var_map
						.get(&variable)
						.expect(format!("Undeclared variable {}", variable).as_str());
					writeln!(file, "    mov [rbp-{}], rax", var_address).unwrap();
				}
				Statement::Let(variable, expr) => {
					if let Some(expr) = expr {
						generate_expression(file, expr, &var_map);
					}

					stack_index += 8;
					var_map.insert(variable, stack_index);
					let var_address = var_map
						.get(&variable)
						.expect(format!("Undeclared variable {}", variable).as_str());
					context.insert(variable.clone());

					if let Some(_) = expr {
						writeln!(file, "    push rax").unwrap();
					} else {
						writeln!(file, "    push 0").unwrap();
					}
				}
				Statement::Compound(_) => {
					generate_compound_statement(file, statement, &var_map, &stack_index);
				}
				_ => unimplemented!(),
			}
		}
	} else {
		panic!("Expected a compound statement, got {:?}", cmp_statement);
	}

	// Pop all the variables from the stack
	writeln!(file, "    add rsp, {}		;end of block, pop local variables", context.len() * 8).unwrap();
}

fn generate_function(file: &mut File, function: &Function) {
    let var_map: HashMap<&String, i64> = HashMap::new();
    let stack_index = 0;

    let Function::Function(name, _params, statement) = function;
    writeln!(file, "").unwrap();
    writeln!(file, "global {}", name).unwrap();
    writeln!(file, "{}:", name).unwrap();
	writeln!(file, "    push rbp		;save previous base pointer").unwrap();
    writeln!(file, "    push rbx		;functions should preserve rbx").unwrap();
	writeln!(file, "    mov rbp, rsp	;set base pointer").unwrap();
    generate_compound_statement(file, statement, &var_map, &stack_index);
}

pub fn generate(ast: &Ast, out_path: &str) {
    let path = path::Path::new(out_path);
    let mut file = File::create(path).unwrap();

    // Post-order traversal of the AST to generate x86_64 (+nasm pseudo-instructions)

    let Program::Program(function_vector) = &ast.program;
    writeln!(file, "[BITS 64]").unwrap();
    writeln!(file, "section .text").unwrap();
    writeln!(file, "").unwrap();
    writeln!(file, "global _start").unwrap();
    writeln!(file, "_start:").unwrap(); // Entry point
    writeln!(file, "    call main").unwrap(); // Call main
    writeln!(file, "    mov rdi, rax").unwrap(); // Return value of main
    writeln!(file, "    mov rax, 60").unwrap(); // Exit syscall
    writeln!(file, "    syscall").unwrap();

    for function in function_vector.iter() {
        generate_function(&mut file, function);
    }

    writeln!(file, "").unwrap();
    writeln!(file, "section .data").unwrap();
}
