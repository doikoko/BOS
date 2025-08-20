#![cfg(target_pointer_width = "64")]
use memory::mem64::{memzero, memzero_smart};

const MEM_MULTIPLES_8: usize = 104; // 104 / 8 = 13
macro_rules! set_one {
    ($mem : expr) => {
        $mem
            .iter_mut()
            .for_each(|el| *el = 1)
    };
}
macro_rules! find_one {
    ($mem : expr) => {
        assert_eq!(None, $mem
            .iter()
            .find(|&&el| el != 0)
        )
    }
}
#[test]
fn memzero_test(){
    let mut mem = Vec::from([1; MEM_MULTIPLES_8]); // create "garbage" memory
    // use for this heap to prevent addressing to unreacheble memory

    memzero::<u8>(mem.as_mut_ptr(), MEM_MULTIPLES_8);
    find_one!(mem); // find "garbage"
    set_one!(mem); // return "garbage"
    
    memzero::<u16>(mem.as_mut_ptr() as *mut u16, MEM_MULTIPLES_8);
    find_one!(mem);
    set_one!(mem);
    
    memzero::<u32>(mem.as_mut_ptr() as *mut u32, MEM_MULTIPLES_8);
    find_one!(mem);
    set_one!(mem);

    memzero::<u64>(mem.as_mut_ptr() as *mut u64, MEM_MULTIPLES_8);
    find_one!(mem);
    set_one!(mem);
}

#[test]
fn memzero_smart_test(){
    let start_num = 1;
    let end_num = 200;

    let mut mem = Vec::new();
    let mut ptr: *mut u8;
    // check this function with 1 - 200 bytes for many
    // possible ways
    for i in  start_num..end_num{
        mem.push(1u8);

        ptr = mem.as_mut_ptr();
        
        memzero_smart(ptr, i);
        find_one!(mem); // find 1
        set_one!(mem) // return "garbage" values
    }
}