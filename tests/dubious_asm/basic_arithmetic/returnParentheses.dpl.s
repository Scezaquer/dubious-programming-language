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
    mov rax, 1
    pop rcx
    xchg rax, rcx
    add rax, rcx
    pop rcx
    xchg rax, rcx
    imul rax, rcx
    pop rbx
    ret

section .data
