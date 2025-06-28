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
    mov rax, 6
    push rax
    mov rax, 5
    neg rax
    push rax
    mov rax, 1
    pop rcx
    sub rax, rcx
    pop rcx
    imul rax, rcx
    push rax
    mov rax, 2
    pop rcx
    add rax, rcx
    push rax
    mov rax, 1
    pop rcx
    imul rax, rcx
    push rax
    mov rax, 2
    push rax
    mov rax, 3
    pop rcx
    sub rax, rcx
    pop rcx
    add rax, rcx
    push rax
    mov rax, 6
    pop rcx
    imul rax, rcx
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
	malloc_counter dd 0
