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
    pub trait UnsignedInt: Copy + Default{}
    impl UnsignedInt for u8 {}
    impl UnsignedInt for u16 {}
    impl UnsignedInt for u32 {}
    impl UnsignedInt for u64 {}

    pub fn memzero<T: UnsignedInt>(start_ptr: *mut T, bytes: usize){
        for i in 0..(bytes / size_of::<T>()){
            unsafe{ *(start_ptr.add(i)) = T::default(); };
        }
    }
    /// this function smartly calculating the shortest way
    /// slowly than memzero_step8 but gives guarantees
    pub fn memzero_smart(mut start_ptr: *mut u8, bytes: usize){
        let bytes_8 = bytes / 8 * 8;
        let bytes_4 = (bytes - bytes_8) / 4 * 4;
        let bytes_2 = (bytes - bytes_8 - bytes_4) / 2 * 2;
        let bytes_1 = bytes - bytes_8 - bytes_4 - bytes_2;

        memzero::<u64>(start_ptr as *mut u64, bytes_8);
        unsafe { start_ptr = start_ptr.add(bytes_8); };

        memzero::<u32>(start_ptr as *mut u32, bytes_4);
        unsafe { start_ptr = start_ptr.add(bytes_4); };

        memzero::<u16>(start_ptr as *mut u16, bytes_2);
        unsafe { start_ptr = start_ptr.add(bytes_2); };

        memzero::<u8>(start_ptr, bytes_1);
    }
}
// in this case use 1 function because in 32 bit mode works only 
// loader and for him 1 function will be enough, and 
// because project is compilated as static rust library
// need to save loader memory
#[cfg(target_pointer_width = "32")]
pub mod mem32{
    pub fn memzero_step4(start_addr: usize, end_addr: usize) -> super::Result {
        if bytes % 4 != 0 { super::Result::Err }
        else {
            for i in 0..(bytes / 4){
                unsafe { *(start_ptr.add(i)) = 0; };
            }
            super::Result::Ok
        }
    }
}
