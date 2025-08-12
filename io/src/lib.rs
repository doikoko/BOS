#![no_std]

use ports::outb;

#[derive(Clone, Copy)]
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

const FRAMEBUFFER: *mut u8 = 0xB8000 as *mut u8;
macro_rules! print_char {
    ($symb: expr/*&u8*/, $fg: expr/*u8*/, $bg: expr/*u8*/) => {
        unsafe{
            *FRAMEBUFFER = *$symb;
            *FRAMEBUFFER.add(1) = (($fg as u8) << 4) 
                | (($bg as u8) & 0x0F);        
        }
    };
}
#[inline(always)]
pub fn print(string: &str, fg: Colors, bg: Colors){
    for symb in string.as_bytes(){
        print_char!(symb, fg as u8, bg as u8);
    }
}

const FB_COMMAND_PORT: u16 = 0x3D4;
const FB_DATA_PORT: u16 = 0x3D5;

const FB_HIGH_BYTE_COMMAND: u8 = 14;
const FB_LOW_BYTE_COMMAND: u8 = 15;

#[inline(always)]
pub fn move_cursor(pos: u16){
    outb(FB_COMMAND_PORT, FB_HIGH_BYTE_COMMAND);
    outb(FB_DATA_PORT, (pos >> 8) as u8);
    outb(FB_COMMAND_PORT, FB_LOW_BYTE_COMMAND);
    outb(FB_DATA_PORT, pos as u8);
}