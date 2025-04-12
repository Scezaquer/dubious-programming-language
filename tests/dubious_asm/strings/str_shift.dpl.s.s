[BITS 64]
section .text

global _start
_start:
    call .toplevel.main
    mov rdi, rax
    mov rax, 60
    syscall

.toplevel.main:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
    mov rax, 0x616263	;abc
    push rax
    mov rax, rsp	; Move the address of the array to rax
    push rax
    mov rax, 8
    push rax
    mov rax, [rbp-16]
    push rax
    mov rax, 0
    pop rcx
    mov rax, [rcx + rax * 8]
    pop rcx
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
