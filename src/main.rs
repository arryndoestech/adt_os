#![no_std]
#![no_main]
#![feature(asm_const)]

use core::panic::PanicInfo;
use core::arch::global_asm;
use crate::serial::*;

mod serial;

#[no_mangle]
#[link_section = ".stack"]
static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
const STACK_SIZE: usize = 1024;

global_asm!(include_str!("boot.s"), sym STACK, const STACK_SIZE);

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_puts("PANIC!!!");
    loop{}
}

#[no_mangle]
fn main() {
    serial_puts("Hello world!");
    panic!();
    loop{}
}
