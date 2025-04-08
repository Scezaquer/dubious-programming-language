[BITS 64]
section .text

global _start
_start:
    call main
    mov rdi, rax
    mov rax, 60
    syscall

global ftoint
ftoint:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
    movsd xmm0, [rbp+24]
	cvtsd2si rax, xmm0  ; Convert double in xmm0 to 64-bit integer in rax
    add rsp, 0		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

global inttof
inttof:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
    mov rax, [rbp+24]
	cvtsi2sd xmm0, rax	; Convert 64-bit integer in rax to double in xmm0
    add rsp, 0		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

global main
main:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
	;push function arguments to the stack in reverse order
    movsd xmm0, [.float.0]	; Load float into xmm0
    movq rax, xmm0
    push rax
    movsd xmm0, [.float.1]	; Load float into xmm0
    movq rax, xmm0
    push rax
    movsd xmm0, [.float.2]	; Load float into xmm0
	pop rcx
	movq xmm1, rcx
	mulsd xmm0, xmm1
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
	pop rcx
	movq xmm1, rcx
	mulsd xmm0, xmm1
	pop rcx
	movq xmm1, rcx
	divsd xmm0, xmm1
	pop rcx
	movq xmm1, rcx
	divsd xmm0, xmm1
	pop rcx
	movq xmm1, rcx
	addsd xmm0, xmm1
	pop rcx
	movq xmm1, rcx
	addsd xmm0, xmm1
	pop rcx
	movq xmm1, rcx
	subsd xmm0, xmm1
	pop rcx
	movq xmm1, rcx
	addsd xmm0, xmm1
    movq rax, xmm0
    push rax
    call ftoint
    add rsp, 8	;pop arguments
    add rsp, 0		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 0		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

section .data
	.float.7: dq 0.5
	.float.0: dq 20.0
	.float.1: dq 3.1415926535
	.float.2: dq 6.821
	.float.5: dq 1.41
	.float.6: dq 0.3
	.float.4: dq 1.2
	.float.3: dq 1.3
	.float.8: dq 2.5
