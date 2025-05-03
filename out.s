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
	pop rcx
	movq xmm1, rcx
	divsd xmm0, xmm1
    movq rax, xmm0
    push rax
    mov rax, 31
    push rax
	;push function arguments to the stack in reverse order
    mov rax, [rbp-16]
    push rax
    call .toplevel.std.io.printi
    add rsp, 8	;pop arguments
    add rsp, 16		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.std.exception:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
	;push function arguments to the stack in reverse order
    mov rax, 0x203a6e	;n:?
    push rax
    mov rax, 0x6f69747065637845	;Exceptio
    push rax
    mov rax, 2		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    push rax
    call .toplevel.std.io.print
    add rsp, 32	;pop arguments
	;push function arguments to the stack in reverse order
    mov rax, [rbp+24]
    push rax
    call .toplevel.std.io.print
    add rsp, 8	;pop arguments
	;push function arguments to the stack in reverse order
    mov rax, 0xa	;\n
    push rax
    mov rax, 1		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    push rax
    call .toplevel.std.io.print
    add rsp, 24	;pop arguments
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
    je .else_0
	;push function arguments to the stack in reverse order
    mov rax, 1
    push rax
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
    call .toplevel.std.exception
    add rsp, 48	;pop arguments
    add rsp, 0		;end of block, pop local variables
    jmp .end_0
.else_0:
    ;if statement
    mov rax, 0
    push rax
    mov rax, [rbp+32]
    pop rcx
    cmp rax, rcx
    setl al
    movzx rax, al
    cmp rax, 0
    je .else_1
	;push function arguments to the stack in reverse order
    mov rax, 1
    push rax
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
    call .toplevel.std.exception
    add rsp, 48	;pop arguments
    add rsp, 0		;end of block, pop local variables
    jmp .end_1
.else_1:
.end_1:
    add rsp, 0		;end of block, pop local variables
.end_0:
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
    je .else_2
	;push function arguments to the stack in reverse order
    mov rax, 1
    push rax
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
    call .toplevel.std.exception
    add rsp, 48	;pop arguments
    add rsp, 0		;end of block, pop local variables
    jmp .end_2
.else_2:
    ;if statement
    mov rax, 0
    push rax
    mov rax, [rbp+32]
    pop rcx
    cmp rax, rcx
    setl al
    movzx rax, al
    cmp rax, 0
    je .else_3
	;push function arguments to the stack in reverse order
    mov rax, 1
    push rax
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
    call .toplevel.std.exception
    add rsp, 48	;pop arguments
    add rsp, 0		;end of block, pop local variables
    jmp .end_3
.else_3:
.end_3:
    add rsp, 0		;end of block, pop local variables
.end_2:
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
    je .else_4
    mov rax, 0
    add rsp, 0		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 0		;end of block, pop local variables
    jmp .end_4
.else_4:
.end_4:
    push 0
    ;for statement
    mov rax, 7
    mov [rbp-16], rax
.for_start_0:
    mov rax, 0
    push rax
    mov rax, [rbp-16]
    pop rcx
    cmp rax, rcx
    setge al
    movzx rax, al
    cmp rax, 0
    je .for_end_0
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
    je .else_5
    jmp .for_end_0	;break statement
    add rsp, 0		;end of block, pop local variables
    jmp .end_5
.else_5:
.end_5:
    add rsp, 0		;end of block, pop local variables
    mov rax, 1
    push rax
    mov rax, [rbp-16]
    pop rcx
    sub rax, rcx
    mov [rbp-16], rax
    jmp .for_start_0
.for_end_0:
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
    je .else_6
	;push function arguments to the stack in reverse order
    mov rax, 1
    push rax
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
    call .toplevel.std.exception
    add rsp, 56	;pop arguments
    add rsp, 0		;end of block, pop local variables
    jmp .end_6
.else_6:
.end_6:
    push 0
    ;for statement
    mov rax, 0
    mov [rbp-16], rax
.for_start_1:
    mov rax, [rbp-8]
    push rax
    mov rax, [rbp-16]
    pop rcx
    cmp rax, rcx
    setl al
    movzx rax, al
    cmp rax, 0
    je .for_end_1
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
    jmp .for_start_1
.for_end_1:
    mov rax, [rbp+24]
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
    mov rax, 1
    neg rax
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
	;push function arguments to the stack in reverse order
    mov rax, 0xa	;\n
    push rax
    mov rax, 1		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    push rax
    call .toplevel.std.io.print
    add rsp, 24	;pop arguments
    add rsp, 0		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

.toplevel.std.io.printchar:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
	;push function arguments to the stack in reverse order
    mov rax, [rbp+24]
    push rax
    mov rax, 1		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    push rax
    call .toplevel.std.io.print
    add rsp, 24	;pop arguments
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
    je .else_7
	;push function arguments to the stack in reverse order
    mov rax, 0x30	;0
    push rax
    mov rax, 1		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    push rax
    call .toplevel.std.io.print
    add rsp, 24	;pop arguments
    mov rax, 0
    add rsp, 0		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 0		;end of block, pop local variables
    jmp .end_7
.else_7:
.end_7:
    ;if statement
    mov rax, 0
    push rax
    mov rax, [rbp+24]
    pop rcx
    cmp rax, rcx
    setl al
    movzx rax, al
    cmp rax, 0
    je .else_8
	;push function arguments to the stack in reverse order
    mov rax, 0x2d	;-
    push rax
    mov rax, 1		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    push rax
    call .toplevel.std.io.print
    add rsp, 24	;pop arguments
    mov rax, [rbp+24]
    neg rax
    mov [rbp+24], rax
    add rsp, 0		;end of block, pop local variables
    jmp .end_8
.else_8:
.end_8:
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
.while_start_2:
    mov rax, 0
    push rax
    mov rax, [rbp+24]
    pop rcx
    cmp rax, rcx
    setg al
    movzx rax, al
    cmp rax, 0
    je .while_end_2
    mov rax, 8
    push rax
    mov rax, 8
    push rax
    mov rax, [rbp-88]
    pop rcx
    cqo
    idiv rcx
    mov rax, rdx
    pop rcx
    imul rax, rcx
    push rax
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
    pop rcx
    shl rax, cl
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
    and rax, rcx
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
    jmp .while_start_2
.while_end_2:
	;push function arguments to the stack in reverse order
    mov rax, [rbp-80]
    push rax
    call .toplevel.std.io.print
    add rsp, 8	;pop arguments
    add rsp, 88		;end of block, pop local variables
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

section .data
	.float.1: dq 3.14159265
	.float.0: dq 4.0
