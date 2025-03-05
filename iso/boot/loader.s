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
	LGDT gdt_param 

	hlt 

gdt_param:
	dq 0x00000000 ; Base segment (8 zeroes)
	dd 0x0000 ; Flags (4 zeroes)
	dd 0x0000 ; Limit (4 zeroes)
	db 0b10011010 ; Access byte

msg: db "hello world", 0

times 510 - ($ - $$) db 0

dw 0xAA55
