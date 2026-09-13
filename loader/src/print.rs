// because in 32 bit mode call convention is other need to 
// call function as in other parts of code
#[macro_export]
macro_rules! print {
    ($arg : expr) => {
        unsafe {
            core::arch::asm!(
                "push eax",
                "push ebx",
                "push edi",
                
                "mov edi, {0}",
                "call {1}",

                "pop edi",
                "pop ebx",
                "pop eax",
                in(reg) $arg.as_bytes().as_ptr(),
                in(reg) PRINT32_ADDR,
            )
        };
    };
}
