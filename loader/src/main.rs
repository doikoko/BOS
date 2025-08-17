#![no_std]
#![no_main]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![cfg(target_pointer_width = "32")]
// use io::io::{print, Colors};

//const KERNEL_FUNC_ADDR: usize = 0x200_000;
//
//const KERNEL_START_ADDR: usize = 0x200_000;
//const KERNEL_SIZE: usize = 0x200_000;
//const KERNEL_START_ADDR_IN_ISO: usize = 0x32_000;
//
//const SECTOR_SIZE: usize = 512;
//
//const KERNEL_SECTORS_PER_ITERATION: usize = 256;
//const KERNEL_SECTORS_IN_KERNEL: usize = KERNEL_SIZE / SECTOR_SIZE;
//const COMMAND_ITERATIONS: usize = KERNEL_SECTORS_IN_KERNEL / KERNEL_SECTORS_PER_ITERATION;
//
//const MEMORY_PER_ITERATION: usize = KERNEL_SECTORS_PER_ITERATION * SECTOR_SIZE;
macro_rules! hlt {
    () => {
        unsafe {core::arch::asm!("hlt")}
    };
}
//
//trait ReadLba{
//    fn read_pio_lba_48(&self, sectors: u16, lba: usize, buffer: *mut u16);
//}
//impl ReadLba for ATAPI{
//    fn read_pio_lba_48(&self, sectors: u16, lba: usize, mut buffer: *mut u16) {
//        self.wait_drq_and_busy().expect("error while read kernel");
//
//        // high bytes
//        outb(self.io_registers.sector_count_rw_w, (sectors >> 8) as u8);
//        outb(self.io_registers.lba_low_rw_w,  (lba >> 24) as u8);
//        outb(self.io_registers.lba_mid_rw_w,  (lba >> 32) as u8);
//        outb(self.io_registers.lba_high_rw_w, (lba >> 40) as u8);
//
//        // low bytes
//        outb(self.io_registers.sector_count_rw_w, sectors as u8);
//        outb(self.io_registers.lba_low_rw_w,  lba as u8);
//        outb(self.io_registers.lba_mid_rw_w,  (lba >> 8) as u8);
//        outb(self.io_registers.lba_high_rw_w, (lba >> 16) as u8);
//
//        outb(self.io_registers.command_w_or_status_r_b, 
//            ATAPIOCommands::ReadSectorsExtW as u8);
//        
//        self.clear_cache();
//        for _ in 0..sectors{
//            self.wait_drq_and_busy().expect("error while read kernel from register");
//            
//            for _ in 0..SECTOR_SIZE / 2{
//                unsafe{ 
//                    *buffer = inw(self.io_registers.data_register_rw_w); 
//                    buffer = buffer.add(1);
//                };
//            }
//        }
//        self.wait_drq_and_busy().expect("error after read kernel");
//    }
//}

#[unsafe(link_section = ".loader.loader")]
#[unsafe(no_mangle)]
pub extern "C" fn loader() {
    let letter_count: &mut u8;
    unsafe {
        let addr: usize;
        core::arch::asm!(
            "mov {0}, ecx",
            out(reg) addr,
        );
        letter_count = (addr as *mut u8).as_mut().unwrap();
    };
 //   print(&"hello from rust", Colors::WHITE, Colors::BLUE);
    //let atapi = ATAPI::new(PrimaryOrSecondary::Primary);
    //
    //if !atapi.is_has_device(){
    //    panic!("cannot find ATAPI supported divece, os has looped");
    //}
    //atapi.wait_drq_and_busy().expect("error");
    //atapi.set_dma_or_pio(DMAOrPIO::PIO);
    //atapi.set_flags(MasterOrSlave::Master, LBAOrCHS::LBA);
//
    //for i in 0..COMMAND_ITERATIONS{
    //    atapi.read_pio_lba_48(
    //        KERNEL_SECTORS_PER_ITERATION as u16,
    //        KERNEL_START_ADDR_IN_ISO as usize + (i * MEMORY_PER_ITERATION),
    //        (KERNEL_START_ADDR as usize + (i * MEMORY_PER_ITERATION)) as *mut u16);
    //}
//
    //let kernel_func = KERNEL_FUNC_ADDR as *const extern "C" fn() -> !;
    //unsafe { (*kernel_func)(); }
}

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> !{
    if let Some(msg) = info.message().as_str(){
        //print(msg, Colors::RED, Colors::BLACK);
    } else {
        //print("PANIC! file: loader/src/main.rs\n", Colors::RED, Colors::BLACK);
    }

    loop{hlt!()};
}
