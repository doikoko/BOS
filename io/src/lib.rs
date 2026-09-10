#![no_std]

// function for write to port 1 byte data
pub fn outb(port: u16, data: u8){
    unsafe {
        core::arch::asm!(
            "out dx, al",
            in("dx") port,
            in("al") data
        )
    }
}
// function for write to port 1 byte data
pub fn outw(port: u16, data: u16){
    unsafe {
        core::arch::asm!(
            "out dx, ax",
            in("dx") port,
            in("ax") data
        )
    }
}// function to get 8 bit data from port
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
// function to get 16 bit data from port
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