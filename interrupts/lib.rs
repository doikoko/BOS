#![no_std]

#![feature(abi_x86_interrupt)]
#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(private_interfaces)]
pub mod ints{
    use core::mem::MaybeUninit;

    #[repr(packed, C)]
    struct InterruptStackFrame{
        rip: u64,
        cs: u64,
        rflags: u64,
        rsp: u64,
        ss: u64
    }
    
    #[repr(packed, C)]
    struct IDTGate{
        offset_1: u16,
        selector: u16,
        IST: u8,    /* Interrupt Stack Table offset (3 zeroes),
                            Gate type (interrupt (0b1110 0x0E) / trap(0b1111 0x0F)) in summ 1 byte */ 
        type_attributes: u8,
        offset_2: u16,
        offset_3: u32,
        zero: u32
    }
    impl IDTGate{
        fn new(FnAddress: u64, IST_num: u8, is_gate_type_trap: bool) -> IDTGate{ /* IST_num - 1-3,
        is_gate_type_trap true-trap gate, false-interrupt */
            IDTGate {
                offset_1: FnAddress as u16,
                selector: 0b_00001_00_0, /* 0 bit-privelege 1-2 GDT/LDT  3-7 Code index in GDT*/
                IST: IST_num << 5,
                type_attributes: (0b_111 | (is_gate_type_trap as u8) << 4) | 1, /* gate_type + 0(kernel mode) + 1 always*/
                offset_2: (FnAddress >> 16) as u16,
                offset_3: (FnAddress >> 32) as u32,
                zero: 0 as u32
            }
        }
    }

    #[repr(C)]
    pub struct IntDescrTable64{
        gates: [IDTGate; 255]
    }
    impl IntDescrTable64{
        #[inline(always)]
        pub fn new() -> Self{
            unsafe{ 
                MaybeUninit::<Self>::zeroed()
                    .assume_init()
            }
        }
        pub fn append(&mut self, index: usize, FnAddress: u64, IST_num: u8, is_gate_type_trap: bool) {
            self.gates[index] = IDTGate::new(FnAddress, IST_num, is_gate_type_trap);
        }
    }

    #[no_mangle]
    pub extern "x86-interrupt" fn divide_zero_handler(_stack_frame: &mut InterruptStackFrame){
        loop{}
    }
}