ENTRY(_start)

SECTIONS {
	.text : {
		KEEP(*(.text._start))
		*(.text*)
	}
	.data : {
		*(.data*)
	}
	.rodata : {
		*(.rodata*)
	}
	.stack : {
		KEEP(*(.stack*))
	}
	.bss : {
		*(.bss*)
	}
}
