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
    mov rax, 2
    push rax
    mov rax, [rbp-8]
    push rax
    mov rax, 3
    push rax
    mov rax, [rbp-16]
    push rax
    mov rax, [rbp-24]
    pop rcx
    xchg rax, rcx
    add rax, rcx
    mov [rbp-16], rax
    add rsp, 8		;end of block, pop local variables
    mov rax, [rbp-16]
    push rax
    mov rax, [rbp-8]
    pop rcx
    xchg rax, rcx
    add rax, rcx
    mov [rbp-16], rax
    mov rax, [rbp-16]
    add rsp, 16		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 16		;end of block, pop local variables

section .data