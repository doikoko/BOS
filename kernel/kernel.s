%define KERNEL_STACK_SIZE 0x1000

global outp
global print
section .bss
	resb KERNEL_STACK_SIZE	; allocate 
			; to bss (uninit memory)
		; size of kernel stack instead 
	; just move sp to some location in memory
section .text
outp:
	mov ax, si	; move data to send it
	out di, ax	; send data to port di
	ret
print:
	mov rax, 0x000B8000
L0:	mov [rax], rdi
	mov [rax + 1], fg
	shl [rax + 1], 4
	or  [rax + 1], bg
	add rdi, 1
	cmp [rdi], 0
	jne L0
	ret
