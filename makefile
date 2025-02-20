file = loader

all: kernel.elf

kernel.elf: $(file).o
	ld -T $(file).ld -m elf_x86_64 $(file).o -o kernel.elf && mv kernel.elf iso/boot
$(file).o: $(file).s
	nasm -f elf64 $(file).s -o $(file).o
clean:
	rm -f iso/boot/kernel.elf $(file).o
