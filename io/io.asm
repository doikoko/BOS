extern _outp

global _outp
global _inp
global _print_c
global _print_s
global _move_cursor

section .text
_print_c:			;write text using framebuffer
	;0x000B8000 address of framebuffer
	mov ax, di
	mov edi, 0x000B8000 
	mov byte [edi], al
	mov ax, si
	mov byte [edi + 1], al
	shl byte [edi + 1], 4
	and byte dl, 0x0F
	or byte [eax + 1], dl

	ret 
_print_s:
	cmp byte cl, 0
	je E
	
	mov eax, 0x000B8000
	sub esp, 1
	mov byte [esp], cl ; length
P:	
	cmp byte [edi], 0
	je E
	mov byte cl, [edi]
	mov byte [eax], cl
	mov cx, si
	mov byte [eax + 1], cl
	shl byte [eax + 1], 4
	and byte dl, 0x0F
	or byte [eax + 1], dl

	inc edi
	sub byte [esp], 0x01
	cmp byte [esp], 0
	jne P

	add esp, 1
E:	ret

%define FB_COMMAND_PORT	0X3D4
%define FB_DATA_PORT	0X3D5

%define FB_HIGH_BYTE_COMMAND 14
%define FB_LOW_BYTE_COMMAND  15

_move_cursor:
	add si, di
	mov ax, si ; ax containing position
	shr word ax, 8
	and word ax, 0x00FF
	
	mov di, FB_COMMAND_PORT ; enable 2 times sended command
	mov si, FB_HIGH_BYTE_COMMAND
	call _outp

	mov di, FB_DATA_PORT
	mov si, 0x00 ; send high bytes
	call _outp

	mov di, FB_COMMAND_PORT
	mov si, FB_LOW_BYTE_COMMAND 
	call _outp

	mov di, FB_DATA_PORT
	mov si, ax ; send low bytes
	call _outp

	ret