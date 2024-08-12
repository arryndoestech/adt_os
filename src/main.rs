#![no_std]
#![no_main]
#![feature(asm_const)]

use core::panic::PanicInfo;
use core::arch::global_asm;

#[no_mangle]
#[link_section = ".stack"]
static mut STACK: [u8; 1024] = [0; 1024];
const STACK_SIZE: usize = 1024;

global_asm!(include_str!("boot.s"), sym STACK, const STACK_SIZE);

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_puts("PANIC!!!");
    loop{}
}

fn serial_puts(s: &str) {
    for c in s.chars() {
        serial_putchar(c);
    }
}

fn serial_putchar(c: char) {
    let uart_addr = 0x0900_0000 as *mut u8;
    unsafe{
        *uart_addr = c as u8;
    }
}

#[no_mangle]
fn main() {
    serial_puts("Hello world!");
    loop{}
}
