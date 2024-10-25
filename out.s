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
    mov rax, Int(0)
    push rax
    mov rax, Int(0)
    pop rcx
    xchg rax, rcx
    add rax, rcx
    pop rbx
    ret

section .data
