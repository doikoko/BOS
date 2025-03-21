section .text
	extern hello
	global _start 
_start:
	call hello
	jmp $
