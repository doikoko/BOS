%define KERNEL_STACK_SIZE 0x1000

global outp
global print
global move_cursor
global configure_serial_port_baud_rate

section .bss
	resb KERNEL_STACK_SIZE	; allocate 
			; to bss (uninit memory)
		; size of kernel stack instead 
	; just move sp to some location in memory
section .text
RET: ret
outp:
	mov dx, si	; only rdx must to contain
			; address of port

	mov ax, di	; move data to send it
	out dx, ax	; send data to port di
	ret
print:			;write text using framebuffer
	mov eax, 0x000B8000 ;address of framebuffer
	mov [eax], edi
	mov [eax + 1], esi 
	shl byte [eax + 1], 4
	mov cx, dx
	and byte cl, 0x0F
	or  byte [eax + 1], cl 
	ret

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

; SERIAL_DATA_PORT               (base) (base)
; SERIAL_FIFO_COMMAND_PORT       (base) (base + 2)
; SERIAL_LINE_COMMAND_PORT       (base) (base + 3)
; SERIAL_MODEM_COMMAND_PORT      (base) (base + 4)
; SERIAL_LINE_STATUS_PORT        (base) (base + 5)

%define SERIAL_PORT_ENABLE_DOUBLE_SEND 0x80
configure_serial_port_baud_rate:
	mov ax, di ; save port
	mov cx, si ; save divisor

	mov di, ax
	add di, 3  ; LINE_COMMAND
	mov si, SERIAL_PORT_ENABLE_DOUBLE_SEND
	call outp

	mov di, ax
	mov si, cx
	shr word si, 8
	and word si, 0x00FF
	call outp

	mov di, ax
	mov si, cx
	and word si, 0x00FF
	call outp

	ret

set_serial_port_settings:
	add di, 3
	and si, 0x00FF
	call outp
	
	ret
