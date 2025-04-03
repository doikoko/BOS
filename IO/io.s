%define KERNEL_STACK_SIZE 0x1000

global outp
global print
global cursor

section .bss
	resb KERNEL_STACK_SIZE	; allocate 
			; to bss (uninit memory)
		; size of kernel stack instead 
	; just move sp to some location in memory
section .text
outp:
	mov ax, si	; move data to send it
	mov dx, di	; out can work only with rdx
	out dx, ax	; send data to port di
	ret
print:			;write text using framebuffer
	mov eax, 0x000B8000
L0:	mov [eax], edi
	mov [eax + 1], esi 
	shl byte [eax + 1], 4
	or  [eax + 1], edx
	add edi, 1
	cmp byte [edi], 0
	jne L0
	ret
move_cursor:
	add si, di
	mov ax, si
	mov di, 0x3D4
	mov si, 14
	call outp
	mov di, 0x3D5
	mov si, 0x00
	call outp
	mov di, 0x3D4
	mov si, 15
	call outp
	mov di, 0x3D5
	mov si, ax
	call outp

	ret

