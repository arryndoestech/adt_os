#![no_std]
#![no_main]
#![feature(asm_const)]

use core::arch::{global_asm, asm};
use crate::serial::*;
use crate::psci::*;

#[macro_use]
mod print;
#[macro_use]
mod platform;
mod serial;
mod panic;
mod psci;

#[no_mangle]
#[link_section = ".stack"]
static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
const STACK_SIZE: usize = 1024;
const SERIAL_ADDR: *mut u8 = 0x0900_0000 as *mut u8;

global_asm!(include_str!("boot.s"), sym STACK, const STACK_SIZE);

#[allow(improper_ctypes)]
extern "C" {
    static _start: fn();
}

fn cpu_id() -> u64 {
    unsafe {
        read_msr!("tpidr_el1")
    }
}

#[no_mangle]
fn main() {
    if cpu_id() == 0 {
        unsafe {
            for core_id in 1..4 {
               cpu_init(core_id, &_start);
            }
        }
    } else {
        kprintln!("Hello from core: {}", cpu_id());
    }
    loop{}
}
