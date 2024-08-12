use core::arch::asm;
const CPU_ON_FID: u32 = 0xC400_0003;

pub unsafe fn cpu_init(cpu_id: usize, entry_point: *const fn()) {
    let return_code: i32;
    asm!("hvc 0",
        inout("x0") CPU_ON_FID => return_code,
        in("x1") cpu_id,
        in("x2") entry_point,
        in("x3") cpu_id,
        options(nomem, nostack)
        );
    if return_code != 0 {
        kprintln!("Couldn't init cpu {}: {}", cpu_id, return_code);
    }
}
