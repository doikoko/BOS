#![no_std]

use core::mem::MaybeUninit;

pub mod Ints{
    #[derive(Copy, Clone)]
    #[repr(packed)]
    pub struct IDTGate{
        offset_1: u16,
        selector: u16,
        IST_GateType: u8,    /* Interrupt Stack Table offset (3 zeroes),
                            Gate type (interrupt (0b1110 0x0E) / trap(0b1111 0x0F)) in summ 1 byte */ 
        type_attributes: u8,
        offset_2: u16,
        offset_3: u32,
        zero: u32
    }
    impl IDTGate{
        #[inline(always)]
        pub fn new(FnAddress: u64, ) -> IDTGate{
            IDTGate {
                offset_1: FnAddress as u16,
                selector: (0x00 | 0b_00001_00_0) /* 0 bit-privelege 1-2 GDT/LDT  3-7 Code index in GDT*/
                IST_GateType
            }
        }
    }

    pub struct IntDescrTable64{
        gates: [IDTGate; 255]
    }
    impl IntDescrTable64{
        #[inline(always)]
        pub fn new() -> IntDescrTable64{
            unsafe{ 
                MaybeUninit::<IntDescrTable64>::zeroed()
                    .assume_init()
            }
        }
    }
    pub extern "x86_interrupt" fn 
}