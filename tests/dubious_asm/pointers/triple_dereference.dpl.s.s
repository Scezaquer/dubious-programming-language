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
    mov rax, 5
    push rax
    push rsp
    push rsp
    push rsp
    mov rax, [rbp-32]
    mov rax, [rax]
    mov rax, [rax]
    mov rax, [rax]
    add rsp, 32		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 32		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

section .data
	mxcsr_val dd 0
	malloc_counter dd 0
