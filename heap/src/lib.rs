#![no_std]
#![allow(dead_code)]

pub mod _box;
pub mod vec;

use result::Result;

const HEAP_FIRST_ADDR: usize = 0x400000;
const HEAP_MEMORY_FIRST_ADDR: usize = 0x400000 + HEAP_TABLE_SIZE;
const HEAP_LAST_ADDR: usize = 0x600000;
const HEAP_MEMORY_SIZE: usize = HEAP_LAST_ADDR - HEAP_FIRST_ADDR - HEAP_TABLE_SIZE;

const HEAP_TABLE_SIZE: usize = 0x4000;
const HEAP_TABLE_FIRST_ADDR: usize = HEAP_FIRST_ADDR;

// 1bit management 0x10 bytes of heap
const HEAP_PACKET_SIZE: u8 = 0x10;

unsafe extern "C" {
    fn _malloc(bytes: u32) -> *mut ();
    fn _free(ptr: *mut (), bytes: u32) -> u8;
}
#[repr(C)]
struct Heap{
    heap_table: [u8; HEAP_TABLE_SIZE],
    heap_memory: [u8; HEAP_MEMORY_SIZE]
}
impl<'a> Heap{
    pub fn new() -> &'a mut Self{
        let ptr = HEAP_TABLE_FIRST_ADDR as *mut Heap;
        unsafe {
            ptr.write(Heap {
                heap_table: [0u8; HEAP_TABLE_SIZE], 
                heap_memory: [0u8; HEAP_MEMORY_SIZE] 
            });

            &mut *ptr
        }
    }
    pub(crate) fn malloc(len: u32) -> Option<*mut ()>{
        let ptr = unsafe{ 
            _malloc(len) as *mut u8
        };
        if ptr.is_null(){
            None
        }
        else {
            Some(ptr as *mut ())
        }
    }
    pub(crate) fn free(ptr: *mut (), len: u32) -> Result{
        unsafe{
            if _free(ptr, len) == 0{
                Result::Ok
            }
            else {
                Result::Err
            }
        }
    }
    pub(crate) fn realloc(ptr: *mut (), source_len: u32, res_len: u32) -> Option<*mut ()>{
        if (source_len % HEAP_PACKET_SIZE as u32) == (res_len % HEAP_PACKET_SIZE as u32){
            Some(ptr)
        } else {
            let data: *mut (); 
            if let Some(mem) = Self::malloc(res_len){
                data = mem;
                for i in 0..(source_len as usize){
                    unsafe { 
                        (data as *mut u8)
                            .add(i)
                            .write_unaligned(
                                (ptr as *mut u8)
                                    .add(i)
                                    .read_unaligned()
                                ) 
                    }
                }
                Self::drop(ptr, source_len, "Realloc panicked");
                Some(data)
            }
            else{
                Self::drop(ptr, source_len, "Realloc panicked");
                None
            }
        }
    }
    pub(crate) fn drop(ptr: *mut (), len: u32, msg: &str){
        if let Result::Err = Heap::free(ptr, len){
            panic!("{}", msg);
        }
    }
}