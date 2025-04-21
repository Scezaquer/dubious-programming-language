[BITS 64]
section .text

global _start
_start:
    call .toplevel.main
    mov rdi, rax
    mov rax, 60
    syscall

.toplevel.main:
    push rbp		;save previous base pointer
    push rbx		;functions should preserve rbx
    mov rbp, rsp	;set base pointer
    mov rax, 0x37383930	;7890
    push rax
    mov rax, 0x797a313233343536	;yz123456
    push rax
    mov rax, 0x7172737475767778	;qrstuvwx
    push rax
    mov rax, 0x696a6b6c6d6e6f70	;ijklmnop
    push rax
    mov rax, 0x6162636465666768	;abcdefgh
    push rax
    mov rax, 5		; length of the array
    push rax
    mov rax, rsp	; Move the address of the array to rax
    add rax, 8		; we also pushed the array's length so we need to add 8 to point to the right address
    push rax
    mov rax, 8
    push rax
    mov rax, 4
    pop rcx
    imul rax, rcx
    push rax
    mov rax, [rbp-56]
    push rax
    mov rax, 3
    pop rcx
    mov rax, [rcx + rax * 8]
    pop rcx
    shr rax, cl
    add rsp, 56		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 8		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

section .data
