// 32 bit target

#![no_std]
#![no_main]
#![allow(unreachable_code)]

mod print;
mod long_mode;
mod read_lba;

use bootinfo::BootInfo;
use paging::paging32::setup_pml4;
use TSC::Tsc;
use atapi::{ATAPI, LBAOrCHS, DMAOrPIO, MasterOrSlave, PrimaryOrSecondary};
use read_lba::ReadLba;

use crate::long_mode::switch_to_64_bit_jmp_to_kernel;

const KERNEL_START_ADDR: usize = 0x200_000;
const KERNEL_START_SECTOR: usize = 64;
const KERNEL_SECTORS_TO_READ: u8 = 32;

const SECTOR_SIZE: usize = 2048;
macro_rules! hlt {
    () => {
        unsafe {core::arch::asm!("hlt")}
    };
}

// this function defined in loader.asm
// and address to this func contains in rdi register(passed as argument from asm)
static mut PRINT32_ADDR: usize = 0;
// address of "switch_to_64_bit" function from loader.asm. It useed because after 
// switching this function works with 64 bit registers. It won't work in rust
static mut SWITCH_TO_64_BIT: usize = 0;

#[unsafe(link_section = ".loader.loader")]
#[unsafe(no_mangle)]
extern "C" fn loader(/* PINT32_ADDR: usize, GDT64_ADDR: usize */) {
    unsafe{
        core::arch::asm!(
            "mov {}, edi",
            "mov {}, esi",
            out(reg) PRINT32_ADDR,
            out(reg) SWITCH_TO_64_BIT,
            options(nostack)
        );
    }
    let mut tsc = Tsc::new();
    tsc.init();
    BootInfo::set_tsc(tsc);

    print!("load kernel to 0x200_000 / \0");
    let atapi = ATAPI::new(PrimaryOrSecondary::Secondary);
    atapi.set_flags(MasterOrSlave::Master, LBAOrCHS::LBA);
    if !atapi.is_has_device(){
        panic!();
    }
    atapi.wait_drq_and_busy().unwrap();
    atapi.set_dma_or_pio(DMAOrPIO::PIO);
    atapi.set_flags(MasterOrSlave::Master, LBAOrCHS::LBA);
    
    atapi.read_pio_lba_28(
        KERNEL_SECTORS_TO_READ,
        KERNEL_START_SECTOR,
        KERNEL_START_ADDR as *mut u16
    ).unwrap();

    print!("init PML4 in 32 bit mode / \0");
    setup_pml4();

    switch_to_64_bit_jmp_to_kernel();
}

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> !{
    print!("PANIC! file: loader/src/main.rs / reason: \0");
    print!(
        info
            .message()
            .as_str()
            .as_ref()
            .unwrap()
    );

    loop{hlt!()};
}
