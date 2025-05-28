global _outp
global _inp
global _configure_serial_port_baud_rate
global _set_serial_port_settings
global _set_serial_port_FIFO_buffers
global _set_serial_port_modem
global _is_serial_port_free

section .text
__outp:
	mov dx, si	; only rdx must to contain
			; address of port

	mov ax, di	; move data to send it
	out dx, ax	; send data to port di
	ret
__inp:
	mov dx, si 
	in al, dx
	ret

; SERIAL_DATA_PORT               (base) (base)
; SERIAL_FIFO_COMMAND_PORT       (base) (base + 2)
; SERIAL_LINE_COMMAND_PORT       (base) (base + 3)
; SERIAL_MODEM_COMMAND_PORT      (base) (base + 4)
; SERIAL_LINE_STATUS_PORT        (base) (base + 5)

%define SERIAL_PORT_ENABLE_DOUBLE_SEND 0x80
_configure_serial_port_baud_rate:
	mov ax, di ; save port
	mov cx, si ; save divisor

	mov di, ax
	add di, 3  ; LINE_COMMAND
	mov si, SERIAL_PORT_ENABLE_DOUBLE_SEND
	call _outp

	mov di, ax
	mov si, cx
	shr word si, 8
	and word si, 0x00FF
	call _outp

	mov di, ax
	mov si, cx
	and word si, 0x00FF
	call _outp

	ret

_set_serial_port_settings:
	add di, 3
	and si, 0x00FF
	call _outp
	
	ret
_set_serial_port_FIFO_buffers:
	add di, 2
	and si, 0x00FF
	call _outp

	ret
_set_serial_port_modem:
	add di, 4
	and si, 0x00FF
	call _outp

	ret
_is_serial_port_free:
	call _inp
	and al, 0b00100000
	cmp al, 0
	je Tr
	mov ax, 0
	ret
Tr:	
	mov ax, 1
	ret
