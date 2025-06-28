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
    mov rax, 12
    push rax
    mov rax, 11
    push rax
    mov rax, 10
    push rax
    mov rax, 9
    push rax
    mov rax, 8
    push rax
    mov rax, 7
    push rax
    mov rax, 6
    push rax
    mov rax, 5
    push rax
    mov rax, 4
    push rax
    mov rax, 9		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    push rax
    mov rax, 3
    push rax	;pushing array dimensions onto stack
    mov rax, 3
    push rax	;pushing array dimensions onto stack
    mov rax, [rbp-88]
    push rax
    mov rax, 2
    push rax
    mov rax, 3
    push rax
    mov rax, 1
    push rax
    mov rax, 3
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
    add rsp, 104		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 104		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

section .data
	mxcsr_val dd 0
	malloc_counter dd 0
