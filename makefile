kernel = kernel/kernel
IO = IO/io
ports = ports/ports
loader = loader/loader
CC = gcc
flags = -no-pie -Wall -Wextra -Werror -m32 -nostdlib -nostdinc -fno-builtin -fno-stack-protector -nostartfiles -nodefaultlibs

all:
	nasm -f bin $(loader).s -o ./bin/loader.bin
	
	nasm -f elf32 $(IO).s -o $(IO).o 
	nasm -f elf32 $(ports).s -o $(ports).o

	nasm -f elf32 $(kernel).s -o $(kernel).o

	ld -m i386pe $(IO).o $(ports).o $(kernel).o -T $(kernel).ld -o ./kernel/completeKernel.o
	$(CC) $(flags) ./kernel/completeKernel.o -o ./bin/kernel.bin

	dd if=./bin/loader.bin of=./bin/os.bin conv=notrunc
	dd if=./bin/kernel.bin of=./bin/os.bin bs=512 seek=1 conv=notrunc
	dd if=/dev/zero bs=512 count=8 >> ./bin/os.bin
	
	cp ./bin/os.bin ./iso/os.bin
	xorriso -as mkisofs -r -b os.bin -no-emul-boot \
		-boot-load-size 4 -boot-info-table -o BOS.iso ./iso

clean:
	rm -f iso/boot/kernel.elf iso/boot/$(loader).bin $(IO).o $(ports).o BOS.iso ./bin/os.bin ./bin/kernel.bin ./bin/loader.bin ./kernel/completeKernel.o
