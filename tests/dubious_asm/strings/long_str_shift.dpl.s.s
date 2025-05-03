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
    mov rax, 0x30393837	;7890
    push rax
    mov rax, 0x3635343332317a79	;yz123456
    push rax
    mov rax, 0x7877767574737271	;qrstuvwx
    push rax
    mov rax, 0x706f6e6d6c6b6a69	;ijklmnop
    push rax
    mov rax, 0x6867666564636261	;abcdefgh
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
    add rsp, 56		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

section .data
