[BITS 64]
section .text

global _start
_start:
    call main
    mov rax, 60
    xor edi, edi
    syscall
global main
main:
    push rbx
    mov rax, 0
    pop rbx
    ret

section .data
