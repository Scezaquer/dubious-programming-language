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
    mov rax, 0
    push rax
    ;loop statement
loop_start_0:
    mov rax, [rbp-8]
    push rax
    mov rax, 1
    pop rcx
    xchg rax, rcx
    add rax, rcx
    mov [rbp-8], rax
    ;if statement
    mov rax, [rbp-8]
    push rax
    mov rax, 5
    pop rcx
    xchg rax, rcx
    cmp rax, rcx
    setge al
    movzx rax, al
    cmp rax, 0
    je else_0
    jmp loop_end_0	;break statement
    add rsp, 0		;end of block, pop local variables
    jmp end_0
else_0:
end_0:
    add rsp, 0		;end of block, pop local variables
    jmp loop_start_0
loop_end_0:
    mov rax, [rbp-8]
    add rsp, 8		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 8		;end of block, pop local variables

section .data
