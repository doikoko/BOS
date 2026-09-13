use crate::SWITCH_TO_64_BIT;

pub fn switch_to_64_bit_jmp_to_kernel() -> ! {
    unsafe {
        core::arch::asm!(
            "jmp {0}",
            in(reg) SWITCH_TO_64_BIT,
            options(noreturn)
        );
    }
}