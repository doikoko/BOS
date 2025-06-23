#![no_std]

pub mod Paging{
    use core::mem::{zeroed, MaybeUninit};

    #[repr(packed, C)]
    pub struct PML4{
        pub pdptes: [&PDPTE; 256]
    }
    impl PML4{
        #[inline(always)]
        pub fn new() -> &Self{
            let pml4 = 0x1000 as *mut Self;
            unsafe {
                core::arch::asm!(
                    "mov edi, 0x1000", // 4kb
                    "mov cr3, edi",  // set control register
                )
                *pml4 = Self{ pdptes: [None; 256] };
                *pml4[0] = Some(0x2003 as &PDPTE);
            };

            pml4.as_ref().unwrap()
        }
    }
    #[repr(packed, C)]
    pub struct PDPTE{
        pub directories: [&PD; 256]
    }
    impl PDTE{
        #[inline(always)]
        pub fn new() -> &Self{
            let pdpte = 0x2000 as *mut Self;
            unsafe { 
                *pdpte = Self{ directories: [None; 256] };
                *pdpte[0] = Some(0x3003 as &PD);
            };
            
            pdpte.as_ref().unwrap()
        }
    }

    #[repr(packed, C)]
    pub struct PD {
        pub tables: [Option<&PT>; 256]
    }
    impl PD{
        #[inline(always)]
        pub fn new() -> &Self{
            let pt = 0x3000 as *mut Self;
            unsafe {
                *pdpte = Self{ tables: [None; 256] };
                *pdpte[0] = Some(0x4003 as &PT);
            }

            pt.as_ref().unwrap()
        }
    }

    #[repr(packed, C)]
    pub struct PT {
        pub pages: [Option<&PTentry>; 256]
    }
    impl PT {
        #[inline(always)]
        pub fn new() -> Self{
            Self{ pages: [None; 256] }
        }
    }
    pub struct PTentry(u64);
    impl PTentry {
        #[inline(always)]
        pub fn new(addr: u64) -> Self{
            Self(0 as u64 | addr | 0x3)
        }
    }
}
    
	mov edi, 0x1000	; 4kb
	mov cr3, edi	; set control register
	xor eax, eax
	mov ecx, 0x1000	; PML4
	rep stosd	; set 16kb to 0
	mov edi, cr3

	mov qword [edi], 0x2003 ; PDPTE
	add edi, 0x1000
	mov qword [edi], 0x3003	; PD
	add edi, 0x1000
	mov qword [edi], 0x4003	; PT
	add edi, 0x1000

	mov ebx, 0x3
	mov ecx, 511	; 1 is framebuffer

.add_framebuffer	; in this section you can add physical addresses like 0xB8000
					; to avoid virt memory
	mov qword [edi], 0xB8003
.set_entry:
	mov qword [edi], ebx	; set flags to all pages
	add ebx, 0x1000
	add edi, 8
	loop .set_entry

	mov eax, cr4	; enable PAE-paging
	or eax, 1 << 5
	mov cr4, eax