#![no_std]
#![no_main]
#![feature(asm_const)]

use core::arch::{global_asm, asm};
use crate::serial::*;
use crate::psci::*;
use crate::mutex::Mutex;
use crate::memory_map::*;
use crate::platform::*;

#[macro_use]
mod print;
#[macro_use]
mod platform;
mod serial;
mod panic;
mod psci;
mod mutex;
mod exception;
mod memory_map;

#[no_mangle]
#[link_section = ".stack"]
static mut STACK: [u8; STACK_SIZE*NUM_PROCS] = [0; STACK_SIZE*NUM_PROCS];
const STACK_SIZE: usize = 1024*32;
const NUM_PROCS: usize = 4;

global_asm!(include_str!("boot.s"), sym STACK, const STACK_SIZE);
global_asm!(include_str!("exception.s"));

#[allow(improper_ctypes)]
extern "C" {
    static _common_start: fn();
}

fn cpu_id() -> u64 {
    unsafe {
        read_msr!("tpidr_el1")
    }
}

static PRINT_LOCK: Mutex<u8> = Mutex::new(0);
static SERIAL_CONSOLE: Mutex<Serial> = Mutex::new(
    Serial::new(SERIAL_ADDR)
);

#[no_mangle]
fn main() {
    if cpu_id() == 0 {
        unsafe {
            for core_id in 1..4 {
               cpu_init(core_id, &_common_start);
            }
            asm!("svc #0");
        }
        kprintln!("Timer freq: {}", get_system_timer_freq());
        loop {
            kprintln!("Current timer: {}", get_system_timer());
            delay_s(1);
        }
    }
        kprintln!("Hello from core: {}", cpu_id());
        loop{
            wait_for_event();
        }
}
