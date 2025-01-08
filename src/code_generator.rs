use crate::ast_build::{
    AssignmentIdentifier, AssignmentOp, Ast, Atom, BinOp, Constant, Expression, Function, Program, Statement, Struct, UnOp
};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Write;
use std::path;
use std::sync::atomic::AtomicUsize;

// Separate counters for branch and loop labels for break and continue statements
/// Branch label counter for if statements
static BRANCH_LABEL: AtomicUsize = AtomicUsize::new(0);
/// Loop label counter for while, loop, and for statements
static LOOP_LABEL: AtomicUsize = AtomicUsize::new(0);

// TODO: test all of these one by one (pain)

/// Generates the assembly code for an atom
fn generate_atom(file: &mut File, atom: &Atom, context: &mut Context) {
    match atom {
        Atom::Literal(constant) => {
            // NOTE: change the std::fmt::Display trait for Constant in build_ast.rs in case it doesn't print the asm correctly
            writeln!(file, "    mov rax, {}", constant).unwrap(); // TODO: Doesn't work for floats
        }
        Atom::Variable(variable) => {
            let var_address = context.var_map.get(variable);
			if let Some(var_address) = var_address {
				writeln!(file, "    mov rax, [rbp{}{}]", if *var_address < 0 { "" } else { "+" }, var_address).unwrap();
				return;
			}
			
			let constant = context.constants.get(variable).expect(format!("Undeclared variable {}", variable).as_str());
			writeln!(file, "    mov rax, {}", constant).unwrap();
        }
        Atom::Expression(expression) => {
            generate_expression(file, expression, context);
        }
		Atom::FunctionCall(name, args) => {
			writeln!(file, "	;push function arguments to the stack in reverse order").unwrap();
			for arg in args.iter().rev() {		// Push arguments in reverse order (C convention)
				generate_expression(file, arg, context);
				writeln!(file, "    push rax").unwrap();
			}
			writeln!(file, "    call {}", name).unwrap();
			writeln!(file, "    add rsp, {}	;pop arguments", args.len() * 8).unwrap();	// Pop arguments from stack
		}
		Atom::ArrayAccess(variable, indices) => {
			let var_address = context.var_map.get(variable).expect(format!("Undeclared variable {}", variable).as_str()).clone();

			// TODO: Insert runtime checks for array bounds
			// Don't forget multi-dimensional array bounds:
			// if i >= dim1 || j >= dim2 || k >= dim3 || ... { panic!() }
			// if i < 0 || j < 0 || k < 0 || ... { panic!() }
			// Yes, this will make the code slower, but it's better than undefined behavior

			if indices.len() == 1 {
				generate_expression(file, &indices[0], context);
				writeln!(file, "    mov rcx, rax").unwrap();
				writeln!(file, "    mov rax, rbp").unwrap();
				if var_address < 0 {
					writeln!(file, "	sub rax, {}", var_address.abs()).unwrap();
				} else {
					writeln!(file, "	add rax, {}", var_address).unwrap();
				}
				// At this stage, the value at address rax is a pointer to the beginning of the array
				writeln!(file, "	mov rax, [rax]").unwrap();
				writeln!(file, "    mov rax, [rax + rcx * 8]").unwrap();
				return;
			} else if indices.len() == *context.dimensions.get(variable).expect(format!("Undeclared array {}", variable).as_str()) {
				// Indexing a multi-dimensional array
				// Calculate the address of the element arr[i, j, k, ...] = arr + ((i * dim1 + j) * dim2 + k) * dim3 + ...
				// Use rcx as accumulator
				writeln!(file, "    mov rcx, 0").unwrap();
				let mut current_dim = 0;
				for index in indices.iter() {
					generate_expression(file, index, context);
					// get the dimension from the stack into rdx
					let dim = context.var_map.get(&format!("{}:dim{}", variable, current_dim)).expect(format!("Undeclared array {}", variable).as_str());
					writeln!(file, "    mov rdx, [rbp{}{}]", if var_address < 0 { "" } else { "+" }, dim).unwrap();
					
					// rcx = rcx * dim + rax
					writeln!(file, "    imul rcx, rdx").unwrap();
					writeln!(file, "    add rcx, rax").unwrap();
					current_dim += 1;
				}
				// Move the base address of the array to rax
				writeln!(file, "	;Move the base address of the array to rax").unwrap();
				writeln!(file, "    mov rax, rbp").unwrap();
				if var_address < 0 {
					writeln!(file, "	sub rax, {}", var_address.abs()).unwrap();
				} else {
					writeln!(file, "	add rax, {}", var_address).unwrap();
				}
				// At this stage, the value at address rax is a pointer to the beginning of the array
				writeln!(file, "	mov rax, [rax]").unwrap();
				writeln!(file, "    mov rax, [rax + rcx * 8]").unwrap();
			} else {
				panic!("Incorrect number of indices for array access: expected {}, got {}", context.dimensions.get(variable).expect(format!("Undeclared array {}", variable).as_str()), indices.len());
			}
		}
		Atom::Array(expressions, _) => {
				// TODO: Push array's dimensions on the stack upon creation, refer 
				// back to them when accessing the array

				// Push expressions on stack in reverse order
				for expr in expressions.iter().rev() {
					generate_expression(file, expr, context);
					writeln!(file, "    push rax").unwrap();
					context.stack_index -= 8;
					context.len += 1;
				}
				// Move the address of the array to rax
				writeln!(file, "    mov rax, rsp	; Move the address of the array to rax").unwrap();
			}
		Atom::StructInstance(_, members) => {
			// Like an array, but with named fields
			// Push expressions on stack in reverse order
			for expr in members.iter().rev() {
				generate_expression(file, expr, context);
				writeln!(file, "    push rax").unwrap();
				context.stack_index -= 8;
				context.len += 1;
			}
			// Move the address of the struct to rax
			writeln!(file, "    mov rax, rsp	; Move the address of the struct to rax").unwrap();
		}
		Atom::MemberAccess(id, member) => {
			let member = member.as_ref();
			match member{
				Atom::Variable(name) => {
					let var_address = context.var_map.get(id).expect(format!("Undeclared variable {}", id).as_str()).clone();

					let var_type = context.var_types.get(id).expect(format!("Undeclared variable {}", id).as_str());

					writeln!(file, "    mov rcx, {}", context.structs.get(var_type).unwrap().get(name).unwrap()).unwrap();
					writeln!(file, "    mov rax, rbp").unwrap();
					if var_address < 0 {
						writeln!(file, "	sub rax, {}", var_address.abs()).unwrap();
					} else {
						writeln!(file, "	add rax, {}", var_address).unwrap();
					}
					// At this stage, the value at address rax is a pointer to the beginning of the array
					writeln!(file, "	mov rax, [rax]").unwrap();
					writeln!(file, "    mov rax, [rax + rcx * 8]").unwrap();
					return;
				}
				Atom::MemberAccess(id, sub_member) => {
					panic!("Nested struct members are not supported yet");
				}
				_ => { panic!("Expected a variable, got {:?}", member); }
			}
		}
        //_ => unimplemented!(),
    }
}

/// Generates the assembly code for an assignment operation
fn generate_assignment(op: &AssignmentOp, file: &mut File, var_address: &i64, pointer_dereference: bool) {
	let write_address;
	if pointer_dereference {
		write_address = format!("[rax]");
	} else {
		write_address = format!("[rbp{}{}]", if *var_address < 0 { "" } else { "+" }, var_address);
	}
    match op {
        AssignmentOp::Assign => {
            writeln!(file, "    mov {}, rax", write_address).unwrap();
        }
        AssignmentOp::AddAssign => {
            writeln!(file, "    add {}, rax", write_address).unwrap();
        }
        AssignmentOp::SubtractAssign => {
            writeln!(file, "    sub {}, rax", write_address).unwrap();
        }
        AssignmentOp::MultiplyAssign => {
			writeln!(file, "    mov rcx, {}", write_address).unwrap();
			writeln!(file, "    imul rax, rcx").unwrap();
			writeln!(file, "    mov {}, rax", write_address).unwrap();
        }
        AssignmentOp::DivideAssign => {
			writeln!(file, "    mov rcx, rax").unwrap();
			writeln!(file, "    mov rax, {}", write_address).unwrap();
			writeln!(file, "    cqo").unwrap();
			writeln!(file, "    idiv rcx").unwrap();
			writeln!(file, "    mov {}, rax", write_address).unwrap();
        }
        AssignmentOp::ModulusAssign => {
			writeln!(file, "    mov rcx, rax").unwrap();
			writeln!(file, "    mov rax, {}", write_address).unwrap();
			writeln!(file, "    cqo").unwrap();
			writeln!(file, "    idiv rcx").unwrap();
			writeln!(file, "    mov {}, rdx", write_address).unwrap();
        }
        AssignmentOp::LeftShiftAssign => {
			writeln!(file, "    mov rcx, rax").unwrap();
			writeln!(file, "    mov rax, {}", write_address).unwrap();
			writeln!(file, "    shl rax, cl").unwrap();
			writeln!(file, "    mov {}, rax", write_address).unwrap();
        }
        AssignmentOp::RightShiftAssign => {
			writeln!(file, "    mov rcx, rax").unwrap();
			writeln!(file, "    mov rax, {}", write_address).unwrap();
			writeln!(file, "    shr rax, cl").unwrap();
			writeln!(file, "    mov {}, rax", write_address).unwrap();
        }
        AssignmentOp::BitwiseAndAssign => {
            writeln!(file, "    and {}, rax", write_address).unwrap();
        }
        AssignmentOp::BitwiseXorAssign => {
            writeln!(file, "    xor {}, rax", write_address).unwrap();
        }
        AssignmentOp::BitwiseOrAssign => {
            writeln!(file, "    or {}, rax", write_address).unwrap();
        }
        _ => unimplemented!(),
    }
}

/// Generates the assembly code for an expression
fn generate_expression(file: &mut File, expression: &Expression, context: &mut Context) {
    match expression {
        Expression::Atom(atom) => {
            generate_atom(file, atom, context);
        }
        Expression::UnaryOp(expr, unop) => {
            generate_expression(file, expr, context);
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
            generate_expression(file, left, context);
            writeln!(file, "    push rax").unwrap();
            generate_expression(file, right, context);
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
			generate_expression(file, expr, context);
			
			let mut number_of_dereferences = 0;
			let mut variable = variable;
			while let AssignmentIdentifier::Dereference(new_expr) = variable {
				variable = new_expr;
				number_of_dereferences += 1;
			}

			if let AssignmentIdentifier::Variable(variable) = variable {
				let var_address = context.var_map.get(variable);
				let dereference = number_of_dereferences > 0;
				while number_of_dereferences > 0 {
					writeln!(file, "    mov rax, [rax]").unwrap();
					number_of_dereferences -= 1;
				}

				if let Some(var_address) = var_address {
					generate_assignment(op, file, var_address, dereference);
					return;
				}

				if let Some(_) = context.constants.get(variable) {
					panic!("Cannot assign to a constant variable");
				}
				panic!("Undeclared variable {:?}", variable);
			}
        }
		Expression::TypeCast(expr, t) => {
			// Unsure if I should be doing something here or nah
			generate_expression(file, expr, context);
		}
    }
}

/// Context struct to keep track of local variables and stack index
struct Context {
	/// HashMap to store the variables and their addresses
    var_map: HashMap<String, i64>,
	/// HashMap to store the var types
	var_types: HashMap<String, String>,
	/// Set of constant variables
	constants: HashSet<String>,
	/// Structs
	structs: HashMap<String, HashMap<String, i64>>,
	/// Stack index to keep track of the stack pointer
    stack_index: i64,
	/// HashSet to store the variables that were declared in the current block
    local_variables: HashSet<String>,
	/// Label a continue statement should jump to
    continue_label: Option<String>,
	/// Label a break statement should jump to
    break_label: Option<String>,
	/// Length of the context (in number of 64 bit words)
	len: usize,
	/// Arrays number of dimensions
	dimensions: HashMap<String, usize>,
}

impl Context {
    fn new() -> Context {
        Context {
            var_map: HashMap::new(),
			var_types: HashMap::new(),
			constants: HashSet::new(),
			structs: HashMap::new(),
            stack_index: 0,
            local_variables: HashSet::new(),
            continue_label: None,
            break_label: None,
			len: 0,
			dimensions: HashMap::new(),
        }
    }

	/// Create a new context from the last context
	/// 
	/// This clones all the info from the last context except the local variables
    fn from_last_context(context: &Context) -> Context {
        Context {
            var_map: context.var_map.clone(),
			var_types: context.var_types.clone(),
			constants: context.constants.clone(),
			structs: context.structs.clone(),
            stack_index: context.stack_index.clone(),
            local_variables: HashSet::new(),
            continue_label: context.continue_label.clone(),
            break_label: context.break_label.clone(),
			len: 0,
			dimensions: context.dimensions.clone(),
        }
    }

	/// Insert a variable into the context
    fn insert(&mut self, key: String, value: i64, var_type: String) {
        self.var_map.insert(key.clone(), value);
		self.var_types.insert(key.clone(), var_type);
        self.local_variables.insert(key);
		self.len += 1;
    }
}

/// Generates the assembly code for a compound statement
fn generate_compound_statement(file: &mut File, cmp_statement: &Statement, last_context: &Context) {
    let mut context = Context::from_last_context(last_context);

    // Yes I do need to generate compound statements like that otherwise
    // local variables can't be a thing

    if let Statement::Compound(statements) = cmp_statement {
        for statement in statements.iter() {
            match statement {
                Statement::Return(expression) => {
                    generate_expression(file, expression,  &mut context);
                    // Pop all the variables from the stack
                    writeln!(
                        file,
                        "    add rsp, {}		;pop local variables before return",
                        context.len * 8
                    )
                    .unwrap();
                    writeln!(file, "    pop rbx		;restore rbx for caller function").unwrap();
                    writeln!(file, "    pop rbp		;restore base pointer").unwrap();
                    writeln!(file, "    ret").unwrap();
                }
                Statement::Expression(expression) => {
                    generate_expression(file, expression, &mut context);
                }
                Statement::Let(variable, expr, var_type) => {
                    if let Some(expr) = expr {
                        generate_expression(file, expr, &mut context);
                    }

					// Dereference the variable if it is a pointer
					let mut number_of_dereferences = 0;
					let mut variable = variable;
					while let AssignmentIdentifier::Dereference(new_expr) = variable {
						variable = new_expr;
						number_of_dereferences += 1;
					}

					if let Some(_) = expr {
						writeln!(file, "    push rax").unwrap();
					} else {
						writeln!(file, "    push 0").unwrap();	// Undefined variables default to 0
					}
					context.stack_index -= 8;

					while number_of_dereferences > 0 {
						writeln!(file, "    push rsp").unwrap();
						number_of_dereferences -= 1;
						context.stack_index -= 8;
						context.len += 1;
					}
					
					if let AssignmentIdentifier::Variable(variable) = variable {
						context.insert(variable.clone(), context.stack_index, var_type.to_string());
					} else if let AssignmentIdentifier::Array(variable, dimensions) = variable {
						context.insert(variable.clone(), context.stack_index, var_type.to_string());

						let mut dimensions = dimensions.clone();
						context.dimensions.insert(variable.clone(), dimensions.len());
						// Push the dimensions on the stack
						while let Some(expr) = dimensions.pop() {
							generate_expression(file, &expr, &mut context);
							writeln!(file, "    push rax	;pushing array dimensions onto stack").unwrap();
							context.stack_index -= 8;
							context.insert(format!("{}:dim{}", variable, dimensions.len()), context.stack_index, "int".to_string());
						}
					} else {
						panic!("Expected a variable or an array, got {:?}", variable);
					}
                }
                Statement::Compound(_) => {
                    generate_compound_statement(file, statement, &context);
                }
                Statement::If(condition, if_statement, else_statement) => {
                    let branch_label =
                        BRANCH_LABEL.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                    let else_label = format!("else_{}", branch_label);
                    let end_label = format!("end_{}", branch_label);

                    writeln!(file, "    ;if statement").unwrap();
                    generate_expression(file, condition, &mut context);
                    writeln!(file, "    cmp rax, 0").unwrap();
                    writeln!(file, "    je {}", else_label).unwrap();
                    generate_compound_statement(file, if_statement, &context);
                    writeln!(file, "    jmp {}", end_label).unwrap();
                    writeln!(file, "{}:", else_label).unwrap();
                    if let Some(else_statement) = else_statement {
                        generate_compound_statement(file, else_statement, &context);
                    }
                    writeln!(file, "{}:", end_label).unwrap();
                }
                Statement::While(condition, statement) => {
                    let loop_label = LOOP_LABEL.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                    let start_label = format!("while_start_{}", loop_label);
                    let end_label = format!("while_end_{}", loop_label);

                    context.continue_label = Some(start_label.clone());
                    context.break_label = Some(end_label.clone());

                    writeln!(file, "    ;while statement").unwrap();
                    writeln!(file, "{}:", start_label).unwrap();
                    generate_expression(file, condition, &mut context);
                    writeln!(file, "    cmp rax, 0").unwrap();
                    writeln!(file, "    je {}", end_label).unwrap();
                    generate_compound_statement(file, statement, &context);
                    writeln!(file, "    jmp {}", start_label).unwrap();
                    writeln!(file, "{}:", end_label).unwrap();
                }
                Statement::Loop(statement) => {
                    let loop_label = LOOP_LABEL.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                    let start_label = format!("loop_start_{}", loop_label);
                    let end_label = format!("loop_end_{}", loop_label);

                    context.continue_label = Some(start_label.clone());
                    context.break_label = Some(end_label.clone());

                    writeln!(file, "    ;loop statement").unwrap();
                    writeln!(file, "{}:", start_label).unwrap();
                    generate_compound_statement(file, statement, &context);
                    writeln!(file, "    jmp {}", start_label).unwrap();
                    writeln!(file, "{}:", end_label).unwrap();
                }
                Statement::Dowhile(condition, statement) => {
                    let loop_label = LOOP_LABEL.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                    let start_label = format!("dowhile_start_{}", loop_label);
                    let end_label = format!("dowhile_end_{}", loop_label);

                    context.continue_label = Some(start_label.clone());
                    context.break_label = Some(end_label.clone());

                    writeln!(file, "    ;do while statement").unwrap();
                    writeln!(file, "{}:", start_label).unwrap();
                    generate_compound_statement(file, statement, &context);
                    generate_expression(file, condition, &mut context);
                    writeln!(file, "    cmp rax, 0").unwrap();
                    writeln!(file, "    jne {}", start_label).unwrap();
                    writeln!(file, "{}:", end_label).unwrap();
                }
                Statement::For(init, condition, update, statement) => {
                    let loop_label = LOOP_LABEL.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                    let start_label = format!("for_start_{}", loop_label);
                    let end_label = format!("for_end_{}", loop_label);

                    context.continue_label = Some(start_label.clone());
                    context.break_label = Some(end_label.clone());

                    writeln!(file, "    ;for statement").unwrap();
                    generate_expression(file, init, &mut context);
                    writeln!(file, "{}:", start_label).unwrap();
                    generate_expression(file, condition, &mut context);
                    writeln!(file, "    cmp rax, 0").unwrap();
                    writeln!(file, "    je {}", end_label).unwrap();
                    generate_compound_statement(file, statement, &context);
                    generate_expression(file, update, &mut context);
                    writeln!(file, "    jmp {}", start_label).unwrap();
                    writeln!(file, "{}:", end_label).unwrap();
                }
                Statement::Break => {
                    if let Some(label) = &context.break_label {
                        writeln!(file, "    jmp {}	;break statement", label).unwrap();
                    } else {
                        panic!("Break statement outside of loop");
                    }
                }
                Statement::Continue => {
                    if let Some(label) = &context.continue_label {
                        writeln!(file, "    jmp {}	;continue statement", label).unwrap();
                    } else {
                        panic!("Continue statement outside of loop");
                    }
                }
                //_ => unimplemented!(),
            }
        }
    } else {
        panic!("Expected a compound statement, got {:?}", cmp_statement);
    }

    // Pop all the variables from the stack
    writeln!(
        file,
        "    add rsp, {}		;end of block, pop local variables",
        context.local_variables.len() * 8
    )
    .unwrap();
}

/// Generates the assembly code for a function
fn generate_function(file: &mut File, function: &Function, context: &Context) {
    let mut context = Context::from_last_context(context);

    let Function::Function(name, params, statement, return_type) = function;

	context.stack_index = 24;	// Skip rbp, rbx and the return address
	for param in params.iter() {
		context.insert(param.0.clone(), context.stack_index, param.1.to_string());
		context.stack_index += 8;
	}
	context.stack_index = 0;

    writeln!(file, "").unwrap();
    writeln!(file, "global {}", name).unwrap();
    writeln!(file, "{}:", name).unwrap();
    writeln!(file, "    push rbp		;save previous base pointer").unwrap();
    writeln!(file, "    push rbx		;functions should preserve rbx").unwrap();
    writeln!(file, "    mov rbp, rsp	;set base pointer").unwrap();
    generate_compound_statement(file, statement, &context);
	writeln!(file, "    pop rbx			;restore rbx for caller function").unwrap();
	writeln!(file, "    pop rbp			;restore base pointer").unwrap();
	writeln!(file, "    ret				;return by default if no return statement was reached").unwrap();
}

fn generate_constant(file: &mut File, constant: &Constant) {
	let Constant::Constant(name, constant, _) = constant;
	writeln!(file, "    {} equ {}", name, constant).unwrap();
}

/// Generates the assembly code for the given AST
pub fn generate(ast: &Ast, out_path: &str) {
    let path = path::Path::new(out_path);
    let mut file = File::create(path).unwrap();

    // Post-order traversal of the AST to generate x86_64 (+nasm pseudo-instructions)

	// TODO: structs
    let Program::Program(function_vector, const_vector, structs) = &ast.program;
    writeln!(file, "[BITS 64]").unwrap();
    writeln!(file, "section .text").unwrap();
    writeln!(file, "").unwrap();
    writeln!(file, "global _start").unwrap();
    writeln!(file, "_start:").unwrap(); // Entry point
    writeln!(file, "    call main").unwrap(); // Call main
    writeln!(file, "    mov rdi, rax").unwrap(); // Return value of main
    writeln!(file, "    mov rax, 60").unwrap(); // Exit syscall
    writeln!(file, "    syscall").unwrap();

	let mut context = Context::new();

	for constant in const_vector.iter() {
		generate_constant(&mut file, constant);
		let Constant::Constant(name, _, _) = constant;
		if let Some(_) = context.var_map.get(name) {
			panic!("Variable {} already declared as a constant. Constants cannot be declared twice", name);
		}
		context.constants.insert(name.clone());
	}

	for struct_ in structs.iter() {
		let Struct{ id, members} = struct_;
		let mut member_names = HashMap::new();
		for (i, member) in members.iter().enumerate() {
			let (name, _) = member;
			member_names.insert(name.clone(), i as i64);
		}
		context.structs.insert(id.clone(), member_names);
	}

    for function in function_vector.iter() {
        generate_function(&mut file, function, &context);
    }

    writeln!(file, "").unwrap();
    writeln!(file, "section .data").unwrap();
}
