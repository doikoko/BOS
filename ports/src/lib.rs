#![no_std]

// 2 functions to write in port
pub fn outb(port: u16, data: u8){
    unsafe {
        core::arch::asm!(
            "out dx, al",
            in("dx") port,
            in("al") data
        )
    }
}
pub fn outw(port: u16, data: u16){
    unsafe {
        core::arch::asm!(
            "out dx, ax",
            in("dx") port,
            in("ax") data
        )
    }
}
// 2 functions to read from port
pub fn inb(port: u16) -> u8{
    unsafe {
        let value: u8;
        core::arch::asm!(
            "in al, dx",
            in("dx") port,
            out("al") value
        );
        value
    }
}
pub fn inw(port: u16) -> u16{
    unsafe {
        let value: u16;
        core::arch::asm!(
            "in ax, dx",
            in("dx") port,
            out("ax") value
        );
        value
    }
}