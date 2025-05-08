; it's a free small bootloader
; goal of this - economy pc recourses
; by replacing huge GRUB on this
; and increase my own knowledges in this sphere
; if you want you can copy this to your own project

[ORG 0x7C00] 
[BITS 16]

%define CODE_OFFSET 0x8
%define DATA_OFFSET 0x10

%define KERNEL_POS 0x1000	; 4096 bits dec, 512 bytes
			; loader - 512 size bytes,
			; then kernel will placed to 512
loader:
	mov si, msg
	call PRINT

	mov sp, 0x7BFF
	xor ax, ax	
	mov ss, ax
	mov es, ax
	mov ds, ax

read_kernel:
	mov bx, KERNEL_POS 
	mov dl, 0x80	; always to int 0x13
	mov cl, 0x02	; second sector(each 512 bytes)
	mov ch, 0x00	; first cylinder
	mov ah, 0x02	; read
	mov al, 8	; 8 sectors to read
	int 0x13

	jc .error	; if CF == 1 => int error
	cmp ah, 0	; if ah == 0 => success
	jne .warning
	jmp .success

.error:
	mov si, error
	call PRINT
	hlt
	jmp $

.warning:
	add ah, 0x30	; func PRINT will modified ah
	mov byte [status], ah

	mov si, warning
	call PRINT

	mov si, status
	call PRINT

	jmp prot_mode_switch

.success:
	mov si, success
	call PRINT
	
	jmp prot_mode_switch

prot_mode_switch:
	[BITS 32]
	lgdt [gdt_start]
	xor eax, eax
	or eax, 0x01
	mov cr0, eax
	jmp CODE_OFFSET:prot_mode_main
gdt_start:
	; NULL descriptor
	db 0x00000000
	db 0x00000000

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
	db 0b10010010	; Access byte
	db 0b11001111	; Flags
	db 0x00		; Base

gdt_end:
gdt_descriptor:
	dw gdt_end - gdt_descriptor - 1 ; size - 1
	dd gdt_descriptor		

prot_mode_main:
	mov sp, 0x7BFF	; initialize stack for prot mode
	mov bp, sp
	mov ax, DATA_OFFSET ; initialize segment registers
	mov ds, ax	; for prot mode
	mov ss, ax
	mov fs, ax
	mov gs, ax
	mov es, ax

	in al, 0x92	; enabling a20 line
	or al, 0x2
	out 0x92, al	

CPUID_check:
	pushfd	; if 0x00200000 in EFLAGS is modifable
	pushfd	; that processor support CPUID
	xor dword [esp], 0x00200000
	popfd
	pushfd
	pop eax
	xor eax, [esp]
	popfd
	and eax, 0x00200000
	
	cmp eax, 0
	jne .long_mode_error

	mov eax, 1
	cpuid

	hlt
	jmp $
.long_mode_error:
	mov si, long_mode_error
	call PRINT
	jmp $
PRINT:
	mov ah, 0x0E
	mov al, [si]
	inc si
	int 0x10
	cmp al, 0
	jne PRINT
	ret

msg: db "loading os, ", 0
error: db "read kernel error", 0
warning: db "read kernel warning: AH register is ", 0
status: times 2 db 0
success: db "success reading kernel", 0
long_mode_error: db "your processor not support 64 bit mode, exiting", 0
intel_support: db "your processor is intel", 0
times 510 - ($ - $$) db 0

dw 0xAA55
