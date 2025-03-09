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
    movsd xmm0, [.float_0]	; Load float into xmm0
    movq rax, xmm0
    push rax
    movsd xmm0, [.float_1]	; Load float into xmm0
	pop rcx
	movq xmm1, rcx
	movapd xmm2, xmm0
	movapd xmm0, xmm1
	movapd xmm1, xmm2
	movapd xmm2, xmm0
	divsd xmm2, xmm1  ; xmm2 = xmm0 / xmm1
	roundsd xmm2, xmm2, 0b0011  ; Round towards zero
	mulsd xmm2, xmm1         ; xmm2 = truncated(xmm0 / xmm1) * xmm1
	subsd xmm0, xmm2         ; xmm0 = xmm0 - xmm2 (remainder)
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
	.float_0: dq 5.5
	.float_1: dq 1.5
