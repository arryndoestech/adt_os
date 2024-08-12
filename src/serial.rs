use core::fmt::{Result, Write};

#[derive(Debug)]
pub struct Serial {
    uart_addr: *mut u8
}

unsafe impl Send for Serial {}

impl Serial {
    pub const fn new(uart_addr: *mut u8) -> Self {
        Serial {
            uart_addr: uart_addr
        }
    }

    pub fn serial_puts(&mut self, s: &str) {
        for c in s.chars() {
            self.serial_putchar(c);
        }
    }

    pub fn serial_putchar(&mut self, c: char) {
        unsafe {
            self.uart_addr.write_volatile(c as u8);
        }
    }
}

impl Write for Serial {
    fn write_str(&mut self, string: &str) -> Result {
        self.serial_puts(string);
        Ok(())
    }
}
