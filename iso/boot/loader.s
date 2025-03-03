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
	
	hlt 

msg: db "hello world", 0

times 510 - ($ - $$) db 0

dw 0xAA55
