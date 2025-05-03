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
    mov rax, 3
    push rax
    mov rax, 0x61	;a
    push rax
    mov rax, 2
    push rax
    mov rax, 1
    push rax
    mov rax, 4		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    push rax
    mov rax, [rbp-48]
    mov rcx, 0
    mov rax, [rax + rcx * 8]
    push rax
    mov rax, [rbp-48]
    mov rcx, 3
    mov rax, [rax + rcx * 8]
    push rax
    mov rax, [rbp-48]
    mov rcx, 1
    mov rax, [rax + rcx * 8]
    pop rcx
    add rax, rcx
    pop rcx
    add rax, rcx
    add rsp, 48		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 48		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

section .data
