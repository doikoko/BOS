#![no_std]

mod pit;
mod pause;
mod rdtsc;

use core::{cell::OnceCell, option::Option};

use crate::pit::PIT;

#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct Tsc {
    pub tsc_frequency: u64,
    pub ticks_per_us: u64,
}

impl Tsc {
    pub const fn new() -> Self{
        Tsc { tsc_frequency: 0, ticks_per_us: 0 }
    }

    pub fn init(&mut self) {
        let mut max_leaf = 0u32;
        unsafe {
            core::arch::asm!(
                "mov edi, ebx",
                "cpuid",
                "mov ebx, edi",
                inout("eax") max_leaf,
                out("edi") _,
                out("ecx") _,
                out("edx") _,
            );
        };

        let mut tsc_frequency = 0u64;
        
        if max_leaf >= 0x15 {
            let mut eax = 0x15u32;
            let mut ebx = 0u32;
            let mut ecx = 0u32;
            unsafe {
                core::arch::asm!(
                    "mov edi, ebx",
                    "cpuid",
                    "mov {ebx_out:e}, ebx",
                    "mov ebx, edi",
                    ebx_out = out(reg) ebx,
                    inout("eax") eax,
                    out("ecx") ecx,
                    out("edx") _,
                    out("edi") _,
                );
            };
            if eax != 0 && ebx != 0 && ecx != 0 {
                tsc_frequency = (ecx as u64 * ebx as u64) / eax as u64;
            };
        }

        if tsc_frequency == 0 {
            let pit = PIT::new();
            tsc_frequency = pit.get_tsc_frequency();
        }
        
        const US_IN_SECONDS: u64 = 1_000_000;

        let ticks_per_us = tsc_frequency / US_IN_SECONDS;

        self.tsc_frequency = tsc_frequency;
        self.ticks_per_us = ticks_per_us;
    }

    pub fn delay(&self, us: u64) {
        if self.ticks_per_us == 0 {
            return;
        }
        let target_ticks = us * self.ticks_per_us;
        let start = 0u64;
        rdtsc!(start);

        let current = start;
        while current.wrapping_sub(start) < target_ticks {
            pause::pause();
            rdtsc!(current);
        }
    }
}