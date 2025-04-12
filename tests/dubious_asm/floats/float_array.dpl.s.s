[BITS 64]
section .text

global _start
_start:
    call .toplevel.main
    mov rdi, rax
    mov rax, 60
    syscall

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
    pcmpeqd xmm1, xmm1	; xmm1 = all ones (0xFFFFFFFFFFFFFFFF)
    psllq xmm1, 63		; xmm1 = 0x8000000000000000 (sign bit mask)
    xorpd xmm0, xmm1	; Flip the sign bit of xmm0
    movq rax, xmm0
    push rax
    movsd xmm0, [.float.6]	; Load float into xmm0
    pcmpeqd xmm1, xmm1	; xmm1 = all ones (0xFFFFFFFFFFFFFFFF)
    psllq xmm1, 63		; xmm1 = 0x8000000000000000 (sign bit mask)
    xorpd xmm0, xmm1	; Flip the sign bit of xmm0
    movq rax, xmm0
    push rax
    movsd xmm0, [.float.7]	; Load float into xmm0
    movq rax, xmm0
    push rax
    movsd xmm0, [.float.8]	; Load float into xmm0
    movq rax, xmm0
    push rax
    mov rax, rsp	; Move the address of the array to rax
    push rax
    mov rax, 3
    push rax	;pushing array dimensions onto stack
    mov rax, 3
    push rax	;pushing array dimensions onto stack
    mov rax, [rbp-80]
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
    mov rax, [rbp-80]
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
	;push function arguments to the stack in reverse order
    mov rax, [rbp-80]
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
    mov rax, [rbp-80]
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
    call .toplevel.ftoint
    add rsp, 8	;pop arguments
    add rsp, 96		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 24		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

section .data
	.float.8: dq 0.2
	.float.7: dq 0.3
	.float.1: dq 7.0
	.float.4: dq 6.71
	.float.6: dq 0.7
	.float.3: dq 3.14159
	.float.0: dq 3.99
	.float.5: dq 1.3
	.float.2: dq 2.0
