#![no_std]
pub mod ports{
    unsafe extern "C"{
        // write data to port
        fn _outp(port: u8, data: u16);
        // read data from port
        fn _inp(port: u8) -> u16;
        
        
    }
    pub fn outp(port: u8, data: u16){
        unsafe {
            _outp(port, data);
        }
    }
    pub fn inp(port: u8) -> u16{
        unsafe {
            _inp(port)
        }
    }

}