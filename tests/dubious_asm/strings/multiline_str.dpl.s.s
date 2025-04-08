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
    mov rax, 0x6f726c0a64	;orl\nd
    push rax
    mov rax, 0x48656c6c6f0a0977	;Hello\n\tw
    push rax
    mov rax, rsp	; Move the address of the array to rax
    push rax
    mov rax, [rbp-24]
    push rax
    mov rax, 1
    pop rcx
    mov rax, [rcx + rax * 8]
    add rsp, 24		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 8		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

section .data
