; it's a free small bootloader
; goal of this - economy pc recourses
; by replacing huge GRUB on this
; and increase my own knowledges in this sphere
; if you want you can copy this to your own project

[ORG 0x7C00] 
[BITS 16]

loader:
	cli
	xor ax, ax
	mov ds, ax

	sti
	mov ax, 0x0003 ; set text mode
	int 0x10	; switch videocard mode to text
	cli

	mov si, msg
	call PRINT

	mov sp, 0x7BFF
	xor ax, ax	
	mov ss, ax
	mov es, ax
	mov ds, ax

unreal_mode_switch:
	in al, 0x92	; enabling a20 line
	or al, 0x2
	out 0x92, al	

	[BITS 32]
	lgdt [GDT32]
	xor eax, eax
	or eax, 0x01
	mov cr0, eax

	jmp GDT32.Code:.next

.next:
	[BITS 32]
	mov ax, GDT32.Data
	mov es, ax
	mov ds, ax
	mov fs, ax
	mov gs, ax
	mov eax, cr0
	xor eax, 0x01
	mov cr0, eax

	jmp GDT32.Code:load_kernel
load_kernel:
	sti

%define DAP_LENGTH 16
%define EMPTY_BYTE 0
%assign KERNEL_PACKET_SIZE 0x2000
%assign KERNEL_FIRST_ADDR 0x5000
%assign KERNEL_RESULT_LAST_ADDR 0x400000
%assign KERNEL_LAST_ADDR 0x7000
%assign KERNEL_POS_IN_ISO_IN_SECTORS 0x32000 / 2048
%assign KERNEL_PACKET_SIZE_IN_SECTORS KERNEL_PACKET_SIZE / 2048
; BIOS can load only 32 sectors in 1 time, so we need to
; load kernel next times

	xor ax, ax
	mov ds, ax

	; don't changed dl register before, it's containing 
	; device info
	mov bx, 0
.interrupt_loop:
; zeroed 0x5000-0x7000 to load part of kernel
	mov ecx, KERNEL_PACKET_SIZE
	xor edi, edi
	mov es, di
	mov edi, KERNEL_FIRST_ADDR
	mov esi, DAP ; load addr of DAP
	rep stosb
	
	xor eax, eax
	mov ah, 0x42 ; read sectors from drive
	int 0x13
	jc .error_loading_kernel

	inc bx

	mov byte [es:esi], DAP_LENGTH
	mov byte [es:esi + 1], EMPTY_BYTE
	mov word [es:esi + 2], KERNEL_PACKET_SIZE_IN_SECTORS
	mov word [es:esi + 4], KERNEL_FIRST_ADDR
	and word [es:esi + 6], EMPTY_BYTE
	mov ax, bx
	mov dx, KERNEL_PACKET_SIZE_IN_SECTORS
	mul dx
	mov word [es:esi + 8], ax
	add word [es:esi + 8], KERNEL_POS_IN_ISO_IN_SECTORS
	xor word [es:esi + 10], EMPTY_BYTE
	xor dword [es:esi + 12], EMPTY_BYTE
	
	xor eax, eax
	mov es, eax
	mov eax, KERNEL_FIRST_ADDR
;move packet to 0x200000 + packets address
.copy_to_final_addr:
	mov edi, dword [es:eax]
	mov esi, KERNEL_RESULT_ADDR
	mov dword [es:esi], edi 
	add eax, 4
	add dword [es:esi], 4
	cmp dword [es:esi], KERNEL_RESULT_LAST_ADDR
	jae prot_mode_switch
	cmp ax, KERNEL_LAST_ADDR
	jb .copy_to_final_addr
	jmp .interrupt_loop

.error_loading_kernel:
	mov si, error
	call PRINT
	hlt
	jmp $
prot_mode_switch:
	cli
	[BITS 32]
	xor eax, eax
	or eax, 0x01
	mov cr0, eax
	jmp GDT32.Code:prot_mode_main
GDT32:
.Null: equ $ - GDT32       
	dq 0                   

.Code: equ $ - GDT32       
	dd 0xFFFF              
	db 0                   
	db PRESENT | NOT_SYS | EXEC | RW  
	db GRAN_4K | SZ_32 | 0xF  
	db 0                   

.Data: equ $ - GDT32       
	dd 0xFFFF              
	db 0                   
	db PRESENT | NOT_SYS | RW  
	db GRAN_4K | SZ_32 | 0xF   
	db 0                   

.Pointer:                  
	dw $ - GDT32 - 1       
	dd GDT32               
	db 0x00000000
	db 0x00000000

prot_mode_main:
	cli
	mov ax, 0xB800
	mov es, ax

	mov sp, 0x8B00	; initialize stack for prot mode
	mov bp, sp
	mov ax, 0x10 ; initialize segment registers
	mov ds, ax	; for prot mode
	mov ss, ax
	mov fs, ax
	mov gs, ax
	mov ax, 0x10
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
	jne no_long_mode

	mov eax, 0x80000000
	cpuid
	cmp eax, 0x80000001	; if eax bellow => long mode 
				; not supported
	jb no_long_mode

	mov eax, 0x80000001	; if bit edx 29 = 0 => long mode
				; not supported
	cpuid			; return value to eax:edx
	test edx, 1 << 29
	jz no_long_mode
	

switch_to_64_bit:
	mov ecx, 0xC0000080	; loading address of specific register
	rdmsr
	or eax, 1 << 8
	wrmsr			; writing data to specific register

	mov eax, cr0
	or eax, 1 << 31
	mov cr0, eax

	lgdt [GDT64]
	jmp GDT64.Code:long_mode_main

%define TSS_size 104
%define TSS_addr 0x8000
section .gdt
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

no_long_mode:
	hlt
	jmp $
long_mode_main:
	[BITS 64]
	sti
	mov ax, GDT64.Data
	mov rsp, 0x3FFFFF
	mov ax, GDT64.TSS - GDT64.Null
	ltr ax

	mov rax, [KERNEL_FIRST_ADDR + 24]
	jmp [rax]
	; end of loader
INC_DS:
	[BITS 16]
	mov si, ds
	add si, 0x1000
	mov ds, si
	ret 
PRINT:
	[BITS 16]
	mov ah, 0x0E
	mov al, [si]
	inc si
	int 0x10
	cmp al, 0
	jne PRINT
	ret

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

msg: db "loading os, ", 0
error: db "kernel error", 0
KERNEL_RESULT_ADDR: dd 0x00200000
align 16
DAP:	; structure for int 0x13 arguments
		; Disk Address Packet
	db DAP_LENGTH
	db EMPTY_BYTE
	dw KERNEL_PACKET_SIZE_IN_SECTORS
	dd KERNEL_FIRST_ADDR
	dq KERNEL_POS_IN_ISO_IN_SECTORS

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
