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
    ;if statement
    movsd xmm0, [.float.0]	; Load float into xmm0
    movq rax, xmm0
    push rax
    movsd xmm0, [.float.1]	; Load float into xmm0
	pop rcx
	movq xmm1, rcx
	ucomisd xmm0, xmm1
	sete al
	movzx rax, al
    cmp rax, 0
    je .else_0
    mov rax, 1
    add rsp, 0		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 0		;end of block, pop local variables
    jmp .end_0
.else_0:
    ;if statement
    movsd xmm0, [.float.0]	; Load float into xmm0
    movq rax, xmm0
    push rax
    movsd xmm0, [.float.0]	; Load float into xmm0
	pop rcx
	movq xmm1, rcx
	ucomisd xmm0, xmm1
	sete al
	movzx rax, al
    cmp rax, 0
    je .else_1
    mov rax, 0
    add rsp, 0		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 0		;end of block, pop local variables
    jmp .end_1
.else_1:
.end_1:
    add rsp, 0		;end of block, pop local variables
.end_0:
    mov rax, 1
    add rsp, 0		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 0		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

section .data
	.float.1: dq -0.1
	.float.0: dq 0.0
	mxcsr_val dd 0
	malloc_counter dd 0
