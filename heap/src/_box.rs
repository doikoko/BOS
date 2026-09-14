use crate::{Heap, Result};

pub struct Box<T>(*mut T);
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