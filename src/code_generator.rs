use crate::ast_build::{
    AssignmentOp, Ast, Atom, BinOp, Expression, Function, Program, Statement, UnOp,
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
fn generate_atom(file: &mut File, atom: &Atom, var_map: &HashMap<String, i64>) {
    match atom {
        Atom::Constant(constant) => {
            // NOTE: change the std::fmt::Display trait for Constant in build_ast.rs in case it doesn't print the asm correctly
            writeln!(file, "    mov rax, {}", constant).unwrap(); // TODO: Doesn't work for floats
        }
        Atom::Variable(variable) => {
            let var_address = var_map
                .get(variable)
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

/// Generates the assembly code for an assignment operation
fn generate_assignment(op: &AssignmentOp, file: &mut File, var_address: &i64) {
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
			writeln!(file, "    mov rcx, [rbp-{}]", var_address).unwrap();
			writeln!(file, "    imul rax, rcx").unwrap();
			writeln!(file, "    mov [rbp-{}], rax", var_address).unwrap();
        }
        AssignmentOp::DivideAssign => {
			writeln!(file, "    mov rcx, rax").unwrap();
			writeln!(file, "    mov rax, [rbp-{}]", var_address).unwrap();
			writeln!(file, "    cqo").unwrap();
			writeln!(file, "    idiv rcx").unwrap();
			writeln!(file, "    mov [rbp-{}], rax", var_address).unwrap();
        }
        AssignmentOp::ModulusAssign => {
			writeln!(file, "    mov rcx, rax").unwrap();
			writeln!(file, "    mov rax, [rbp-{}]", var_address).unwrap();
			writeln!(file, "    cqo").unwrap();
			writeln!(file, "    idiv rcx").unwrap();
			writeln!(file, "    mov [rbp-{}], rdx", var_address).unwrap();
        }
        AssignmentOp::LeftShiftAssign => {
			writeln!(file, "    mov rcx, rax").unwrap();
			writeln!(file, "    mov rax, [rbp-{}]", var_address).unwrap();
			writeln!(file, "    shl rax, cl").unwrap();
			writeln!(file, "    mov [rbp-{}], rax", var_address).unwrap();
        }
        AssignmentOp::RightShiftAssign => {
			writeln!(file, "    mov rcx, rax").unwrap();
			writeln!(file, "    mov rax, [rbp-{}]", var_address).unwrap();
			writeln!(file, "    shr rax, cl").unwrap();
			writeln!(file, "    mov [rbp-{}], rax", var_address).unwrap();
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

/// Generates the assembly code for an expression
fn generate_expression(file: &mut File, expression: &Expression, var_map: &HashMap<String, i64>) {
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
                .get(variable)
                .expect(format!("Undeclared variable {}", variable).as_str());
            generate_assignment(op, file, var_address);
        }
    }
}

/// Context struct to keep track of local variables and stack index
struct Context {
	/// HashMap to store the variables and their addresses
    var_map: HashMap<String, i64>,
	/// Stack index to keep track of the stack pointer
    stack_index: i64,
	/// HashSet to store the variables that were declared in the current block
    local_variables: HashSet<String>,
	/// Label a continue statement should jump to
    continue_label: Option<String>,
	/// Label a break statement should jump to
    break_label: Option<String>,
}

impl Context {
    fn new() -> Context {
        Context {
            var_map: HashMap::new(),
            stack_index: 0,
            local_variables: HashSet::new(),
            continue_label: None,
            break_label: None,
        }
    }

	/// Create a new context from the last context
	/// 
	/// This clones all the info from the last context except the local variables
    fn from_last_context(context: &Context) -> Context {
        Context {
            var_map: context.var_map.clone(),
            stack_index: context.stack_index.clone(),
            local_variables: HashSet::new(),
            continue_label: context.continue_label.clone(),
            break_label: context.break_label.clone(),
        }
    }

    fn len(&self) -> usize {
        self.local_variables.len()
    }

	/// Insert a variable into the context
    fn insert(&mut self, key: String, value: i64) {
        self.var_map.insert(key.clone(), value);
        self.local_variables.insert(key);
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
                    generate_expression(file, expression, &context.var_map);
                    // Pop all the variables from the stack
                    writeln!(
                        file,
                        "    add rsp, {}		;pop local variables before return",
                        context.len() * 8
                    )
                    .unwrap();
                    writeln!(file, "    pop rbx		;restore rbx for caller function").unwrap();
                    writeln!(file, "    pop rbp		;restore base pointer").unwrap();
                    writeln!(file, "    ret").unwrap();
                }
                Statement::Expression(expression) => {
                    generate_expression(file, expression, &context.var_map);
                }
                Statement::Assignment(variable, expr, op) => {
                    generate_expression(file, expr, &context.var_map);
                    let var_address = context
                        .var_map
                        .get(variable)
                        .expect(format!("Undeclared variable {}", variable).as_str());
					generate_assignment(op, file, var_address);
                }
                Statement::Let(variable, expr) => {
                    if let Some(expr) = expr {
                        generate_expression(file, expr, &context.var_map);
                    }

                    context.stack_index += 8;
                    context.insert(variable.clone(), context.stack_index);

                    if let Some(_) = expr {
                        writeln!(file, "    push rax").unwrap();
                    } else {
                        writeln!(file, "    push 0").unwrap();
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
                    generate_expression(file, condition, &context.var_map);
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
                    generate_expression(file, condition, &context.var_map);
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
                    generate_expression(file, condition, &context.var_map);
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
                    generate_expression(file, init, &context.var_map);
                    writeln!(file, "{}:", start_label).unwrap();
                    generate_expression(file, condition, &context.var_map);
                    writeln!(file, "    cmp rax, 0").unwrap();
                    writeln!(file, "    je {}", end_label).unwrap();
                    generate_compound_statement(file, statement, &context);
                    generate_expression(file, update, &context.var_map);
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
                _ => unimplemented!(),
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
fn generate_function(file: &mut File, function: &Function) {
    let context = Context::new();

    let Function::Function(name, _params, statement) = function;
    writeln!(file, "").unwrap();
    writeln!(file, "global {}", name).unwrap();
    writeln!(file, "{}:", name).unwrap();
    writeln!(file, "    push rbp		;save previous base pointer").unwrap();
    writeln!(file, "    push rbx		;functions should preserve rbx").unwrap();
    writeln!(file, "    mov rbp, rsp	;set base pointer").unwrap();
    generate_compound_statement(file, statement, &context);
}

/// Generates the assembly code for the given AST
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
