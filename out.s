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
	;push function arguments to the stack in reverse order
    mov rax, 10
    push rax
    call .toplevel.std.mem.malloc
    add rsp, 8	;pop arguments
    push rax
	;push function arguments to the stack in reverse order
    mov rax, [rbp-8]
    push rax
    call .toplevel.std.io.printarrayln..int
    add rsp, 8	;pop arguments
    mov rax, [rbp-8]
    push rax
    mov rax, -1
    pop rcx
    mov rax, [rcx + rax * 8]
    push rax
	;push function arguments to the stack in reverse order
    mov rax, [rbp-16]
    push rax
    call .toplevel.std.io.printiln
    add rsp, 16	;pop arguments
    mov rax, [rbp-8]
    push rax
    mov rax, 10
    pop rcx
    mov rax, [rcx + rax * 8]
    push rax
	;push function arguments to the stack in reverse order
    mov rax, [rbp-16]
    push rax
    call .toplevel.std.io.printiln
    add rsp, 16	;pop arguments
    mov rax, [rbp-8]
    push rax
    mov rax, 11
    pop rcx
    mov rax, [rcx + rax * 8]
    push rax
	;push function arguments to the stack in reverse order
    mov rax, [rbp-16]
    push rax
    call .toplevel.std.io.printiln
    add rsp, 16	;pop arguments
    mov rax, 0
    add rsp, 8		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 8		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.std.io.printarray..int:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
    mov rax, 0x5b	;[
    push rax
    mov rax, 1		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    push rax
	;push function arguments to the stack in reverse order
    mov rax, [rbp-24]
    push rax
    call .toplevel.std.io.print
    add rsp, 32	;pop arguments
    push 0
    mov rax, 0
    mov [rbp-8], rax
    ;for statement
.for_start_0:
    mov rax, [rbp+24]
    push rax
    mov rax, -1
    pop rcx
    mov rax, [rcx + rax * 8]
    push rax
    mov rax, [rbp-8]
    pop rcx
    cmp rax, rcx
    setl al
    movzx rax, al
    cmp rax, 0
    je .for_end_0
    ;if statement
    mov rax, 0
    push rax
    mov rax, [rbp-8]
    pop rcx
    cmp rax, rcx
    setg al
    movzx rax, al
    cmp rax, 0
    je .else_0
    mov rax, 0x202c	;,?
    push rax
    mov rax, 1		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    push rax
	;push function arguments to the stack in reverse order
    mov rax, [rbp-32]
    push rax
    call .toplevel.std.io.print
    add rsp, 32	;pop arguments
    add rsp, 0		;end of block, pop local variables
    jmp .end_0
.else_0:
.end_0:
    mov rax, [rbp+24]
    push rax
    mov rax, [rbp-8]
    pop rcx
    mov rax, [rcx + rax * 8]
    push rax
	;push function arguments to the stack in reverse order
    mov rax, [rbp-16]
    push rax
    call .toplevel.std.io.printi
    add rsp, 16	;pop arguments
    add rsp, 0		;end of block, pop local variables
    mov rax, 1
    push rax
    mov rax, [rbp-8]
    pop rcx
    add rax, rcx
    mov [rbp-8], rax
    jmp .for_start_0
.for_end_0:
    mov rax, 0x5d	;]
    push rax
    mov rax, 1		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    push rax
	;push function arguments to the stack in reverse order
    mov rax, [rbp-32]
    push rax
    call .toplevel.std.io.print
    add rsp, 32	;pop arguments
    add rsp, 8		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.std.io.printarrayln..int:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
	;push function arguments to the stack in reverse order
    mov rax, [rbp+24]
    push rax
    call .toplevel.std.io.printarray..int
    add rsp, 8	;pop arguments
    mov rax, 0xa	;\n
    push rax
    mov rax, 1		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    push rax
	;push function arguments to the stack in reverse order
    mov rax, [rbp-24]
    push rax
    call .toplevel.std.io.print
    add rsp, 32	;pop arguments
    add rsp, 0		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.std.printerr:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
    mov rax, 8
    push rax
    mov rax, [rbp+24]
    push rax
    mov rax, -1
    pop rcx
    mov rax, [rcx + rax * 8]
    pop rcx
    imul rax, rcx
	mov rdx, rax	; length of string
    mov rax, [rbp+24]
	mov rsi, rax	; pointer to string
	mov rax, 1		; syscall: sys_write
	mov rdi, 2		; file descriptor: stderr
	syscall
    add rsp, 0		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.std.exception:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
    mov rax, 0x203a6e	;n:?
    push rax
    mov rax, 0x6f69747065637845	;Exceptio
    push rax
    mov rax, 2		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    push rax
	;push function arguments to the stack in reverse order
    mov rax, [rbp-32]
    push rax
    call .toplevel.std.printerr
    add rsp, 40	;pop arguments
	;push function arguments to the stack in reverse order
    mov rax, [rbp+24]
    push rax
    call .toplevel.std.printerr
    add rsp, 8	;pop arguments
    mov rax, 0xa	;\n
    push rax
    mov rax, 1		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    push rax
	;push function arguments to the stack in reverse order
    mov rax, [rbp-24]
    push rax
    call .toplevel.std.printerr
    add rsp, 32	;pop arguments
    mov rax, [rbp+32]
	mov rdi, rax
	mov rax, 60
	syscall
	
    add rsp, 0		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.std.strget:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
    mov rax, 8
    push rax
    mov rax, [rbp+32]
    pop rcx
    cqo
    idiv rcx
    mov rax, rdx
    push rax
    mov rax, 8
    push rax
    mov rax, [rbp+32]
    pop rcx
    cqo
    idiv rcx
    push rax
    ;if statement
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
    setge al
    movzx rax, al
    cmp rax, 0
    je .else_1
    mov rax, 0x73646e	;nds
    push rax
    mov rax, 0x756f6220666f2074	;t?of?bou
    push rax
    mov rax, 0x756f207865646e49	;Index?ou
    push rax
    mov rax, 3		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    push rax
	;push function arguments to the stack in reverse order
    mov rax, 1
    push rax
    mov rax, [rbp-56]
    push rax
    call .toplevel.std.exception
    add rsp, 56	;pop arguments
    add rsp, 0		;end of block, pop local variables
    jmp .end_1
.else_1:
    ;if statement
    mov rax, 0
    push rax
    mov rax, [rbp+32]
    pop rcx
    cmp rax, rcx
    setl al
    movzx rax, al
    cmp rax, 0
    je .else_2
    mov rax, 0x73646e	;nds
    push rax
    mov rax, 0x756f6220666f2074	;t?of?bou
    push rax
    mov rax, 0x756f207865646e49	;Index?ou
    push rax
    mov rax, 3		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    push rax
	;push function arguments to the stack in reverse order
    mov rax, 1
    push rax
    mov rax, [rbp-56]
    push rax
    call .toplevel.std.exception
    add rsp, 56	;pop arguments
    add rsp, 0		;end of block, pop local variables
    jmp .end_2
.else_2:
.end_2:
    add rsp, 0		;end of block, pop local variables
.end_1:
    mov rax, 0xff
    push rax
    mov rax, 8
    push rax
    mov rax, [rbp-8]
    pop rcx
    imul rax, rcx
    push rax
    mov rax, [rbp+24]
    push rax
    mov rax, [rbp-16]
    pop rcx
    mov rax, [rcx + rax * 8]
    pop rcx
    shr rax, cl
    pop rcx
    and rax, rcx
    add rsp, 16		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 16		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.std.strset:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
    mov rax, 8
    push rax
    mov rax, [rbp+32]
    pop rcx
    cqo
    idiv rcx
    mov rax, rdx
    push rax
    mov rax, 8
    push rax
    mov rax, [rbp+32]
    pop rcx
    cqo
    idiv rcx
    push rax
    ;if statement
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
    setge al
    movzx rax, al
    cmp rax, 0
    je .else_3
    mov rax, 0x73646e	;nds
    push rax
    mov rax, 0x756f6220666f2074	;t?of?bou
    push rax
    mov rax, 0x756f207865646e49	;Index?ou
    push rax
    mov rax, 3		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    push rax
	;push function arguments to the stack in reverse order
    mov rax, 1
    push rax
    mov rax, [rbp-56]
    push rax
    call .toplevel.std.exception
    add rsp, 56	;pop arguments
    add rsp, 0		;end of block, pop local variables
    jmp .end_3
.else_3:
    ;if statement
    mov rax, 0
    push rax
    mov rax, [rbp+32]
    pop rcx
    cmp rax, rcx
    setl al
    movzx rax, al
    cmp rax, 0
    je .else_4
    mov rax, 0x73646e	;nds
    push rax
    mov rax, 0x756f6220666f2074	;t?of?bou
    push rax
    mov rax, 0x756f207865646e49	;Index?ou
    push rax
    mov rax, 3		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    push rax
	;push function arguments to the stack in reverse order
    mov rax, 1
    push rax
    mov rax, [rbp-56]
    push rax
    call .toplevel.std.exception
    add rsp, 56	;pop arguments
    add rsp, 0		;end of block, pop local variables
    jmp .end_4
.else_4:
.end_4:
    add rsp, 0		;end of block, pop local variables
.end_3:
    mov rax, 8
    push rax
    mov rax, [rbp-8]
    pop rcx
    imul rax, rcx
    push rax
    mov rax, [rbp+40]
    pop rcx
    shl rax, cl
    push rax
    mov rax, 8
    push rax
    mov rax, [rbp-8]
    pop rcx
    imul rax, rcx
    push rax
    mov rax, 0xff
    pop rcx
    shl rax, cl
    not rax
    push rax
    mov rax, [rbp+24]
    push rax
    mov rax, [rbp-16]
    pop rcx
    mov rax, [rcx + rax * 8]
    pop rcx
    and rax, rcx
    pop rcx
    or rax, rcx
    push rax
    mov rax, [rbp+24]
    push rax
    mov rax, [rbp-16]
    mov r8, rax
    imul r8, 8
    pop rax
    add r8, rax
    pop rax
    mov [r8], rax
    mov rax, [rbp+24]
    add rsp, 16		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 16		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.std.strlen:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
    mov rax, 1
    push rax
    mov rax, [rbp+24]
    push rax
    mov rax, 1
    neg rax
    pop rcx
    mov rax, [rcx + rax * 8]
    pop rcx
    sub rax, rcx
    push rax
    ;if statement
    mov rax, 0
    push rax
    mov rax, [rbp-8]
    pop rcx
    cmp rax, rcx
    sete al
    movzx rax, al
    cmp rax, 0
    je .else_5
    mov rax, 0
    add rsp, 0		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 0		;end of block, pop local variables
    jmp .end_5
.else_5:
.end_5:
    push 0
    mov rax, 7
    mov [rbp-16], rax
    ;for statement
.for_start_1:
    mov rax, 0
    push rax
    mov rax, [rbp-16]
    pop rcx
    cmp rax, rcx
    setge al
    movzx rax, al
    cmp rax, 0
    je .for_end_1
    ;if statement
    mov rax, 0
    push rax
    mov rax, 8
    push rax
    mov rax, [rbp-16]
    pop rcx
    imul rax, rcx
    push rax
    mov rax, 0xff
    pop rcx
    shl rax, cl
    push rax
    mov rax, [rbp+24]
    push rax
    mov rax, [rbp-8]
    pop rcx
    mov rax, [rcx + rax * 8]
    pop rcx
    and rax, rcx
    pop rcx
    cmp rax, rcx
    sete al
    movzx rax, al
    cmp rax, 0
    je .else_6
    jmp .for_end_1	;break statement
    add rsp, 0		;end of block, pop local variables
    jmp .end_6
.else_6:
.end_6:
    add rsp, 0		;end of block, pop local variables
    mov rax, 1
    push rax
    mov rax, [rbp-16]
    pop rcx
    sub rax, rcx
    mov [rbp-16], rax
    jmp .for_start_1
.for_end_1:
    mov rax, 1
    push rax
    mov rax, [rbp-16]
    push rax
    mov rax, 8
    push rax
    mov rax, [rbp-8]
    pop rcx
    imul rax, rcx
    pop rcx
    add rax, rcx
    pop rcx
    sub rax, rcx
    add rsp, 16		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 16		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.std.strcpy:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
    mov rax, [rbp+32]
    push rax
    mov rax, 1
    neg rax
    pop rcx
    mov rax, [rcx + rax * 8]
    push rax
    ;if statement
    mov rax, [rbp+24]
    push rax
    mov rax, 1
    neg rax
    pop rcx
    mov rax, [rcx + rax * 8]
    push rax
    mov rax, [rbp-8]
    pop rcx
    cmp rax, rcx
    setne al
    movzx rax, al
    cmp rax, 0
    je .else_7
    mov rax, 0x73	;s
    push rax
    mov rax, 0x6874676e656c2067	;g?length
    push rax
    mov rax, 0x6e69727473206465	;ed?strin
    push rax
    mov rax, 0x686374616d73694d	;Mismatch
    push rax
    mov rax, 4		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    push rax
	;push function arguments to the stack in reverse order
    mov rax, 1
    push rax
    mov rax, [rbp-56]
    push rax
    call .toplevel.std.exception
    add rsp, 64	;pop arguments
    add rsp, 0		;end of block, pop local variables
    jmp .end_7
.else_7:
.end_7:
    push 0
    mov rax, 0
    mov [rbp-16], rax
    ;for statement
.for_start_2:
    mov rax, [rbp-8]
    push rax
    mov rax, [rbp-16]
    pop rcx
    cmp rax, rcx
    setl al
    movzx rax, al
    cmp rax, 0
    je .for_end_2
    mov rax, [rbp+32]
    push rax
    mov rax, [rbp-16]
    pop rcx
    mov rax, [rcx + rax * 8]
    push rax
    mov rax, [rbp+24]
    push rax
    mov rax, [rbp-16]
    mov r8, rax
    imul r8, 8
    pop rax
    add r8, rax
    pop rax
    mov [r8], rax
    add rsp, 0		;end of block, pop local variables
    mov rax, 1
    push rax
    mov rax, [rbp-16]
    pop rcx
    add rax, rcx
    mov [rbp-16], rax
    jmp .for_start_2
.for_end_2:
    mov rax, [rbp+24]
    add rsp, 16		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 16		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.std.io.ftoint:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
    movsd xmm0, [rbp+24]
	cvtsd2si rax, xmm0  ; Convert double in xmm0 to 64-bit integer in rax
    add rsp, 0		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.std.io.inttof:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
    mov rax, [rbp+24]
	cvtsi2sd xmm0, rax	; Convert 64-bit integer in rax to double in xmm0
    add rsp, 0		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.std.io.floor:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
	stmxcsr [mxcsr_val]
	mov eax, [mxcsr_val]
	and eax, 0xFFFF9FFF	; Clear the round control bits
	or eax, 0x00002000	; 
	mov [mxcsr_val], eax
	ldmxcsr [mxcsr_val]
    movsd xmm0, [rbp+24]
	cvtsd2si rax, xmm0; round down (toward -inf)
    add rsp, 0		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.std.io.ceil:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
	stmxcsr [mxcsr_val]
	mov eax, [mxcsr_val]
	and eax, 0xFFFF9FFF	; 
	or eax, 0x00003000	; 
	mov [mxcsr_val], eax
	ldmxcsr [mxcsr_val]
    movsd xmm0, [rbp+24]
	cvtsd2si rax, xmm0; round up (toward +inf)
    add rsp, 0		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.std.io.isint:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
	;push function arguments to the stack in reverse order
    movsd xmm0, [rbp+24]
    movq rax, xmm0
    push rax
    call .toplevel.std.io.floor
    add rsp, 8	;pop arguments
    push rax
	;push function arguments to the stack in reverse order
    mov rax, [rbp-8]
    push rax
    call .toplevel.std.io.inttof
    add rsp, 8	;pop arguments
    movq rax, xmm0
    push rax
    movsd xmm0, [rbp+24]
    movq rax, xmm0
    push rax
    movsd xmm0, [rbp-16]
	pop rcx
	movq xmm1, rcx
	ucomisd xmm0, xmm1
	sete al
	movzx rax, al
    add rsp, 16		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 16		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.std.io.print:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
    mov rax, 8
    push rax
    mov rax, [rbp+24]
    push rax
    mov rax, -1
    pop rcx
    mov rax, [rcx + rax * 8]
    pop rcx
    imul rax, rcx
	mov rdx, rax	; length of string
    mov rax, [rbp+24]
	mov rsi, rax	; pointer to string
	mov rax, 1		; syscall: sys_write
	mov rdi, 1		; file descriptor: stdout
	syscall
    add rsp, 0		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.std.io.println:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
	;push function arguments to the stack in reverse order
    mov rax, [rbp+24]
    push rax
    call .toplevel.std.io.print
    add rsp, 8	;pop arguments
    mov rax, 0xa	;\n
    push rax
    mov rax, 1		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    push rax
	;push function arguments to the stack in reverse order
    mov rax, [rbp-24]
    push rax
    call .toplevel.std.io.print
    add rsp, 32	;pop arguments
    add rsp, 0		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.std.io.printchar:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
    mov rax, [rbp+24]
    push rax
    mov rax, 1		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    push rax
	;push function arguments to the stack in reverse order
    mov rax, [rbp-24]
    push rax
    call .toplevel.std.io.print
    add rsp, 32	;pop arguments
    add rsp, 0		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.std.io.printcharln:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
	;push function arguments to the stack in reverse order
    mov rax, [rbp+24]
    push rax
    call .toplevel.std.io.printchar
    add rsp, 8	;pop arguments
    mov rax, 0xa	;\n
    push rax
    mov rax, 1		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    push rax
	;push function arguments to the stack in reverse order
    mov rax, [rbp-24]
    push rax
    call .toplevel.std.io.print
    add rsp, 32	;pop arguments
    add rsp, 0		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.std.io.printi:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
    ;if statement
    mov rax, 0
    push rax
    mov rax, [rbp+24]
    pop rcx
    cmp rax, rcx
    sete al
    movzx rax, al
    cmp rax, 0
    je .else_8
    mov rax, 0x30	;0
    push rax
    mov rax, 1		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    push rax
	;push function arguments to the stack in reverse order
    mov rax, [rbp-24]
    push rax
    call .toplevel.std.io.print
    add rsp, 32	;pop arguments
    mov rax, 0
    add rsp, 0		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 0		;end of block, pop local variables
    jmp .end_8
.else_8:
.end_8:
    ;if statement
    mov rax, 0
    push rax
    mov rax, [rbp+24]
    pop rcx
    cmp rax, rcx
    setl al
    movzx rax, al
    cmp rax, 0
    je .else_9
    mov rax, 0x2d	;-
    push rax
    mov rax, 1		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    push rax
	;push function arguments to the stack in reverse order
    mov rax, [rbp-24]
    push rax
    call .toplevel.std.io.print
    add rsp, 32	;pop arguments
    mov rax, [rbp+24]
    neg rax
    mov [rbp+24], rax
    add rsp, 0		;end of block, pop local variables
    jmp .end_9
.else_9:
.end_9:
    mov rax, 0x0	;
    push rax
    mov rax, 0x0	;
    push rax
    mov rax, 0x0	;
    push rax
    mov rax, 0x0	;
    push rax
    mov rax, 0x0	;
    push rax
    mov rax, 0x0	;
    push rax
    mov rax, 0x0	;
    push rax
    mov rax, 0x0	;
    push rax
    mov rax, 8		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    push rax
    mov rax, 0
    push rax
    ;while statement
.while_start_3:
    mov rax, 0
    push rax
    mov rax, [rbp+24]
    pop rcx
    cmp rax, rcx
    setg al
    movzx rax, al
    cmp rax, 0
    je .while_end_3
    mov rax, 0x30	;0
    push rax
    mov rax, 10
    push rax
    mov rax, [rbp+24]
    pop rcx
    cqo
    idiv rcx
    mov rax, rdx
    pop rcx
    add rax, rcx
    push rax
    mov rax, 8
    push rax
    mov rax, [rbp-80]
    push rax
    mov rax, 8
    push rax
    mov rax, [rbp-88]
    pop rcx
    cqo
    idiv rcx
    push rax
    mov rax, 7
    pop rcx
    sub rax, rcx
    pop rcx
    mov rax, [rcx + rax * 8]
    pop rcx
    shl rax, cl
    pop rcx
    or rax, rcx
    push rax
    mov rax, [rbp-80]
    push rax
    mov rax, 8
    push rax
    mov rax, [rbp-88]
    pop rcx
    cqo
    idiv rcx
    push rax
    mov rax, 7
    pop rcx
    sub rax, rcx
    mov r8, rax
    imul r8, 8
    pop rax
    add r8, rax
    pop rax
    mov [r8], rax
    mov rax, 10
    push rax
    mov rax, [rbp+24]
    pop rcx
    cqo
    idiv rcx
    mov [rbp+24], rax
    mov rax, 1
    push rax
    mov rax, [rbp-88]
    pop rcx
    add rax, rcx
    mov [rbp-88], rax
    add rsp, 0		;end of block, pop local variables
    jmp .while_start_3
.while_end_3:
	;push function arguments to the stack in reverse order
    mov rax, [rbp-80]
    push rax
    call .toplevel.std.io.print
    add rsp, 8	;pop arguments
    add rsp, 88		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.std.io.printiln:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
	;push function arguments to the stack in reverse order
    mov rax, [rbp+24]
    push rax
    call .toplevel.std.io.printi
    add rsp, 8	;pop arguments
    mov rax, 0xa	;\n
    push rax
    mov rax, 1		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    push rax
	;push function arguments to the stack in reverse order
    mov rax, [rbp-24]
    push rax
    call .toplevel.std.io.print
    add rsp, 32	;pop arguments
    add rsp, 0		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.std.io.printf:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
    ;if statement
    movsd xmm0, [.float.0]	; Load float into xmm0
    movq rax, xmm0
    push rax
    movsd xmm0, [rbp+24]
	pop rcx
	movq xmm1, rcx
	ucomisd xmm0, xmm1
	setb al
	movzx rax, al
    cmp rax, 0
    je .else_10
    mov rax, 0x2d	;-
    push rax
    mov rax, 1		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    push rax
	;push function arguments to the stack in reverse order
    mov rax, [rbp-24]
    push rax
    call .toplevel.std.io.print
    add rsp, 32	;pop arguments
    movsd xmm0, [rbp+24]
    pcmpeqd xmm1, xmm1	; xmm1 = all ones (0xFFFFFFFFFFFFFFFF)
    psllq xmm1, 63		; xmm1 = 0x8000000000000000 (sign bit mask)
    xorpd xmm0, xmm1	; Flip the sign bit of xmm0
    movq [rbp+24], xmm0
    add rsp, 0		;end of block, pop local variables
    jmp .end_10
.else_10:
.end_10:
	;push function arguments to the stack in reverse order
    movsd xmm0, [rbp+24]
    movq rax, xmm0
    push rax
    call .toplevel.std.io.floor
    add rsp, 8	;pop arguments
    push rax
	;push function arguments to the stack in reverse order
    mov rax, [rbp-8]
    push rax
    call .toplevel.std.io.printi
    add rsp, 8	;pop arguments
	;push function arguments to the stack in reverse order
    mov rax, [rbp-8]
    push rax
    call .toplevel.std.io.inttof
    add rsp, 8	;pop arguments
    movq rax, xmm0
    push rax
    movsd xmm0, [rbp+24]
	pop rcx
	movq xmm1, rcx
	subsd xmm0, xmm1
    movq rax, xmm0
    push rax
    mov rax, 0x2e	;.
    push rax
    mov rax, 1		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    push rax
	;push function arguments to the stack in reverse order
    mov rax, [rbp-40]
    push rax
    call .toplevel.std.io.print
    add rsp, 32	;pop arguments
    movsd xmm0, [.float.1]	; Load float into xmm0
    mulsd xmm0, [rbp-16]
    movq [rbp-16], xmm0
	;push function arguments to the stack in reverse order
    movsd xmm0, [rbp-16]
    movq rax, xmm0
    push rax
    call .toplevel.std.io.floor
    add rsp, 8	;pop arguments
    mov [rbp-8], rax
	;push function arguments to the stack in reverse order
    mov rax, [rbp-8]
    push rax
    call .toplevel.std.io.printi
    add rsp, 8	;pop arguments
    mov rax, 0
    add rsp, 16		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 16		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.std.io.printfln:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
	;push function arguments to the stack in reverse order
    movsd xmm0, [rbp+24]
    movq rax, xmm0
    push rax
    call .toplevel.std.io.printf
    add rsp, 8	;pop arguments
    mov rax, 0xa	;\n
    push rax
    mov rax, 1		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    push rax
	;push function arguments to the stack in reverse order
    mov rax, [rbp-24]
    push rax
    call .toplevel.std.io.print
    add rsp, 32	;pop arguments
    add rsp, 0		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.std.io.printbool:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
    ;if statement
    mov rax, [rbp+24]
    cmp rax, 0
    je .else_11
    mov rax, 0x65757274	;true
    push rax
    mov rax, 1		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    push rax
	;push function arguments to the stack in reverse order
    mov rax, [rbp-24]
    push rax
    call .toplevel.std.io.print
    add rsp, 32	;pop arguments
    add rsp, 0		;end of block, pop local variables
    jmp .end_11
.else_11:
    mov rax, 0x65736c6166	;false
    push rax
    mov rax, 1		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    push rax
	;push function arguments to the stack in reverse order
    mov rax, [rbp-24]
    push rax
    call .toplevel.std.io.print
    add rsp, 32	;pop arguments
    add rsp, 0		;end of block, pop local variables
.end_11:
    add rsp, 0		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.std.io.printboolln:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
	;push function arguments to the stack in reverse order
    mov rax, [rbp+24]
    push rax
    call .toplevel.std.io.printbool
    add rsp, 8	;pop arguments
    mov rax, 0xa	;\n
    push rax
    mov rax, 1		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    push rax
	;push function arguments to the stack in reverse order
    mov rax, [rbp-24]
    push rax
    call .toplevel.std.io.print
    add rsp, 32	;pop arguments
    add rsp, 0		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.std.math.ftoint:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
    movsd xmm0, [rbp+24]
	cvtsd2si rax, xmm0  ; Convert double in xmm0 to 64-bit integer in rax
    add rsp, 0		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.std.math.inttof:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
    mov rax, [rbp+24]
	cvtsi2sd xmm0, rax	; Convert 64-bit integer in rax to double in xmm0
    add rsp, 0		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.std.math.floor:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
	stmxcsr [mxcsr_val]
	mov eax, [mxcsr_val]
	and eax, 0xFFFF9FFF	; Clear the round control bits
	or eax, 0x00002000	; 
	mov [mxcsr_val], eax
	ldmxcsr [mxcsr_val]
    movsd xmm0, [rbp+24]
	cvtsd2si rax, xmm0; round down (toward -inf)
    add rsp, 0		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.std.math.ceil:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
	stmxcsr [mxcsr_val]
	mov eax, [mxcsr_val]
	and eax, 0xFFFF9FFF	; 
	or eax, 0x00003000	; 
	mov [mxcsr_val], eax
	ldmxcsr [mxcsr_val]
    movsd xmm0, [rbp+24]
	cvtsd2si rax, xmm0; round up (toward +inf)
    add rsp, 0		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.std.math.isint:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
	;push function arguments to the stack in reverse order
    movsd xmm0, [rbp+24]
    movq rax, xmm0
    push rax
    call .toplevel.std.math.floor
    add rsp, 8	;pop arguments
    push rax
	;push function arguments to the stack in reverse order
    mov rax, [rbp-8]
    push rax
    call .toplevel.std.math.inttof
    add rsp, 8	;pop arguments
    movq rax, xmm0
    push rax
    movsd xmm0, [rbp+24]
    movq rax, xmm0
    push rax
    movsd xmm0, [rbp-16]
	pop rcx
	movq xmm1, rcx
	ucomisd xmm0, xmm1
	sete al
	movzx rax, al
    add rsp, 16		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 16		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.std.math.mod:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
    ;if statement
    movsd xmm0, [.float.2]	; Load float into xmm0
    movq rax, xmm0
    push rax
	;push function arguments to the stack in reverse order
    movsd xmm0, [rbp+32]
    movq rax, xmm0
    push rax
    call .toplevel.std.math.absf
    add rsp, 8	;pop arguments
	pop rcx
	movq xmm1, rcx
	ucomisd xmm0, xmm1
	setb al
	movzx rax, al
    cmp rax, 0
    je .else_12
    mov rax, 0x6f72657a20796220	;?by?zero
    push rax
    mov rax, 0x6e6f697369766944	;Division
    push rax
    mov rax, 2		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    push rax
	;push function arguments to the stack in reverse order
    mov rax, 1
    push rax
    mov rax, [rbp-32]
    push rax
    call .toplevel.std.math.exception
    add rsp, 48	;pop arguments
    add rsp, 0		;end of block, pop local variables
    jmp .end_12
.else_12:
.end_12:
    movsd xmm0, [rbp+32]
    movq rax, xmm0
    push rax
    movsd xmm0, [rbp+24]
	pop rcx
	movq xmm1, rcx
	divsd xmm0, xmm1
    movq rax, xmm0
    push rax
	;push function arguments to the stack in reverse order
    movsd xmm0, [rbp-8]
    movq rax, xmm0
    push rax
    call .toplevel.std.math.floor
    add rsp, 16	;pop arguments
    push rax
	;push function arguments to the stack in reverse order
    mov rax, [rbp-8]
    push rax
    call .toplevel.std.math.inttof
    add rsp, 16	;pop arguments
    movq rax, xmm0
    push rax
    movsd xmm0, [rbp+32]
	pop rcx
	movq xmm1, rcx
	mulsd xmm0, xmm1
    movq rax, xmm0
    push rax
    movsd xmm0, [rbp+24]
	pop rcx
	movq xmm1, rcx
	subsd xmm0, xmm1
    movq rax, xmm0
    push rax
    ;if statement
    movsd xmm0, [.float.0]	; Load float into xmm0
    movq rax, xmm0
    push rax
    movsd xmm0, [rbp-8]
	pop rcx
	movq xmm1, rcx
	ucomisd xmm0, xmm1
	setb al
	movzx rax, al
    cmp rax, 0
    je .else_13
    movsd xmm0, [rbp+32]
    addsd xmm0, [rbp-8]
    movq [rbp-8], xmm0
    add rsp, 0		;end of block, pop local variables
    jmp .end_13
.else_13:
.end_13:
    movsd xmm0, [rbp-8]
    add rsp, 8		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 8		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.std.math.absf:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
    ;if statement
    movsd xmm0, [.float.0]	; Load float into xmm0
    movq rax, xmm0
    push rax
    movsd xmm0, [rbp+24]
	pop rcx
	movq xmm1, rcx
	ucomisd xmm0, xmm1
	setb al
	movzx rax, al
    cmp rax, 0
    je .else_14
    movsd xmm0, [rbp+24]
    pcmpeqd xmm1, xmm1	; xmm1 = all ones (0xFFFFFFFFFFFFFFFF)
    psllq xmm1, 63		; xmm1 = 0x8000000000000000 (sign bit mask)
    xorpd xmm0, xmm1	; Flip the sign bit of xmm0
    add rsp, 0		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 0		;end of block, pop local variables
    jmp .end_14
.else_14:
.end_14:
    movsd xmm0, [rbp+24]
    add rsp, 0		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 0		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.std.math.absi:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
    ;if statement
    mov rax, 0
    push rax
    mov rax, [rbp+24]
    pop rcx
    cmp rax, rcx
    setl al
    movzx rax, al
    cmp rax, 0
    je .else_15
    mov rax, [rbp+24]
    neg rax
    add rsp, 0		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 0		;end of block, pop local variables
    jmp .end_15
.else_15:
.end_15:
    mov rax, [rbp+24]
    add rsp, 0		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 0		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.std.math.printerr:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
    mov rax, 8
    push rax
    mov rax, [rbp+24]
    push rax
    mov rax, -1
    pop rcx
    mov rax, [rcx + rax * 8]
    pop rcx
    imul rax, rcx
	mov rdx, rax	; length of string
    mov rax, [rbp+24]
	mov rsi, rax	; pointer to string
	mov rax, 1		; syscall: sys_write
	mov rdi, 2		; file descriptor: stderr
	syscall
    add rsp, 0		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.std.math.exception:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
    mov rax, 0x203a6e	;n:?
    push rax
    mov rax, 0x6f69747065637845	;Exceptio
    push rax
    mov rax, 2		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    push rax
	;push function arguments to the stack in reverse order
    mov rax, [rbp-32]
    push rax
    call .toplevel.std.math.printerr
    add rsp, 40	;pop arguments
	;push function arguments to the stack in reverse order
    mov rax, [rbp+24]
    push rax
    call .toplevel.std.math.printerr
    add rsp, 8	;pop arguments
    mov rax, 0xa	;\n
    push rax
    mov rax, 1		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    push rax
	;push function arguments to the stack in reverse order
    mov rax, [rbp-24]
    push rax
    call .toplevel.std.math.printerr
    add rsp, 32	;pop arguments
    mov rax, [rbp+32]
	mov rdi, rax
	mov rax, 60
	syscall
	
    add rsp, 0		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.std.math.factorial:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
    ;if statement
    mov rax, 0
    push rax
    mov rax, [rbp+24]
    pop rcx
    cmp rax, rcx
    setl al
    movzx rax, al
    cmp rax, 0
    je .else_16
    mov rax, 0x7265626d	;mber
    push rax
    mov rax, 0x756e206576697461	;ative?nu
    push rax
    mov rax, 0x67656e20666f206c	;l?of?neg
    push rax
    mov rax, 0x6169726f74636146	;Factoria
    push rax
    mov rax, 4		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    push rax
	;push function arguments to the stack in reverse order
    mov rax, 1
    push rax
    mov rax, [rbp-48]
    push rax
    call .toplevel.std.math.exception
    add rsp, 64	;pop arguments
    add rsp, 0		;end of block, pop local variables
    jmp .end_16
.else_16:
.end_16:
    ;if statement
    mov rax, 1
    push rax
    mov rax, [rbp+24]
    pop rcx
    cmp rax, rcx
    sete al
    movzx rax, al
    push rax
    mov rax, 0
    push rax
    mov rax, [rbp+24]
    pop rcx
    cmp rax, rcx
    sete al
    movzx rax, al
    pop rcx
    or rax, rcx
    cmp rax, 0
    je .else_17
    mov rax, 1
    add rsp, 0		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 0		;end of block, pop local variables
    jmp .end_17
.else_17:
.end_17:
    mov rax, 1
    push rax
    push 0
    mov rax, 2
    mov [rbp-16], rax
    ;for statement
.for_start_4:
    mov rax, [rbp+24]
    push rax
    mov rax, [rbp-16]
    pop rcx
    cmp rax, rcx
    setle al
    movzx rax, al
    cmp rax, 0
    je .for_end_4
    mov rax, [rbp-16]
    mov rcx, [rbp-8]
    imul rax, rcx
    mov [rbp-8], rax
    add rsp, 0		;end of block, pop local variables
    mov rax, 1
    add [rbp-16], rax
    jmp .for_start_4
.for_end_4:
    mov rax, [rbp-8]
    add rsp, 16		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 16		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.std.math.binomial:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
    ;if statement
    mov rax, [rbp+24]
    push rax
    mov rax, [rbp+32]
    pop rcx
    cmp rax, rcx
    setg al
    movzx rax, al
    push rax
    mov rax, 0
    push rax
    mov rax, [rbp+32]
    pop rcx
    cmp rax, rcx
    setl al
    movzx rax, al
    pop rcx
    or rax, rcx
    cmp rax, 0
    je .else_18
    mov rax, 0x73726574656d61	;ameters
    push rax
    mov rax, 0x72617020746e6569	;ient?par
    push rax
    mov rax, 0x63696666656f6320	;?coeffic
    push rax
    mov rax, 0x6c61696d6f6e6962	;binomial
    push rax
    mov rax, 0x2064696c61766e49	;Invalid?
    push rax
    mov rax, 5		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    push rax
	;push function arguments to the stack in reverse order
    mov rax, 1
    push rax
    mov rax, [rbp-56]
    push rax
    call .toplevel.std.math.exception
    add rsp, 72	;pop arguments
    add rsp, 0		;end of block, pop local variables
    jmp .end_18
.else_18:
.end_18:
    ;if statement
    mov rax, [rbp+24]
    push rax
    mov rax, [rbp+32]
    pop rcx
    cmp rax, rcx
    sete al
    movzx rax, al
    push rax
    mov rax, 0
    push rax
    mov rax, [rbp+32]
    pop rcx
    cmp rax, rcx
    sete al
    movzx rax, al
    pop rcx
    or rax, rcx
    cmp rax, 0
    je .else_19
    mov rax, 1
    add rsp, 0		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 0		;end of block, pop local variables
    jmp .end_19
.else_19:
.end_19:
    mov rax, 1
    push rax
    push 0
    mov rax, 1
    mov [rbp-16], rax
    ;for statement
.for_start_5:
    mov rax, [rbp+32]
    push rax
    mov rax, [rbp-16]
    pop rcx
    cmp rax, rcx
    setle al
    movzx rax, al
    cmp rax, 0
    je .for_end_5
    mov rax, 1
    push rax
    mov rax, [rbp-16]
    push rax
    mov rax, [rbp+24]
    pop rcx
    sub rax, rcx
    pop rcx
    add rax, rcx
    mov rcx, [rbp-8]
    imul rax, rcx
    mov [rbp-8], rax
    mov rax, [rbp-16]
    mov rcx, rax
    mov rax, [rbp-8]
    cqo
    idiv rcx
    mov [rbp-8], rax
    add rsp, 0		;end of block, pop local variables
    mov rax, 1
    add [rbp-16], rax
    jmp .for_start_5
.for_end_5:
    mov rax, [rbp-8]
    add rsp, 16		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 16		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.std.math.sqrt:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
    ;if statement
    movsd xmm0, [.float.0]	; Load float into xmm0
    movq rax, xmm0
    push rax
    movsd xmm0, [rbp+24]
	pop rcx
	movq xmm1, rcx
	ucomisd xmm0, xmm1
	setb al
	movzx rax, al
    cmp rax, 0
    je .else_20
    mov rax, 0x7265626d756e	;number
    push rax
    mov rax, 0x2065766974616765	;egative?
    push rax
    mov rax, 0x6e20666f20746f6f	;oot?of?n
    push rax
    mov rax, 0x7220657261757153	;Square?r
    push rax
    mov rax, 4		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    push rax
	;push function arguments to the stack in reverse order
    mov rax, 1
    push rax
    mov rax, [rbp-48]
    push rax
    call .toplevel.std.math.exception
    add rsp, 64	;pop arguments
    add rsp, 0		;end of block, pop local variables
    jmp .end_20
.else_20:
.end_20:
    movsd xmm0, [rbp+24]
    movq rax, xmm0
    push rax
    push 0
    mov rax, 0
    mov [rbp-16], rax
    ;for statement
.for_start_6:
    mov rax, 10
    push rax
    mov rax, [rbp-16]
    pop rcx
    cmp rax, rcx
    setl al
    movzx rax, al
    cmp rax, 0
    je .for_end_6
    movsd xmm0, [.float.3]	; Load float into xmm0
    movq rax, xmm0
    push rax
    movsd xmm0, [rbp-8]
    movq rax, xmm0
    push rax
    movsd xmm0, [rbp+24]
	pop rcx
	movq xmm1, rcx
	divsd xmm0, xmm1
    movq rax, xmm0
    push rax
    movsd xmm0, [rbp-8]
	pop rcx
	movq xmm1, rcx
	addsd xmm0, xmm1
	pop rcx
	movq xmm1, rcx
	divsd xmm0, xmm1
    movq [rbp-8], xmm0
    add rsp, 0		;end of block, pop local variables
    mov rax, 1
    push rax
    mov rax, [rbp-16]
    pop rcx
    add rax, rcx
    mov [rbp-16], rax
    jmp .for_start_6
.for_end_6:
    movsd xmm0, [rbp-8]
    add rsp, 16		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 16		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.std.math.ln:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
    ;if statement
    movsd xmm0, [.float.0]	; Load float into xmm0
    movq rax, xmm0
    push rax
    movsd xmm0, [rbp+24]
	pop rcx
	movq xmm1, rcx
	ucomisd xmm0, xmm1
	setbe al
	movzx rax, al
    cmp rax, 0
    je .else_21
    mov rax, 0x7265626d756e2065	;e?number
    push rax
    mov rax, 0x76697469736f702d	;-positiv
    push rax
    mov rax, 0x6e6f6e20666f206d	;m?of?non
    push rax
    mov rax, 0x6874697261676f4c	;Logarith
    push rax
    mov rax, 4		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    push rax
	;push function arguments to the stack in reverse order
    mov rax, 1
    push rax
    mov rax, [rbp-48]
    push rax
    call .toplevel.std.math.exception
    add rsp, 64	;pop arguments
    add rsp, 0		;end of block, pop local variables
    jmp .end_21
.else_21:
.end_21:
    mov rax, 35
    push rax
    movsd xmm0, [.float.4]	; Load float into xmm0
    movq rax, xmm0
    push rax
    movsd xmm0, [rbp+24]
	pop rcx
	movq xmm1, rcx
	subsd xmm0, xmm1
    movq rax, xmm0
    push rax
    push 0
    mov rax, 0
    mov [rbp-24], rax
    ;for statement
.for_start_7:
    mov rax, [rbp-8]
    push rax
    mov rax, [rbp-24]
    pop rcx
    cmp rax, rcx
    setl al
    movzx rax, al
    cmp rax, 0
    je .for_end_7
    movsd xmm0, [rbp-16]
    movq rax, xmm0
    push rax
    movsd xmm0, [.float.4]	; Load float into xmm0
	pop rcx
	movq xmm1, rcx
	addsd xmm0, xmm1
    movq rax, xmm0
    push rax
	;push function arguments to the stack in reverse order
    movsd xmm0, [rbp-32]
    movq rax, xmm0
    push rax
    call .toplevel.std.math.sqrt
    add rsp, 16	;pop arguments
    movq rax, xmm0
    push rax
    movsd xmm0, [.float.4]	; Load float into xmm0
	pop rcx
	movq xmm1, rcx
	addsd xmm0, xmm1
    movq rax, xmm0
    push rax
    movsd xmm0, [rbp-16]
	pop rcx
	movq xmm1, rcx
	divsd xmm0, xmm1
    movq [rbp-16], xmm0
    add rsp, 0		;end of block, pop local variables
    mov rax, 1
    push rax
    mov rax, [rbp-24]
    pop rcx
    add rax, rcx
    mov [rbp-24], rax
    jmp .for_start_7
.for_end_7:
    mov rax, 0
    mov [rbp-24], rax
    ;for statement
.for_start_8:
    mov rax, [rbp-8]
    push rax
    mov rax, [rbp-24]
    pop rcx
    cmp rax, rcx
    setl al
    movzx rax, al
    cmp rax, 0
    je .for_end_8
    movsd xmm0, [.float.3]	; Load float into xmm0
    movq rax, xmm0
    push rax
    movsd xmm0, [rbp-16]
	pop rcx
	movq xmm1, rcx
	mulsd xmm0, xmm1
    movq [rbp-16], xmm0
    add rsp, 0		;end of block, pop local variables
    mov rax, 1
    push rax
    mov rax, [rbp-24]
    pop rcx
    add rax, rcx
    mov [rbp-24], rax
    jmp .for_start_8
.for_end_8:
    movsd xmm0, [rbp-16]
    add rsp, 24		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 24		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.std.math.powi:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
    ;if statement
    mov rax, 0
    push rax
    mov rax, [rbp+32]
    pop rcx
    cmp rax, rcx
    setl al
    movzx rax, al
    cmp rax, 0
    je .else_22
    mov rax, 0x74	;t
    push rax
    mov rax, 0x6e656e6f70786520	;?exponen
    push rax
    mov rax, 0x657669746167654e	;Negative
    push rax
    mov rax, 3		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    push rax
	;push function arguments to the stack in reverse order
    mov rax, 1
    push rax
    mov rax, [rbp-40]
    push rax
    call .toplevel.std.math.exception
    add rsp, 56	;pop arguments
    add rsp, 0		;end of block, pop local variables
    jmp .end_22
.else_22:
.end_22:
    mov rax, 1
    push rax
    push 0
    mov rax, 0
    mov [rbp-16], rax
    ;for statement
.for_start_9:
    mov rax, [rbp+32]
    push rax
    mov rax, [rbp-16]
    pop rcx
    cmp rax, rcx
    setl al
    movzx rax, al
    cmp rax, 0
    je .for_end_9
    mov rax, [rbp+24]
    mov rcx, [rbp-8]
    imul rax, rcx
    mov [rbp-8], rax
    add rsp, 0		;end of block, pop local variables
    mov rax, 1
    push rax
    mov rax, [rbp-16]
    pop rcx
    add rax, rcx
    mov [rbp-16], rax
    jmp .for_start_9
.for_end_9:
    mov rax, [rbp-8]
    add rsp, 16		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 16		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.std.math.powfi:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
    ;if statement
    mov rax, 0
    push rax
    mov rax, [rbp+32]
    pop rcx
    cmp rax, rcx
    setl al
    movzx rax, al
    cmp rax, 0
    je .else_23
    mov rax, 0x74	;t
    push rax
    mov rax, 0x6e656e6f70786520	;?exponen
    push rax
    mov rax, 0x657669746167654e	;Negative
    push rax
    mov rax, 3		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    push rax
	;push function arguments to the stack in reverse order
    mov rax, 1
    push rax
    mov rax, [rbp-40]
    push rax
    call .toplevel.std.math.exception
    add rsp, 56	;pop arguments
    add rsp, 0		;end of block, pop local variables
    jmp .end_23
.else_23:
.end_23:
    movsd xmm0, [.float.4]	; Load float into xmm0
    movq rax, xmm0
    push rax
    push 0
    mov rax, 0
    mov [rbp-16], rax
    ;for statement
.for_start_10:
    mov rax, [rbp+32]
    push rax
    mov rax, [rbp-16]
    pop rcx
    cmp rax, rcx
    setl al
    movzx rax, al
    cmp rax, 0
    je .for_end_10
    movsd xmm0, [rbp+24]
    mulsd xmm0, [rbp-8]
    movq [rbp-8], xmm0
    add rsp, 0		;end of block, pop local variables
    mov rax, 1
    push rax
    mov rax, [rbp-16]
    pop rcx
    add rax, rcx
    mov [rbp-16], rax
    jmp .for_start_10
.for_end_10:
    movsd xmm0, [rbp-8]
    add rsp, 16		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 16		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.std.math.exp:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
    ;if statement
    movsd xmm0, [.float.5]	; Load float into xmm0
    pcmpeqd xmm1, xmm1	; xmm1 = all ones (0xFFFFFFFFFFFFFFFF)
    psllq xmm1, 63		; xmm1 = 0x8000000000000000 (sign bit mask)
    xorpd xmm0, xmm1	; Flip the sign bit of xmm0
    movq rax, xmm0
    push rax
    movsd xmm0, [rbp+24]
	pop rcx
	movq xmm1, rcx
	ucomisd xmm0, xmm1
	setb al
	movzx rax, al
    cmp rax, 0
    je .else_24
    movsd xmm0, [.float.0]	; Load float into xmm0
    add rsp, 0		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 0		;end of block, pop local variables
    jmp .end_24
.else_24:
    ;if statement
    movsd xmm0, [.float.5]	; Load float into xmm0
    movq rax, xmm0
    push rax
    movsd xmm0, [rbp+24]
	pop rcx
	movq xmm1, rcx
	ucomisd xmm0, xmm1
	seta al
	movzx rax, al
    cmp rax, 0
    je .else_25
    mov rax, 0x6567	;ge
    push rax
    mov rax, 0x72616c206f6f7420	;?too?lar
    push rax
    mov rax, 0x746e656e6f707845	;Exponent
    push rax
    mov rax, 3		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    push rax
	;push function arguments to the stack in reverse order
    mov rax, 1
    push rax
    mov rax, [rbp-40]
    push rax
    call .toplevel.std.math.exception
    add rsp, 56	;pop arguments
    movsd xmm0, [.float.0]	; Load float into xmm0
    add rsp, 0		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 0		;end of block, pop local variables
    jmp .end_25
.else_25:
.end_25:
    add rsp, 0		;end of block, pop local variables
.end_24:
    mov rax, 35
    push rax
    movsd xmm0, [rbp+24]
    movq rax, xmm0
    push rax
    push 0
    mov rax, 0
    mov [rbp-24], rax
    ;for statement
.for_start_11:
    mov rax, [rbp-8]
    push rax
    mov rax, [rbp-24]
    pop rcx
    cmp rax, rcx
    setl al
    movzx rax, al
    cmp rax, 0
    je .for_end_11
    movsd xmm0, [.float.3]	; Load float into xmm0
    movq rax, xmm0
    push rax
    movsd xmm0, [rbp-16]
	pop rcx
	movq xmm1, rcx
	divsd xmm0, xmm1
    movq [rbp-16], xmm0
    add rsp, 0		;end of block, pop local variables
    mov rax, 1
    push rax
    mov rax, [rbp-24]
    pop rcx
    add rax, rcx
    mov [rbp-24], rax
    jmp .for_start_11
.for_end_11:
    movsd xmm0, [rbp-16]
    movq rax, xmm0
    push rax
    movsd xmm0, [.float.4]	; Load float into xmm0
	pop rcx
	movq xmm1, rcx
	addsd xmm0, xmm1
    movq [rbp-16], xmm0
    mov rax, 0
    mov [rbp-24], rax
    ;for statement
.for_start_12:
    mov rax, [rbp-8]
    push rax
    mov rax, [rbp-24]
    pop rcx
    cmp rax, rcx
    setl al
    movzx rax, al
    cmp rax, 0
    je .for_end_12
    movsd xmm0, [rbp-16]
    mulsd xmm0, [rbp-16]
    movq [rbp-16], xmm0
    add rsp, 0		;end of block, pop local variables
    mov rax, 1
    push rax
    mov rax, [rbp-24]
    pop rcx
    add rax, rcx
    mov [rbp-24], rax
    jmp .for_start_12
.for_end_12:
    movsd xmm0, [rbp-16]
    add rsp, 24		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 24		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.std.math.sin:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
    mov rax, 0
    push rax
    ;if statement
    movsd xmm0, [.float.0]	; Load float into xmm0
    movq rax, xmm0
    push rax
    movsd xmm0, [rbp+24]
	pop rcx
	movq xmm1, rcx
	ucomisd xmm0, xmm1
	setb al
	movzx rax, al
    cmp rax, 0
    je .else_26
    mov rax, [rbp-8]
    not rax
    mov [rbp-8], rax
    movsd xmm0, [rbp+24]
    pcmpeqd xmm1, xmm1	; xmm1 = all ones (0xFFFFFFFFFFFFFFFF)
    psllq xmm1, 63		; xmm1 = 0x8000000000000000 (sign bit mask)
    xorpd xmm0, xmm1	; Flip the sign bit of xmm0
    movq [rbp+24], xmm0
    add rsp, 0		;end of block, pop local variables
    jmp .end_26
.else_26:
.end_26:
    movsd xmm0, [.constant.toplevel.std.math.PI]
    movq rax, xmm0
    push rax
    movsd xmm0, [.float.3]	; Load float into xmm0
	pop rcx
	movq xmm1, rcx
	mulsd xmm0, xmm1
    movq rax, xmm0
    push rax
	;push function arguments to the stack in reverse order
    movsd xmm0, [rbp-16]
    movq rax, xmm0
    push rax
    movsd xmm0, [rbp+24]
    movq rax, xmm0
    push rax
    call .toplevel.std.math.mod
    add rsp, 24	;pop arguments
    movq [rbp+24], xmm0
    ;if statement
    movsd xmm0, [.constant.toplevel.std.math.PI]
    movq rax, xmm0
    push rax
    movsd xmm0, [rbp+24]
	pop rcx
	movq xmm1, rcx
	ucomisd xmm0, xmm1
	seta al
	movzx rax, al
    cmp rax, 0
    je .else_27
    mov rax, [rbp-8]
    not rax
    mov [rbp-8], rax
    movsd xmm0, [rbp+24]
    movq rax, xmm0
    push rax
    movsd xmm0, [.constant.toplevel.std.math.PI]
    movq rax, xmm0
    push rax
    movsd xmm0, [.float.3]	; Load float into xmm0
	pop rcx
	movq xmm1, rcx
	mulsd xmm0, xmm1
	pop rcx
	movq xmm1, rcx
	subsd xmm0, xmm1
    movq [rbp+24], xmm0
    add rsp, 0		;end of block, pop local variables
    jmp .end_27
.else_27:
.end_27:
    ;if statement
    movsd xmm0, [.float.3]	; Load float into xmm0
    movq rax, xmm0
    push rax
    movsd xmm0, [.constant.toplevel.std.math.PI]
	pop rcx
	movq xmm1, rcx
	divsd xmm0, xmm1
    movq rax, xmm0
    push rax
    movsd xmm0, [rbp+24]
	pop rcx
	movq xmm1, rcx
	ucomisd xmm0, xmm1
	seta al
	movzx rax, al
    cmp rax, 0
    je .else_28
    movsd xmm0, [rbp+24]
    movq rax, xmm0
    push rax
    movsd xmm0, [.constant.toplevel.std.math.PI]
	pop rcx
	movq xmm1, rcx
	subsd xmm0, xmm1
    movq [rbp+24], xmm0
    add rsp, 0		;end of block, pop local variables
    jmp .end_28
.else_28:
.end_28:
    movsd xmm0, [.float.0]	; Load float into xmm0
    movq rax, xmm0
    push rax
    push 0
    mov rax, 0
    mov [rbp-24], rax
    ;for statement
.for_start_13:
    mov rax, 10
    push rax
    mov rax, [rbp-24]
    pop rcx
    cmp rax, rcx
    setl al
    movzx rax, al
    cmp rax, 0
    je .for_end_13
    mov rax, 1
    push rax
    mov rax, [rbp-24]
    push rax
    mov rax, 2
    pop rcx
    imul rax, rcx
    pop rcx
    add rax, rcx
    push rax
	;push function arguments to the stack in reverse order
    mov rax, [rbp-32]
    push rax
    call .toplevel.std.math.factorial
    add rsp, 16	;pop arguments
    push rax
	;push function arguments to the stack in reverse order
    mov rax, [rbp-32]
    push rax
    call .toplevel.std.math.inttof
    add rsp, 16	;pop arguments
    movq rax, xmm0
    push rax
    mov rax, 1
    push rax
    mov rax, [rbp-24]
    push rax
    mov rax, 2
    pop rcx
    imul rax, rcx
    pop rcx
    add rax, rcx
    push rax
	;push function arguments to the stack in reverse order
    mov rax, [rbp-40]
    push rax
    movsd xmm0, [rbp+24]
    movq rax, xmm0
    push rax
    call .toplevel.std.math.powfi
    add rsp, 24	;pop arguments
    movq rax, xmm0
    push rax
    mov rax, 1
    neg rax
    push rax
	;push function arguments to the stack in reverse order
    mov rax, [rbp-24]
    push rax
    mov rax, [rbp-48]
    push rax
    call .toplevel.std.math.powi
    add rsp, 24	;pop arguments
    push rax
	;push function arguments to the stack in reverse order
    mov rax, [rbp-48]
    push rax
    call .toplevel.std.math.inttof
    add rsp, 16	;pop arguments
	pop rcx
	movq xmm1, rcx
	mulsd xmm0, xmm1
	pop rcx
	movq xmm1, rcx
	divsd xmm0, xmm1
    addsd xmm0, [rbp-16]
    movq [rbp-16], xmm0
    add rsp, 0		;end of block, pop local variables
    mov rax, 1
    add [rbp-24], rax
    jmp .for_start_13
.for_end_13:
    ;if statement
    mov rax, [rbp-8]
    cmp rax, 0
    je .else_29
    movsd xmm0, [.float.4]	; Load float into xmm0
    pcmpeqd xmm1, xmm1	; xmm1 = all ones (0xFFFFFFFFFFFFFFFF)
    psllq xmm1, 63		; xmm1 = 0x8000000000000000 (sign bit mask)
    xorpd xmm0, xmm1	; Flip the sign bit of xmm0
    mulsd xmm0, [rbp-16]
    movq [rbp-16], xmm0
    add rsp, 0		;end of block, pop local variables
    jmp .end_29
.else_29:
.end_29:
    movsd xmm0, [rbp-16]
    add rsp, 24		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 24		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.std.math.cos:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
    movsd xmm0, [.float.3]	; Load float into xmm0
    movq rax, xmm0
    push rax
    movsd xmm0, [.constant.toplevel.std.math.PI]
	pop rcx
	movq xmm1, rcx
	divsd xmm0, xmm1
    addsd xmm0, [rbp+24]
    movq [rbp+24], xmm0
	;push function arguments to the stack in reverse order
    movsd xmm0, [rbp+24]
    movq rax, xmm0
    push rax
    call .toplevel.std.math.sin
    add rsp, 8	;pop arguments
    add rsp, 0		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 0		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.std.math.tan:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
	;push function arguments to the stack in reverse order
    movsd xmm0, [rbp+24]
    movq rax, xmm0
    push rax
    call .toplevel.std.math.sin
    add rsp, 8	;pop arguments
    movq rax, xmm0
    push rax
	;push function arguments to the stack in reverse order
    movsd xmm0, [rbp+24]
    movq rax, xmm0
    push rax
    call .toplevel.std.math.cos
    add rsp, 8	;pop arguments
    movq rax, xmm0
    push rax
    ;if statement
    movsd xmm0, [.float.6]	; Load float into xmm0
    movq rax, xmm0
    push rax
	;push function arguments to the stack in reverse order
    movsd xmm0, [rbp-16]
    movq rax, xmm0
    push rax
    call .toplevel.std.math.absf
    add rsp, 8	;pop arguments
	pop rcx
	movq xmm1, rcx
	ucomisd xmm0, xmm1
	setb al
	movzx rax, al
    cmp rax, 0
    je .else_30
    mov rax, 0x6e6174206e6920	;?in?tan
    push rax
    mov rax, 0x6f72657a20796220	;?by?zero
    push rax
    mov rax, 0x6e6f697369766944	;Division
    push rax
    mov rax, 3		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    push rax
	;push function arguments to the stack in reverse order
    mov rax, 1
    push rax
    mov rax, [rbp-56]
    push rax
    call .toplevel.std.math.exception
    add rsp, 56	;pop arguments
    add rsp, 0		;end of block, pop local variables
    jmp .end_30
.else_30:
.end_30:
    movsd xmm0, [rbp-16]
    movq rax, xmm0
    push rax
    movsd xmm0, [rbp-8]
	pop rcx
	movq xmm1, rcx
	divsd xmm0, xmm1
    add rsp, 16		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 16		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.std.math.cot:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
	;push function arguments to the stack in reverse order
    movsd xmm0, [rbp+24]
    movq rax, xmm0
    push rax
    call .toplevel.std.math.sin
    add rsp, 8	;pop arguments
    movq rax, xmm0
    push rax
	;push function arguments to the stack in reverse order
    movsd xmm0, [rbp+24]
    movq rax, xmm0
    push rax
    call .toplevel.std.math.cos
    add rsp, 8	;pop arguments
    movq rax, xmm0
    push rax
    ;if statement
    movsd xmm0, [.float.6]	; Load float into xmm0
    movq rax, xmm0
    push rax
	;push function arguments to the stack in reverse order
    movsd xmm0, [rbp-8]
    movq rax, xmm0
    push rax
    call .toplevel.std.math.absf
    add rsp, 8	;pop arguments
	pop rcx
	movq xmm1, rcx
	ucomisd xmm0, xmm1
	setb al
	movzx rax, al
    cmp rax, 0
    je .else_31
    mov rax, 0x746f63206e6920	;?in?cot
    push rax
    mov rax, 0x6f72657a20796220	;?by?zero
    push rax
    mov rax, 0x6e6f697369766944	;Division
    push rax
    mov rax, 3		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    push rax
	;push function arguments to the stack in reverse order
    mov rax, 1
    push rax
    mov rax, [rbp-56]
    push rax
    call .toplevel.std.math.exception
    add rsp, 56	;pop arguments
    add rsp, 0		;end of block, pop local variables
    jmp .end_31
.else_31:
.end_31:
    movsd xmm0, [rbp-8]
    movq rax, xmm0
    push rax
    movsd xmm0, [rbp-16]
	pop rcx
	movq xmm1, rcx
	divsd xmm0, xmm1
    add rsp, 16		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 16		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.std.math.sec:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
	;push function arguments to the stack in reverse order
    movsd xmm0, [rbp+24]
    movq rax, xmm0
    push rax
    call .toplevel.std.math.cos
    add rsp, 8	;pop arguments
    movq rax, xmm0
    push rax
    ;if statement
    movsd xmm0, [.float.6]	; Load float into xmm0
    movq rax, xmm0
    push rax
	;push function arguments to the stack in reverse order
    movsd xmm0, [rbp-8]
    movq rax, xmm0
    push rax
    call .toplevel.std.math.absf
    add rsp, 8	;pop arguments
	pop rcx
	movq xmm1, rcx
	ucomisd xmm0, xmm1
	setb al
	movzx rax, al
    cmp rax, 0
    je .else_32
    mov rax, 0x636573206e6920	;?in?sec
    push rax
    mov rax, 0x6f72657a20796220	;?by?zero
    push rax
    mov rax, 0x6e6f697369766944	;Division
    push rax
    mov rax, 3		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    push rax
	;push function arguments to the stack in reverse order
    mov rax, 1
    push rax
    mov rax, [rbp-48]
    push rax
    call .toplevel.std.math.exception
    add rsp, 56	;pop arguments
    add rsp, 0		;end of block, pop local variables
    jmp .end_32
.else_32:
.end_32:
    movsd xmm0, [rbp-8]
    movq rax, xmm0
    push rax
    movsd xmm0, [.float.4]	; Load float into xmm0
	pop rcx
	movq xmm1, rcx
	divsd xmm0, xmm1
    add rsp, 8		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 8		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.std.math.csc:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
	;push function arguments to the stack in reverse order
    movsd xmm0, [rbp+24]
    movq rax, xmm0
    push rax
    call .toplevel.std.math.sin
    add rsp, 8	;pop arguments
    movq rax, xmm0
    push rax
    ;if statement
    movsd xmm0, [.float.6]	; Load float into xmm0
    movq rax, xmm0
    push rax
	;push function arguments to the stack in reverse order
    movsd xmm0, [rbp-8]
    movq rax, xmm0
    push rax
    call .toplevel.std.math.absf
    add rsp, 8	;pop arguments
	pop rcx
	movq xmm1, rcx
	ucomisd xmm0, xmm1
	setb al
	movzx rax, al
    cmp rax, 0
    je .else_33
    mov rax, 0x637363206e6920	;?in?csc
    push rax
    mov rax, 0x6f72657a20796220	;?by?zero
    push rax
    mov rax, 0x6e6f697369766944	;Division
    push rax
    mov rax, 3		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    push rax
	;push function arguments to the stack in reverse order
    mov rax, 1
    push rax
    mov rax, [rbp-48]
    push rax
    call .toplevel.std.math.exception
    add rsp, 56	;pop arguments
    add rsp, 0		;end of block, pop local variables
    jmp .end_33
.else_33:
.end_33:
    movsd xmm0, [rbp-8]
    movq rax, xmm0
    push rax
    movsd xmm0, [.float.4]	; Load float into xmm0
	pop rcx
	movq xmm1, rcx
	divsd xmm0, xmm1
    add rsp, 8		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 8		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.std.math.maxi:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
    ;if statement
    mov rax, 0
    push rax
    mov rax, [rbp+24]
    push rax
    mov rax, -1
    pop rcx
    mov rax, [rcx + rax * 8]
    pop rcx
    cmp rax, rcx
    sete al
    movzx rax, al
    cmp rax, 0
    je .else_34
    mov rax, 0x7974706d6520	;?empty
    push rax
    mov rax, 0x7369207961727241	;Array?is
    push rax
    mov rax, 2		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    push rax
	;push function arguments to the stack in reverse order
    mov rax, 1
    push rax
    mov rax, [rbp-32]
    push rax
    call .toplevel.std.math.exception
    add rsp, 48	;pop arguments
    add rsp, 0		;end of block, pop local variables
    jmp .end_34
.else_34:
.end_34:
    mov rax, [rbp+24]
    push rax
    mov rax, 0
    pop rcx
    mov rax, [rcx + rax * 8]
    push rax
    push 0
    mov rax, 1
    mov [rbp-16], rax
    ;for statement
.for_start_14:
    mov rax, [rbp+24]
    push rax
    mov rax, -1
    pop rcx
    mov rax, [rcx + rax * 8]
    push rax
    mov rax, [rbp-16]
    pop rcx
    cmp rax, rcx
    setl al
    movzx rax, al
    cmp rax, 0
    je .for_end_14
    ;if statement
    mov rax, [rbp-8]
    push rax
    mov rax, [rbp+24]
    push rax
    mov rax, [rbp-16]
    pop rcx
    mov rax, [rcx + rax * 8]
    pop rcx
    cmp rax, rcx
    setg al
    movzx rax, al
    cmp rax, 0
    je .else_35
    mov rax, [rbp+24]
    push rax
    mov rax, [rbp-16]
    pop rcx
    mov rax, [rcx + rax * 8]
    mov [rbp-8], rax
    add rsp, 0		;end of block, pop local variables
    jmp .end_35
.else_35:
.end_35:
    add rsp, 0		;end of block, pop local variables
    mov rax, 1
    push rax
    mov rax, [rbp-16]
    pop rcx
    add rax, rcx
    mov [rbp-16], rax
    jmp .for_start_14
.for_end_14:
    mov rax, [rbp-8]
    add rsp, 16		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 16		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.std.math.mini:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
    ;if statement
    mov rax, 0
    push rax
    mov rax, [rbp+24]
    push rax
    mov rax, -1
    pop rcx
    mov rax, [rcx + rax * 8]
    pop rcx
    cmp rax, rcx
    sete al
    movzx rax, al
    cmp rax, 0
    je .else_36
    mov rax, 0x7974706d6520	;?empty
    push rax
    mov rax, 0x7369207961727241	;Array?is
    push rax
    mov rax, 2		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    push rax
	;push function arguments to the stack in reverse order
    mov rax, 1
    push rax
    mov rax, [rbp-32]
    push rax
    call .toplevel.std.math.exception
    add rsp, 48	;pop arguments
    add rsp, 0		;end of block, pop local variables
    jmp .end_36
.else_36:
.end_36:
    mov rax, [rbp+24]
    push rax
    mov rax, 0
    pop rcx
    mov rax, [rcx + rax * 8]
    push rax
    push 0
    mov rax, 1
    mov [rbp-16], rax
    ;for statement
.for_start_15:
    mov rax, [rbp+24]
    push rax
    mov rax, -1
    pop rcx
    mov rax, [rcx + rax * 8]
    push rax
    mov rax, [rbp-16]
    pop rcx
    cmp rax, rcx
    setl al
    movzx rax, al
    cmp rax, 0
    je .for_end_15
    ;if statement
    mov rax, [rbp-8]
    push rax
    mov rax, [rbp+24]
    push rax
    mov rax, [rbp-16]
    pop rcx
    mov rax, [rcx + rax * 8]
    pop rcx
    cmp rax, rcx
    setl al
    movzx rax, al
    cmp rax, 0
    je .else_37
    mov rax, [rbp+24]
    push rax
    mov rax, [rbp-16]
    pop rcx
    mov rax, [rcx + rax * 8]
    mov [rbp-8], rax
    add rsp, 0		;end of block, pop local variables
    jmp .end_37
.else_37:
.end_37:
    add rsp, 0		;end of block, pop local variables
    mov rax, 1
    push rax
    mov rax, [rbp-16]
    pop rcx
    add rax, rcx
    mov [rbp-16], rax
    jmp .for_start_15
.for_end_15:
    mov rax, [rbp-8]
    add rsp, 16		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 16		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.std.math.sumi:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
    mov rax, 0
    push rax
    push 0
    mov rax, 0
    mov [rbp-16], rax
    ;for statement
.for_start_16:
    mov rax, [rbp+24]
    push rax
    mov rax, -1
    pop rcx
    mov rax, [rcx + rax * 8]
    push rax
    mov rax, [rbp-16]
    pop rcx
    cmp rax, rcx
    setl al
    movzx rax, al
    cmp rax, 0
    je .for_end_16
    mov rax, [rbp+24]
    push rax
    mov rax, [rbp-16]
    pop rcx
    mov rax, [rcx + rax * 8]
    push rax
    mov rax, [rbp-8]
    pop rcx
    add rax, rcx
    mov [rbp-8], rax
    add rsp, 0		;end of block, pop local variables
    mov rax, 1
    push rax
    mov rax, [rbp-16]
    pop rcx
    add rax, rcx
    mov [rbp-16], rax
    jmp .for_start_16
.for_end_16:
    mov rax, [rbp-8]
    add rsp, 16		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 16		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.std.math.maxf:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
    ;if statement
    mov rax, 0
    push rax
    mov rax, [rbp+24]
    push rax
    mov rax, -1
    pop rcx
    mov rax, [rcx + rax * 8]
    pop rcx
    cmp rax, rcx
    sete al
    movzx rax, al
    cmp rax, 0
    je .else_38
    mov rax, 0x7974706d6520	;?empty
    push rax
    mov rax, 0x7369207961727241	;Array?is
    push rax
    mov rax, 2		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    push rax
	;push function arguments to the stack in reverse order
    mov rax, 1
    push rax
    mov rax, [rbp-32]
    push rax
    call .toplevel.std.math.exception
    add rsp, 48	;pop arguments
    add rsp, 0		;end of block, pop local variables
    jmp .end_38
.else_38:
.end_38:
    mov rax, [rbp+24]
    push rax
    mov rax, 0
    pop rcx
    movq xmm0, [rcx + rax * 8]
    movq rax, xmm0
    push rax
    push 0
    mov rax, 1
    mov [rbp-16], rax
    ;for statement
.for_start_17:
    mov rax, [rbp+24]
    push rax
    mov rax, -1
    pop rcx
    mov rax, [rcx + rax * 8]
    push rax
    mov rax, [rbp-16]
    pop rcx
    cmp rax, rcx
    setl al
    movzx rax, al
    cmp rax, 0
    je .for_end_17
    ;if statement
    movsd xmm0, [rbp-8]
    movq rax, xmm0
    push rax
    mov rax, [rbp+24]
    push rax
    mov rax, [rbp-16]
    pop rcx
    movq xmm0, [rcx + rax * 8]
	pop rcx
	movq xmm1, rcx
	ucomisd xmm0, xmm1
	seta al
	movzx rax, al
    cmp rax, 0
    je .else_39
    mov rax, [rbp+24]
    push rax
    mov rax, [rbp-16]
    pop rcx
    movq xmm0, [rcx + rax * 8]
    movq [rbp-8], xmm0
    add rsp, 0		;end of block, pop local variables
    jmp .end_39
.else_39:
.end_39:
    add rsp, 0		;end of block, pop local variables
    mov rax, 1
    push rax
    mov rax, [rbp-16]
    pop rcx
    add rax, rcx
    mov [rbp-16], rax
    jmp .for_start_17
.for_end_17:
    movsd xmm0, [rbp-8]
    add rsp, 16		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 16		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.std.math.minf:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
    ;if statement
    mov rax, 0
    push rax
    mov rax, [rbp+24]
    push rax
    mov rax, -1
    pop rcx
    mov rax, [rcx + rax * 8]
    pop rcx
    cmp rax, rcx
    sete al
    movzx rax, al
    cmp rax, 0
    je .else_40
    mov rax, 0x7974706d6520	;?empty
    push rax
    mov rax, 0x7369207961727241	;Array?is
    push rax
    mov rax, 2		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    push rax
	;push function arguments to the stack in reverse order
    mov rax, 1
    push rax
    mov rax, [rbp-32]
    push rax
    call .toplevel.std.math.exception
    add rsp, 48	;pop arguments
    add rsp, 0		;end of block, pop local variables
    jmp .end_40
.else_40:
.end_40:
    mov rax, [rbp+24]
    push rax
    mov rax, 0
    pop rcx
    movq xmm0, [rcx + rax * 8]
    movq rax, xmm0
    push rax
    push 0
    mov rax, 1
    mov [rbp-16], rax
    ;for statement
.for_start_18:
    mov rax, [rbp+24]
    push rax
    mov rax, -1
    pop rcx
    mov rax, [rcx + rax * 8]
    push rax
    mov rax, [rbp-16]
    pop rcx
    cmp rax, rcx
    setl al
    movzx rax, al
    cmp rax, 0
    je .for_end_18
    ;if statement
    movsd xmm0, [rbp-8]
    movq rax, xmm0
    push rax
    mov rax, [rbp+24]
    push rax
    mov rax, [rbp-16]
    pop rcx
    movq xmm0, [rcx + rax * 8]
	pop rcx
	movq xmm1, rcx
	ucomisd xmm0, xmm1
	setb al
	movzx rax, al
    cmp rax, 0
    je .else_41
    mov rax, [rbp+24]
    push rax
    mov rax, [rbp-16]
    pop rcx
    movq xmm0, [rcx + rax * 8]
    movq [rbp-8], xmm0
    add rsp, 0		;end of block, pop local variables
    jmp .end_41
.else_41:
.end_41:
    add rsp, 0		;end of block, pop local variables
    mov rax, 1
    push rax
    mov rax, [rbp-16]
    pop rcx
    add rax, rcx
    mov [rbp-16], rax
    jmp .for_start_18
.for_end_18:
    movsd xmm0, [rbp-8]
    add rsp, 16		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 16		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.std.math.sumf:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
    movsd xmm0, [.float.0]	; Load float into xmm0
    movq rax, xmm0
    push rax
    push 0
    mov rax, 0
    mov [rbp-16], rax
    ;for statement
.for_start_19:
    mov rax, [rbp+24]
    push rax
    mov rax, -1
    pop rcx
    mov rax, [rcx + rax * 8]
    push rax
    mov rax, [rbp-16]
    pop rcx
    cmp rax, rcx
    setl al
    movzx rax, al
    cmp rax, 0
    je .for_end_19
    mov rax, [rbp+24]
    push rax
    mov rax, [rbp-16]
    pop rcx
    movq xmm0, [rcx + rax * 8]
    movq rax, xmm0
    push rax
    movsd xmm0, [rbp-8]
	pop rcx
	movq xmm1, rcx
	addsd xmm0, xmm1
    movq [rbp-8], xmm0
    add rsp, 0		;end of block, pop local variables
    mov rax, 1
    push rax
    mov rax, [rbp-16]
    pop rcx
    add rax, rcx
    mov [rbp-16], rax
    jmp .for_start_19
.for_end_19:
    movsd xmm0, [rbp-8]
    add rsp, 16		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 16		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.std.random.printerr:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
    mov rax, 8
    push rax
    mov rax, [rbp+24]
    push rax
    mov rax, -1
    pop rcx
    mov rax, [rcx + rax * 8]
    pop rcx
    imul rax, rcx
	mov rdx, rax	; length of string
    mov rax, [rbp+24]
	mov rsi, rax	; pointer to string
	mov rax, 1		; syscall: sys_write
	mov rdi, 2		; file descriptor: stderr
	syscall
    add rsp, 0		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.std.random.exception:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
    mov rax, 0x203a6e	;n:?
    push rax
    mov rax, 0x6f69747065637845	;Exceptio
    push rax
    mov rax, 2		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    push rax
	;push function arguments to the stack in reverse order
    mov rax, [rbp-32]
    push rax
    call .toplevel.std.random.printerr
    add rsp, 40	;pop arguments
	;push function arguments to the stack in reverse order
    mov rax, [rbp+24]
    push rax
    call .toplevel.std.random.printerr
    add rsp, 8	;pop arguments
    mov rax, 0xa	;\n
    push rax
    mov rax, 1		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    push rax
	;push function arguments to the stack in reverse order
    mov rax, [rbp-24]
    push rax
    call .toplevel.std.random.printerr
    add rsp, 32	;pop arguments
    mov rax, [rbp+32]
	mov rdi, rax
	mov rax, 60
	syscall
	
    add rsp, 0		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.std.random.ftoint:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
    movsd xmm0, [rbp+24]
	cvtsd2si rax, xmm0  ; Convert double in xmm0 to 64-bit integer in rax
    add rsp, 0		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.std.random.inttof:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
    mov rax, [rbp+24]
	cvtsi2sd xmm0, rax	; Convert 64-bit integer in rax to double in xmm0
    add rsp, 0		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.std.random.floor:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
	stmxcsr [mxcsr_val]
	mov eax, [mxcsr_val]
	and eax, 0xFFFF9FFF	; Clear the round control bits
	or eax, 0x00002000	; 
	mov [mxcsr_val], eax
	ldmxcsr [mxcsr_val]
    movsd xmm0, [rbp+24]
	cvtsd2si rax, xmm0; round down (toward -inf)
    add rsp, 0		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.std.random.ceil:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
	stmxcsr [mxcsr_val]
	mov eax, [mxcsr_val]
	and eax, 0xFFFF9FFF	; 
	or eax, 0x00003000	; 
	mov [mxcsr_val], eax
	ldmxcsr [mxcsr_val]
    movsd xmm0, [rbp+24]
	cvtsd2si rax, xmm0; round up (toward +inf)
    add rsp, 0		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.std.random.isint:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
	;push function arguments to the stack in reverse order
    movsd xmm0, [rbp+24]
    movq rax, xmm0
    push rax
    call .toplevel.std.random.floor
    add rsp, 8	;pop arguments
    push rax
	;push function arguments to the stack in reverse order
    mov rax, [rbp-8]
    push rax
    call .toplevel.std.random.inttof
    add rsp, 8	;pop arguments
    movq rax, xmm0
    push rax
    movsd xmm0, [rbp+24]
    movq rax, xmm0
    push rax
    movsd xmm0, [rbp-16]
	pop rcx
	movq xmm1, rcx
	ucomisd xmm0, xmm1
	sete al
	movzx rax, al
    add rsp, 16		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 16		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.std.random.random:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
    mov rax, 1664525
    push rax
    mov rax, 1013904223
    push rax
    mov rax, 0xffffffff
    push rax
    mov rax, [rbp-24]
    push rax
    mov rax, [rbp-16]
    push rax
    mov rax, [rbp+24]
    push rax
    mov rax, [rbp-8]
    pop rcx
    imul rax, rcx
    pop rcx
    add rax, rcx
    pop rcx
    and rax, rcx
    mov [rbp+24], rax
    mov rax, [rbp+24]
    add rsp, 24		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 24		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.std.random.random_range:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
    ;if statement
    mov rax, [rbp+40]
    push rax
    mov rax, [rbp+32]
    pop rcx
    cmp rax, rcx
    setge al
    movzx rax, al
    cmp rax, 0
    je .else_42
    mov rax, 0x78616d206e616874	;than?max
    push rax
    mov rax, 0x207373656c206562	;be?less?
    push rax
    mov rax, 0x207473756d206e69	;in?must?
    push rax
    mov rax, 0x6d203a65676e6172	;range:?m
    push rax
    mov rax, 0x2064696c61766e49	;Invalid?
    push rax
    mov rax, 5		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    push rax
	;push function arguments to the stack in reverse order
    mov rax, 1
    push rax
    mov rax, [rbp-56]
    push rax
    call .toplevel.std.random.exception
    add rsp, 72	;pop arguments
    add rsp, 0		;end of block, pop local variables
    jmp .end_42
.else_42:
.end_42:
    mov rax, [rbp+32]
    push rax
    mov rax, [rbp+40]
    pop rcx
    sub rax, rcx
    push rax
    mov rax, [rbp-8]
    push rax
	;push function arguments to the stack in reverse order
    mov rax, [rbp+24]
    push rax
    call .toplevel.std.random.random
    add rsp, 8	;pop arguments
    pop rcx
    cqo
    idiv rcx
    mov rax, rdx
    push rax
    mov rax, [rbp-16]
    push rax
    mov rax, [rbp+32]
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

.toplevel.std.random.random_float:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
	;push function arguments to the stack in reverse order
    mov rax, [rbp+24]
    push rax
    call .toplevel.std.random.random
    add rsp, 8	;pop arguments
    push rax
	;push function arguments to the stack in reverse order
    mov rax, 0xffffffff
    push rax
    call .toplevel.std.random.inttof
    add rsp, 8	;pop arguments
    movq rax, xmm0
    push rax
	;push function arguments to the stack in reverse order
    mov rax, [rbp-8]
    push rax
    call .toplevel.std.random.inttof
    add rsp, 8	;pop arguments
	pop rcx
	movq xmm1, rcx
	divsd xmm0, xmm1
    add rsp, 8		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 8		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.std.mem.printerr:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
    mov rax, 8
    push rax
    mov rax, [rbp+24]
    push rax
    mov rax, -1
    pop rcx
    mov rax, [rcx + rax * 8]
    pop rcx
    imul rax, rcx
	mov rdx, rax	; length of string
    mov rax, [rbp+24]
	mov rsi, rax	; pointer to string
	mov rax, 1		; syscall: sys_write
	mov rdi, 2		; file descriptor: stderr
	syscall
    add rsp, 0		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.std.mem.exception:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
    mov rax, 0x203a6e	;n:?
    push rax
    mov rax, 0x6f69747065637845	;Exceptio
    push rax
    mov rax, 2		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    push rax
	;push function arguments to the stack in reverse order
    mov rax, [rbp-32]
    push rax
    call .toplevel.std.mem.printerr
    add rsp, 40	;pop arguments
	;push function arguments to the stack in reverse order
    mov rax, [rbp+24]
    push rax
    call .toplevel.std.mem.printerr
    add rsp, 8	;pop arguments
    mov rax, 0xa	;\n
    push rax
    mov rax, 1		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    push rax
	;push function arguments to the stack in reverse order
    mov rax, [rbp-24]
    push rax
    call .toplevel.std.mem.printerr
    add rsp, 32	;pop arguments
    mov rax, [rbp+32]
	mov rdi, rax
	mov rax, 60
	syscall
	
    add rsp, 0		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.std.mem.get_current_break:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
mov rax, 12	; syscall: sys_brk
    mov rdi, 0	; get current break
    syscall
    add rsp, 0		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.std.mem.sbrk:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
    ;if statement
    mov rax, 0
    push rax
    mov rax, [rbp+24]
    pop rcx
    cmp rax, rcx
    setle al
    movzx rax, al
    cmp rax, 0
    je .else_43
    mov rax, 0x3120747361	;ast?1
    push rax
    mov rax, 0x656c207461206562	;be?at?le
    push rax
    mov rax, 0x207473756d20657a	;ze?must?
    push rax
    mov rax, 0x6973203a6b726273	;sbrk:?si
    push rax
    mov rax, 4		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    push rax
	;push function arguments to the stack in reverse order
    mov rax, 1
    push rax
    mov rax, [rbp-48]
    push rax
    call .toplevel.std.mem.exception
    add rsp, 64	;pop arguments
    add rsp, 0		;end of block, pop local variables
    jmp .end_43
.else_43:
.end_43:
    mov rax, [rbp+24]
	; Get current break point first
    mov rax, 12	; syscall: sys_brk  
    mov rdi, 0	; get current break
    syscall
    mov rbx, rax	; save old break in rbx
    mov rax, [rbp+24]
    ; Now allocate new memory
    mov rdi, rbx    ; rbx contains the old break point
    add rdi, rax	; add size to the old break point
    add rdi, 24	; add 24 bytes for metadata
    mov rax, 12	; syscall: sys_brk
    syscall

    ; rbx now contains the start of our allocated block, and rax contains the new break point
    mov [rax - 16], rbx	; previous break point is stored at rax - 16
    mov dword [rax - 8], 1	; mark the block as not free (1 = not free, 0 = free)
    mov rax, 8
    push rax
    mov rax, [rbp+24]
    pop rcx
    cqo
    idiv rcx
	mov [rbx], rax	; store array length at start of block
    mov rax, rbx	; return the pointer
    add rax, 8	; move past the metadata
    add rsp, 0		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.std.mem.malloc_call_count:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
mov rax, [malloc_used]
    add rsp, 0		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.std.mem.malloc:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
	;push function arguments to the stack in reverse order
    call .toplevel.std.mem.malloc_call_count
    add rsp, 0	;pop arguments
    push rax
   add rax, 1
    mov [malloc_used], rax
    push 0
    ;if statement
    mov rax, 0
    push rax
    mov rax, [rbp-8]
    pop rcx
    cmp rax, rcx
    setg al
    movzx rax, al
    cmp rax, 0
    je .else_44
    mov rax, 8
    push rax
    mov rax, [rbp+24]
    pop rcx
    imul rax, rcx
    push rax
	;push function arguments to the stack in reverse order
    mov rax, [rbp-24]
    push rax
    call .toplevel.std.mem.sbrk
    add rsp, 16	;pop arguments
    mov [rbp-16], rax
    mov rax, 0
    push rax
    mov rax, [rbp-16]
    push rax
    mov rax, [rbp-16]
    push rax
    mov rax, -1
    pop rcx
    mov rax, [rcx + rax * 8]
    mov r8, rax
    imul r8, 8
    pop rax
    add r8, rax
    pop rax
    mov [r8], rax
    add rsp, 0		;end of block, pop local variables
    jmp .end_44
.else_44:
	;push function arguments to the stack in reverse order
    call .toplevel.std.mem.get_current_break
    add rsp, 0	;pop arguments
    push rax
    mov rax, 0
    push rax
    ;while statement
.while_start_20:
    mov rax, [rbp-32]
    not rax
    push rax
    mov rax, 0
    push rax
    mov rax, [rbp-24]
    pop rcx
    cmp rax, rcx
    setne al
    movzx rax, al
    pop rcx
    and rax, rcx
    cmp rax, 0
    je .while_end_20
    ;if statement
    mov rax, [rbp+24]
    push rax
    mov rax, [rbp-24]
    push rax
    mov rax, 2
    neg rax
    pop rcx
    mov rax, [rcx + rax * 8]
    push rax
    mov rax, 0
    pop rcx
    mov rax, [rcx + rax * 8]
    pop rcx
    cmp rax, rcx
    setge al
    movzx rax, al
    push rax
    mov rax, 0
    push rax
    mov rax, [rbp-24]
    push rax
    mov rax, 1
    neg rax
    pop rcx
    mov rax, [rcx + rax * 8]
    pop rcx
    cmp rax, rcx
    sete al
    movzx rax, al
    pop rcx
    and rax, rcx
    cmp rax, 0
    je .else_45
    mov rax, 0xFFFFFFFFFFFFFFFF
    mov [rbp-32], rax
    mov rax, 0
    add rsp, 0		;end of block, pop local variables
    jmp .end_45
.else_45:
    mov rax, [rbp-24]
    push rax
    mov rax, 2
    neg rax
    pop rcx
    mov rax, [rcx + rax * 8]
    mov [rbp-24], rax
    add rsp, 0		;end of block, pop local variables
.end_45:
    add rsp, 0		;end of block, pop local variables
    jmp .while_start_20
.while_end_20:
    ;if statement
    mov rax, [rbp-32]
    not rax
    cmp rax, 0
    je .else_46
    mov rax, 8
    push rax
    mov rax, [rbp+24]
    pop rcx
    imul rax, rcx
    push rax
	;push function arguments to the stack in reverse order
    mov rax, [rbp-40]
    push rax
    call .toplevel.std.mem.sbrk
    add rsp, 16	;pop arguments
    mov [rbp-16], rax
    mov rax, 0
    add rsp, 0		;end of block, pop local variables
    jmp .end_46
.else_46:
    mov rax, [rbp-24]
    mov [rbp-16], rax
    mov rax, 1
    push rax
    mov rax, [rbp-16]
    push rax
    mov rax, 1
    push rax
    mov rax, [rbp-16]
    push rax
    mov rax, -1
    pop rcx
    mov rax, [rcx + rax * 8]
    pop rcx
    add rax, rcx
    mov r8, rax
    imul r8, 8
    pop rax
    add r8, rax
    pop rax
    mov [r8], rax
    add rsp, 0		;end of block, pop local variables
.end_46:
    add rsp, 16		;end of block, pop local variables
.end_44:
    mov rax, [rbp-16]
    add rsp, 16		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 16		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.std.mem.free:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
    ;if statement
    mov rax, 0
    push rax
    mov rax, [rbp+24]
    push rax
    mov rax, -1
    pop rcx
    mov rax, [rcx + rax * 8]
    pop rcx
    cmp rax, rcx
    setle al
    movzx rax, al
    cmp rax, 0
    je .else_47
    mov rax, 0x64696c	;lid
    push rax
    mov rax, 0x6176206562207473	;st?be?va
    push rax
    mov rax, 0x756d207265746e69	;inter?mu
    push rax
    mov rax, 0x6f70203a65657266	;free:?po
    push rax
    mov rax, 4		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    push rax
	;push function arguments to the stack in reverse order
    mov rax, 1
    push rax
    mov rax, [rbp-48]
    push rax
    call .toplevel.std.mem.exception
    add rsp, 64	;pop arguments
    add rsp, 0		;end of block, pop local variables
    jmp .end_47
.else_47:
.end_47:
    mov rax, [rbp+24]
    push rax
    mov rax, -1
    pop rcx
    mov rax, [rcx + rax * 8]
    push rax
    mov rax, 0
    push rax
    mov rax, [rbp+24]
    push rax
    mov rax, 1
    push rax
    mov rax, [rbp+24]
    push rax
    mov rax, -1
    pop rcx
    mov rax, [rcx + rax * 8]
    pop rcx
    add rax, rcx
    mov r8, rax
    imul r8, 8
    pop rax
    add r8, rax
    pop rax
    mov [r8], rax
    add rsp, 8		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.std.mem.arrset..int:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
    mov rax, 0
    push rax
    ;for statement
.for_start_21:
    mov rax, [rbp+24]
    push rax
    mov rax, -1
    pop rcx
    mov rax, [rcx + rax * 8]
    push rax
    mov rax, [rbp-8]
    pop rcx
    cmp rax, rcx
    setl al
    movzx rax, al
    cmp rax, 0
    je .for_end_21
    mov rax, [rbp+32]
    push rax
    mov rax, [rbp+24]
    push rax
    mov rax, [rbp-8]
    mov r8, rax
    imul r8, 8
    pop rax
    add r8, rax
    pop rax
    mov [r8], rax
    add rsp, 0		;end of block, pop local variables
    mov rax, 1
    add [rbp-8], rax
    jmp .for_start_21
.for_end_21:
    add rsp, 8		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.std.mem.calloc:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
	;push function arguments to the stack in reverse order
    mov rax, [rbp+24]
    push rax
    call .toplevel.std.mem.malloc
    add rsp, 8	;pop arguments
    push rax
	;push function arguments to the stack in reverse order
    mov rax, 0
    push rax
    mov rax, [rbp-8]
    push rax
    call .toplevel.std.mem.arrset..int
    add rsp, 16	;pop arguments
    mov rax, [rbp-8]
    add rsp, 8		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 8		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.std.mem.arrset..float:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
    mov rax, 0
    push rax
    ;for statement
.for_start_22:
    mov rax, [rbp+24]
    push rax
    mov rax, -1
    pop rcx
    mov rax, [rcx + rax * 8]
    push rax
    mov rax, [rbp-8]
    pop rcx
    cmp rax, rcx
    setl al
    movzx rax, al
    cmp rax, 0
    je .for_end_22
    movsd xmm0, [rbp+32]
    movq rax, xmm0
    push rax
    mov rax, [rbp+24]
    push rax
    mov rax, [rbp-8]
    mov r8, rax
    imul r8, 8
    pop rax
    add r8, rax
    pop rax
    movq xmm0, rax
    movq [r8], xmm0
    add rsp, 0		;end of block, pop local variables
    mov rax, 1
    add [rbp-8], rax
    jmp .for_start_22
.for_end_22:
    add rsp, 8		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.std.mem.falloc:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
	;push function arguments to the stack in reverse order
    mov rax, [rbp+24]
    push rax
    call .toplevel.std.mem.malloc
    add rsp, 8	;pop arguments
    push rax
	;push function arguments to the stack in reverse order
    movsd xmm0, [.float.0]	; Load float into xmm0
    movq rax, xmm0
    push rax
    mov rax, [rbp-8]
    push rax
    call .toplevel.std.mem.arrset..float
    add rsp, 16	;pop arguments
    mov rax, [rbp-8]
    add rsp, 8		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 8		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.std.mem.realloc:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
    ;if statement
    mov rax, 0
    push rax
    mov rax, [rbp+32]
    pop rcx
    cmp rax, rcx
    setle al
    movzx rax, al
    cmp rax, 0
    je .else_48
    mov rax, 0x31207473	;st?1
    push rax
    mov rax, 0x61656c2074612065	;e?at?lea
    push rax
    mov rax, 0x62207473756d2065	;e?must?b
    push rax
    mov rax, 0x7a69732077656e20	;?new?siz
    push rax
    mov rax, 0x3a636f6c6c616572	;realloc:
    push rax
    mov rax, 5		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    push rax
	;push function arguments to the stack in reverse order
    mov rax, 1
    push rax
    mov rax, [rbp-56]
    push rax
    call .toplevel.std.mem.exception
    add rsp, 72	;pop arguments
    add rsp, 0		;end of block, pop local variables
    jmp .end_48
.else_48:
.end_48:
    mov rax, [rbp+24]
    push rax
    mov rax, -1
    pop rcx
    mov rax, [rcx + rax * 8]
    push rax
	;push function arguments to the stack in reverse order
    mov rax, [rbp+32]
    push rax
    call .toplevel.std.mem.malloc
    add rsp, 8	;pop arguments
    push rax
    mov rax, 0
    push rax
    ;for statement
.for_start_23:
    mov rax, [rbp+32]
    push rax
    mov rax, [rbp-24]
    pop rcx
    cmp rax, rcx
    setl al
    movzx rax, al
    push rax
    mov rax, [rbp-8]
    push rax
    mov rax, [rbp-24]
    pop rcx
    cmp rax, rcx
    setl al
    movzx rax, al
    pop rcx
    and rax, rcx
    cmp rax, 0
    je .for_end_23
    mov rax, [rbp+24]
    push rax
    mov rax, [rbp-24]
    pop rcx
    mov rax, [rcx + rax * 8]
    push rax
    mov rax, [rbp-16]
    push rax
    mov rax, [rbp-24]
    mov r8, rax
    imul r8, 8
    pop rax
    add r8, rax
    pop rax
    mov [r8], rax
    add rsp, 0		;end of block, pop local variables
    mov rax, 1
    add [rbp-24], rax
    jmp .for_start_23
.for_end_23:
	;push function arguments to the stack in reverse order
    mov rax, [rbp+24]
    push rax
    call .toplevel.std.mem.free
    add rsp, 8	;pop arguments
    mov rax, [rbp-16]
    add rsp, 24		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 24		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.std.mem.memset:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
    ;if statement
    mov rax, 0
    push rax
    mov rax, [rbp+40]
    pop rcx
    cmp rax, rcx
    setl al
    movzx rax, al
    cmp rax, 0
    je .else_49
    mov rax, 0x7265626d756e	;number
    push rax
    mov rax, 0x206576697469736f	;ositive?
    push rax
    mov rax, 0x7020612065622074	;t?be?a?p
    push rax
    mov rax, 0x73756d20657a6973	;size?mus
    push rax
    mov rax, 0x203a7465736d656d	;memset:?
    push rax
    mov rax, 5		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    push rax
	;push function arguments to the stack in reverse order
    mov rax, 1
    push rax
    mov rax, [rbp-56]
    push rax
    call .toplevel.std.mem.exception
    add rsp, 72	;pop arguments
    add rsp, 0		;end of block, pop local variables
    jmp .end_49
.else_49:
.end_49:
    mov rax, 0
    push rax
    ;for statement
.for_start_24:
    mov rax, [rbp+40]
    push rax
    mov rax, [rbp-8]
    pop rcx
    cmp rax, rcx
    setl al
    movzx rax, al
    cmp rax, 0
    je .for_end_24
    mov rax, [rbp+32]
    push rax
    mov rax, [rbp+24]
    push rax
    mov rax, [rbp-8]
    mov r8, rax
    imul r8, 8
    pop rax
    add r8, rax
    pop rax
    mov [r8], rax
    add rsp, 0		;end of block, pop local variables
    mov rax, 1
    add [rbp-8], rax
    jmp .for_start_24
.for_end_24:
    add rsp, 8		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

section .data
    .constant.toplevel.std.math.PI: dq 3.141592653589793
    .constant.toplevel.std.math.E: dq 2.718281828459045
    .constant.toplevel.std.math.SQRT2: dq 1.4142135623730951
    .constant.toplevel.std.math.LN2: dq 0.6931471805599453
    .constant.toplevel.std.math.LOG2E: dq 1.4426950408889634
    .constant.toplevel.std.math.LOG10E: dq 0.4342944819032518
    .constant.toplevel.std.math.LOG2_10: dq 3.321928094887362
    .constant.toplevel.std.math.LOG10_2: dq 0.3010299956639812
    .constant.toplevel.std.math.PHI: dq 1.618033988749895
    .constant.toplevel.std.math.GOLDEN_RATIO: dq 1.618033988749895
    .constant.toplevel.std.math.SQRT3: dq 1.7320508075688772
	.float.2: dq 1e-15
	.float.1: dq 1000000.0
	.float.4: dq 1.0
	.float.5: dq 709.0
	.float.6: dq 1e-13
	.float.0: dq 0.0
	.float.3: dq 2.0
	mxcsr_val dd 0
	malloc_used dd 0
