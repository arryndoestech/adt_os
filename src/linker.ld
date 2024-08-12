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
	.stack (NOLOAD) : {
		KEEP(*(.stack*))
	}
	.bss (NOLOAD) : {
		*(.bss*)
	}
}
