MEMORY
{
  FLASH : ORIGIN = 0x20000000, LENGTH = 0x4000000
  RAM : ORIGIN = 0x80000000, LENGTH = 0x07000000
}

PROVIDE(_stack_start = 0x87000000);
ENTRY(_start);

SECTIONS
{
	.text ORIGIN(RAM) : {
		*(.init)
		*(.text.*)
		*(.interrupt_handler)
	} > RAM

	.rodata : { *(.rodata.*) } > RAM
	.eh_frame : { *(.eh_frame.*) } > RAM
	.sdata : { *(.sdata) } > RAM
}
