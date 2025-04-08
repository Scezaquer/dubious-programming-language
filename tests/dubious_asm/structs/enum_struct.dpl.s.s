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
    mov rax, 0x68656c6c6f203a29	;hello?:)
    push rax
    mov rax, rsp	; Move the address of the array to rax
    mov rax, rsp
    add rax, 8
    push rax
    sub rax, 8
    push rax
    mov rax, 1
    pop rcx
    sub rsp, 8
    push rax
    mov rax, rcx
    mov rax, rsp	; Move the address of the array to rax
    push rax
    mov rax, 0
    push rax
    mov rax, [rbp-40]
    push rax
    mov rax, 0
    mov r8, rax
    imul r8, 8
    pop rax
    add r8, rax
    pop rax
    mov [r8], rax
    mov rax, [rbp-40]
    mov rcx, 0
    mov rax, [rax + rcx * 8]
    add rsp, 40		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 8		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

section .data
