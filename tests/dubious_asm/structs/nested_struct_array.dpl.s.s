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
    mov rax, 18
    push rax
    mov rax, 17
    push rax
    mov rax, 16
    push rax
    mov rax, 3		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    mov rax, 15
    push rax
    mov rax, 14
    push rax
    mov rax, 2		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    mov rax, 13
    push rax
    mov rax, 1		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    mov rax, rsp
    add rax, 80
    sub rax, 32
    push rax
    sub rax, 24
    push rax
    sub rax, 16
    push rax
    mov rax, 3		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    mov rax, 12
    push rax
    mov rax, 11
    push rax
    mov rax, 10
    push rax
    mov rax, 3		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    mov rax, 9
    push rax
    mov rax, 8
    push rax
    mov rax, 2		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    mov rax, 7
    push rax
    mov rax, 1		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    mov rax, rsp
    add rax, 80
    sub rax, 32
    push rax
    sub rax, 24
    push rax
    sub rax, 16
    push rax
    mov rax, 3		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    mov rax, 6
    push rax
    mov rax, 5
    push rax
    mov rax, 4
    push rax
    mov rax, 3		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    mov rax, 3
    push rax
    mov rax, 2
    push rax
    mov rax, 2		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    mov rax, 1
    push rax
    mov rax, 1		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    mov rax, rsp
    add rax, 80
    sub rax, 32
    push rax
    sub rax, 24
    push rax
    sub rax, 16
    push rax
    mov rax, 3		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    mov rax, rsp
    add rax, 320
    sub rax, 104
    push rax
    sub rax, 104
    push rax
    sub rax, 104
    push rax
    mov rax, 3		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    push rax
    mov rax, [rbp-352]
    mov rcx, 1
    mov rax, [rax + rcx * 8]
    mov rcx, 1
    mov rax, [rax + rcx * 8]
    push rax
    mov rax, 1
    pop rcx
    mov rax, [rcx + rax * 8]
    add rsp, 352		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 352		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

section .data
