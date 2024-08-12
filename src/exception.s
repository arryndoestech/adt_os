.macro save_regs
	sub sp, sp, #192
	stp x0, x1, [sp, #0]
	stp x2, x3, [sp, #16]
	stp x4, x5, [sp, #32]
	stp x6, x7, [sp, #48]
	stp x8, x9, [sp, #64]
	stp x10, x11, [sp, #80]
	stp x12, x13, [sp, #96]
	stp x14, x15, [sp, #112]
	stp x16, x17, [sp, #128]
	stp x18, x29, [sp, #144]
	mrs x0, ELR_EL1
	stp x30, x0, [sp, #160]
	mrs x0, ESR_EL1
	mrs x1, FAR_EL1
	stp x0, x1, [sp, #176]
	mov x0, sp
.endm

.macro restore_regs
	ldp x0, x1, [sp, #0]
    ldp x2, x3, [sp, #16]
    ldp x4, x5, [sp, #32]
    ldp x6, x7, [sp, #48]
    ldp x8, x9, [sp, #64]
    ldp x10, x11, [sp, #80]
    ldp x12, x13, [sp, #96]
    ldp x14, x15, [sp, #112]
    ldp x16, x17, [sp, #128]
    ldp x18, x29, [sp, #144]
    ldp x30, xzr, [sp, #160]
    add sp, sp, #192
.endm

.section .text
.balign 0x800
vector_table_el1:
curr_el_sp0_sync:        // The exception handler for a synchronous 
                         // exception from the current EL using SP0.
    save_regs
    bl unhandled_exception_vector
    restore_regs
    eret

.balign 0x80
curr_el_sp0_irq:         // The exception handler for an IRQ exception
                         // from the current EL using SP0.
    save_regs
    bl unhandled_exception_vector
    restore_regs
    eret

.balign 0x80
curr_el_sp0_fiq:         // The exception handler for an FIQ exception
                         // from the current EL using SP0.
    save_regs
    bl unhandled_exception_vector
    restore_regs
    eret

.balign 0x80
curr_el_sp0_serror:      // The exception handler for a System Error 
                         // exception from the current EL using SP0.
    save_regs
    bl unhandled_exception_vector
    restore_regs
    eret

.balign 0x80
curr_el_spx_sync:        // The exception handler for a synchrous 
                         // exception from the current EL using the
                         // current SP.
    save_regs
    bl exception_handler
    restore_regs
    eret



.balign 0x80
curr_el_spx_irq:         // The exception handler for an IRQ exception from 
                         // the current EL using the current SP.
    save_regs
    bl irq_handler
    restore_regs
    eret

.balign 0x80
curr_el_spx_fiq:         // The exception handler for an FIQ from 
                         // the current EL using the current SP.
    save_regs
    bl unhandled_exception_vector
    restore_regs
    eret

.balign 0x80
curr_el_spx_serror:      // The exception handler for a System Error 
                         // exception from the current EL using the
                         // current SP.
    save_regs
    bl unhandled_exception_vector
    restore_regs
    eret

 .balign 0x80
lower_el_aarch64_sync:   // The exception handler for a synchronous 
                         // exception from a lower EL (AArch64).
    save_regs
    bl exception_handler
    restore_regs
    eret

.balign 0x80
lower_el_aarch64_irq:    // The exception handler for an IRQ from a lower EL
                         // (AArch64).
    save_regs
    bl unhandled_exception_vector
    restore_regs
    eret

.balign 0x80
lower_el_aarch64_fiq:    // The exception handler for an FIQ from a lower EL
                         // (AArch64).
    save_regs
    bl unhandled_exception_vector
    restore_regs
    eret

.balign 0x80
lower_el_aarch64_serror: // The exception handler for a System Error 
                         // exception from a lower EL(AArch64).
    save_regs
    bl unhandled_exception_vector
    restore_regs
    eret

.balign 0x80
lower_el_aarch32_sync:   // The exception handler for a synchronous 
                         // exception from a lower EL(AArch32).
    save_regs
    bl unhandled_exception_vector
    restore_regs
    eret

.balign 0x80
lower_el_aarch32_irq:    // The exception handler for an IRQ exception 
                         // from a lower EL (AArch32).
    save_regs
    bl unhandled_exception_vector
    restore_regs
    eret

.balign 0x80
lower_el_aarch32_fiq:    // The exception handler for an FIQ exception from 
                         // a lower EL (AArch32).
    save_regs
    bl unhandled_exception_vector
    restore_regs
    eret

.balign 0x80
lower_el_aarch32_serror: // The exception handler for a System Error
                         // exception from a lower EL(AArch32).
    save_regs
    bl unhandled_exception_vector
    restore_regs
    eret

