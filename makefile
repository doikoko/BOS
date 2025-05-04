kernel = kernel/kernel
IO = IO/io
ports = ports/ports
loader = loader/loader
CC = gcc
flags = -no-pie -Wall -Wextra -Werror -m32 -nostdlib -nostdinc -fno-builtin -fno-stack-protector -nostartfiles -nodefaultlibs
prog = xorriso -as mkisofs

all: loader.sys bootcat IO.o ports.o kernel.elf
bootcat:
	touch ./iso/boot/bootcat
loader.sys:
	nasm -f bin ./$(loader).s -o ./iso/boot/$(loader).bin
	dd if=./iso/boot/loader/loader.bin of=./iso/boot/loader/compLoader.sys bs=2048 conv=sync
	rm -f ./iso/boot/loader/loader.bin
	mv ./iso/boot/loader/compLoader.sys ./iso/boot/loader/loader.sys
IO.o:
	nasm -f elf32 ./$(IO).s -o ./$(IO).o 
ports.o:
	nasm -f elf32 ./$(ports).s -o ./$(ports).o
kernel.elf:
	nasm -f elf32 ./$(kernel).s -o ./$(kernel).o
	$(CC) $(flags) ./$(IO).o ./$(ports).o ./$(kernel).o -T ./$(kernel).ld -o ./iso/boot/kernel.elf -e kernel
u:
	$(prog) -R -J -c /boot/loader/bootcat -b /boot/$(loader).sys -no-emul-boot -boot-load-size 4 -o BOS.iso ./iso
b:
	$(prog) -b /boot/$(loader).sys -c /boot/bootcat -no-emul-boot -boot-load-size 4 \
  	-boot-info-table -o BOS.iso ./iso
clean:
	rm -f ./iso/boot/loader/loader.sys
	rm -f ./$(IO).o
	rm -f ./$(ports).o
	rm -f ./$(kernel).o
	rm -f ./iso/boot/kernel.elf
	rm -f ./BOS.iso
	rm -f ./iso/boot/loader/bootcat
