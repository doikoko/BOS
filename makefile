file = kernel/kernel
loader = loader/loader

all:
	nasm -f elf64 $(file).s -o $(file)_asm.o 
	gcc -c $(file).c -o $(file)_c.o
	gcc $(file)_c.o $(file)_asm.o -T $(file).ld -o $(file).elf -e _start -nostartfiles 
	mv $(file).elf ./iso/boot/
	nasm -f bin $(loader).s -o $(loader).bin
	mv $(loader).bin iso/boot/$(loader).bin
	xorriso -as mkisofs -r -b boot/$(loader).bin -no-emul-boot -boot-load-size 4 -boot-info-table -o BOS.iso ./iso
clean:
	rm -f iso/boot/kernel.elf $(file)_asm.o $(file)_c.o iso/boot/$(loader).bin BOS.iso
