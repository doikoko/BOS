#![no_std]

pub enum Result {
    Ok,
    Err
}
impl Result {
    pub fn unwrap(&self){
        if let Result::Err = self { panic!() };
    }
}
// in this case use 4 functions to any alines of data
// Example: need to zeroed [u8, 3]
// to this we call 3 times memzero_step1 func
#[cfg(target_pointer_width = "64")]
pub mod mem64{
    /// this function work with odd and even sizes but slow,
    /// because any size / 1 = int (11/1 = 11, not float)
    pub fn memzero_step1(start_ptr: *mut u8, bytes: usize){
        for i in 0..bytes{
            unsafe { *(start_ptr.add(i)) = 0; };
        }
    }
    /// this function with only even sizes and faster than 
    /// memzero_step1 (11/2 = 5.5, one byte is not changed)
    pub fn memzero_step2(start_ptr: *mut u16, bytes: usize) -> super::Result{
        if bytes % 2 != 0 { super::Result::Err }
        else {
            for i in 0..(bytes / 2){
                unsafe { *(start_ptr.add(i)) = 0; };
            }
            super::Result::Ok
        }
    }
    /// this function with only multiples of 4 sizes and faster than 
    /// memzero_step2 (18/4 = 4.5, any bytes is not changed)
    pub fn memzero_step4(start_ptr: *mut u32, bytes: usize) -> super::Result{
        if bytes % 4 != 0 { super::Result::Err }
        else {
            for i in 0..(bytes / 4){
                unsafe { *(start_ptr.add(i)) = 0; };
            }
            super::Result::Ok
        }
    }
    /// this function with only multiples of 8 sizes and faster than 
    /// memzero_step4 (18/8 = 2.25, any bytes is not changed)
    pub fn memzero_step8(start_ptr: *mut u64, bytes: usize) -> super::Result{
        if bytes % 8 != 0 { super::Result::Err }
        else {
            for i in 0..(bytes / 8){
                unsafe { *(start_ptr.add(i)) = 0; };
            }
            super::Result::Ok
        }
    }
    /// this function smartly calculating the shortest way
    /// slowly than memzero_step8 but gives guarantees
    pub fn memzero_smart(mut start_ptr: *mut u8, bytes: usize){
        let bytes_8 = bytes / 8 * 8;
        let bytes_4 = (bytes - bytes_8) / 4 * 4;
        let bytes_2 = (bytes - bytes_8 - bytes_4) / 2 * 2;
        let bytes_1 = bytes - bytes_8 - bytes_4 - bytes_2;

        memzero_step8(start_ptr as *mut u64, bytes_8);
        unsafe { start_ptr = start_ptr.add(bytes_8); };

        memzero_step4(start_ptr as *mut u32, bytes_4);
        unsafe { start_ptr = start_ptr.add(bytes_4); };

        memzero_step2(start_ptr as *mut u16, bytes_2);
        unsafe { start_ptr = start_ptr.add(bytes_2); };
        
        memzero_step1(start_ptr, bytes_1);
    }
}
// in this case use 1 function because in 32 bit mode works only 
// loader and for him 1 function will be enough, and 
// because project is compilated as static rust library
// need to save loader memory
#[cfg(target_pointer_width = "32")]
pub mod mem32{
    pub fn memzero_step4(start_addr: usize, end_addr: usize){
        let ptr = start_addr as *mut u32;
        unsafe {
            for i in (0..(end_addr - start_addr)).step_by(4){
                *(ptr.add(i)) = 0;
            }
        }
    }
}
