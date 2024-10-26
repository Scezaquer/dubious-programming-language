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
    ;if statement
    mov rax, 0
    cmp rax, 0
    je else_0
    mov rax, 0
    add rsp, 0		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 0		;end of block, pop local variables
    jmp end_0
else_0:
    ;if statement
    mov rax, 0
    cmp rax, 0
    je else_1
    mov rax, 1
    add rsp, 0		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 0		;end of block, pop local variables
    jmp end_1
else_1:
    ;if statement
    mov rax, 0
    cmp rax, 0
    je else_2
    mov rax, 2
    add rsp, 0		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 0		;end of block, pop local variables
    jmp end_2
else_2:
    ;if statement
    mov rax, 1
    cmp rax, 0
    je else_3
    mov rax, 3
    add rsp, 0		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 0		;end of block, pop local variables
    jmp end_3
else_3:
    ;if statement
    mov rax, 0
    cmp rax, 0
    je else_4
    mov rax, 4
    add rsp, 0		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 0		;end of block, pop local variables
    jmp end_4
else_4:
    ;if statement
    mov rax, 0
    cmp rax, 0
    je else_5
    mov rax, 5
    add rsp, 0		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 0		;end of block, pop local variables
    jmp end_5
else_5:
    ;if statement
    mov rax, 0
    cmp rax, 0
    je else_6
    mov rax, 6
    add rsp, 0		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 0		;end of block, pop local variables
    jmp end_6
else_6:
    ;if statement
    mov rax, 0
    cmp rax, 0
    je else_7
    mov rax, 7
    add rsp, 0		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 0		;end of block, pop local variables
    jmp end_7
else_7:
    mov rax, 8
    add rsp, 0		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 0		;end of block, pop local variables
end_7:
    add rsp, 0		;end of block, pop local variables
end_6:
    add rsp, 0		;end of block, pop local variables
end_5:
    add rsp, 0		;end of block, pop local variables
end_4:
    add rsp, 0		;end of block, pop local variables
end_3:
    add rsp, 0		;end of block, pop local variables
end_2:
    add rsp, 0		;end of block, pop local variables
end_1:
    add rsp, 0		;end of block, pop local variables
end_0:
    mov rax, 9
    add rsp, 0		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 0		;end of block, pop local variables

section .data
