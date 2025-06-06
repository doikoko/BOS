global _outp
global _inp

section .text
__outp:
	mov dx, di	; only rdx must to contain
			; address of port

	mov ax, si	; move data to send it
	out dx, al	; send data to port di
	ret
__inp:
	mov dx, di 
	in al, dx
	ret