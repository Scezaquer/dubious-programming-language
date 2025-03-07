[BITS 64]
section .text

global _start
_start:
    call main
    mov rdi, rax
    mov rax, 60
    syscall

global main
main:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
    mov rax, [.a]
    push rax
    mov rax, [.b]
    pop rcx
    xchg rax, rcx
    add rax, rcx
    push rax
    mov rax, [.c]
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

section .data
    .a: dq 10
    .b: dq 15
    .c: dq 3
