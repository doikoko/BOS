ENTRY(loader)

SECTIONS {
    . = 0x1000;
    .loader : {
        KEEP(*(loader.loader))
        *(.loader*) 
    }

    . = ALIGN(0x1000);
    .text : {
        *(.text*)
    }

    . = ALIGN(0x1000);
    .rodata : {
        *(.rodata*)
    }

    . = ALIGN(0x1000);
    .data : {
        *(.data)
    }

    . = ALIGN(0x1000);
    .bss : {
        *(.bss)
        *(COMMON)
    }
}