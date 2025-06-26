#![no_std]

pub mod ports{
    unsafe extern "C"{
        // write data to port
        fn _outb(port: u16, data: u8); /* b - byte, w - word */
        fn _outw(port: u16, data: u16);
        // read data from port
        fn _inb(port: u16) -> u16;
    }
    pub fn outb(port: u16, data: u8){
        unsafe {
            _outb(port, data);
        }
    }
    pub fn inb(port: u16) -> u16{
        unsafe {
            _inb(port)
        }
    }
    pub fn outw(port: u16, data: u16){
        unsafe{
            _outw(port, data);
        }
    }
}