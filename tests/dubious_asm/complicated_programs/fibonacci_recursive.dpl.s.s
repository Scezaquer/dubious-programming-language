[BITS 64]
section .text

global _start
_start:
    call main
    mov rdi, rax
    mov rax, 60
    syscall

global fib
fib:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
    ;if statement
    mov rax, [rbp+24]
    push rax
    mov rax, 0
    pop rcx
    xchg rax, rcx
    cmp rax, rcx
    sete al
    movzx rax, al
    cmp rax, 0
    je else_0
    mov rax, 0
    add rsp, 0		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 0		;end of block, pop local variables
    jmp end_0
else_0:
end_0:
    ;if statement
    mov rax, [rbp+24]
    push rax
    mov rax, 1
    pop rcx
    xchg rax, rcx
    cmp rax, rcx
    sete al
    movzx rax, al
    cmp rax, 0
    je else_1
    mov rax, 1
    add rsp, 0		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 0		;end of block, pop local variables
    jmp end_1
else_1:
end_1:
	;push function arguments to the stack in reverse order
    mov rax, [rbp+24]
    push rax
    mov rax, 1
    pop rcx
    xchg rax, rcx
    sub rax, rcx
    push rax
    call fib
    add rsp, 8	;pop arguments
    push rax
	;push function arguments to the stack in reverse order
    mov rax, [rbp+24]
    push rax
    mov rax, 2
    pop rcx
    xchg rax, rcx
    sub rax, rcx
    push rax
    call fib
    add rsp, 8	;pop arguments
    pop rcx
    xchg rax, rcx
    add rax, rcx
    add rsp, 0		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 0		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

global main
main:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
	;push function arguments to the stack in reverse order
    mov rax, 10
    push rax
    call fib
    add rsp, 8	;pop arguments
    add rsp, 0		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 0		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

section .data