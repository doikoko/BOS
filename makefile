kernel = kernel/kernel
IO = IO/io
ports = ports/ports
loader = loader/loader
CC = gcc
flags = -Wall -Wextra -Werror -m32 -nostdlib -nostdinc -fno-builtin -fno-stack-protector -nostartfiles -nodefaultlibs
all:
	nasm -f elf32 $(IO).s -o $(IO).o 
	nasm -f elf32 $(ports).s -o $(ports).o
	nasm -f elf32 $(kernel).s -o $(kernel).o

	$(CC) $(flags) $(kernel).o $(IO).o $(ports).o -T $(kernel).ld -elf32 -o $(kernel).elf -e _start -nostartfiles 
	mv $(kernel).elf ./iso/boot/
	
	nasm -f bin $(loader).s -o $(loader).bin
	mv $(loader).bin iso/boot/$(loader).bin
	
	xorriso -as mkisofs -r -b boot/$(loader).bin -no-emul-boot \
		-boot-load-size 4 -boot-info-table -o BOS.iso ./iso
clean:
	rm -f iso/boot/kernel.elf iso/boot/$(loader).bin \ 
	$(IO).o $(ports).o BOS.iso
