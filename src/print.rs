#[macro_export]
macro_rules! kprint {
    ($($arg:tt)*) => {
        {
            use core::fmt::Write;
            let mut console = crate::serial::Serial::new(0x0900_0000 as *mut u8);
            let _ = console.write_fmt(format_args!($($arg)*));
        }
    }
}

#[macro_export]
macro_rules! kprintln {
    () => {
        $crate::kprint!("\n\r");
    };
    ($($arg:tt)*) => {
        $crate::kprint!("{}{}", format_args!($($arg)*), "\n\r");
    }
}
