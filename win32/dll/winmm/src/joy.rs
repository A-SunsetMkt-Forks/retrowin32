use win32_system::System;

pub type JOYCAPSA = u32;
pub type JOYINFOEX = u32;

#[win32_derive::dllexport]
pub fn joyGetNumDevs(sys: &dyn System) -> u32 {
    0
}

#[win32_derive::dllexport]
pub fn joyGetDevCapsA(sys: &dyn System, uJoyID: u32, pjc: Option<&mut JOYCAPSA>, cbjc: u32) -> u32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn joyGetPosEx(sys: &dyn System, uJoyID: u32, pji: Option<&mut JOYINFOEX>) -> u32 {
    todo!()
}
