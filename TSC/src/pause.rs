pub fn pause(){
    unsafe{
        core::arch::asm!(
            "pause"
        )
    }
}