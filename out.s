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
	;push function arguments to the stack in reverse order
    mov rax, 1
    push rax
    call .toplevel.test
    add rsp, 8	;pop arguments
    add rsp, 0		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 0		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.test:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
    ;if statement
    mov rax, 0
    push rax
    mov rax, [rbp+24]
    pop rcx
    cmp rax, rcx
    sete al
    movzx rax, al
    cmp rax, 0
    je .else_0
    mov rax, 0
    add rsp, 0		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 0		;end of block, pop local variables
    jmp .end_0
.else_0:
    mov rax, 2
    push rax
    mov rax, 1
    push rax
    mov rax, [rbp+24]
    pop rcx
    sub rax, rcx
    push rax
	;push function arguments to the stack in reverse order
    mov rax, [rbp-8]
    push rax
    call .toplevel.test
    add rsp, 16	;pop arguments
    pop rcx
    add rax, rcx
    add rsp, 0		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 0		;end of block, pop local variables
.end_0:
    add rsp, 0		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

section .data
	mxcsr_val dd 0
