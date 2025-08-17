#![no_std]
#![no_main]
#![allow(dead_code)]
#![allow(unused_macros)]
#![cfg(target_pointer_width = "64")]
// kernel of OS

use paging::{ADDRESSES_IN_PD, DISABLE_CACHE, GLOBAL, PAGE_SIZE,
    PDS_IN_PDPTE, PML4, PRESENT, USER_ACCESS, WRITABLE, WRITE_THROUGH};
use io::{Colors, print};

const SERIAL_COM1_BASE: u16 = 0x3F80;

macro_rules! GET_SERIAL_DATA_PORT { ( $base: expr ) => { $base }; }
macro_rules! GET_SERIAL_FIFO_COMMAND_PORT { ( $base: expr ) => { $base + 2 }; }
macro_rules! GET_SERIAL_LINE_COMMAND_PORT { ( $base: expr ) => { $base + 3 }; }
macro_rules! GET_SERIAL_MODEM_COMMAND_PORT { ( $base: expr ) => { $base + 4 }; }
macro_rules! GET_SERIAL_LINE_STATUS_PORT { ( $base: expr ) => { $base + 5 }; }

const SERIAL_PORT_ENABLE_DOUBLE_SEND: u8 = 0x80;
const SERIAL_PORT_SETTINGS: u8 = 0x03;
// 7| 0 |6| 0 |5| 000 |2| 0 |1| 11 |0|
// 0,1: 	8 bit data
// 2: 	number of stop bytes
// 3,4,5:number of parity
// 6:	break control
// 7:	access byte
const FIFO_ENABLE: u8 = 0xC7;
const MODEM_READY_STATUS: u8 = 0x03;

macro_rules! hlt {
    () => { unsafe { core::arch::asm!("hlt"); } }
}

// static mut IDT: ints::IntDescrTable64 = MaybeUninit::uninit().assume_init();
#[unsafe(link_section = "kernel.kernel")]
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    PML4::set_zeroes();
    PML4::init();
    let pml4 = PML4::new();
    
    pml4.set(0);
    let pdpte = pml4.get(0);
    let mut current_addr: usize = 0;

    // identity mapping (virt mem = phys mem) for first 2mb(pre-kernel memory)
    pdpte.set(0);
    let first_pd = pdpte.get(0);
    first_pd.set(0, current_addr, PRESENT | WRITABLE | 
        WRITE_THROUGH | DISABLE_CACHE | PAGE_SIZE | GLOBAL);
    
    current_addr += 0x400_000;

    // init all remaining pages in first PD
    for i in 1..ADDRESSES_IN_PD{
        first_pd.set(i, current_addr, PRESENT | WRITABLE | 
            USER_ACCESS | PAGE_SIZE);
        
        current_addr += 0x200_000;
    }

    // init all PD's without first and last
    for i in 1..(PDS_IN_PDPTE - 1) {
        pdpte.set(i);
        for j in 0..ADDRESSES_IN_PD {
            let pd = pdpte.get(i);
            pd.set(j, current_addr, PRESENT | WRITABLE | 
                USER_ACCESS | PAGE_SIZE);

            current_addr += 0x200_000;
        }
    }
    
    // map kernel to up of virt memory
    pdpte.set(PDS_IN_PDPTE - 1);
    let last_pd = pdpte.get(PDS_IN_PDPTE - 1);
    last_pd.set(ADDRESSES_IN_PD - 1, 0x200_000, PRESENT | WRITABLE | WRITE_THROUGH |
            DISABLE_CACHE | PAGE_SIZE | GLOBAL);
    
    for i in 0..(ADDRESSES_IN_PD - 1){
        last_pd.set(i, current_addr, PRESENT | WRITABLE | 
            USER_ACCESS | PAGE_SIZE);
        
        current_addr += 0x200_000;
    }

    pml4.enable_pae(); 
    print(&"A", Colors::BLUE, Colors::RED);
    
    // set up interrupt descriptor table
    // unsafe {
    //     idt.append(0, ints::divide_zero_handler, 1, true);
    //     idt.append(1, ints::debug_handler, 1, true);
    //     idt.append(2, ints::nmi_handler, 1, false);
    //     idt.append(3, ints::breakpoint_handler, 1, true);
    //     idt.append(4, ints::default_handler, 1, true);
    //     idt.append(5, ints::default_handler, 1, false);
    //     idt.append(6, ints::invalid_opcode_handler, 1, false);
    //     idt.append(7, ints::device_not_available_handler, 1, false);
    //     idt.append(8, ints::double_fault_handler, 2, false);
    //     idt.append(9, ints::default_handler, 1, false);
    //     idt.append(10, ints::invalid_tss_handler, 1, false);
    //     idt.append(11, ints::coprocessor_segment_overrun_handler, 2, false);
    //     idt.append(12, ints::infinity_handler, 1, false);
    //     idt.append(13, ints::general_protection_handler, 3, false);
    //     idt.append(14, ints::page_fault_handler, 2, false);
    //     /* 15 reserved */
    //     for i in 0..240{
    //         idt.append(i + 15, ints::default_handler, 3, false);
    //     }
    // }
 
    // unsafe {
    //     idt
    //     static idtr: ints::IDTR = ints::IDTR {
    //         limit: (core::mem::size_of::<ints::IntDescrTable64>() - 1) as u16,
    //         base: &idt as *const _ as u64
    //     };
    //     core::arch::asm!(
    //         "lidt [{}]",
    //         "sti",
    //         in(reg) &idtr,
    //         options(nostack, readonly, preserves_flags)
    //     );
    // };

    // configure serial port baud rate (115200 / 2 bouds)
  //  outb(GET_SERIAL_LINE_COMMAND_PORT!(SERIAL_COM1_BASE),
  //  SERIAL_PORT_ENABLE_DOUBLE_SEND); 
  //  outb(GET_SERIAL_LINE_COMMAND_PORT!(SERIAL_COM1_BASE),
  //  0x00); // high byte
  //  outb(GET_SERIAL_LINE_COMMAND_PORT!(SERIAL_COM1_BASE),
  //  0x02); // low byte (divisor)
  //  
  //  // set serial port settings
  //  outb(GET_SERIAL_LINE_COMMAND_PORT!(SERIAL_COM1_BASE), 
  //  SERIAL_PORT_SETTINGS);
  //  // set serial port FIFO buffers
  //  outb(GET_SERIAL_FIFO_COMMAND_PORT!(SERIAL_COM1_BASE),
  //  FIFO_ENABLE);
  //  // set serial port modem
  //  outb(GET_SERIAL_MODEM_COMMAND_PORT!(SERIAL_COM1_BASE),
  //  MODEM_READY_STATUS);
    
    loop {hlt!()}
}

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> !{
    loop{hlt!()}
}
