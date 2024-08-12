use core::arch::global_asm;

#[derive(Debug)]
#[repr(C)]
pub struct ExceptionFrame {
    x0: u64,
    x1: u64,
    x2: u64,
    x3: u64,
    x4: u64,
    x5: u64,
    x6: u64,
    x7: u64,
    x8: u64,
    x9: u64,
    x10: u64,
    x11: u64,
    x12: u64,
    x13: u64,
    x14: u64,
    x15: u64,
    x16: u64,
    x17: u64,
    x18: u64,
    fp: u64,
    lr: u64,
    elr: u64,
    esr: u64,
    far: u64,
}


#[no_mangle]
#[inline(always)]
pub extern "C" fn exception_handler(frame: *mut ExceptionFrame) {
    unsafe { exception(&mut *frame) }
}

#[no_mangle]
pub extern "C" fn irq_handler(frame: *mut ExceptionFrame) {
    kprintln!("Got IRQ!");
}

#[no_mangle]
pub extern "C" fn unhandled_exception_vector(frame: *mut ExceptionFrame) {
    kprintln!("Got unhandled exception!");
}

fn exception(frame: &mut ExceptionFrame) {
    let ec: u8 = (frame.esr >> 26 & 0b11_1111) as u8;
    kprintln!("Exception taken: {:#x}", ec);
    kprintln!("{:#x?}", frame);
}

