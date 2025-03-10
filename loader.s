; it's a free small bootloader
; goal of this - economy pc recourses
; by replacing huge GRUB on this
; and increase my own knowledges in this sphere
; if you want you can copy this to your own project

[ORG 0x7C00] 
[BITS 16]

_start:
	xor ax, ax
	mov ss, ax
	mov es, ax
	mov ds, ax

	mov sp, 0x7C00

	mov si, msg
	mov ah, 0x0E
PRINT:
	mov al, [si]
	inc si
	int 0x10
	cmp al, 0
	jne PRINT
GDT:
	lgdt [gdt_start]
	
prot_mode:
	xor cr0, cr0
	mov 0x01, cr0

init:
	mov sp, 0x9C00
	mov ax, 0x10
	mov ds, ax
	mov ss, ax
	mov fs, ax
	mov gs, ax
	mov ex, ax

	hlt 

gdt_start:
	; NULL descriptor
	dd 0x00000000
	dd 0x00000000

	;KERNEL MODE:
	
	;code segment (0x8)
	dw 0xFFFF	; Limit 
	dw 0x0000	; Base
	db 0x00 	; Base
	db 0b10011010	; Access byte
	db 0b11001111	; Flags
	db 0x00		; Base

	;data segment (0x10)
	dw 0xFFFF	; Limit
	dw 0x0000	; Base
	db 0x00 	; Base
	db 0b10011010	; Access byte
	db 0b11001111	; Flags
	db 0x00		; Base

gdt_end:
gdt_descriptor:
	dw gdt_end - gdt_start - 1 ; size - 1
	dd gdt_start
msg: db "hello world", 0

times 510 - ($ - $$) db 0

dw 0xAA55
