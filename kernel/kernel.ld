ENTRY(_start)
SECTIONS{
    . = 0x200000;
    .kernel : {
        KEEP(*(kernel.kernel))
        *(.kernel*) 
    }

    . = ALIGN(0x10000);
    .text : {
        *(.text*)
    }

    . = ALIGN(0x10000);
    .rodata : {
        *(.rodata*)
    }

    . = ALIGN(0x10000);
    .data : {
        *(.data*)
    }

    . = ALIGN(0x10000);
    .bss : {
        *(.bss)
        *(COMMON)
    }
    /DISCARD/ : {
        *(.comment)
    }
}
