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
    mov rax, 3
    push rax
    mov rax, 2
    push rax
    mov rax, 1
    push rax
    mov rax, 0
    push rax
    mov rax, rsp	; Move the address of the array to rax
    push rax
    mov rax, 2
    push rax	;pushing array dimensions onto stack
    mov rax, [rbp-8]
    push rax	;pushing array dimensions onto stack
    mov rax, 1

    mov [rbp-8], rax
    mov rax, [rbp-48]
    push rax
    mov rax, 0
    push rax
    mov rax, [rbp-8]
    pop rcx
    xchg rax, rcx
    imul rax, rcx
    push rax
    mov rax, 1
    pop rcx
    xchg rax, rcx
    add rax, rcx
    push rax
    mov rax, 2
    pop rcx
    xchg rax, rcx
    imul rax, rcx
    push rax
    mov rax, 1
    pop rcx
    xchg rax, rcx
    add rax, rcx
    pop rcx
    mov rax, [rcx + rax * 8]
    add rsp, 64		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 32		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

section .data
