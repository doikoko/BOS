ENTRY(_start)
SECTIONS
{
	. = 0x100000;
	.text ALIGN(0x1000):
	{
		*(.text)
	}
	.bss ALIGN(0x1000):
	{
		*(.bss)
		*(.COMMON)
	}
	.rodata ALIGN(0x1000):
	{
		*(.rodata*)		
	}	
	.data ALIGN(0x5000):
	{
		*(.data)
	}
}
