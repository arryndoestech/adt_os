#[macro_export]
macro_rules! kprint {
    ($($arg:tt)*) => {
        {
            use core::fmt::Write;
            let mut console = crate::SERIAL_CONSOLE.acquire();
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
