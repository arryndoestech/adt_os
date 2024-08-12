pub fn serial_puts(s: &str) {
    for c in s.chars() {
        serial_putchar(c);
    }
}

pub fn serial_putchar(c: char) {
    let uart_addr = 0x0900_0000 as *mut u8;
    unsafe {
        uart_addr.write_volatile(c as u8);
    }
}

