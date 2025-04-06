extern outp
extern inp

global configure_serial_port_baud_rate
global set_serial_port_settings
global set_serial_port_FIFO_buffers
global set_serial_port_modem

section .text

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
set_serial_port_FIFO_buffers:
	add di, 2
	and si, 0x00FF
	call outp

	ret
set_serial_port_modem:
	add di, 4
	and si, 0x00FF
	call outp

	ret
is_serial_port_free:
	call inp
	and al, 0b00100000
	cmp al, 0
	je Tr
	mov ax, 0
	ret
Tr:	
	mov ax, 1
	ret
