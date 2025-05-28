#![no_std]

use io;
use ports;

// kernel of OS

const SERIAL_PORT: u16 = 0x3F80;
const SERIAL_PORT_SETTINGS: u8 = 0x03;
// 7| 0 |6| 0 |5| 000 |2| 0 |1| 11 |0|
// 0,1: 	8 bit data
// 2: 	number of stop bytes
// 3,4,5:number of parity
// 6:	break control
// 7:	access byte
const FIFO_ENABLE: u8 = 0xC7;
const MODEM_READY_STATUS: u8 = 0x03;
const KERNEL_STACK_SIZE: u16 = 0x1000;


#[link_section = ".bss"] 
static mut KM: [u8; KERNEL_STACK_SIZE as usize] = 
    [0; KERNEL_STACK_SIZE as usize];
    
fn main() {
    
}
