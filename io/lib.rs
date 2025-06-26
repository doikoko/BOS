#![no_std]

pub mod io{
    #[repr(u8)]
    pub enum Colors{
        BLACK    = 0, 
        BLUE     = 1,
        GREEN    = 2,
        CYAN     = 3,
        RED      = 4,
        MAGENTA  = 5,
        BROWN    = 6,
        LGREY    = 7,
        DGREY    = 8,
        LBLUE    = 9,
        LGREEN   = 10,
        LCYAN    = 11,
        LRED     = 12,
        LMAGENTA = 13,
        LBROWN   = 14,
        WHITE    = 15
    }
    unsafe extern "C"{
        fn _print_c();
        fn _print_s(buf: *const u8, fg: u8, bg: u8, len: u8);
        fn _move_cursor(row: u8, column: u8);
    }
    pub fn print_c(){
        unsafe {
            _print_c();
        }
    }
    pub fn print_s(buf: &[u8], fg: Colors, bg: Colors, len: u8){
        unsafe {
            let buf: *const u8 = buf.as_ptr();
            _print_s(buf, fg as u8, bg as u8, len);
        }
    }
    pub fn move_cursor(row: u8, column: u8){
        unsafe {
            _move_cursor(row, column);
        }
    }
}