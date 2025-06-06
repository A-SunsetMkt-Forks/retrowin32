//! Functions that work with .ini files.

use win32_system::System;
use win32_winapi::{Str16, calling_convention::ArrayOut};

#[win32_derive::dllexport]
pub fn GetPrivateProfileIntA(
    sys: &dyn System,
    lpAppName: Option<&str>,
    lpKeyName: Option<&str>,
    nDefault: u32,
    lpFileName: Option<&str>,
) -> u32 {
    nDefault // not found
}

#[win32_derive::dllexport]
pub fn GetPrivateProfileIntW(
    sys: &dyn System,
    lpAppName: Option<&Str16>,
    lpKeyName: Option<&Str16>,
    nDefault: u32,
    lpFileName: Option<&Str16>,
) -> u32 {
    nDefault // not found
}

#[win32_derive::dllexport]
pub fn GetPrivateProfileStringA(
    sys: &dyn System,
    lpAppName: Option<&str>,
    lpKeyName: Option<&str>,
    lpDefault: Option<&str>,
    lpReturnedString: Option<&str>,
    nSize: u32,
    lpFileName: Option<&str>,
) -> u32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn GetPrivateProfileStringW(
    sys: &dyn System,
    lpAppName: Option<&Str16>,
    lpKeyName: Option<&Str16>,
    lpDefault: Option<&Str16>,
    mut lpReturnedString: ArrayOut<u16>,
    lpFileName: Option<&Str16>,
) -> u32 {
    let dst = &mut lpReturnedString;
    let src = lpDefault.unwrap();
    let copy_len = std::cmp::min(dst.len() - 1, src.len());
    dst[..copy_len].copy_from_slice(&src.as_bytes()[..copy_len]);
    dst[copy_len] = 0;
    copy_len as u32
}

#[win32_derive::dllexport]
pub fn GetProfileIntW(
    sys: &dyn System,
    lpAppName: Option<&Str16>,
    lpKeyName: Option<&Str16>,
    nDefault: i32,
) -> u32 {
    nDefault as u32
}

#[win32_derive::dllexport]
pub fn GetProfileStringW(
    sys: &dyn System,
    lpAppName: Option<&Str16>,
    lpKeyName: Option<&Str16>,
    lpDefault: Option<&Str16>,
    mut lpReturnedString: ArrayOut<u16>,
) -> u32 {
    let dst = &mut lpReturnedString;
    let src = lpDefault.unwrap();
    let copy_len = std::cmp::min(dst.len() - 1, src.len());
    dst[..copy_len].copy_from_slice(&src.as_bytes()[..copy_len]);
    dst[copy_len] = 0;
    copy_len as u32
}

#[win32_derive::dllexport]
pub fn WriteProfileStringW(
    sys: &dyn System,
    lpAppName: Option<&Str16>,
    lpKeyName: Option<&Str16>,
    lpString: Option<&Str16>,
) -> bool {
    todo!();
}

#[win32_derive::dllexport]
pub fn WritePrivateProfileStringA(
    sys: &dyn System,
    lpAppName: Option<&str>,
    lpKeyName: Option<&str>,
    lpString: Option<&str>,
    lpFileName: Option<&str>,
) -> bool {
    todo!()
}
