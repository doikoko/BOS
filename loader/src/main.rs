#![no_std]
#![no_main]

use io::io::{print, Colors};

macro_rules! hlt {
    () => {
        unsafe {core::arch::asm!("hlt")}
    };
}

const kernel_func_addr: usize = 0x200_000;

#[unsafe(link_section = ".loader.loader")]
#[unsafe(no_mangle)]
pub extern "C" fn loader() {
    
    let kernel_func = kernel_func_addr as *const extern "C" fn() -> !;
    unsafe { (*kernel_func)(); }
}

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo, msg: &str) -> !{
    print(msg, Colors::RED, Colors::BLACK);
    loop{hlt!()};
}
