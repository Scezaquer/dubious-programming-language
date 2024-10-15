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
    mov rax, 1
    mov [rbp-a], 0
    mov rax, [rbp-a]
    push rax
    mov rax, 1
    pop rcx
    xchg rax, rcx
    add rax, rcx
    mov [rbp-a], rax
    mov rax, [rbp-a]
    pop rbx
    ret

section .data
