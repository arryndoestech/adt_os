#![no_std]
#![no_main]
#![feature(asm_const)]

use core::arch::global_asm;
use crate::serial::*;

mod serial;
mod panic;
#[macro_use]
mod print;

#[no_mangle]
#[link_section = ".stack"]
static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
const STACK_SIZE: usize = 1024;
const SERIAL_ADDR: *mut u8 = 0x0900_0000 as *mut u8;

global_asm!(include_str!("boot.s"), sym STACK, const STACK_SIZE);

#[no_mangle]
fn main() {
    kprintln!("Hello world!");
    kprintln!("It works!");
    panic!("Panic test");
    loop{}
}
