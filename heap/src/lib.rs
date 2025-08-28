#![no_std]
#![allow(dead_code)]

use core::mem::{size_of, MaybeUninit};

use result::Result;

const HEAP_FIRST_ADDR: usize = 0x400000 + HEAP_TABLE_SIZE;
const HEAP_LAST_ADDR: usize = 0x600000;
const HEAP_SIZE: usize = 0x200000 - HEAP_TABLE_SIZE;

const HEAP_TABLE_SIZE: usize = 0x4000;
const HEAP_TABLE_FIRST_ADDR: usize = 0x400000;
const HEAP_TABLE_LAST_ADDR: usize = 0x400000 + HEAP_TABLE_SIZE;

// 1bit management 0x10 bytes of heap
const HEAP_PACKET_SIZE: u8 = 0x10;

unsafe extern "C" {
    fn _malloc(heap: *mut Heap, bytes: u32) -> *mut ();
    fn _free(heap: *mut Heap, ptr: *mut (), bytes: u32) -> u8;
}
#[repr(C)]
struct Heap{
    heap_table: [u8; HEAP_TABLE_SIZE],
    heap_memory: [u8; HEAP_SIZE]
}
impl<'a> Heap{
    pub fn new() -> &'a mut Self{
        let ptr = HEAP_TABLE_FIRST_ADDR as *mut Heap;
        unsafe {
            ptr.write(Heap {
                heap_table: [0u8; HEAP_TABLE_SIZE], 
                heap_memory: [0u8; HEAP_SIZE] 
            });

            &mut *ptr
        }
    }
    pub(crate) fn malloc(len: u32) -> Option<*mut ()>{
        let ptr = unsafe{ 
            _malloc(HEAP_TABLE_FIRST_ADDR as *mut Heap, len) as *mut u8
        };
        if ptr.is_null(){
            None
        }
        else {
            Some( unsafe{ &mut *(ptr as *mut ()) })
        }
    }
    pub(crate) fn free(ptr: *mut (), len: u32) -> Result{
        unsafe{
            if _free(HEAP_TABLE_FIRST_ADDR as *mut Heap, ptr, len) == 0{
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
struct Box<T>(*mut T);
impl<T> Box<T>{
    pub fn new(data: T) -> Self{
        Self (
            match Heap::malloc(size_of::<T>() as u32){
                Some(mem) => {
                    unsafe { (mem as *mut T).write(data); };
                    mem as *mut T
                },
                None => panic!("Box panicked while malloc")
            },
        )
    }
    pub fn get(&self) -> T{
        unsafe { self.0.read() }
    }
    pub fn set(&mut self, data: T) {
        unsafe { self.0.write(data); };
    }
}
impl<T> Drop for Box<T>{
    fn drop(&mut self) {
        if let Result::Err = Heap::free(self.0 as *mut (), size_of::<T>() as u32){
            panic!("Box panicked while drop")
        }
    }
}

#[derive(Copy, Clone)]
struct Vec<T>{
    ptr: *mut T,
    len: u32
}
impl<T> Vec<T>{
    pub fn new(first: T) -> Self{
        Self{
            ptr: match Heap::malloc(size_of::<T>() as u32){
                Some(mem) => {
                    unsafe { (mem as *mut T).write(first); };
                    mem as *mut T
                },
                None => panic!("Box panicked while malloc")
            },
            len: size_of::<T>() as u32
        }
    }
    pub fn push(&mut self, data: T){
        let size = size_of::<T>() as u32;
        let offset = self.len + size;
        if (HEAP_PACKET_SIZE as u32) - offset > 0{
            unsafe{
                self.ptr.add(offset as usize).write_unaligned(data);
            }
        } else{
            let temp = self;
                    
        }
    }
}