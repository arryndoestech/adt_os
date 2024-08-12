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
