%define KERNEL_STACK_SIZE 0x1000

global outp
global inp
global print
global move_cursor

section .bss
	resb KERNEL_STACK_SIZE	; allocate 
			; to bss (uninit memory)
		; size of kernel stack instead 
	; just move sp to some location in memory
section .text
outp:
	mov dx, si	; only rdx must to contain
			; address of port

	mov ax, di	; move data to send it
	out dx, ax	; send data to port di
	ret
inp:
	mov dx, si 
	in al, dx
	ret
print_c:			;write text using framebuffer
	mov eax, 0x000B8000 ;address of framebuffer
	mov cx, [di]
	mov [eax], cl 
	mov cx, si
	and byte cl, 0x0F
	or byte [eax + 1], cl 
	shl byte [eax + 1], 4
	mov cx, dx
	and byte cl, 0x0F
	or  byte [eax + 1], cl 
	
	ret
print_s:
	cmp byte cl, 0
	je E
	sub esp, 1
	mov byte [esp], cl ; length
	
P:	
	call print_c	
	sub byte [esp], 0x01
	cmp byte [esp], 0
	jne P

	add esp, 1
E:	ret

%define FB_COMMAND_PORT	0X3D4
%define FB_DATA_PORT	0X3D5

%define FB_HIGH_BYTE_COMMAND 14
%define FB_LOW_BYTE_COMMAND  15

move_cursor:
	add si, di
	mov ax, si ; ax containing position
	shr word ax, 8
	and word ax, 0x00FF
	
	mov di, FB_COMMAND_PORT ; enable 2 times sended command
	mov si, FB_HIGH_BYTE_COMMAND
	call outp

	mov di, FB_DATA_PORT
	mov si, 0x00 ; send high bytes
	call outp

	mov di, FB_COMMAND_PORT
	mov si, FB_LOW_BYTE_COMMAND 
	call outp

	mov di, FB_DATA_PORT
	mov si, ax ; send low bytes
	call outp

	ret

	
