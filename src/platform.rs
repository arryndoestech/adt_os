use core::arch::asm;

#[macro_export]
macro_rules! read_msr {
    ($reg_name:literal) => {
        {
            let reg_val: u64;
            asm!(concat!("mrs {}, ", $reg_name), out(reg) reg_val, options(nostack, nomem));
            reg_val
        }
    };
}

#[macro_export]
macro_rules! write_msr {
    ($reg_name:literal, $reg_val:expr) => {
        {
            asm!(concat!("msr ", $reg_name, ", {}"),
                in(reg) $reg_val);
        }
    }
}

pub fn get_system_timer() -> u64 {
    unsafe {
        read_msr!("CNTPCT_EL0")
    }
}

pub fn get_system_timer_freq() -> u64 {
    unsafe {
        read_msr!("CNTFRQ_EL0")
    }
}

pub fn delay_ticks(ticks: u64) {
    let start_time = get_system_timer();
    while get_system_timer() - start_time < ticks {}
}

pub fn delay_us(us: u32) {
    const US_FACTOR: f64 = 0.000_001;
    delay_ticks(((us as u64 * get_system_timer_freq()) as f64 * US_FACTOR) as u64);
}

pub fn delay_ms(ms: u32) {
    const MS_FACTOR: f64 = 0.001;
    delay_ticks(((ms as u64 * get_system_timer_freq()) as f64 * MS_FACTOR) as u64);
}

pub fn delay_s(s: u32) {
    delay_ticks(s as u64 * get_system_timer_freq());
}

pub fn wait_for_interrupt() {
    unsafe {
        asm!("dsb sy", "wfi");
    }
}

pub fn wait_for_event() {
    unsafe {
        asm!("dsb sy", "wfe");
    }
}

pub fn send_event() {
    unsafe {
        asm!("sev");
    }
}

