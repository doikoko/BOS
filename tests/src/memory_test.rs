#![cfg(target_pointer_width = "64")]
use memory::mem64::{memzero_smart, memzero_step1, 
    memzero_step2, memzero_step4, memzero_step8};

const MEM_EVEN_SIZE: usize = 100;
const MEM_ODD_SIZE: usize = 101;

const MEM_MULTIPLES_8: usize = 104; // 104 / 8 = 13

#[test]
fn memzero_step1_test(){
    let mut mem_even = Vec::from([1; MEM_EVEN_SIZE]); // create "garbage" memory
    // use for this heap to prevent addressing to unreacheble memory
    let mut mem_odd = Vec::from([1; MEM_ODD_SIZE]);

    memzero_step1(mem_even.as_mut_ptr(), MEM_EVEN_SIZE);
    memzero_step1(mem_odd.as_mut_ptr(), MEM_ODD_SIZE);
    
    assert_eq!(None, mem_even
        .iter()
        .find(|&&el| el == 1)
    );
    
    assert_eq!(None, mem_odd
        .iter()
        .find(|&&el| el == 1)
    );
}

#[test]
fn memzero_step2_test(){
    let mut mem_even = Vec::from([1u8; MEM_EVEN_SIZE]); // create "garbage" memory
    // use for this heap to prevent addressing to unreacheble memory
    
    memzero_step2(mem_even.as_mut_ptr() as *mut u16, MEM_EVEN_SIZE)
        .unwrap();

    assert_eq!(None, mem_even
        .iter()
        .find(|&&el| el == 1)
    );
}

#[test]
fn memzero_step4_test(){
    let mut mem_even = Vec::from([1u8; MEM_EVEN_SIZE]); // create "garbage" memory
    // use for this heap to prevent addressing to unreacheble memory
    
    memzero_step4(mem_even.as_mut_ptr() as *mut u32, MEM_EVEN_SIZE)
        .unwrap();

    assert_eq!(None, mem_even
        .iter()
        .find(|&&el| el == 1)
    );
}

#[test]
fn memzero_step8_test(){
    let mut mem_even = Vec::from([1u8; MEM_MULTIPLES_8]); // create "garbage" memory
    // use for this heap to prevent addressing to unreacheble memory
    
    memzero_step8(mem_even.as_mut_ptr() as *mut u64, MEM_MULTIPLES_8)
        .unwrap();

    assert_eq!(None, mem_even
        .iter()
        .find(|&&el| el == 1)
    );
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
        assert_eq!(None, mem
            .iter()
            .find(|&&el| el == 1)
        );

        // return "garbage" values
        mem
            .iter_mut()
            .for_each(|el| *el = 1);
    }
}