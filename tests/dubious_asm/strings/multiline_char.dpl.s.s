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
    mov rax, 0x6f6c0a6c0a6548	;He\nl\nlo
    push rax
    mov rax, 8
    push rax
    mov rax, 6
    pop rcx
    imul rax, rcx
    push rax
    mov rax, [rbp-8]
    pop rcx
    shr rax, cl
    add rsp, 8		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 8		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

section .data
	mxcsr_val dd 0
	malloc_counter dd 0
