section .data
	FLAGS		equ 0
	MAGIC_NUMBER	equ 0xCAFEBABE
	CHECKSUM	equ -MAGIC_NUMBER

section .text
global loader
align 4
	
	dd FLAGS	
	dd MAGIC_NUMBER
	dd CHECKSUM
loader:
	mov eax, 0xCAFEBABE
loop:
	jmp loop	
