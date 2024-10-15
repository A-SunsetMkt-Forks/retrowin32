#![allow(non_snake_case)]

mod builtin;

pub use builtin::DLL;

use crate::machine::Machine;

#[win32_derive::dllexport]
pub fn GetFileVersionInfoSizeA(
    _machine: &mut Machine,
    lptstrFilename: Option<&str>,
    lpdwHandle: Option<&mut u32>,
) -> u32 {
    0 // TODO
}