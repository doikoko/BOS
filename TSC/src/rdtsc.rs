#[macro_export]
macro_rules! rdtsc {
    ($buf:ident) => {{
        unsafe {
            core::arch::asm!(
                "rdtsc",
                "mov [{0}], eax",
                "mov [{0} + 4], edx",
                in(reg) &$buf,
                out("eax") _,
                out("edx") _,
            );
        }
    }};
}