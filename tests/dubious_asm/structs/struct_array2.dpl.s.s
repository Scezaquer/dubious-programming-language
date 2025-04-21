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
    mov rax, 5
    push rax
    mov rax, 4
    push rax
    mov rax, 3
    push rax
    mov rax, 3		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    mov rax, rsp
    add rax, 40
    push rax
    mov rax, 0x62	;b
    pop rcx
    sub rsp, 8
    push rax
    mov rax, rcx
    sub rax, 32
    push rax
    mov rax, 0x61	;a
    pop rcx
    sub rsp, 8
    push rax
    mov rax, rcx
    mov rax, 3		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    push rax
    mov rax, [rbp-80]
    mov rcx, 2
    mov rax, [rax + rcx * 8]
    add rsp, 80		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 8		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

section .data
