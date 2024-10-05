use std::path;
use std::fs::File;
use crate::ast::build_ast::{Ast, Program, Function, Statement, Expression, Term, Factor, UnaryOp, BinaryOp};
use std::io::Write;

fn generate_factor(file: &mut File, factor: &Factor){
    match factor {
        Factor::Constant(constant) => {
            writeln!(file, "    mov rax, {}", constant).unwrap();
        },
        Factor::Variable(variable) => {
            writeln!(file, "    mov rax, [rbp-{}]", variable).unwrap();
        },
        Factor::UnaryOp(factor, unary_op) => {
            generate_factor(file, factor);
            match unary_op {
                UnaryOp::Neg => {
                    writeln!(file, "    neg rax").unwrap();
                },
                UnaryOp::Not => {
                    writeln!(file, "    not rax").unwrap();
                },
                _ => unimplemented!(),
            }
        },
        /*Factor::FunctionCall(name, args) => {
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

fn generate_term(file: &mut File, term: &Term) {
    match term {
        Term::Factor(factor) => {
            generate_factor(file, factor);
        },
        Term::BinaryOp(left, right, op) => {
            generate_term(file, left);
            writeln!(file, "    push rax").unwrap();
            generate_factor(file, right);
            writeln!(file, "    pop rcx").unwrap();
            match op {
                BinaryOp::Add => {
                    writeln!(file, "    add rax, rcx").unwrap();
                },
                BinaryOp::Sub => {
                    writeln!(file, "    sub rax, rcx").unwrap();
                },
                BinaryOp::Mul => {
                    writeln!(file, "    imul rax, rcx").unwrap();
                },
                BinaryOp::Div => {
                    writeln!(file, "    cqo").unwrap();
                    writeln!(file, "    idiv rcx").unwrap();
                },
                BinaryOp::Mod => {
                    writeln!(file, "    cqo").unwrap();
                    writeln!(file, "    idiv rcx").unwrap();
                    writeln!(file, "    mov rax, rdx").unwrap();
                },
                BinaryOp::Pow => {
                    writeln!(file, "    mov rbx, rax").unwrap();
                    writeln!(file, "    mov rcx, rax").unwrap();
                    writeln!(file, "    mov rdx, rcx").unwrap();
                    writeln!(file, "    mov rax, rbx").unwrap();
                    writeln!(file, "    mov rsi, rcx").unwrap();
                    writeln!(file, "    mov rdi, rdx").unwrap();
                    writeln!(file, "    call pow").unwrap();//TODO
                },
                BinaryOp::And => {
                    writeln!(file, "    and rax, rcx").unwrap();
                },
                BinaryOp::Or => {
                    writeln!(file, "    or rax, rcx").unwrap();
                },
                BinaryOp::Less => {
                    writeln!(file, "    cmp rax, rcx").unwrap();
                    writeln!(file, "    setl al").unwrap();
                    writeln!(file, "    movzx rax, al").unwrap();
                },
                BinaryOp::Greater => {
                    writeln!(file, "    cmp rax, rcx").unwrap();
                    writeln!(file, "    setg al").unwrap();
                    writeln!(file, "    movzx rax, al").unwrap();
                },
                BinaryOp::Equal => {
                    writeln!(file, "    cmp rax, rcx").unwrap();
                    writeln!(file, "    sete al").unwrap();
                    writeln!(file, "    movzx rax, al").unwrap();
                },
                _ => unimplemented!(),
            }
        },
    }
}

fn generate_expression(file: &mut File, expression: &Expression) {
    match expression {
        Expression::Term(term) => {
            generate_term(file, term);
        },
        Expression::BinaryOp(left, right, op) => {
            generate_expression(file, left);
            writeln!(file, "    push rax").unwrap();
            generate_term(file, right);
            writeln!(file, "    pop rcx").unwrap();
            match op {
                BinaryOp::Add => {
                    writeln!(file, "    add rax, rcx").unwrap();
                },
                BinaryOp::Sub => {
                    writeln!(file, "    sub rax, rcx").unwrap();
                },
                BinaryOp::Mul => {
                    writeln!(file, "    imul rax, rcx").unwrap();
                },
                BinaryOp::Div => {
                    writeln!(file, "    cqo").unwrap();
                    writeln!(file, "    idiv rcx").unwrap();
                },
                BinaryOp::Mod => {
                    writeln!(file, "    cqo").unwrap();
                    writeln!(file, "    idiv rcx").unwrap();
                    writeln!(file, "    mov rax, rdx").unwrap();
                },
                BinaryOp::Pow => {
                    writeln!(file, "    mov rbx, rax").unwrap();
                    writeln!(file, "    mov rcx, rax").unwrap();
                    writeln!(file, "    mov rdx, rcx").unwrap();
                    writeln!(file, "    mov rax, rbx").unwrap();
                    writeln!(file, "    mov rsi, rcx").unwrap();
                    writeln!(file, "    mov rdi, rdx").unwrap();
                    writeln!(file, "    call pow").unwrap();//TODO
                },
                BinaryOp::And => {
                    writeln!(file, "    and rax, rcx").unwrap();
                },
                BinaryOp::Or => {
                    writeln!(file, "    or rax, rcx").unwrap();
                },
                BinaryOp::Less => {
                    writeln!(file, "    cmp rax, rcx").unwrap();
                    writeln!(file, "    setl al").unwrap();
                    writeln!(file, "    movzx rax, al").unwrap();
                },
                BinaryOp::Greater => {
                    writeln!(file, "    cmp rax, rcx").unwrap();
                    writeln!(file, "    setg al").unwrap();
                    writeln!(file, "    movzx rax, al").unwrap();
                },
                BinaryOp::Equal => {
                    writeln!(file, "    cmp rax, rcx").unwrap();
                    writeln!(file, "    sete al").unwrap();
                    writeln!(file, "    movzx rax, al").unwrap();
                },
                _ => unimplemented!(),
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
    let Function::Function(name, params, statements) = function;
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
    writeln!(file, "_start:").unwrap();
    writeln!(file, "    call main").unwrap();
    writeln!(file, "    mov rdi, rax").unwrap();
    writeln!(file, "    mov rax, 60").unwrap();
    writeln!(file, "    syscall").unwrap();

    for function in function_vector.iter(){
        generate_function(&mut file, function); 
    }

    writeln!(file, "").unwrap();
    writeln!(file, "section .data").unwrap();
}