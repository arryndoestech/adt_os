use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kprintln!(
        "Panicked at {}:\r\n{}",
        info.location().unwrap(),
        info.message()
    );
    loop{}
}
