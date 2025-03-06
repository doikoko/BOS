
file = kernel
loader = iso/boot/loader

all: kernel.elf

kernel.elf: $(file).o
	ld -T $(file).ld -m elf_x86_64 $(file).o -o kernel.elf && mv kernel.elf iso/boot && rm -f $(file).o

$(file).o: $(file).s
	nasm -f elf64 $(file).s -o $(file).o 

iso: BOS.iso
BOS.iso: $(loader).bin
	xorriso -as mkisofs -r -b boot/loader.bin -no-emul-boot -boot-load-size 4 -boot-info-table -o BOS.iso ./iso
ld: $(loader).bin
$(loader).bin:
	nasm -f bin loader.s -o $(loader).bin
clean:
	rm -f iso/boot/kernel.elf $(file).o BOS.iso $(loader).bin
