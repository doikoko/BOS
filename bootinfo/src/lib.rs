#![no_std]

use TSC::Tsc;

use core::option::Option::{self, Some};

const BOOT_INFO_PTR: usize = 0x6000;

macro_rules! BootInfoMut {
    () => {
        unsafe { &mut *(BOOT_INFO_PTR as *mut BootInfo) }
    };
}

macro_rules! BootInfo {
    () => {
        unsafe { &*(BOOT_INFO_PTR as *const BootInfo) }
    };
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct BootInfo{
    tsc: Option<Tsc>
}
impl BootInfo{
    pub fn set_tsc(tsc: Tsc) {
        BootInfoMut!().tsc = Some(tsc);
    }

    pub fn get_tsc_mut() -> Option<&'static mut Tsc> {
        BootInfoMut!().tsc.as_mut()
    }

    pub fn get_tsc() -> Option<&'static Tsc> {
        BootInfo!().tsc.as_ref()
    }
}
