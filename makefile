kernel = kernel/kernel
IO = IO/io
ports = ports/ports
loader = loader/loader
CC = gcc
flags = -no-pie -Wall -Wextra -Werror -m32 -nostdlib -nostdinc -fno-builtin -fno-stack-protector -nostartfiles -nodefaultlibs

all: loader.bin IO.o ports.o kernel.elf BOS.iso
loader.bin:
	nasm -f bin ./$(loader).s -o ./iso/boot/$(loader).bin
	dd if=./iso/boot/loader/loader.bin of=./iso/boot/loader/compLoader.bin bs=2048 conv=sync
	rm -f ./iso/boot/loader/loader.bin
	mv ./iso/boot/loader/compLoader.bin ./iso/boot/loader/loader.bin
IO.o:
	nasm -f elf32 ./$(IO).s -o ./$(IO).o 
ports.o:
	nasm -f elf32 ./$(ports).s -o ./$(ports).o
kernel.elf:
	nasm -f elf32 ./$(kernel).s -o ./$(kernel).o
	ld -m i386pe ./$(IO).o ./$(ports).o ./$(kernel).o -T ./$(kernel).ld -o ./kernel/compKernel.o
	$(CC) $(flags) ./kernel/compKernel.o -T ./$(kernel).ld -o ./iso/boot/kernel.elf -e kernel
BOS.iso:
	xorriso -as mkisofs -b /boot/$(loader).bin -no-emul-boot -boot-load-size 4 -o BOS.iso ./iso
clean:
	rm -f ./iso/boot/loader/loader.bin
	rm -f ./$(IO).o
	rm -f ./$(ports).o
	rm -f ./$(kernel).o
	rm -f ./kernel/compKernel.o
	rm -f ./iso/boot/kernel.elf
	rm -f ./BOS.iso
