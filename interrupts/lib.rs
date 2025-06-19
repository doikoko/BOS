#![no_std]

#![feature(abi_x86_interrupt)]

extern crate io;
extern crate ports;


pub mod ints{
    use ports::ports::{outw, inb};
    use core::mem::MaybeUninit;

    #[repr(packed, C)]
    pub struct IDTR{
        pub limit: u16,
        pub base: u64
    }
    #[repr(packed, C)]
    pub struct InterruptStackFrame{
        pub rip: u64,
        pub cs: u64,
        pub rflags: u64,
        pub rsp: u64,
        pub ss: u64
    }
    
    #[repr(packed, C)]
    pub struct IDTGate{
        pub offset_1: u16,
        pub selector: u16,
        pub IST: u8,    /* Interrupt Stack Table offset (3 zeroes),
                            Gate type (interrupt (0b1110 0x0E) / trap(0b1111 0x0F)) in summ 1 byte */ 
        pub type_attributes: u8,
        pub offset_2: u16,
        pub offset_3: u32,
        pub zero: u32
    }
    impl IDTGate{
        pub fn new(FnAddress: u64, IST_num: u8, is_gate_type_trap: bool) -> IDTGate{ /* IST_num - 1-3,
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
        pub gates: [IDTGate; 255]
    }
    impl IntDescrTable64{
        #[inline(always)]
        pub fn new() -> Self{
            unsafe{ 
                MaybeUninit::<Self>::zeroed()
                    .assume_init()
            }
        }
        pub fn append(
            &mut self,
            index: usize,
            FnAddress: extern "x86-interrupt" fn(& mut InterruptStackFrame),
            IST_num: u8,
            is_gate_type_trap: bool
        ){
            self.gates[index] = IDTGate::new(FnAddress as u64, IST_num, is_gate_type_trap);
        }
    }
    macro_rules! shutdown {
        () => {
            unsafe { core::arch::asm!(
                "cli",
                "lidt[0]",
                "int3"
            ) } /* triple fault -> reboot */
            outw(0x604, 0x2000); /* this and next lines shutdown for VM */
            outw(0xB004, 0);
        }
    }
    macro_rules! cli {
        () => {
            unsafe { core::arch::asm!("cli"); };
        }
    }
    macro_rules! sti {
        () => {
            unsafe { core::arch::asm!("sti"); };
        }
    }
    macro_rules! hlt {
        () => {
            unsafe { core::arch::asm!("hlt"); };
        }
    }
    pub extern "x86-interrupt"  fn infinity_handler(_stack_frame: &mut InterruptStackFrame){
        loop{
            hlt!();
        }
    }
    
    pub extern "x86-interrupt"  fn default_handler(_stack_frame: &mut InterruptStackFrame){}
    
    pub extern "x86-interrupt" fn divide_zero_handler(_stack_frame: &mut InterruptStackFrame){
        unsafe { core::arch::asm!("mov rax, 0", options(nostack, readonly, preserves_flags)); };
    }
    
    pub extern "x86-interrupt" fn debug_handler(_stack_frame: &mut InterruptStackFrame){
        unsafe { core::arch::asm!(
            "pushfq",
            "or word ptr [rsp], 0x0100", /* set trap flag */
            "popfq",
            options(nostack, readonly, preserves_flags)
        )};
    }
    
    pub extern "x86-interrupt" fn nmi_handler(_stack_frame: &mut InterruptStackFrame){ 
        let port1 = inb(0x92);
        let port2 = inb(0x61);

        if ((port1 & 0b_0001_0000) | (port2 & (0b_1000_0000 | 0b_0100_0000))) != 0{
            shutdown!();
        }
    }
    pub extern "x86-interrupt" fn breakpoint_handler(_stack_frame: &mut InterruptStackFrame){
        /* print("breakpoint") */
    }
    pub extern "x86-interrupt" fn invalid_opcode_handler(_stack_frame: &mut InterruptStackFrame){
        /* print("invalid instruction") */
    }
    pub extern "x86-interrupt" fn device_not_available_handler(_stack_frame: &mut InterruptStackFrame){
        /* print("device not available") */
    }
    pub extern "x86-interrupt" fn double_fault_handler(_stack_frame: &mut InterruptStackFrame){
        /* print("double fault") */
        hlt!();
    }
}