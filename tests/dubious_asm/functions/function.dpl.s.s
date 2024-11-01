[BITS 64]
section .text

global _start
_start:
    call main
    mov rdi, rax
    mov rax, 60
    syscall

global add
add:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
    mov rax, [rbp+24]
    push rax
    mov rax, [rbp+32]
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
    mov rax, 3
    neg rax
    push rax
    mov rax, 5
    push rax
	;push function arguments to the stack in reverse order
    mov rax, [rbp-8]
    push rax
    mov rax, [rbp-16]
    push rax
    call add
    add rsp, 16	;pop arguments
    add rsp, 16		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 16		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

section .data
