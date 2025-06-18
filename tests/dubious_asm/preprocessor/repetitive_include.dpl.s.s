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
    mov rax, 5
    push rax
    mov rax, 1
    push rax
    call .toplevel.add
    add rsp, 16	;pop arguments
    push rax
	;push function arguments to the stack in reverse order
    mov rax, 2
    push rax
    mov rax, [rbp-8]
    push rax
    call .toplevel.sub
    add rsp, 24	;pop arguments
    add rsp, 0		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 0		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.sub:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
    mov rax, [rbp+32]
    push rax
    mov rax, [rbp+24]
    pop rcx
    sub rax, rcx
    add rsp, 0		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 0		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.add:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
    mov rax, [rbp+32]
    push rax
    mov rax, [rbp+24]
    pop rcx
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
	mxcsr_val dd 0
