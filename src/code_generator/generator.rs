use std::path;
use std::fs::File;
use crate::ast::build_ast::{Ast, Program, Function, Statement, Expression, Atom, UnOp, BinOp};
use std::io::Write;

// TODO: test all of these one by one (pain)

fn generate_atom(file: &mut File, atom: &Atom){
    match atom {
        Atom::Constant(constant) => {
            writeln!(file, "    mov rax, {}", constant).unwrap(); // TODO: Doesn't work for floats
        },
        Atom::Variable(variable) => {
            writeln!(file, "    mov rax, [rbp-{}]", variable).unwrap();
        },
        Atom::Expression(expression) => {
            generate_expression(file, expression);
        },
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

fn generate_expression(file: &mut File, expression: &Expression) {
    match expression {
        Expression::Atom(atom) => {
            generate_atom(file, atom);
        },
        Expression::UnaryOp(expr, unop) => {
            generate_expression(file, expr);
            match unop {
                UnOp::UnaryMinus => {
                    writeln!(file, "    neg rax").unwrap();
                },
                UnOp::BitwiseNot => {
                    writeln!(file, "    not rax").unwrap();
                },
                UnOp::LogicalNot => {
                    writeln!(file, "    not rax").unwrap();
                },
                UnOp::PreIncrement => {
                    writeln!(file, "    inc rax").unwrap();
                },
                UnOp::PreDecrement => {
                    writeln!(file, "    dec rax").unwrap();
                },
                UnOp::UnaryPlus => {
                    // Do nothing
                },
                UnOp::Dereference => {
                    writeln!(file, "    mov rax, [rax]").unwrap();
                },
                UnOp::AddressOf => {
                    // Do nothing
                },
                _ => unimplemented!(),
            }
        },
        Expression::BinaryOp(left, right, bin_op) => {
            generate_expression(file, left);
            writeln!(file, "    push rax").unwrap();
            generate_expression(file, right);
            writeln!(file, "    pop rcx").unwrap();
            writeln!(file, "    xchg rax, rcx").unwrap();
            match bin_op {
                BinOp::Add => {
                    writeln!(file, "    add rax, rcx").unwrap();
                },
                BinOp::Subtract => {
                    writeln!(file, "    sub rax, rcx").unwrap();
                },
                BinOp::Multiply => {
                    writeln!(file, "    imul rax, rcx").unwrap();
                },
                BinOp::Divide => {
                    writeln!(file, "    cqo").unwrap();
                    writeln!(file, "    idiv rcx").unwrap();
                },
                BinOp::Modulus => {
                    writeln!(file, "    cqo").unwrap();
                    writeln!(file, "    idiv rcx").unwrap();
                    writeln!(file, "    mov rax, rdx").unwrap();
                },
                BinOp::Exponent => {
                    writeln!(file, "    mov rbx, rax").unwrap();
                    writeln!(file, "    mov rcx, rax").unwrap();
                    writeln!(file, "    mov rdx, rcx").unwrap();
                    writeln!(file, "    mov rax, rbx").unwrap();
                    writeln!(file, "    mov rsi, rcx").unwrap();
                    writeln!(file, "    mov rdi, rdx").unwrap();
                    writeln!(file, "    call pow").unwrap();//TODO
                },
                BinOp::LessThan => {
                    writeln!(file, "    cmp rax, rcx").unwrap();
                    writeln!(file, "    setl al").unwrap();
                    writeln!(file, "    movzx rax, al").unwrap();
                },
                BinOp::GreaterThan => {
                    writeln!(file, "    cmp rax, rcx").unwrap();
                    writeln!(file, "    setg al").unwrap();
                    writeln!(file, "    movzx rax, al").unwrap();
                },
                BinOp::Equal => {
                    writeln!(file, "    cmp rax, rcx").unwrap();
                    writeln!(file, "    sete al").unwrap();
                    writeln!(file, "    movzx rax, al").unwrap();
                },
                BinOp::NotEqual => {
                    writeln!(file, "    cmp rax, rcx").unwrap();
                    writeln!(file, "    setne al").unwrap();
                    writeln!(file, "    movzx rax, al").unwrap();
                },
                BinOp::LessOrEqualThan => {
                    writeln!(file, "    cmp rax, rcx").unwrap();
                    writeln!(file, "    setle al").unwrap();
                    writeln!(file, "    movzx rax, al").unwrap();
                },
                BinOp::GreaterOrEqualThan => {
                    writeln!(file, "    cmp rax, rcx").unwrap();
                    writeln!(file, "    setge al").unwrap();
                    writeln!(file, "    movzx rax, al").unwrap();
                },
                BinOp::BitwiseAnd => {
                    writeln!(file, "    and rax, rcx").unwrap();
                },
                BinOp::BitwiseXor => {
                    writeln!(file, "    xor rax, rcx").unwrap();
                },
                BinOp::BitwiseOr => {
                    writeln!(file, "    or rax, rcx").unwrap();
                },
                BinOp::LogicalAnd => {
                    writeln!(file, "    and rax, rcx").unwrap();
                },
                BinOp::LogicalOr => {
                    writeln!(file, "    or rax, rcx").unwrap();
                },
                BinOp::LogicalXor => {
                    writeln!(file, "    xor rax, rcx").unwrap();
                },
                BinOp::LeftShift => {
                    writeln!(file, "    shl rax, cl").unwrap();
                },
                BinOp::RightShift => {
                    writeln!(file, "    shr rax, cl").unwrap();
                },
                _ => unimplemented!(),
                /*BinOp::Assign => {
                    writeln!(file, "    mov [rbp-{}], rax", right).unwrap();
                },
                BinOp::AddAssign => {
                    writeln!(file, "    add [rbp-{}], rax", right).unwrap();
                },
                BinOp::SubtractAssign => {
                    writeln!(file, "    sub [rbp-{}], rax", right).unwrap();
                },
                BinOp::MultiplyAssign => {
                    writeln!(file, "    imul [rbp-{}], rax", right).unwrap();
                },
                BinOp::DivideAssign => {
                    writeln!(file, "    cqo").unwrap();
                    writeln!(file, "    idiv [rbp-{}]", right).unwrap();
                },
                BinOp::ModulusAssign => {
                    writeln!(file, "    cqo").unwrap();
                    writeln!(file, "    idiv [rbp-{}]", right).unwrap();
                    writeln!(file, "    mov rax, rdx").unwrap();
                },
                BinOp::LeftShiftAssign => {
                    writeln!(file, "    mov rcx, rax").unwrap();
                    writeln!(file, "    shl [rbp-{}], cl", right).unwrap();
                },
                BinOp::RightShiftAssign => {
                    writeln!(file, "    mov rcx, rax").unwrap();
                    writeln!(file, "    shr [rbp-{}], cl", right).unwrap();
                },
                BinOp::BitwiseAndAssign => {
                    writeln!(file, "    and [rbp-{}], rax", right).unwrap();
                },
                BinOp::BitwiseXorAssign => {
                    writeln!(file, "    xor [rbp-{}], rax", right).unwrap();
                },
                BinOp::BitwiseOrAssign => {
                    writeln!(file, "    or [rbp-{}], rax", right).unwrap();
                },*/
            }
        },
    }
}


fn generate_statement(file: &mut File, statement: &Statement) {
    match statement {
        Statement::Return(expression) => {
            generate_expression(file, expression);
            writeln!(file, "    pop rbx").unwrap();
            writeln!(file, "    ret").unwrap();
        },
        Statement::Expression(expression) => {
            generate_expression(file, expression);
        },
        _ => unimplemented!(),
    }
}

fn generate_function(file: &mut File, function: &Function) {
    let Function::Function(name, _params, statements) = function;
    writeln!(file, "").unwrap();
    writeln!(file, "global {}", name).unwrap();
    writeln!(file, "{}:", name).unwrap();
    writeln!(file, "    push rbx").unwrap();
    for statement in statements.iter() {
        generate_statement(file, statement);
    }
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
    writeln!(file, "_start:").unwrap();             // Entry point
    writeln!(file, "    call main").unwrap();       // Call main
    writeln!(file, "    mov rdi, rax").unwrap();    // Return value of main
    writeln!(file, "    mov rax, 60").unwrap();     // Exit syscall
    writeln!(file, "    syscall").unwrap();

    for function in function_vector.iter(){
        generate_function(&mut file, function); 
    }

    writeln!(file, "").unwrap();
    writeln!(file, "section .data").unwrap();
}