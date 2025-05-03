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
    mov rax, 0x6968	;hi
    push rax
    mov rax, 1		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    mov rax, rsp
    add rax, 24
    push rax
    sub rax, 16
    push rax
    mov rax, 0x61	;a
    pop rcx
    sub rsp, 8
    push rax
    mov rax, rcx
    mov rax, 2
    pop rcx
    sub rsp, 8
    push rax
    mov rax, rcx
    mov rax, 1
    pop rcx
    sub rsp, 8
    push rax
    mov rax, rcx
    mov rax, 4		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    push rax
    mov rax, 3
    push rax
    mov rax, [rbp-72]
    push rax
    mov rax, 1
    mov r8, rax
    imul r8, 8
    pop rax
    add r8, rax
    pop rax
    mov [r8], rax
	;push function arguments to the stack in reverse order
    mov rax, [rbp-72]
    push rax
    call .toplevel.return2..toplevel..S..int...char.
    add rsp, 8	;pop arguments
    mov rcx, 0
    mov rax, [rax + rcx * 8]
    push rax
    mov rax, [rbp-72]
    mov rcx, 1
    mov rax, [rax + rcx * 8]
    pop rcx
    add rax, rcx
    add rsp, 72		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 72		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.return2..toplevel..S..int...char.:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
    push 0
    mov rax, [rbp+24]
    add rsp, 8		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 8		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

section .data
