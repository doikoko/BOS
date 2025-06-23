#![no_std]
#![no_main]

// kernel of OS
extern crate io;
extern crate ports;
extern crate interrupts;

//use ports::ports::outb;
use interrupts::ints;

const SERIAL_COM1_BASE: u16 = 0x3F80;

#[allow(unused_macros)]
macro_rules! GET_SERIAL_DATA_PORT { ( $base: expr ) => { $base }; }
macro_rules! GET_SERIAL_FIFO_COMMAND_PORT { ( $base: expr ) => { $base + 2 }; }
macro_rules! GET_SERIAL_LINE_COMMAND_PORT { ( $base: expr ) => { $base + 3 }; }
macro_rules! GET_SERIAL_MODEM_COMMAND_PORT { ( $base: expr ) => { $base + 4 }; }
#[allow(unused_macros)]
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
const KERNEL_STACK_SIZE: usize = 0x1000;

macro_rules! hlt {
    () => { unsafe { core::arch::asm!("hlt"); } }
}

#[allow(dead_code)]
#[unsafe(link_section = ".bss")] 
static mut KM: [u8; KERNEL_STACK_SIZE] = 
[0; KERNEL_STACK_SIZE];

// static mut IDT: ints::IntDescrTable64 = MaybeUninit::uninit().assume_init();
#[no_mangle]
pub extern "C" fn _start() -> ! {

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