; it's a kernel of operating system

; ==========INCLUDED FUNCTIONS======
; ===============IO.S=============
%define CL_BLACK    0x0 ; colors of text
%define CL_BLUE     0x1
%define CL_GREEN    0x2
%define CL_CYAN     0x3
%define CL_RED      0x4
%define CL_MAGENTA  0x5
%define CL_BROWN    0x6
%define CL_LGREY    0x7
%define CL_DGREY    0x8
%define CL_LBLUE    0x9
%define CL_LGREEN   0xA
%define CL_LCYAN    0xB
%define CL_LRED     0xC
%define CL_LMAGENTA 0xD
%define CL_LBROWN   0xE
%define CL_WHITE    0xF

extern print_c
extern print_s
extern move_cursor

; void print_c(char *symb, unsigned short fg, unsigned short bg);
; void print_s(char *buf, unsigned short fg, unsigned short bg, unsigned short length); 
; void move_cursor(unsigned int row, unsigned int column);

; ==============PORTS.S==========

; function to configure speed in bouds of serial port (default is like in macro)
; default speed is 115200hz, divisor is a velue that speed will divide
; if divisor = 2, speed will be (115200 / 2)bouds
%define SERIAL_PORT 0x3F8
extern configure_serial_port_baud_rate
; void configure_serial_port_baud_rate(unsigned int serial_port, unsigned int divisor);


; 7| 0 |6| 0 |5| 000 |2| 0 |1| 11 |0|
; 0,1: 	8 bit data
; 2: 	number of stop bytes
; 3,4,5:number of parity
; 6:	break control
; 7:	access byte
%define SERIAL_PORT_SETTING 0x03
extern set_serial_port_settings
; void set_serial_port_settings(unsigned int serial_port, unsigned short settings);

; function to enable FIFO in buffers
%define FIFO_ENABLE 0xC7
extern set_serial_port_FIFO_buffers
; void  set_serial_port_FIFO_buffers(unsigned short serial_port, unsigned short settings);

; function to set modem ready status
%define MODEM_READY_STATUS 0x03
extern set_serial_port_modem
; void set_serial_port_modem(unsigned short serial_port, unsigned short settings);

; this function provide a ability of check empty or not port
; if this empty, function returns 1; 
; if port has a velue, function return 0
extern is_serial_port_free
; int is_serial_port_free(unsigned short port);
; ==============================================

%define KERNEL_STACK_SIZE 0x1000

global kernel
;section .bss
;	; km - kernel memory
;	km resb KERNEL_STACK_SIZE ; locate free memory to bss
;		 		; instead just move sp register
section .text
kernel:
;	mov di, SERIAL_PORT
;	mov si, 2
;	call configure_serial_port_baud_rate ; 115200 / 2 bouds
;
;	mov di, SERIAL_PORT 
;	mov si, SERIAL_PORT_SETTING
;	call set_serial_port_settings ; set a valid settings
;
;	mov di, SERIAL_PORT 
;	mov si, FIFO_ENABLE
;	call set_serial_port_FIFO_buffers ; set a FIFO
;
;	mov di, SERIAL_PORT
;	mov si, MODEM_READY_STATUS
;	call set_serial_port_modem ; set a modem in ready status
;	
	mov edi, 0x31
	mov si, CL_WHITE
	mov dx, CL_BLACK
	call print_c
msg: db "test", 0
