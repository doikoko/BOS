; it's a free small bootloader
; goal of this - economy pc recourses
; by replacing huge GRUB on this
; and increase my own knowledges in this sphere
; if you want you can copy this to your own project

[ORG 0x7C00] 
[BITS 16]
loader:
	xor ax, ax	
	mov ss, ax
	mov es, ax
	mov ds, ax
	mov sp, 0x3FFF
	
	mov [boot_device], dl

	sti
	mov ax, 0x0003 ; set text mode
	int 0x10	; switch videocard mode to text
	cli

	mov di, msg
	call PRINT


.loading_rust_part_of_loader:
	xor ax, ax
	mov di, 0x4000
	mov cx, 0x1000
	cld
	rep stosw

	sti
	mov si, DAP
	mov dl, [boot_device]
	mov ah, 0x42
	int 0x13
	jc .error_loading_kernel
	jmp protected_mode_switch
.error_loading_kernel:
	mov di, error
	call PRINT
.LOOP1:
	hlt
	jmp .LOOP1
protected_mode_switch:
	cli
	in al, 0x92	; enabling a20 line
	or al, 0x2
	out 0x92, al	

	lgdt [GDT32.Pointer]
	xor eax, eax
	or al, 0x01
	mov cr0, eax
	jmp dword GDT32.Code:prot_mode_main

prot_mode_main:
	[BITS 32]
	cli
	mov ax, 0x10 ; initialize segment registers
	mov ds, ax	; for prot mode
	mov ss, ax
	mov fs, ax
	mov gs, ax
	mov es, ax
	mov esp, 0x3FFF	; initialize stack for prot mode
	mov ebp, esp

long_mode_support_check:
	mov eax, 0x80000000
	cpuid
	cmp eax, 0x80000001	; if eax bellow => long mode 
				; not supported
	jb .no_long_mode

	mov eax, 0x80000001	; if bit edx 29 = 0 => long mode
				; not supported
	cpuid			; return value to eax:edx
	test edx, 1 << 29 ; if equal => long mode not supported
	
	jne jump_to_rust

.no_long_mode:
	mov edi, long_mode_unsupported
	call PRINT32
.LOOP2:
	hlt
	jmp .LOOP2

jump_to_rust:
%assign RUST_LOADER_ENTRY 0x4000
	mov edi, PRINT32
	mov esi, GDT64
	xor eax, eax
	mov eax, RUST_LOADER_ENTRY
	jmp eax

;switch_to_64_bit:
;	mov ecx, 0xC0000080	; loading address of specific register
;	rdmsr
;	or eax, 1 << 8
;	wrmsr			; writing data to specific register
;	
;	mov eax, cr4
;	or eax, 1 << 5 ; enabling paging
;	mov cr4, eax
;
;	mov eax, cr0
;	or eax, 1 << 31
;	mov cr0, eax
;
;	lgdt [GDT64.Pointer]
;	jmp GDT64.Code:long_mode_main
;
;long_mode_main:
;	[BITS 64]
;	cli
;	mov ax, GDT64.Data
;	mov rsp, 0x3FFF
;	mov ax, GDT64.TSS - GDT64.Null
;	ltr ax

	; end of loader
PRINT:
%macro XOR_DS 0
	xor ax, ax
	mov ds, ax
%endmacro

%macro SET_DS 0
	mov ax, 0xB800
	mov ds, ax
%endmacro

	[BITS 16]
	XOR_DS
	mov byte bl, [di]
	xor ax, ax
	mov byte al, [letters_count]
	mov si, ax
	SET_DS
.LOOP3:
	mov byte [si], bl
	inc si,
	mov byte [si], 0
	or byte [si], 0x0F
	inc si
	inc di

	XOR_DS
	mov byte bl, [di]
	SET_DS
	cmp byte bl, 0
	jne .LOOP3

	XOR_DS
	mov ax, si
	mov byte [letters_count], al
	ret

PRINT32:
	[BITS 32]
	xor eax, eax
	mov byte al, [letters_count]
	add eax, 0xB8000
	mov byte bl, [edi]
.LOOP4:
	mov byte [eax], bl
	inc eax,
	mov byte [eax], 0
	or byte [eax], 0x0F
	inc eax
	inc edi
	mov byte bl, [edi]
	cmp byte bl, 0 
	jne .LOOP4
	mov byte [letters_count], al
	ret

boot_device: db 0
GDT32:
.Null: equ $ - GDT32       
	dq 0                   

.Code: equ $ - GDT32       
	dw 0xFFFF	; limit
	dw 0x0000	; base
	db 0        ; base       
	db PRESENT | NOT_SYS | EXEC | RW  
	db GRAN_4K | SZ_32 | 0xF  
	db 0        ; base

.Data: equ $ - GDT32       
	dw 0xFFFF
	dw 0x0000
	db 0                   
	db PRESENT | NOT_SYS | RW  
	db GRAN_4K | SZ_32 | 0xF   
	db 0                   

.Pointer:                  
	dw $ - GDT32 - 1       
	dd GDT32               

%define TSS_size 104
%define TSS_addr 0x8000
align 8
GDT64:
.Null: equ $ - GDT64
	dq 0
.Code: equ $ - GDT64
	dd 0xFFFF                                   
	db 0                                        
	db PRESENT | NOT_SYS | EXEC | RW            
	db GRAN_4K | LONG_MODE | 0xF                
	db 0                                        
.Data: equ $ - GDT64
	dd 0xFFFF                                   
	db 0                                        
	db PRESENT | NOT_SYS | RW                   
	db GRAN_4K | SZ_32 | 0xF                    
	db 0                                        
.TSS: equ $ - GDT64
    dw TSS_Size - 1                          
    dw TSS_addr & 0xFFFF                     
    db (TSS_addr >> 16) & 0xFF               
    db 0x89                                  
    db 0                                     
    db (TSS_addr >> 24) & 0xFF               
    dd (TSS_addr >> 32) & 0xFFFFFFFF         
    dd 0                                     
.Pointer:
	dw $ - GDT64 - 1
	dq GDT64

msg: db "loading os / ", 0
error: db "kernel error / ", 0
long_mode_unsupported: db "long mode unsupported / ", 0
letters_count: db 0

gdt_flags:
.access:
	PRESENT        equ 1 << 7     ; Segment in memory
	NOT_SYS        equ 1 << 4     ; Code/date descriptor
	EXEC           equ 1 << 3     ; Executable(code)
	DC             equ 1 << 2     ; direction (0 - to up, 1 - to bottom)
	RW             equ 1 << 1     ; Write (code) read(data)
	ACCESSED       equ 1 << 0     ; Access to segment
.flags:
	GRAN_4K       equ 1 << 7      ; granularity
	SZ_32         equ 1 << 6      ; 32-bits segment (1) / 16-bits (0)
	LONG_MODE     equ 1 << 5      ; 64-bits segment


%define DAP_LENGTH 16
%define EMPTY_BYTE 0
%assign TRANSFER_SIZE 0x2000
%assign RUST_LOADER_START 0x4000
%assign ENTRY_POINT_POS_IN_ISO_IN_SECTORS	50 + (0x1000 / 2048) ; 50 check in build.py, 
															; 0x1000 offset of entry point
%assign TRANSFER_SIZE_IN_SECTORS TRANSFER_SIZE / 2048

DAP:	; structure for int 0x13 arguments
		; Disk Address Packet
	db DAP_LENGTH
	db EMPTY_BYTE
	dw TRANSFER_SIZE_IN_SECTORS
	dd RUST_LOADER_START
	dd ENTRY_POINT_POS_IN_ISO_IN_SECTORS

times TSS_addr - ($ - $$) db 0
section .tss
align 16
%define RSP_0 0x000000000000D000
%define IST_1 0x000000000000E000
%define IST_2 0x000000000000F000
%define IST_3 0x0000000000010000
TSS_Base:
    dd 0x00000000                    
    dq RSP_0
    dq 0
    dq 0                 
    dq IST_1                            
    dq IST_2                         
    dq IST_3
    dq 0
    dq 0
    dq 0
    dq 0
    dq 0
    dw 0                             
    dw 0
TSS_Size equ $ - TSS_Base
times IST_3 - ($ - $$) db 0

dw 0xAA55
