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
    push 0
    mov rax, 0
    push rax
    ;for statement
    mov rax, 0
    mov [rbp-8], rax
for_start_0:
    mov rax, [rbp-8]
    push rax
    mov rax, 10
    pop rcx
    xchg rax, rcx
    cmp rax, rcx
    setl al
    movzx rax, al
    cmp rax, 0
    je for_end_0
    mov rax, [rbp-16]
    push rax
    mov rax, 2
    pop rcx
    xchg rax, rcx
    add rax, rcx
    mov [rbp-16], rax
    add rsp, 0		;end of block, pop local variables
    mov rax, [rbp-8]
    inc rax
    mov [rbp-8], rax
    jmp for_start_0
for_end_0:
    mov rax, [rbp-8]
    push rax
    mov rax, [rbp-16]
    pop rcx
    xchg rax, rcx
    add rax, rcx
    add rsp, 16		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 16		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

section .data
