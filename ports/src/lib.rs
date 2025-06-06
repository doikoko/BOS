#![no_std]

pub mod ports{
    unsafe extern "C"{
        // write data to port
        fn _outp(port: u16, data: u8);
        // read data from port
        fn _inp(port: u16) -> u16;
        
        
    }
    pub fn outp(port: u16, data: u8){
        unsafe {
            _outp(port, data);
        }
    }
    pub fn inp(port: u16) -> u16{
        unsafe {
            _inp(port)
        }
    }

}