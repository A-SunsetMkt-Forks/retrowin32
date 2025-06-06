use memory::Pod;
use win32_system::System;

#[repr(C)]
#[derive(Debug)]
pub struct INIT_ONCE {
    ptr: u32,
}
unsafe impl Pod for INIT_ONCE {}

#[win32_derive::dllexport]
pub fn InitOnceBeginInitialize(
    sys: &dyn System,
    lpInitOnce: Option<&mut INIT_ONCE>,
    dwFlags: u32,
    fPending: Option<&mut u32>,
    lpContext: u32,
) -> bool {
    if dwFlags != 0 {
        todo!();
    }
    *fPending.unwrap() = 1;
    true
}

#[win32_derive::dllexport]
pub fn InitOnceComplete(
    sys: &dyn System,
    lpInitOnce: Option<&mut INIT_ONCE>,
    dwFlags: u32,
    lpContext: u32,
) -> bool {
    if dwFlags != 0 {
        todo!();
    }
    lpInitOnce.unwrap().ptr = lpContext;
    true
}
