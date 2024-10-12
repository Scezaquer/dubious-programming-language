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
    mov rax, 23
    not rax
    pop rbx
    ret

section .data
