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
    mov rax, 'abc'
    push rax
    mov rax, rsp	; Move the address of the array to rax
    push rax
    mov rax, 0
    mov rcx, rax
    mov rax, rbp
	sub rax, 16
	mov rax, [rax]
    mov rax, [rax + rcx * 8]
    push rax
    mov rax, 8
    pop rcx
    xchg rax, rcx
    shr rax, cl
    add rsp, 16		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 8		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

section .data
