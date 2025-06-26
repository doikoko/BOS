#![no_std]

pub mod paging{
    const PML4_ADDR: usize = 0x10100;
    const PDPTE_ADDR: usize = 0x10200;
    const PD_ADDR: usize = 0x10300;
    const PT_ADDR: usize = 0x11300;
    const PT_SIZE: usize = 0x1000;
    const PT_ENTRY_FIRST_ADDR: usize = 0x12300;
    const PT_ENTRY_SIZE: usize = 8;

    const FLAGS: usize = 0x03;
    #[repr(packed, C)]
    pub struct PML4{
        pub pdptes: usize // usize is address of PDPTE
        // it's will be contained 1 PDPTE, 31 slot reserved
        // in future to fill more than 960MB RAM this value
        // can be expanded
        // to contain more PDTEntry need to shift right kernel
        // for now all PML4 entries fill in 0x10100-0x2_00_000(MB)
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
        #[inline(always)]
        pub fn new() -> &'static mut Self{
            let pml4= unsafe{ &mut *(PML4_ADDR as *mut Self) };
            
            pml4
        }
        #[inline(always)]
        pub fn set(&mut self){
            *self = Self{ pdptes: PDPTE_ADDR | FLAGS };
        }
    }
    #[repr(packed, C)]
    pub struct PDPTE{
        pub directories: usize
    }
    impl PDPTE{
        #[inline(always)]
        pub fn new() -> &'static mut Self{
            let pdpte = unsafe{ &mut *(PDPTE_ADDR as *mut Self) };
            
            pdpte            
        }
        #[inline(always)]
        pub fn set(&mut self){
            *self = Self{ directories: PD_ADDR | FLAGS};
        }
    }

    #[repr(packed, C)]
    pub struct PD {
        pub tables: [usize; 480] 
        // it has free space to 512 pointers, but haven't memory for
        // each entry, then 480
    }
    impl PD{
        #[inline(always)]
        pub fn new() -> &'static mut Self{
            let pd = unsafe { &mut *(PD_ADDR as *mut Self) };
            *pd = Self{ tables: [0; 480] };

            pd
        }
        
        #[inline(always)]
        pub fn set(&mut self, index: usize){
            self.tables[index] = (PT_ADDR + (PT_SIZE * index)) | FLAGS;
        }
    }

    #[repr(packed, C)]
    pub struct PT {
        pub pages: [usize; 512]
    }
    impl PT {
        #[inline(always)]
        pub fn new(index: usize) -> &'static mut Self{
            let pt = unsafe { 
                &mut *((PT_ADDR + (index * PT_SIZE)) as *mut Self)
            };
            *pt = Self{ pages: [0; 512]};

            pt
        }
        #[inline(always)]
        pub fn append(&mut self, index: usize, ptentry: &PTentry){
            self.pages[index] = ((ptentry as *const _) as usize) | FLAGS;
        }
        #[inline(always)]
        pub fn set(&mut self){
            self.pages[0] = PT_ENTRY_FIRST_ADDR | FLAGS;
        } 
    }

    #[allow(dead_code)]
    pub struct PTentry(usize);
    impl PTentry {
        #[inline(always)]
        pub fn new(index: usize, value: usize, flags: Option<usize>) -> &'static Self{
            let entry = unsafe { 
                &mut *((PT_ENTRY_FIRST_ADDR + (index * PT_ENTRY_SIZE))
                    as *mut Self)
            };
            let flags = flags.unwrap_or(FLAGS);
            *entry = Self(value | flags);
            
            entry
        }
    }
}