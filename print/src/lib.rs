#![no_std]

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

pub const MAX_COLUMN: u16 = 80;
pub const MAX_ROW: u16 = 25;
pub fn print(string: &str, fg: Colors, bg: Colors, pos: &mut usize){
    if *pos > (MAX_COLUMN * MAX_ROW) as usize{ return; };
    for symb in string.as_bytes(){
        unsafe{
            *(FRAMEBUFFER.add(*pos)) = *symb;
            *(FRAMEBUFFER.add(*pos).add(1)) = ((fg as u8) << 4) 
                | ((bg as u8) & 0x0F);
            *pos += 2;
        };
    }
}
pub fn itos(mut num: i32, buf: &mut [u8]) -> &str{
    buf
        .iter_mut()
        .rev()
        .for_each(|digit| {
            *digit = (num % 10) as u8 + 0x30;
            num /= 10; 
        });
    if num < 0 { buf[0] = b'-' };
    "hello"
}