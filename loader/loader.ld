ENTRY(loader)

SECTIONS {
    . = 0x0;
    .loader : {
        KEEP(*(loader.loader))
        *(.loader*) 
    }

    . = 0x790; /* because .loader section ~ 0x300bytes and addresses is pow of 2*/
    .text : {
        *(.text*)
    }

    . = 0xe10; /* .text section ~ 0x600bytes */
    .rodata : {
        *(.rodata*)
    }

    . = 0xf04; /* .rodata ~ 0xd0bytes */
    .data : {
        *(.data*)
    }

    . = 0xf81; /* .data ~ 0x10 bytes */
    .bss : {
        *(.bss)
        *(COMMON)
    }
    /DISCARD/ : {
        *(.comment)
    }
}