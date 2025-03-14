; it's a free small bootloader
; goal of this - economy pc recourses
; by replacing huge GRUB on this
; and increase my own knowledges in this sphere
; if you want you can copy this to your own project

[ORG 0x7C00] 
[BITS 16]

CODE_OFFSET equ 0x8
DATA_OFFSET equ 0x10
_start:
	xor ax, ax
	mov ss, ax	; initialize for real mode
	mov es, ax
	mov ds, ax

	mov sp, 0x7C00
GDT:
	lgdt [gdt_start]
	
prot_mode_switch:
	mov eax, cr0
	xor eax, eax
	or eax, 0x01
	mov cr0, eax
	jmp DATA_OFFSET:prot_mode_main

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

prot_mode_main:
	[BITS 32]
	
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

	jmp $

times 510 - ($ - $$) db 0	; repeat for fill free memory without 2 last bytes

dw 0xAA55			; 2 last bytes for BIOS correct initialize
