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
    mov rax, 0x63	;c
    push rax
    mov rax, 3
    push rax
    mov rax, rsp	; Move the address of the array to rax
    push rax
    mov rax, [rbp-48]
    mov rcx, 0
    mov rax, [rax + rcx * 8]
    push rax
    mov rax, [rbp-24]
    push rax
    mov rax, 0
    mov r8, rax
    imul r8, 8
    pop rax
    add r8, rax
    pop rax
    add [r8], rax
    mov rax, [rbp-24]
    mov rcx, 0
    mov rax, [rax + rcx * 8]
    push rax
    mov rax, [rbp-24]
    mov rcx, 1
    mov rax, [rax + rcx * 8]
    pop rcx
    add rax, rcx
    push rax
    mov rax, [rbp-48]
    push rax
    mov rax, 1
    mov r8, rax
    imul r8, 8
    pop rax
    add r8, rax
    pop rax
    mov [r8], rax
    mov rax, [.constant.toplevel.test.sub_test.c]
    push rax
    mov rax, [.constant.toplevel.test.c]
    pop rcx
    add rax, rcx
    push rax
    mov rax, 0
    push rax
    mov rax, 2
    pop rcx
    sub rax, rcx
    add [rbp-56], rax
    mov rax, [rbp-56]
    push rax
    mov rax, [rbp-48]
    mov rcx, 1
    mov rax, [rax + rcx * 8]
    push rax
    mov rax, [rbp-24]
    mov rcx, 0
    mov rax, [rax + rcx * 8]
    pop rcx
    add rax, rcx
    pop rcx
    sub rax, rcx
    add rsp, 56		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 24		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.test.return_2:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
    mov rax, 2
    add rsp, 0		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 0		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.test.sub_test.return_2:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
    mov rax, 2
    neg rax
    add rsp, 0		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 0		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

section .data
    .constant.toplevel.test.c: dq 5
    .constant.toplevel.test.sub_test.c: dq 36
