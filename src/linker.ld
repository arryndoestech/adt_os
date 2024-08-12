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
	.rela.dyn : {
		_rela_start = .;
		*(.rela.*)
		_rela_end = .;
	}
	.stack (NOLOAD) : {
		KEEP(*(.stack*))
	}
	.bss (NOLOAD) : {
		*(.bss*)
	}
}
