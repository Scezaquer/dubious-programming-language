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
    mov rax, 0x61	;a
    push rax
    mov rax, 1
    push rax
    mov rax, rsp	; Move the address of the array to rax
    push rax
    mov rax, 2
    push rax
    mov rax, [rbp-24]
    push rax
    mov rax, rsp	; Move the address of the array to rax
    push rax
    mov rax, [rbp-48]
    mov rcx, 0
    mov rax, [rax + rcx * 8]
    mov rcx, 0
    mov rax, [rax + rcx * 8]
    add rsp, 48		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 16		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

section .data
