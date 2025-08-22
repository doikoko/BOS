#![no_std]
#![no_main]
#![allow(unreachable_code)]
#![cfg(target_pointer_width = "32")]

use atapi::*;
use paging::{paging32::*, DISABLE_CACHE, GLOBAL, PAGE_SIZE, PRESENT, WRITABLE, WRITE_THROUGH};

const KERNEL_FUNC_ADDR: usize = 0x200_000;

const KERNEL_START_ADDR: usize = 0x200_000;
const KERNEL_SIZE: usize = 0x200_000;
const KERNEL_START_ADDR_IN_ISO: usize = 0x32_000;

const SECTOR_SIZE: usize = 2048;

const KERNEL_SECTORS_PER_ITERATION: usize = 256;
const KERNEL_SECTORS_IN_KERNEL: usize = KERNEL_SIZE / SECTOR_SIZE;
const COMMAND_ITERATIONS: usize = KERNEL_SECTORS_IN_KERNEL / KERNEL_SECTORS_PER_ITERATION;

const MEMORY_PER_ITERATION: usize = KERNEL_SECTORS_PER_ITERATION * SECTOR_SIZE;
macro_rules! hlt {
    () => {
        unsafe {core::arch::asm!("hlt")}
    };
}

trait ReadLba{
    // maximum sectors - 256 (sectors = 0), 1 sectors to CD-rom is 2048 bytes
    fn read_pio_lba_28(&self, sectors: u8, lba: usize, buffer: *mut u16);
}
impl ReadLba for ATAPI{
    fn read_pio_lba_28(&self, sectors: u8, lba: usize, mut buffer: *mut u16) {
        self.wait_drq_and_busy().unwrap();
        
        outb(self.io_registers.sector_count_rw_w, sectors);
        
        // low bytes
        outb(self.io_registers.lba_low_rw_w, lba as u8);
        // mid bytes
        outb(self.io_registers.lba_mid_rw_w, (lba >> 8) as u8);
        // high bytes
        outb(self.io_registers.lba_high_rw_w, (lba >> 16) as u8);
        // 1 byte
        outb(self.io_registers.device_or_head_rw_b, (lba >> 24 & 0xE0) as u8);
        // send command
        outb(self.io_registers.command_w_or_status_r_b, ATAPIOCommands::ReadSectorsB as u8);
        
        self.clear_cache();
        for _ in 0..(if sectors == 0 {256} else {sectors as u16}){
            self.wait_drq_and_busy().unwrap();
            
            for _ in 0..SECTOR_SIZE / 2{
                unsafe{ 
                    *buffer = inw(self.io_registers.data_register_rw_w); 
                    buffer = buffer.add(1);
                };
            }
        }
        self.wait_drq_and_busy().unwrap();
    }
}

// because in 32 bit mode call convention is other need to 
// call function as in other parts of code
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

// this function defined in loader.asm
// and address to this func contains in rdi register(passed as argument from asm)
static mut PRINT32_ADDR: usize = 0;
// GDT64 containing in loader.asm, through that
// address of GDT64 passed to 'loader' function (below) by esi reg like argument
static mut GDT64_ADDR: usize = 0;

#[unsafe(link_section = ".loader.loader")]
#[unsafe(no_mangle)]
extern "C" fn loader(/* PINT32_ADDR: usize, GDT64_ADDR: usize */) {
    unsafe{
        core::arch::asm!(
            "mov {}, edi",
            "mov {}, esi",
            out(reg) PRINT32_ADDR,
            out(reg) GDT64_ADDR,
            options(nostack)
        );
    }
    
    print!("init PML4 in 32 bit mode / \0");
    // init minimal PML4 for 32 bit
    let pd = PD::new();
    
    const FIRST_ADDR: usize = 0x0;
    const LAST_ADDR: usize = 0x400_000;
    for i in (FIRST_ADDR..LAST_ADDR).step_by(0x200_000){
        pd.set(0, i, PRESENT | WRITABLE | WRITE_THROUGH |
            DISABLE_CACHE | PAGE_SIZE | GLOBAL);
    }
    
    print!("load kernel to 0x200_000 / \0");
    let atapi = ATAPI::new(PrimaryOrSecondary::Primary);
    
    if !atapi.is_has_device(){
        panic!();
    }
    atapi.wait_drq_and_busy().unwrap();
    atapi.set_dma_or_pio(DMAOrPIO::PIO);
    atapi.set_flags(MasterOrSlave::Master, LBAOrCHS::LBA);
    
    for i in 0..COMMAND_ITERATIONS{
        atapi.read_pio_lba_28(
                // 0 because if we send it it will be 256(max value)
                0, KERNEL_START_ADDR_IN_ISO as usize + (i * MEMORY_PER_ITERATION),
                (KERNEL_START_ADDR as usize + (i * MEMORY_PER_ITERATION)) as *mut u16);
    }

    loop{hlt!()};
    let kernel_func: extern "C" fn() -> ! = unsafe {
        core::mem::transmute(KERNEL_FUNC_ADDR)
    };
    kernel_func();
}

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> !{
    print!("PANIC! file: loader/src/main.rs\0");

    loop{hlt!()};
}
