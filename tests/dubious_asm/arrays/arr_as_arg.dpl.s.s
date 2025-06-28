[BITS 64]
section .text

global _start
_start:
    call main
    mov rdi, rax
    mov rax, 60
    syscall

main:
.toplevel.main:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
    mov rax, 6
    push rax
    mov rax, 5
    push rax
    mov rax, 4
    push rax
    mov rax, 3		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    push rax
	;push function arguments to the stack in reverse order
    mov rax, 8
    push rax
    mov rax, [rbp-40]
    push rax
    call .toplevel.test
    add rsp, 16	;pop arguments
    add rsp, 40		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 40		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.test:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
    mov rax, 0
    push rax
    push 0
    mov rax, 0
    mov [rbp-16], rax
    ;for statement
.for_start_0:
    mov rax, [rbp+24]
    push rax
    mov rax, 1
    neg rax
    pop rcx
    mov rax, [rcx + rax * 8]
    push rax
    mov rax, [rbp-16]
    pop rcx
    cmp rax, rcx
    setl al
    movzx rax, al
    cmp rax, 0
    je .for_end_0
    mov rax, [rbp+24]
    push rax
    mov rax, [rbp-16]
    pop rcx
    mov rax, [rcx + rax * 8]
    add [rbp-8], rax
    add rsp, 0		;end of block, pop local variables
    mov rax, 1
    push rax
    mov rax, [rbp-16]
    pop rcx
    add rax, rcx
    mov [rbp-16], rax
    jmp .for_start_0
.for_end_0:
    mov rax, [rbp+32]
    push rax
    mov rax, [rbp-8]
    pop rcx
    add rax, rcx
    add rsp, 16		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 16		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

section .data
	mxcsr_val dd 0
	malloc_counter dd 0
