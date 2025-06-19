global _outp
global _inp

section .text
__outb:
	mov dx, di	; only rdx must to contain
			; address of port

	mov ax, si	; move data to send it
	out dx, al	; send data to port di
	ret
_outw:
	mov dx, di
	mov ax, si
	out dx, ax
	ret
__inb:
	mov dx, di 
	in al, dx
	ret