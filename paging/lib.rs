#![no_std]

pub mod paging{
    /* PML4:|0x11000 - 0x12000|  PDPTE:|0x12000 - 0x13000|
    PD:|0x13000 - 0x1d5000|, reserved:|0x1d5000 - 0x200000| */
    const PML4_ADDR: usize = 0x11000; 
    const PDPTE_FIRST_ADDR: usize = 0x12000;
    const PD_FIRST_ADDR: usize = 0x13000;

    const KERNEL_ADDR: usize = 0x200000;

    const PDPTE_SIZE: usize = 8 * PDS_IN_PDPTE; // 450 entries each 8 bytes
    const PD_SIZE: usize = 8 * ADDRESSES_IN_PD; // size of usize in bytes

    pub const PDPTES_IN_PML4: usize = 1;
    pub const PDS_IN_PDPTE: usize = 450;
    pub const ADDRESSES_IN_PD: usize = 512;

    pub const PRESENT: usize      = 1 << 0;
    pub const WRITABLE: usize     = 1 << 1;
    pub const USER_ACCESS: usize  = 1 << 2;
    pub const WRITE_THROUGH: usize = 1 << 3;
    pub const DISABLE_CACHE: usize = 1 << 4;
    pub const ACCESSED: usize     = 1 << 5;
    pub const DIRTY: usize        = 1 << 6;
    pub const PAGE_SIZE: usize    = 1 << 7;
    pub const GLOBAL: usize       = 1 << 8;
    pub const NO_EXECUTE: usize   = 1 << 63;

    // if parent struct (for example PDPTE) doesn't contain flag, but child PD contain
    // it will not be set, because it calculate like this "PARENT_FLAGS & CHILD_FLAGS"
    const ALL_FLAGS: usize = 0xFFF | 1 << 63; 
    
    #[repr(C)]
    pub struct PML4{
        pdptes: [usize; PDPTES_IN_PML4]
    }
    impl PML4{
        #[inline(always)]
        pub fn init(){
            unsafe {
                core::arch::asm!(
                    "mov eax, cr4",	// enable PAE-paging
                    "or eax, 1 << 5",
                    "mov cr4, eax",
                    "mov cr3, edi",  // set control register
                    in("edi") PML4_ADDR
                );
            }
        }
        // set memory from PML4 start to kernel as 0
        #[inline(always)]
        pub fn set_zeroes() {
            let mut ptr: *mut usize = PML4_ADDR as *mut usize;
            unsafe{
                for _ in 0..((KERNEL_ADDR - PML4_ADDR) / 8){
                    *ptr = 0;
                    ptr = ptr.add(1);
                }
            }
        }
        // create new PML4 in first PML4_ADDR address
        #[inline(always)]
        pub fn new() -> &'static mut Self{
            let pml4= unsafe{ &mut *(PML4_ADDR as *mut Self) };

            pml4
        }
        // set INDEX element of array
        #[inline(always)]
        pub fn set(&mut self, index: usize){
            self.pdptes[index] = ((PDPTE_FIRST_ADDR + index * PDPTE_SIZE) << 12) | ALL_FLAGS;
        }
        // get INDEX element of array as &PDPTE
        #[inline(always)]
        pub fn get(&mut self, index: usize) -> &'static mut PDPTE {
            // create correct addr from addr | ALL_FLAGS
            let pdpte = unsafe { &mut *(((self.pdptes[index] & ALL_FLAGS) >> 12 ) as *mut PDPTE) };

            pdpte
        }
    }
    #[repr(C)]
    pub struct PDPTE{
        pub directories: [usize; PDS_IN_PDPTE]
    }
    impl PDPTE{
        #[inline(always)]
        pub fn set(&mut self, index: usize){
            self.directories[index] = ((PD_FIRST_ADDR + index * PD_SIZE) << 12) | ALL_FLAGS;
        }

        #[inline(always)]
        pub fn get(&mut self, index: usize) -> &'static mut PD {
            let pd = unsafe { &mut *(((self.directories[index] & ALL_FLAGS) >> 12 ) as *mut PD ) };    
            
            pd
        }
    }

    #[repr(C)]
    pub struct PD {
        pub pages: [usize; ADDRESSES_IN_PD]
    }
    impl PD{
        #[inline(always)]
        // page is 2mb value that will be allocated
        pub fn set(&mut self, index: usize, page: usize, flags: usize){
            self.pages[index] = page | flags;
        }  
    }
}