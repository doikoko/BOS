file = loader

all: kernel.elf

kernel.elf: $(file).o
	ld -T $(file).ld -m elf_x86_64 $(file).o -o kernel.elf && mv kernel.elf iso/boot

$(file).o: $(file).s
	nasm -f elf64 $(file).s -o $(file).o

iso: BOS.iso
BOS.iso: kernel.elf
	xorriso -as mkisofs -R -b boot/grub/stage2_eltorito -no-emul-boot -boot-load-size 4 -A BOS -input-charset -quiet -boot-info-table -o BOS.iso iso
clean:
	rm -f iso/boot/kernel.elf $(file).o BOS.iso
