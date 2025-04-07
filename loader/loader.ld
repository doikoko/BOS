ENTRY(loader)
OUTPUT_FORMAT(binary)
SECTIONS
{
	. = 512;
	.text ALIGN(400):
	{
		*(.text)
	}
	.bss ALIGN(40):
	{
		*(.bss)
		*(.COMMON)
	}
	.rodata ALIGN(40):
	{
		*(.rodata*)		
	}	
	.data ALIGN(32):
	{
		*(.data)
	}
}
