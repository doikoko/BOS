ENTRY(loader)

SECTIONS {
    . = 0x0;
    .loader : {
        KEEP(*(loader.loader))
        *(.loader*) 
    }

    . = ALIGN(16); 
    .text : {
        *(.text*)
    }

    . = ALIGN(16);
    .rodata : {
        *(.rodata*)
    }

    . = ALIGN(16);
    .data : {
        *(.data*)
    }

    . = ALIGN(16);
    .bss : {
        *(.bss)
        *(COMMON)
    }
    /DISCARD/ : {
        *(.eh_frame*)
        *(.comment)
    }
}