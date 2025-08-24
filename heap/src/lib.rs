#![no_std]
#![allow(dead_code)]

#[cfg(target_pointer_width = "64")]
pub mod mem64{
    use core::ops::{Index, IndexMut};

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
    
    unsafe extern "C"{
        fn make_arr(arr: *mut u8, len: u8);
    }
    pub struct UnsafeArr<T>(pub *mut T);
    impl<T> UnsafeArr<T>{
        pub fn new(len: u8) -> Self{
            let arr: *mut u8 = 0 as *mut u8;
            unsafe { 
                make_arr(arr, len * (size_of::<T>() as u8));
                UnsafeArr(arr as *mut T)
            }
        }
    }
    impl<T> Index<usize> for UnsafeArr<T> {
        type Output = T;

        fn index(&self, index: usize) -> &Self::Output {
            unsafe { &(*(self.0.add(index))) }
        }
    }
    impl<T> IndexMut<usize> for UnsafeArr<T> {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            unsafe { &mut (*(self.0.add(index))) }
        } 
    }


}
// in this case use 1 function because in 32 bit mode works only 
// loader and for him 1 function will be enough, and 
// because project is compilated as static rust library
// need to save loader memory
#[cfg(target_pointer_width = "32")]
pub mod mem32{
    pub trait UnsignedInt: Copy + Default{}
    impl UnsignedInt for u8 {}
    impl UnsignedInt for u16 {}
    impl UnsignedInt for u32 {}

    pub fn memzero<T: UnsignedInt>(start_ptr: *mut T, bytes: usize){
        for i in 0..(bytes / size_of::<T>()){
            unsafe{ *(start_ptr.add(i)) = T::default(); };
        }
    }
}
