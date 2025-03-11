[BITS 32]

section .text

global _start
export kernel_main
_start:
	call kernel_main

	jmp $
