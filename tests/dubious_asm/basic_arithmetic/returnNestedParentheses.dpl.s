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
    mov rax, 6
    push rax
    mov rax, 3
    push rax
    mov rax, 2
    pop rcx
    xchg rax, rcx
    sub rax, rcx
    push rax
    mov rax, 1
    push rax
    mov rax, 2
    push rax
    mov rax, 1
    push rax
    mov rax, 5
    neg rax
    pop rcx
    xchg rax, rcx
    sub rax, rcx
    push rax
    mov rax, 6
    pop rcx
    xchg rax, rcx
    imul rax, rcx
    pop rcx
    xchg rax, rcx
    add rax, rcx
    pop rcx
    xchg rax, rcx
    imul rax, rcx
    pop rcx
    xchg rax, rcx
    add rax, rcx
    pop rcx
    xchg rax, rcx
    imul rax, rcx
    add rsp, 0		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 0		;end of block, pop local variables

section .data
