[BITS 64]
section .text

global _start
_start:
    call main
    mov rdi, rax
    mov rax, 60
    syscall

main:
.toplevel.main:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
    mov rax, 10
    push rax
    mov rax, 0
    push rax
    mov rax, 1
    push rax
    ;if statement
    mov rax, 0
    push rax
    mov rax, [rbp-8]
    pop rcx
    cmp rax, rcx
    sete al
    movzx rax, al
    cmp rax, 0
    je .else_0
    mov rax, [rbp-16]
    add rsp, 0		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 0		;end of block, pop local variables
    jmp .end_0
.else_0:
    ;if statement
    mov rax, 1
    push rax
    mov rax, [rbp-8]
    pop rcx
    cmp rax, rcx
    sete al
    movzx rax, al
    cmp rax, 0
    je .else_1
    mov rax, [rbp-24]
    add rsp, 0		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 0		;end of block, pop local variables
    jmp .end_1
.else_1:
.end_1:
    add rsp, 0		;end of block, pop local variables
.end_0:
    mov rax, 2
    sub [rbp-8], rax
    push 0
    push 0
    ;for statement
    mov rax, 0
    mov [rbp-40], rax
.for_start_0:
    mov rax, [rbp-8]
    push rax
    mov rax, [rbp-40]
    pop rcx
    cmp rax, rcx
    setle al
    movzx rax, al
    cmp rax, 0
    je .for_end_0
    mov rax, [rbp-16]
    push rax
    mov rax, [rbp-24]
    pop rcx
    add rax, rcx
    mov [rbp-32], rax
    mov rax, [rbp-24]
    mov [rbp-16], rax
    mov rax, [rbp-32]
    mov [rbp-24], rax
    add rsp, 0		;end of block, pop local variables
    mov rax, [rbp-40]
    inc rax
    mov [rbp-40], rax
    jmp .for_start_0
.for_end_0:
    mov rax, [rbp-32]
    add rsp, 40		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 40		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

section .data
