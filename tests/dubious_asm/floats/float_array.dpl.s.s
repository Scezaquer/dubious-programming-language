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
    movsd xmm0, [.float.0]	; Load float into xmm0
    movq rax, xmm0
    push rax
    movsd xmm0, [.float.1]	; Load float into xmm0
    movq rax, xmm0
    push rax
    movsd xmm0, [.float.2]	; Load float into xmm0
    movq rax, xmm0
    push rax
    movsd xmm0, [.float.3]	; Load float into xmm0
    movq rax, xmm0
    push rax
    movsd xmm0, [.float.4]	; Load float into xmm0
    movq rax, xmm0
    push rax
    movsd xmm0, [.float.5]	; Load float into xmm0
    movq rax, xmm0
    push rax
    movsd xmm0, [.float.6]	; Load float into xmm0
    movq rax, xmm0
    push rax
    movsd xmm0, [.float.7]	; Load float into xmm0
    movq rax, xmm0
    push rax
    movsd xmm0, [.float.8]	; Load float into xmm0
    movq rax, xmm0
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
    mov rax, 0
    push rax
    mov rax, 3
    push rax
    mov rax, 2
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
    movq xmm0, [rcx + rax * 8]
    movq rax, xmm0
    push rax
    mov rax, [rbp-88]
    push rax
    mov rax, 1
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
    mov r8, rax
    imul r8, 8
    pop rax
    add r8, rax
    pop rax
    movq xmm0, rax
    movq xmm1, xmm0
    movq xmm0, [r8]
    subsd xmm0, xmm1
    movq [r8], xmm0
    mov rax, [rbp-88]
    push rax
    mov rax, 1
    push rax
    mov rax, 3
    push rax
    mov rax, 0
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
    movq xmm0, [rcx + rax * 8]
    movq rax, xmm0
    push rax
    mov rax, [rbp-88]
    push rax
    mov rax, 1
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
    movq xmm0, [rcx + rax * 8]
	pop rcx
	movq xmm1, rcx
	addsd xmm0, xmm1
    movq rax, xmm0
    push rax
	;push function arguments to the stack in reverse order
    movsd xmm0, [rbp-112]
    movq rax, xmm0
    push rax
    call .toplevel.ftoint
    add rsp, 16	;pop arguments
    add rsp, 104		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 104		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.ftoint:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
    movsd xmm0, [rbp+24]
	cvtsd2si rax, xmm0  ; Convert double in xmm0 to 64-bit integer in rax
    add rsp, 0		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.inttof:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
    mov rax, [rbp+24]
	cvtsi2sd xmm0, rax	; Convert 64-bit integer in rax to double in xmm0
    add rsp, 0		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

section .data
	.float.5: dq -1.3
	.float.2: dq 2.0
	.float.6: dq -0.7
	.float.1: dq 7.0
	.float.0: dq 3.99
	.float.7: dq 0.3
	.float.8: dq 0.2
	.float.3: dq 3.14159
	.float.4: dq 6.71
	mxcsr_val dd 0
	malloc_counter dd 0
