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
    mov rax, 0
    push rax
    mov rax, 0
    push rax
    ;while statement
.while_start_0:
    mov rax, 10
    push rax
    mov rax, [rbp-8]
    pop rcx
    cmp rax, rcx
    setl al
    movzx rax, al
    cmp rax, 0
    je .while_end_0
    mov rax, 1
    push rax
    mov rax, [rbp-8]
    pop rcx
    add rax, rcx
    mov [rbp-8], rax
    jmp .while_start_0	;continue statement
    mov rax, 1
    push rax
    mov rax, [rbp-16]
    pop rcx
    add rax, rcx
    mov [rbp-16], rax
    add rsp, 0		;end of block, pop local variables
    jmp .while_start_0
.while_end_0:
    mov rax, [rbp-16]
    add rsp, 16		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 16		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

section .data
