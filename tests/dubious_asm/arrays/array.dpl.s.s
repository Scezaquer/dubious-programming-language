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
    mov rax, 6
    push rax
    mov rax, 5
    push rax
    mov rax, 4
    push rax
    mov rax, rsp	; Move the address of the array to rax
    push rax
    mov rax, 3
    push rax	;pushing array dimensions onto stack
    mov rax, [rbp-32]
    push rax
    mov rax, 1
    pop rcx
    mov rax, [rcx + rax * 8]
    add rsp, 40		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 16		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

section .data
