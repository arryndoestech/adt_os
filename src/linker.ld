ENTRY(_start)
_page_size = 1024*16; /*We're using 16K pages*/

SECTIONS {
	.text : ALIGN(_page_size) {
		KEEP(*(.text._start))
		*(.text*)
	}
	.data : ALIGN(_page_size) {
		*(.data*)
	}
	.rodata : ALIGN(_page_size) {
		*(.rodata*)
	}
	.rela.dyn : ALIGN(_page_size) {
		_rela_start = .;
		*(.rela.*)
		_rela_end = .;
	}
	.stack (NOLOAD) : ALIGN(_page_size) {
		KEEP(*(.stack*))
	}
	.bss (NOLOAD) : ALIGN(_page_size){
		_bss_start = .;
		*(.bss*)
		. = ALIGN(_page_size);
		_bss_end = .;
	}
}
