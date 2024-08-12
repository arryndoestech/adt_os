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

	#Call into main
	bl main
