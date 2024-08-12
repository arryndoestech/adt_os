.section .text._start
.global _start
.type _start, @function
_start:
	#Initiliaze the stack
	adr x7, {} //Move address of STACK variable
	mov x8, {} //Move STACK_SIZE into x8
	add x7, x7, x8 //Add it to the sp
	mov sp, x7
	
	#Enable FPEN bits
	mrs x7, cpacr_el1
	orr x7, x7, #(3 << 20)	
	msr cpacr_el1, x7
	
	adr x0, _start
	adr x1, _rela_start
	adr x2, _rela_end
	bl _relocate_binary
	
	mrs x7, mpidr_el1
	and x7, x7, 0xFF
	msr tpidr_el1, x7

	#Call into main
	bl main

.equ R_AARCH64_RELATIVE, 1027
_relocate_binary:
	ldp x7, x8, [x1], 16
	ldr x9, [x1], 8

	cmp x8, R_AARCH64_RELATIVE
	bne 1f

	add x10, x0, x7 //Add Offset to base
	add x11, x0, x9 //Add addend + sym to base
	str x11, [x10]
	cmp x1, x2
	bne _relocate_binary
1:
	ret
