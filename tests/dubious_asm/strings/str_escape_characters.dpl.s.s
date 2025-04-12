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
    mov rax, 0x223a292922	;":))"
    push rax
    mov rax, 0x68690a7468657265	;hi\nthere
    push rax
    mov rax, rsp	; Move the address of the array to rax
    push rax
    mov rax, [rbp-24]
    push rax
    mov rax, 1
    pop rcx
    mov rax, [rcx + rax * 8]
    add rsp, 24		;pop local variables before return
    pop rbx		;restore rbx for caller function
    pop rbp		;restore base pointer
    ret
    add rsp, 8		;end of block, pop local variables
    pop rbx			;restore rbx for caller function
    pop rbp			;restore base pointer
    ret				;return by default if no return statement was reached

section .data
