kernel = kernel/kernel
IO = IO/io
loader = loader/loader
CC = gcc
flags = -Wall -Wextra -Werror -m32 -nostdlib -nostdinc -fno-builtin -fno-stack-protector -nostartfiles -nodefaultlibs
all:
	nasm -f elf32 $(IO).s -o $(IO)_asm.o 
	$(CC) -c $(flags) $(IO).c -o $(IO)_c.o

	$(CC) -c $(flags) $(kernel).c -o $(kernel).o
	
	$(CC) $(flags) $(kernel).o $(IO)_asm.o $(IO)_c.o -T $(kernel).ld -elf32 -o $(kernel).elf -e _start -nostartfiles 
	mv $(kernel).elf ./iso/boot/
	
	nasm -f bin $(loader).s -o $(loader).bin
	
	mv $(loader).bin iso/boot/$(loader).bin
	
	xorriso -as mkisofs -r -b boot/$(loader).bin -no-emul-boot \
		-boot-load-size 4 -boot-info-table -o BOS.iso ./iso
clean:
	rm -f iso/boot/kernel.elf iso/boot/$(loader).bin BOS.iso