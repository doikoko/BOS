; it's a free small bootloader
; goal of this - economy pc recourses
; by replacing huge GRUB on this
; and increase my own knowledges in this sphere
; if you want you can copy this to your own project

[ORG 0x7C00] 
[BITS 16]

CODE_OFFSET equ 0x8
DATA_OFFSET equ 0x10

KERNEL_POS equ 0x1000	; 4096 bits dec, 512 bytes
			; loader - 512 size bytes,
			; then kernel will placed to 512
_start:
	mov si, msg	; print message
	call PRINT

	mov sp, 0x7C00
	xor ax, ax	
	mov ss, ax
	mov es, ax
	mov ds, ax
	
	lgdt [gdt_start]

read_kernel:
	mov bx, KERNEL_POS 
	mov dl, 0x80	; always to int 0x13
	mov cl, 0x02	; second sector(each 512 bytes)
	mov ch, 0x00	; first cylinder
	mov ah, 0x02	; read
	mov al, 8	; 8 sectors to read
	int 0x13

prot_mode_switch:
	[BITS 32]
	xor eax, eax
	or eax, 0x01
	mov cr0, eax
	jmp CODE_OFFSET:prot_mode_main
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
	dw gdt_end - gdt_descriptor - 1 ; size - 1
	dd gdt_descriptor		

prot_mode_main:
	mov sp, 0x9C00	; initialize stack for prot mode
	mov bp, sp
	mov ax, 0x10	; initialize segment registers
	mov ds, ax	; for prot mode
	mov ss, ax
	mov fs, ax
	mov gs, ax
	mov es, ax

	in al, 0x92	; enabling a20 line
	or al, 0x2
	out 0x92, al	
PRINT:
	mov ah, 0x0E
	mov al, [si]
	inc si
	int 0x10
	cmp al, 0
	jne PRINT
	ret

msg: dw "loading os:", 0
times 510 - ($ - $$) db 0

dw 0xAA55
