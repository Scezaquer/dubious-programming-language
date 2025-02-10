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
    mov rax, 5
    push rax
    mov rax, 4
    push rax
    mov rax, 3
    push rax
    mov rax, rsp	; Move the address of the array to rax
    mov rax, rsp
    add rax, 24
    push rax
    mov rax, 'b'
    pop rcx
    add rsp, 8
    push rax
    mov rax, rcx
    sub rax, 24
    push rax
    mov rax, 'a'
    pop rcx
    add rsp, 8
    push rax
    mov rax, rcx
    mov rax, rsp	; Move the address of the array to rax
    push rax
    mov rax, [rbp-64]
    mov rcx, 1
    mov rax, [rax + rcx * 8]
    push rax
    mov rax, 1
    pop rcx
    mov rax, [rcx + rax * 8]
    add rsp, 64		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 8		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

section .data
