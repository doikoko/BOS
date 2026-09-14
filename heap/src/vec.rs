use crate::{HEAP_PACKET_SIZE, Heap};

#[derive(Copy, Clone)]
pub struct Vec<T>{
    /// ptr is pointer to heap memory
    ptr: *mut T,
    /// len is lenght vec in bytes
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
        if (HEAP_PACKET_SIZE as u32) - self.len - size == 0{
            let temp_ptr = self.ptr;
            self.ptr = Heap::malloc(self.len + HEAP_PACKET_SIZE as u32)
                .expect("Vec panicked while push") as *mut T;

            for i in 0..self.len as usize{
                unsafe{
                    self.ptr
                    .add(i)
                    .write_unaligned(
                        temp_ptr.add(i)
                        .read_unaligned()
                    );
                };
            }
        }
        unsafe{
            self.len += size;
            self.ptr.add(self.len as usize).write_unaligned(data);
        }
    }
}