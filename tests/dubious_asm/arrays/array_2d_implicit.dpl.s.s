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
    mov rax, 3
    push rax
    mov rax, 2
    push rax
    mov rax, 0
    push rax
    mov rax, 1
    push rax
    mov rax, 4		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    push rax
    mov rax, 2
    push rax	;pushing array dimensions onto stack
    mov rax, 2
    push rax	;pushing array dimensions onto stack
    mov rax, [rbp-48]
    push rax
    mov rax, 0
    push rax
    mov rax, 2
    push rax
    mov rax, 1
    push rax
    mov rax, 2
    push rax
    mov rax, 0
    pop rcx
    imul rax, rcx
    pop rcx
    add rax, rcx
    pop rcx
    imul rax, rcx
    pop rcx
    add rax, rcx
    pop rcx
    mov rax, [rcx + rax * 8]
    add rsp, 64		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 24		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

section .data
