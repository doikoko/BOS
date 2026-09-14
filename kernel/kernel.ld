ENTRY(_start)
SECTIONS{
    . = 0x200000;
    .kernel : {
        KEEP(*(kernel.kernel))
        *(.kernel*) 
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
        *(.comment)
    }
}
