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
    push rbx
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
    pop rbx
    ret

section .data
