#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#[doc = r" Generated code, do not edit."]
use crate::{
    machine::Machine,
    winapi::{self, stack_args::*, types::*},
};
pub struct Symbol {
    pub name: &'static str,
    pub ordinal: Option<usize>,
    pub func: extern "C" fn(&mut Machine, u32) -> u32,
    pub stack_consumed: fn() -> u32,
}
pub struct BuiltinDLL {
    pub file_name: &'static str,
    pub exports: &'static [Symbol],
}
pub mod bass {
    use super::*;
    use winapi::bass::*;
    pub extern "C" fn BASS_Init(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let arg1 = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let arg2 = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let arg3 = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let arg4 = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::bass::BASS_Init(machine, arg1, arg2, arg3, arg4);
        result.to_raw()
    }
    pub extern "C" fn BASS_MusicLoad(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let arg1 = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let arg2 = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let arg3 = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let arg4 = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let arg5 = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::bass::BASS_MusicLoad(machine, arg1, arg2, arg3, arg4, arg5);
        result.to_raw()
    }
    pub extern "C" fn BASS_Start(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let result = winapi::bass::BASS_Start(machine);
        result.to_raw()
    }
    pub extern "C" fn BASS_MusicPlay(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let arg1 = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::bass::BASS_MusicPlay(machine, arg1);
        result.to_raw()
    }
    pub extern "C" fn BASS_ChannelGetPosition(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let arg1 = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::bass::BASS_ChannelGetPosition(machine, arg1);
        result.to_raw()
    }
    const EXPORTS: [Symbol; 5usize] = [
        Symbol {
            name: "BASS_Init",
            ordinal: None,
            func: BASS_Init,
            stack_consumed: || {
                <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
            },
        },
        Symbol {
            name: "BASS_MusicLoad",
            ordinal: None,
            func: BASS_MusicLoad,
            stack_consumed: || {
                <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
            },
        },
        Symbol {
            name: "BASS_Start",
            ordinal: None,
            func: BASS_Start,
            stack_consumed: || 0,
        },
        Symbol {
            name: "BASS_MusicPlay",
            ordinal: None,
            func: BASS_MusicPlay,
            stack_consumed: || <u32>::stack_consumed(),
        },
        Symbol {
            name: "BASS_ChannelGetPosition",
            ordinal: None,
            func: BASS_ChannelGetPosition,
            stack_consumed: || <u32>::stack_consumed(),
        },
    ];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "bass.dll",
        exports: &EXPORTS,
    };
}
pub mod ddraw {
    use super::*;
    use winapi::ddraw::*;
    pub extern "C" fn DirectDrawCreate(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let lpGuid = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let lplpDD = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let pUnkOuter = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::ddraw::DirectDrawCreate(machine, lpGuid, lplpDD, pUnkOuter);
        result.to_raw()
    }
    pub extern "C" fn DirectDrawCreateEx(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let lpGuid = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let lplpDD = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let iid = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let pUnkOuter = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::ddraw::DirectDrawCreateEx(machine, lpGuid, lplpDD, iid, pUnkOuter);
        result.to_raw()
    }
    const EXPORTS: [Symbol; 2usize] = [
        Symbol {
            name: "DirectDrawCreate",
            ordinal: None,
            func: DirectDrawCreate,
            stack_consumed: || {
                <u32>::stack_consumed() + <u32>::stack_consumed() + <u32>::stack_consumed()
            },
        },
        Symbol {
            name: "DirectDrawCreateEx",
            ordinal: None,
            func: DirectDrawCreateEx,
            stack_consumed: || {
                <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
            },
        },
    ];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "ddraw.dll",
        exports: &EXPORTS,
    };
}
pub mod dsound {
    use super::*;
    use winapi::dsound::*;
    pub extern "C" fn DirectSoundCreate(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let _lpGuid = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let ppDS = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let _pUnkOuter = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::dsound::DirectSoundCreate(machine, _lpGuid, ppDS, _pUnkOuter);
        result.to_raw()
    }
    const EXPORTS: [Symbol; 1usize] = [Symbol {
        name: "DirectSoundCreate",
        ordinal: Some(1usize),
        func: DirectSoundCreate,
        stack_consumed: || {
            <u32>::stack_consumed() + <u32>::stack_consumed() + <u32>::stack_consumed()
        },
    }];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "dsound.dll",
        exports: &EXPORTS,
    };
}
pub mod gdi32 {
    use super::*;
    use winapi::gdi32::*;
    pub extern "C" fn GetStockObject(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let _i = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::gdi32::GetStockObject(machine, _i);
        result.to_raw()
    }
    pub extern "C" fn SelectObject(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let hdc = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let hGdiObj = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::gdi32::SelectObject(machine, hdc, hGdiObj);
        result.to_raw()
    }
    pub extern "C" fn GetObjectA(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let handle = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let _bytes = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let _out = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::gdi32::GetObjectA(machine, handle, _bytes, _out);
        result.to_raw()
    }
    pub extern "C" fn CreateCompatibleDC(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let hdc = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::gdi32::CreateCompatibleDC(machine, hdc);
        result.to_raw()
    }
    pub extern "C" fn DeleteDC(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let hdc = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::gdi32::DeleteDC(machine, hdc);
        result.to_raw()
    }
    pub extern "C" fn BitBlt(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let hdc = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let x = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let y = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let cx = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let cy = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let hdcSrc = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let x1 = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let y1 = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let rop = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::gdi32::BitBlt(machine, hdc, x, y, cx, cy, hdcSrc, x1, y1, rop);
        result.to_raw()
    }
    pub extern "C" fn StretchBlt(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let hdcDest = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let xDest = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let yDest = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let wDest = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let hDest = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let hdcSrc = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let xSrc = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let ySrc = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let wSrc = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let hSrc = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let rop = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::gdi32::StretchBlt(
            machine, hdcDest, xDest, yDest, wDest, hDest, hdcSrc, xSrc, ySrc, wSrc, hSrc, rop,
        );
        result.to_raw()
    }
    const EXPORTS: [Symbol; 7usize] = [
        Symbol {
            name: "GetStockObject",
            ordinal: None,
            func: GetStockObject,
            stack_consumed: || <u32>::stack_consumed(),
        },
        Symbol {
            name: "SelectObject",
            ordinal: None,
            func: SelectObject,
            stack_consumed: || <u32>::stack_consumed() + <u32>::stack_consumed(),
        },
        Symbol {
            name: "GetObjectA",
            ordinal: None,
            func: GetObjectA,
            stack_consumed: || {
                <u32>::stack_consumed() + <u32>::stack_consumed() + <u32>::stack_consumed()
            },
        },
        Symbol {
            name: "CreateCompatibleDC",
            ordinal: None,
            func: CreateCompatibleDC,
            stack_consumed: || <u32>::stack_consumed(),
        },
        Symbol {
            name: "DeleteDC",
            ordinal: None,
            func: DeleteDC,
            stack_consumed: || <u32>::stack_consumed(),
        },
        Symbol {
            name: "BitBlt",
            ordinal: None,
            func: BitBlt,
            stack_consumed: || {
                <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
            },
        },
        Symbol {
            name: "StretchBlt",
            ordinal: None,
            func: StretchBlt,
            stack_consumed: || {
                <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
            },
        },
    ];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "gdi32.dll",
        exports: &EXPORTS,
    };
}
pub mod kernel32 {
    use super::*;
    use winapi::kernel32::*;
    pub extern "C" fn GetModuleHandleA(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let lpModuleName = unsafe { <Option<&str>>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <Option<&str>>::stack_consumed();
        let result = winapi::kernel32::GetModuleHandleA(machine, lpModuleName);
        result.to_raw()
    }
    pub extern "C" fn GetModuleHandleW(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let lpModuleName =
            unsafe { <Option<Str16>>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <Option<Str16>>::stack_consumed();
        let result = winapi::kernel32::GetModuleHandleW(machine, lpModuleName);
        result.to_raw()
    }
    pub extern "C" fn GetModuleHandleExW(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let dwFlags = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let lpModuleName =
            unsafe { <Option<Str16>>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <Option<Str16>>::stack_consumed();
        let hModule =
            unsafe { <Option<&mut HMODULE>>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <Option<&mut HMODULE>>::stack_consumed();
        let result = winapi::kernel32::GetModuleHandleExW(machine, dwFlags, lpModuleName, hModule);
        result.to_raw()
    }
    pub extern "C" fn LoadLibraryA(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let filename = unsafe { <Option<&str>>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <Option<&str>>::stack_consumed();
        let result = winapi::kernel32::LoadLibraryA(machine, filename);
        result.to_raw()
    }
    pub extern "C" fn LoadLibraryExW(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let lpLibFileName =
            unsafe { <Option<Str16>>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <Option<Str16>>::stack_consumed();
        let hFile = unsafe { <HFILE>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <HFILE>::stack_consumed();
        let dwFlags = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::LoadLibraryExW(machine, lpLibFileName, hFile, dwFlags);
        result.to_raw()
    }
    pub extern "C" fn GetProcAddress(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let hModule = unsafe { <HMODULE>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <HMODULE>::stack_consumed();
        let lpProcName = unsafe { <Option<&str>>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <Option<&str>>::stack_consumed();
        let result = winapi::kernel32::GetProcAddress(machine, hModule, lpProcName);
        result.to_raw()
    }
    pub extern "C" fn GetStdHandle(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let nStdHandle = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::GetStdHandle(machine, nStdHandle);
        result.to_raw()
    }
    pub extern "C" fn CreateFileA(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let lpFileName = unsafe { <Option<&str>>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <Option<&str>>::stack_consumed();
        let dwDesiredAccess = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let dwShareMode = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let lpSecurityAttributes = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let dwCreationDisposition = unsafe {
            <Result<CreationDisposition, u32>>::from_stack(machine.mem(), esp + stack_offset)
        };
        stack_offset += <Result<CreationDisposition, u32>>::stack_consumed();
        let dwFlagsAndAttributes = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let hTemplateFile = unsafe { <HFILE>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <HFILE>::stack_consumed();
        let result = winapi::kernel32::CreateFileA(
            machine,
            lpFileName,
            dwDesiredAccess,
            dwShareMode,
            lpSecurityAttributes,
            dwCreationDisposition,
            dwFlagsAndAttributes,
            hTemplateFile,
        );
        result.to_raw()
    }
    pub extern "C" fn CreateFileW(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let lpFileName = unsafe { <Option<Str16>>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <Option<Str16>>::stack_consumed();
        let dwDesiredAccess = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let dwShareMode = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let lpSecurityAttributes = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let dwCreationDisposition = unsafe {
            <Result<CreationDisposition, u32>>::from_stack(machine.mem(), esp + stack_offset)
        };
        stack_offset += <Result<CreationDisposition, u32>>::stack_consumed();
        let dwFlagsAndAttributes = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let hTemplateFile = unsafe { <HFILE>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <HFILE>::stack_consumed();
        let result = winapi::kernel32::CreateFileW(
            machine,
            lpFileName,
            dwDesiredAccess,
            dwShareMode,
            lpSecurityAttributes,
            dwCreationDisposition,
            dwFlagsAndAttributes,
            hTemplateFile,
        );
        result.to_raw()
    }
    pub extern "C" fn GetFileType(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let hFile = unsafe { <HFILE>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <HFILE>::stack_consumed();
        let result = winapi::kernel32::GetFileType(machine, hFile);
        result.to_raw()
    }
    pub extern "C" fn SetFilePointer(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let hFile = unsafe { <HFILE>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <HFILE>::stack_consumed();
        let lDistanceToMove = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let lpDistanceToMoveHigh =
            unsafe { <Option<&mut u32>>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <Option<&mut u32>>::stack_consumed();
        let dwMoveMethod = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::SetFilePointer(
            machine,
            hFile,
            lDistanceToMove,
            lpDistanceToMoveHigh,
            dwMoveMethod,
        );
        result.to_raw()
    }
    pub extern "C" fn ReadFile(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let hFile = unsafe { <HFILE>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <HFILE>::stack_consumed();
        let lpBuffer =
            unsafe { <Option<&mut [u8]>>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <Option<&mut [u8]>>::stack_consumed();
        let lpNumberOfBytesRead =
            unsafe { <Option<&mut u32>>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <Option<&mut u32>>::stack_consumed();
        let lpOverlapped = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result =
            winapi::kernel32::ReadFile(machine, hFile, lpBuffer, lpNumberOfBytesRead, lpOverlapped);
        result.to_raw()
    }
    pub extern "C" fn WriteFile(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let hFile = unsafe { <HFILE>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <HFILE>::stack_consumed();
        let lpBuffer = unsafe { <Option<&[u8]>>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <Option<&[u8]>>::stack_consumed();
        let lpNumberOfBytesWritten =
            unsafe { <Option<&mut u32>>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <Option<&mut u32>>::stack_consumed();
        let lpOverlapped = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::WriteFile(
            machine,
            hFile,
            lpBuffer,
            lpNumberOfBytesWritten,
            lpOverlapped,
        );
        result.to_raw()
    }
    pub extern "C" fn HeapAlloc(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let hHeap = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let dwFlags = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let dwBytes = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::HeapAlloc(machine, hHeap, dwFlags, dwBytes);
        result.to_raw()
    }
    pub extern "C" fn HeapFree(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let hHeap = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let dwFlags = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let lpMem = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::HeapFree(machine, hHeap, dwFlags, lpMem);
        result.to_raw()
    }
    pub extern "C" fn HeapSize(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let hHeap = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let dwFlags = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let lpMem = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::HeapSize(machine, hHeap, dwFlags, lpMem);
        result.to_raw()
    }
    pub extern "C" fn HeapReAlloc(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let hHeap = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let dwFlags = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let lpMem = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let dwBytes = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::HeapReAlloc(machine, hHeap, dwFlags, lpMem, dwBytes);
        result.to_raw()
    }
    pub extern "C" fn HeapCreate(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let flOptions = unsafe {
            <Result<HeapCreateFlags, u32>>::from_stack(machine.mem(), esp + stack_offset)
        };
        stack_offset += <Result<HeapCreateFlags, u32>>::stack_consumed();
        let dwInitialSize = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let dwMaximumSize = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::HeapCreate(machine, flOptions, dwInitialSize, dwMaximumSize);
        result.to_raw()
    }
    pub extern "C" fn HeapDestroy(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let hHeap = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::HeapDestroy(machine, hHeap);
        result.to_raw()
    }
    pub extern "C" fn VirtualAlloc(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let lpAddress = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let dwSize = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let _flAllocationType = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let _flProtec = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::VirtualAlloc(
            machine,
            lpAddress,
            dwSize,
            _flAllocationType,
            _flProtec,
        );
        result.to_raw()
    }
    pub extern "C" fn VirtualFree(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let lpAddress = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let dwSize = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let dwFreeType = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::VirtualFree(machine, lpAddress, dwSize, dwFreeType);
        result.to_raw()
    }
    pub extern "C" fn IsBadReadPtr(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let lp = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let ucb = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::IsBadReadPtr(machine, lp, ucb);
        result.to_raw()
    }
    pub extern "C" fn IsBadWritePtr(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let lp = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let ucb = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::IsBadWritePtr(machine, lp, ucb);
        result.to_raw()
    }
    pub extern "C" fn SetLastError(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let dwErrCode = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::SetLastError(machine, dwErrCode);
        result.to_raw()
    }
    pub extern "C" fn GetLastError(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let result = winapi::kernel32::GetLastError(machine);
        result.to_raw()
    }
    pub extern "C" fn ExitProcess(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let uExitCode = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::ExitProcess(machine, uExitCode);
        result.to_raw()
    }
    pub extern "C" fn GetACP(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let result = winapi::kernel32::GetACP(machine);
        result.to_raw()
    }
    pub extern "C" fn IsValidCodePage(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let CodePage = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::IsValidCodePage(machine, CodePage);
        result.to_raw()
    }
    pub extern "C" fn GetCPInfo(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let _CodePage = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let _lpCPInfo = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::GetCPInfo(machine, _CodePage, _lpCPInfo);
        result.to_raw()
    }
    pub extern "C" fn GetCommandLineA(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let result = winapi::kernel32::GetCommandLineA(machine);
        result.to_raw()
    }
    pub extern "C" fn GetCommandLineW(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let result = winapi::kernel32::GetCommandLineW(machine);
        result.to_raw()
    }
    pub extern "C" fn GetEnvironmentStrings(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let result = winapi::kernel32::GetEnvironmentStrings(machine);
        result.to_raw()
    }
    pub extern "C" fn FreeEnvironmentStringsA(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let _penv = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::FreeEnvironmentStringsA(machine, _penv);
        result.to_raw()
    }
    pub extern "C" fn GetEnvironmentStringsW(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let result = winapi::kernel32::GetEnvironmentStringsW(machine);
        result.to_raw()
    }
    pub extern "C" fn GetEnvironmentVariableA(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let name = unsafe { <Option<&str>>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <Option<&str>>::stack_consumed();
        let buf = unsafe { <Option<&mut [u8]>>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <Option<&mut [u8]>>::stack_consumed();
        let result = winapi::kernel32::GetEnvironmentVariableA(machine, name, buf);
        result.to_raw()
    }
    pub extern "C" fn GetModuleFileNameA(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let hModule = unsafe { <HMODULE>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <HMODULE>::stack_consumed();
        let filename =
            unsafe { <Option<&mut [u8]>>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <Option<&mut [u8]>>::stack_consumed();
        let result = winapi::kernel32::GetModuleFileNameA(machine, hModule, filename);
        result.to_raw()
    }
    pub extern "C" fn GetModuleFileNameW(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let hModule = unsafe { <HMODULE>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <HMODULE>::stack_consumed();
        let _lpFilename = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let _nSize = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::GetModuleFileNameW(machine, hModule, _lpFilename, _nSize);
        result.to_raw()
    }
    pub extern "C" fn GetStartupInfoA(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let lpStartupInfo =
            unsafe { <Option<&mut STARTUPINFOA>>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <Option<&mut STARTUPINFOA>>::stack_consumed();
        let result = winapi::kernel32::GetStartupInfoA(machine, lpStartupInfo);
        result.to_raw()
    }
    pub extern "C" fn GetStartupInfoW(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let lpStartupInfo =
            unsafe { <Option<&mut STARTUPINFOA>>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <Option<&mut STARTUPINFOA>>::stack_consumed();
        let result = winapi::kernel32::GetStartupInfoW(machine, lpStartupInfo);
        result.to_raw()
    }
    pub extern "C" fn IsProcessorFeaturePresent(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let feature = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::IsProcessorFeaturePresent(machine, feature);
        result.to_raw()
    }
    pub extern "C" fn IsDebuggerPresent(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let result = winapi::kernel32::IsDebuggerPresent(machine);
        result.to_raw()
    }
    pub extern "C" fn GetCurrentProcessId(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let result = winapi::kernel32::GetCurrentProcessId(machine);
        result.to_raw()
    }
    pub extern "C" fn GetTickCount(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let result = winapi::kernel32::GetTickCount(machine);
        result.to_raw()
    }
    pub extern "C" fn QueryPerformanceCounter(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let lpPerformanceCount =
            unsafe { <Option<&mut LARGE_INTEGER>>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <Option<&mut LARGE_INTEGER>>::stack_consumed();
        let result = winapi::kernel32::QueryPerformanceCounter(machine, lpPerformanceCount);
        result.to_raw()
    }
    pub extern "C" fn QueryPerformanceFrequency(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let lpFrequency = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::QueryPerformanceFrequency(machine, lpFrequency);
        result.to_raw()
    }
    pub extern "C" fn GetSystemTimeAsFileTime(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let _time =
            unsafe { <Option<&mut FILETIME>>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <Option<&mut FILETIME>>::stack_consumed();
        let result = winapi::kernel32::GetSystemTimeAsFileTime(machine, _time);
        result.to_raw()
    }
    pub extern "C" fn GetVersion(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let result = winapi::kernel32::GetVersion(machine);
        result.to_raw()
    }
    pub extern "C" fn GetVersionExA(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let lpVersionInformation =
            unsafe { <Option<&mut OSVERSIONINFO>>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <Option<&mut OSVERSIONINFO>>::stack_consumed();
        let result = winapi::kernel32::GetVersionExA(machine, lpVersionInformation);
        result.to_raw()
    }
    pub extern "C" fn GetProcessHeap(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let result = winapi::kernel32::GetProcessHeap(machine);
        result.to_raw()
    }
    pub extern "C" fn SetHandleCount(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let uNumber = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::SetHandleCount(machine, uNumber);
        result.to_raw()
    }
    pub extern "C" fn OutputDebugStringA(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let msg = unsafe { <Option<&str>>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <Option<&str>>::stack_consumed();
        let result = winapi::kernel32::OutputDebugStringA(machine, msg);
        result.to_raw()
    }
    pub extern "C" fn InitializeCriticalSectionAndSpinCount(
        machine: &mut Machine,
        esp: u32,
    ) -> u32 {
        let mut stack_offset = 4u32;
        let _lpCriticalSection = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let _dwSpinCount = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::InitializeCriticalSectionAndSpinCount(
            machine,
            _lpCriticalSection,
            _dwSpinCount,
        );
        result.to_raw()
    }
    pub extern "C" fn DeleteCriticalSection(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let _lpCriticalSection = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::DeleteCriticalSection(machine, _lpCriticalSection);
        result.to_raw()
    }
    pub extern "C" fn EnterCriticalSection(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let _lpCriticalSection = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::EnterCriticalSection(machine, _lpCriticalSection);
        result.to_raw()
    }
    pub extern "C" fn LeaveCriticalSection(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let _lpCriticalSection = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::LeaveCriticalSection(machine, _lpCriticalSection);
        result.to_raw()
    }
    pub extern "C" fn SetUnhandledExceptionFilter(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let _lpTopLevelExceptionFilter =
            unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result =
            winapi::kernel32::SetUnhandledExceptionFilter(machine, _lpTopLevelExceptionFilter);
        result.to_raw()
    }
    pub extern "C" fn UnhandledExceptionFilter(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let _exceptionInfo = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::UnhandledExceptionFilter(machine, _exceptionInfo);
        result.to_raw()
    }
    pub extern "C" fn NtCurrentTeb(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let result = winapi::kernel32::NtCurrentTeb(machine);
        result.to_raw()
    }
    pub extern "C" fn InitializeSListHead(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let ListHead =
            unsafe { <Option<&mut SLIST_HEADER>>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <Option<&mut SLIST_HEADER>>::stack_consumed();
        let result = winapi::kernel32::InitializeSListHead(machine, ListHead);
        result.to_raw()
    }
    pub extern "C" fn MultiByteToWideChar(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let CodePage = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let dwFlags = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let lpMultiByteStr = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let cbMultiByte = unsafe { <i32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <i32>::stack_consumed();
        let lpWideCharStr =
            unsafe { <Option<&mut [u16]>>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <Option<&mut [u16]>>::stack_consumed();
        let result = winapi::kernel32::MultiByteToWideChar(
            machine,
            CodePage,
            dwFlags,
            lpMultiByteStr,
            cbMultiByte,
            lpWideCharStr,
        );
        result.to_raw()
    }
    pub extern "C" fn WriteConsoleW(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let hConsoleOutput = unsafe { <HFILE>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <HFILE>::stack_consumed();
        let lpBuffer = unsafe { <Option<&[u16]>>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <Option<&[u16]>>::stack_consumed();
        let lpNumberOfCharsWritten =
            unsafe { <Option<&mut u32>>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <Option<&mut u32>>::stack_consumed();
        let _lpReserved = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::WriteConsoleW(
            machine,
            hConsoleOutput,
            lpBuffer,
            lpNumberOfCharsWritten,
            _lpReserved,
        );
        result.to_raw()
    }
    pub extern "C" fn GetCurrentThreadId(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let result = winapi::kernel32::GetCurrentThreadId(machine);
        result.to_raw()
    }
    pub extern "C" fn TlsAlloc(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let result = winapi::kernel32::TlsAlloc(machine);
        result.to_raw()
    }
    pub extern "C" fn TlsFree(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let dwTlsIndex = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::TlsFree(machine, dwTlsIndex);
        result.to_raw()
    }
    pub extern "C" fn TlsSetValue(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let dwTlsIndex = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let lpTlsValue = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::TlsSetValue(machine, dwTlsIndex, lpTlsValue);
        result.to_raw()
    }
    pub extern "C" fn TlsGetValue(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let dwTlsIndex = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::TlsGetValue(machine, dwTlsIndex);
        result.to_raw()
    }
    pub extern "C" fn CreateThread(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let lpThreadAttributes = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let dwStackSize = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let lpStartAddress = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let lpParameter = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let dwCreationFlags = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let lpThreadId = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::CreateThread(
            machine,
            lpThreadAttributes,
            dwStackSize,
            lpStartAddress,
            lpParameter,
            dwCreationFlags,
            lpThreadId,
        );
        result.to_raw()
    }
    pub extern "C" fn SetThreadPriority(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let _hThread = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let _nPriority = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::SetThreadPriority(machine, _hThread, _nPriority);
        result.to_raw()
    }
    pub extern "C" fn InterlockedIncrement(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let addend = unsafe { <Option<&mut u32>>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <Option<&mut u32>>::stack_consumed();
        let result = winapi::kernel32::InterlockedIncrement(machine, addend);
        result.to_raw()
    }
    const EXPORTS: [Symbol; 69usize] = [
        Symbol {
            name: "GetModuleHandleA",
            ordinal: None,
            func: GetModuleHandleA,
            stack_consumed: || <Option<&str>>::stack_consumed(),
        },
        Symbol {
            name: "GetModuleHandleW",
            ordinal: None,
            func: GetModuleHandleW,
            stack_consumed: || <Option<Str16>>::stack_consumed(),
        },
        Symbol {
            name: "GetModuleHandleExW",
            ordinal: None,
            func: GetModuleHandleExW,
            stack_consumed: || {
                <u32>::stack_consumed()
                    + <Option<Str16>>::stack_consumed()
                    + <Option<&mut HMODULE>>::stack_consumed()
            },
        },
        Symbol {
            name: "LoadLibraryA",
            ordinal: None,
            func: LoadLibraryA,
            stack_consumed: || <Option<&str>>::stack_consumed(),
        },
        Symbol {
            name: "LoadLibraryExW",
            ordinal: None,
            func: LoadLibraryExW,
            stack_consumed: || {
                <Option<Str16>>::stack_consumed()
                    + <HFILE>::stack_consumed()
                    + <u32>::stack_consumed()
            },
        },
        Symbol {
            name: "GetProcAddress",
            ordinal: None,
            func: GetProcAddress,
            stack_consumed: || <HMODULE>::stack_consumed() + <Option<&str>>::stack_consumed(),
        },
        Symbol {
            name: "GetStdHandle",
            ordinal: None,
            func: GetStdHandle,
            stack_consumed: || <u32>::stack_consumed(),
        },
        Symbol {
            name: "CreateFileA",
            ordinal: None,
            func: CreateFileA,
            stack_consumed: || {
                <Option<&str>>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <Result<CreationDisposition, u32>>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <HFILE>::stack_consumed()
            },
        },
        Symbol {
            name: "CreateFileW",
            ordinal: None,
            func: CreateFileW,
            stack_consumed: || {
                <Option<Str16>>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <Result<CreationDisposition, u32>>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <HFILE>::stack_consumed()
            },
        },
        Symbol {
            name: "GetFileType",
            ordinal: None,
            func: GetFileType,
            stack_consumed: || <HFILE>::stack_consumed(),
        },
        Symbol {
            name: "SetFilePointer",
            ordinal: None,
            func: SetFilePointer,
            stack_consumed: || {
                <HFILE>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <Option<&mut u32>>::stack_consumed()
                    + <u32>::stack_consumed()
            },
        },
        Symbol {
            name: "ReadFile",
            ordinal: None,
            func: ReadFile,
            stack_consumed: || {
                <HFILE>::stack_consumed()
                    + <Option<&mut [u8]>>::stack_consumed()
                    + <Option<&mut u32>>::stack_consumed()
                    + <u32>::stack_consumed()
            },
        },
        Symbol {
            name: "WriteFile",
            ordinal: None,
            func: WriteFile,
            stack_consumed: || {
                <HFILE>::stack_consumed()
                    + <Option<&[u8]>>::stack_consumed()
                    + <Option<&mut u32>>::stack_consumed()
                    + <u32>::stack_consumed()
            },
        },
        Symbol {
            name: "HeapAlloc",
            ordinal: None,
            func: HeapAlloc,
            stack_consumed: || {
                <u32>::stack_consumed() + <u32>::stack_consumed() + <u32>::stack_consumed()
            },
        },
        Symbol {
            name: "HeapFree",
            ordinal: None,
            func: HeapFree,
            stack_consumed: || {
                <u32>::stack_consumed() + <u32>::stack_consumed() + <u32>::stack_consumed()
            },
        },
        Symbol {
            name: "HeapSize",
            ordinal: None,
            func: HeapSize,
            stack_consumed: || {
                <u32>::stack_consumed() + <u32>::stack_consumed() + <u32>::stack_consumed()
            },
        },
        Symbol {
            name: "HeapReAlloc",
            ordinal: None,
            func: HeapReAlloc,
            stack_consumed: || {
                <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
            },
        },
        Symbol {
            name: "HeapCreate",
            ordinal: None,
            func: HeapCreate,
            stack_consumed: || {
                <Result<HeapCreateFlags, u32>>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
            },
        },
        Symbol {
            name: "HeapDestroy",
            ordinal: None,
            func: HeapDestroy,
            stack_consumed: || <u32>::stack_consumed(),
        },
        Symbol {
            name: "VirtualAlloc",
            ordinal: None,
            func: VirtualAlloc,
            stack_consumed: || {
                <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
            },
        },
        Symbol {
            name: "VirtualFree",
            ordinal: None,
            func: VirtualFree,
            stack_consumed: || {
                <u32>::stack_consumed() + <u32>::stack_consumed() + <u32>::stack_consumed()
            },
        },
        Symbol {
            name: "IsBadReadPtr",
            ordinal: None,
            func: IsBadReadPtr,
            stack_consumed: || <u32>::stack_consumed() + <u32>::stack_consumed(),
        },
        Symbol {
            name: "IsBadWritePtr",
            ordinal: None,
            func: IsBadWritePtr,
            stack_consumed: || <u32>::stack_consumed() + <u32>::stack_consumed(),
        },
        Symbol {
            name: "SetLastError",
            ordinal: None,
            func: SetLastError,
            stack_consumed: || <u32>::stack_consumed(),
        },
        Symbol {
            name: "GetLastError",
            ordinal: None,
            func: GetLastError,
            stack_consumed: || 0,
        },
        Symbol {
            name: "ExitProcess",
            ordinal: None,
            func: ExitProcess,
            stack_consumed: || <u32>::stack_consumed(),
        },
        Symbol {
            name: "GetACP",
            ordinal: None,
            func: GetACP,
            stack_consumed: || 0,
        },
        Symbol {
            name: "IsValidCodePage",
            ordinal: None,
            func: IsValidCodePage,
            stack_consumed: || <u32>::stack_consumed(),
        },
        Symbol {
            name: "GetCPInfo",
            ordinal: None,
            func: GetCPInfo,
            stack_consumed: || <u32>::stack_consumed() + <u32>::stack_consumed(),
        },
        Symbol {
            name: "GetCommandLineA",
            ordinal: None,
            func: GetCommandLineA,
            stack_consumed: || 0,
        },
        Symbol {
            name: "GetCommandLineW",
            ordinal: None,
            func: GetCommandLineW,
            stack_consumed: || 0,
        },
        Symbol {
            name: "GetEnvironmentStrings",
            ordinal: None,
            func: GetEnvironmentStrings,
            stack_consumed: || 0,
        },
        Symbol {
            name: "FreeEnvironmentStringsA",
            ordinal: None,
            func: FreeEnvironmentStringsA,
            stack_consumed: || <u32>::stack_consumed(),
        },
        Symbol {
            name: "GetEnvironmentStringsW",
            ordinal: None,
            func: GetEnvironmentStringsW,
            stack_consumed: || 0,
        },
        Symbol {
            name: "GetEnvironmentVariableA",
            ordinal: None,
            func: GetEnvironmentVariableA,
            stack_consumed: || {
                <Option<&str>>::stack_consumed() + <Option<&mut [u8]>>::stack_consumed()
            },
        },
        Symbol {
            name: "GetModuleFileNameA",
            ordinal: None,
            func: GetModuleFileNameA,
            stack_consumed: || <HMODULE>::stack_consumed() + <Option<&mut [u8]>>::stack_consumed(),
        },
        Symbol {
            name: "GetModuleFileNameW",
            ordinal: None,
            func: GetModuleFileNameW,
            stack_consumed: || {
                <HMODULE>::stack_consumed() + <u32>::stack_consumed() + <u32>::stack_consumed()
            },
        },
        Symbol {
            name: "GetStartupInfoA",
            ordinal: None,
            func: GetStartupInfoA,
            stack_consumed: || <Option<&mut STARTUPINFOA>>::stack_consumed(),
        },
        Symbol {
            name: "GetStartupInfoW",
            ordinal: None,
            func: GetStartupInfoW,
            stack_consumed: || <Option<&mut STARTUPINFOA>>::stack_consumed(),
        },
        Symbol {
            name: "IsProcessorFeaturePresent",
            ordinal: None,
            func: IsProcessorFeaturePresent,
            stack_consumed: || <u32>::stack_consumed(),
        },
        Symbol {
            name: "IsDebuggerPresent",
            ordinal: None,
            func: IsDebuggerPresent,
            stack_consumed: || 0,
        },
        Symbol {
            name: "GetCurrentProcessId",
            ordinal: None,
            func: GetCurrentProcessId,
            stack_consumed: || 0,
        },
        Symbol {
            name: "GetTickCount",
            ordinal: None,
            func: GetTickCount,
            stack_consumed: || 0,
        },
        Symbol {
            name: "QueryPerformanceCounter",
            ordinal: None,
            func: QueryPerformanceCounter,
            stack_consumed: || <Option<&mut LARGE_INTEGER>>::stack_consumed(),
        },
        Symbol {
            name: "QueryPerformanceFrequency",
            ordinal: None,
            func: QueryPerformanceFrequency,
            stack_consumed: || <u32>::stack_consumed(),
        },
        Symbol {
            name: "GetSystemTimeAsFileTime",
            ordinal: None,
            func: GetSystemTimeAsFileTime,
            stack_consumed: || <Option<&mut FILETIME>>::stack_consumed(),
        },
        Symbol {
            name: "GetVersion",
            ordinal: None,
            func: GetVersion,
            stack_consumed: || 0,
        },
        Symbol {
            name: "GetVersionExA",
            ordinal: None,
            func: GetVersionExA,
            stack_consumed: || <Option<&mut OSVERSIONINFO>>::stack_consumed(),
        },
        Symbol {
            name: "GetProcessHeap",
            ordinal: None,
            func: GetProcessHeap,
            stack_consumed: || 0,
        },
        Symbol {
            name: "SetHandleCount",
            ordinal: None,
            func: SetHandleCount,
            stack_consumed: || <u32>::stack_consumed(),
        },
        Symbol {
            name: "OutputDebugStringA",
            ordinal: None,
            func: OutputDebugStringA,
            stack_consumed: || <Option<&str>>::stack_consumed(),
        },
        Symbol {
            name: "InitializeCriticalSectionAndSpinCount",
            ordinal: None,
            func: InitializeCriticalSectionAndSpinCount,
            stack_consumed: || <u32>::stack_consumed() + <u32>::stack_consumed(),
        },
        Symbol {
            name: "DeleteCriticalSection",
            ordinal: None,
            func: DeleteCriticalSection,
            stack_consumed: || <u32>::stack_consumed(),
        },
        Symbol {
            name: "EnterCriticalSection",
            ordinal: None,
            func: EnterCriticalSection,
            stack_consumed: || <u32>::stack_consumed(),
        },
        Symbol {
            name: "LeaveCriticalSection",
            ordinal: None,
            func: LeaveCriticalSection,
            stack_consumed: || <u32>::stack_consumed(),
        },
        Symbol {
            name: "SetUnhandledExceptionFilter",
            ordinal: None,
            func: SetUnhandledExceptionFilter,
            stack_consumed: || <u32>::stack_consumed(),
        },
        Symbol {
            name: "UnhandledExceptionFilter",
            ordinal: None,
            func: UnhandledExceptionFilter,
            stack_consumed: || <u32>::stack_consumed(),
        },
        Symbol {
            name: "NtCurrentTeb",
            ordinal: None,
            func: NtCurrentTeb,
            stack_consumed: || 0,
        },
        Symbol {
            name: "InitializeSListHead",
            ordinal: None,
            func: InitializeSListHead,
            stack_consumed: || <Option<&mut SLIST_HEADER>>::stack_consumed(),
        },
        Symbol {
            name: "MultiByteToWideChar",
            ordinal: None,
            func: MultiByteToWideChar,
            stack_consumed: || {
                <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <i32>::stack_consumed()
                    + <Option<&mut [u16]>>::stack_consumed()
            },
        },
        Symbol {
            name: "WriteConsoleW",
            ordinal: None,
            func: WriteConsoleW,
            stack_consumed: || {
                <HFILE>::stack_consumed()
                    + <Option<&[u16]>>::stack_consumed()
                    + <Option<&mut u32>>::stack_consumed()
                    + <u32>::stack_consumed()
            },
        },
        Symbol {
            name: "GetCurrentThreadId",
            ordinal: None,
            func: GetCurrentThreadId,
            stack_consumed: || 0,
        },
        Symbol {
            name: "TlsAlloc",
            ordinal: None,
            func: TlsAlloc,
            stack_consumed: || 0,
        },
        Symbol {
            name: "TlsFree",
            ordinal: None,
            func: TlsFree,
            stack_consumed: || <u32>::stack_consumed(),
        },
        Symbol {
            name: "TlsSetValue",
            ordinal: None,
            func: TlsSetValue,
            stack_consumed: || <u32>::stack_consumed() + <u32>::stack_consumed(),
        },
        Symbol {
            name: "TlsGetValue",
            ordinal: None,
            func: TlsGetValue,
            stack_consumed: || <u32>::stack_consumed(),
        },
        Symbol {
            name: "CreateThread",
            ordinal: None,
            func: CreateThread,
            stack_consumed: || {
                <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
            },
        },
        Symbol {
            name: "SetThreadPriority",
            ordinal: None,
            func: SetThreadPriority,
            stack_consumed: || <u32>::stack_consumed() + <u32>::stack_consumed(),
        },
        Symbol {
            name: "InterlockedIncrement",
            ordinal: None,
            func: InterlockedIncrement,
            stack_consumed: || <Option<&mut u32>>::stack_consumed(),
        },
    ];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "kernel32.dll",
        exports: &EXPORTS,
    };
}
pub mod ole32 {
    use super::*;
    use winapi::ole32::*;
    const EXPORTS: [Symbol; 0usize] = [];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "ole32.dll",
        exports: &EXPORTS,
    };
}
pub mod oleaut32 {
    use super::*;
    use winapi::oleaut32::*;
    const EXPORTS: [Symbol; 0usize] = [];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "oleaut32.dll",
        exports: &EXPORTS,
    };
}
pub mod user32 {
    use super::*;
    use winapi::user32::*;
    pub extern "C" fn RegisterClassA(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let lpWndClass =
            unsafe { <Option<&WNDCLASSA>>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <Option<&WNDCLASSA>>::stack_consumed();
        let result = winapi::user32::RegisterClassA(machine, lpWndClass);
        result.to_raw()
    }
    pub extern "C" fn RegisterClassExA(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let lpWndClassEx =
            unsafe { <Option<&WNDCLASSEXA>>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <Option<&WNDCLASSEXA>>::stack_consumed();
        let result = winapi::user32::RegisterClassExA(machine, lpWndClassEx);
        result.to_raw()
    }
    pub extern "C" fn CreateWindowExA(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let dwExStyle = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let lpClassName = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let lpWindowName = unsafe { <Option<&str>>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <Option<&str>>::stack_consumed();
        let dwStyle =
            unsafe { <Result<WindowStyle, u32>>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <Result<WindowStyle, u32>>::stack_consumed();
        let X = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let Y = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let nWidth = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let nHeight = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let hWndParent = unsafe { <HWND>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <HWND>::stack_consumed();
        let hMenu = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let hInstance = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let lpParam = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let m: *mut Machine = machine;
        let result = async move {
            let machine = unsafe { &mut *m };
            let result = winapi::user32::CreateWindowExA(
                machine,
                dwExStyle,
                lpClassName,
                lpWindowName,
                dwStyle,
                X,
                Y,
                nWidth,
                nHeight,
                hWndParent,
                hMenu,
                hInstance,
                lpParam,
            )
            .await;
            result.to_raw()
        };
        crate::shims::become_async(machine, Box::pin(result));
        0
    }
    pub extern "C" fn GetForegroundWindow(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let result = winapi::user32::GetForegroundWindow(machine);
        result.to_raw()
    }
    pub extern "C" fn GetActiveWindow(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let result = winapi::user32::GetActiveWindow(machine);
        result.to_raw()
    }
    pub extern "C" fn GetLastActivePopup(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let result = winapi::user32::GetLastActivePopup(machine);
        result.to_raw()
    }
    pub extern "C" fn UpdateWindow(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let hWnd = unsafe { <HWND>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <HWND>::stack_consumed();
        let result = winapi::user32::UpdateWindow(machine, hWnd);
        result.to_raw()
    }
    pub extern "C" fn ShowWindow(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let hWnd = unsafe { <HWND>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <HWND>::stack_consumed();
        let _nCmdShow = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::user32::ShowWindow(machine, hWnd, _nCmdShow);
        result.to_raw()
    }
    pub extern "C" fn SetFocus(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let hWnd = unsafe { <HWND>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <HWND>::stack_consumed();
        let result = winapi::user32::SetFocus(machine, hWnd);
        result.to_raw()
    }
    pub extern "C" fn SetCursor(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let hCursor = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::user32::SetCursor(machine, hCursor);
        result.to_raw()
    }
    pub extern "C" fn MessageBoxA(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let hWnd = unsafe { <HWND>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <HWND>::stack_consumed();
        let lpText = unsafe { <Option<&str>>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <Option<&str>>::stack_consumed();
        let lpCaption = unsafe { <Option<&str>>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <Option<&str>>::stack_consumed();
        let uType = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::user32::MessageBoxA(machine, hWnd, lpText, lpCaption, uType);
        result.to_raw()
    }
    pub extern "C" fn DialogBoxParamA(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let hInstance = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let lpTemplateName = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let hWndParent = unsafe { <HWND>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <HWND>::stack_consumed();
        let lpDialogFunc = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let dwInitParam = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::user32::DialogBoxParamA(
            machine,
            hInstance,
            lpTemplateName,
            hWndParent,
            lpDialogFunc,
            dwInitParam,
        );
        result.to_raw()
    }
    pub extern "C" fn PeekMessageA(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let lpMsg = unsafe { <Option<&mut MSG>>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <Option<&mut MSG>>::stack_consumed();
        let hWnd = unsafe { <HWND>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <HWND>::stack_consumed();
        let wMsgFilterMin = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let wMsgFilterMax = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let wRemoveMsg =
            unsafe { <Result<RemoveMsg, u32>>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <Result<RemoveMsg, u32>>::stack_consumed();
        let result = winapi::user32::PeekMessageA(
            machine,
            lpMsg,
            hWnd,
            wMsgFilterMin,
            wMsgFilterMax,
            wRemoveMsg,
        );
        result.to_raw()
    }
    pub extern "C" fn GetMessageA(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let lpMsg = unsafe { <Option<&mut MSG>>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <Option<&mut MSG>>::stack_consumed();
        let hWnd = unsafe { <HWND>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <HWND>::stack_consumed();
        let wMsgFilterMin = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let wMsgFilterMax = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result =
            winapi::user32::GetMessageA(machine, lpMsg, hWnd, wMsgFilterMin, wMsgFilterMax);
        result.to_raw()
    }
    pub extern "C" fn WaitMessage(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let result = winapi::user32::WaitMessage(machine);
        result.to_raw()
    }
    pub extern "C" fn TranslateMessage(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let lpMsg = unsafe { <Option<&MSG>>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <Option<&MSG>>::stack_consumed();
        let result = winapi::user32::TranslateMessage(machine, lpMsg);
        result.to_raw()
    }
    pub extern "C" fn DispatchMessageA(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let lpMsg = unsafe { <Option<&MSG>>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <Option<&MSG>>::stack_consumed();
        let m: *mut Machine = machine;
        let result = async move {
            let machine = unsafe { &mut *m };
            let result = winapi::user32::DispatchMessageA(machine, lpMsg).await;
            result.to_raw()
        };
        crate::shims::become_async(machine, Box::pin(result));
        0
    }
    pub extern "C" fn DefWindowProcA(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let hWnd = unsafe { <HWND>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <HWND>::stack_consumed();
        let msg = unsafe { <Result<WM, u32>>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <Result<WM, u32>>::stack_consumed();
        let wParam = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let lParam = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::user32::DefWindowProcA(machine, hWnd, msg, wParam, lParam);
        result.to_raw()
    }
    pub extern "C" fn LoadIconA(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let _hInstance = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let _lpIconName = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::user32::LoadIconA(machine, _hInstance, _lpIconName);
        result.to_raw()
    }
    pub extern "C" fn LoadCursorA(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let _hInstance = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let _lpCursorName = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::user32::LoadCursorA(machine, _hInstance, _lpCursorName);
        result.to_raw()
    }
    pub extern "C" fn ShowCursor(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let _bShow = unsafe { <bool>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <bool>::stack_consumed();
        let result = winapi::user32::ShowCursor(machine, _bShow);
        result.to_raw()
    }
    pub extern "C" fn LoadImageA(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let hInstance = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let name = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let typ = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let cx = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let cy = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let fuLoad = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::user32::LoadImageA(machine, hInstance, name, typ, cx, cy, fuLoad);
        result.to_raw()
    }
    pub extern "C" fn GetSystemMetrics(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let nIndex = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::user32::GetSystemMetrics(machine, nIndex);
        result.to_raw()
    }
    const EXPORTS: [Symbol; 23usize] = [
        Symbol {
            name: "RegisterClassA",
            ordinal: None,
            func: RegisterClassA,
            stack_consumed: || <Option<&WNDCLASSA>>::stack_consumed(),
        },
        Symbol {
            name: "RegisterClassExA",
            ordinal: None,
            func: RegisterClassExA,
            stack_consumed: || <Option<&WNDCLASSEXA>>::stack_consumed(),
        },
        Symbol {
            name: "CreateWindowExA",
            ordinal: None,
            func: CreateWindowExA,
            stack_consumed: || {
                <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <Option<&str>>::stack_consumed()
                    + <Result<WindowStyle, u32>>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <HWND>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
            },
        },
        Symbol {
            name: "GetForegroundWindow",
            ordinal: None,
            func: GetForegroundWindow,
            stack_consumed: || 0,
        },
        Symbol {
            name: "GetActiveWindow",
            ordinal: None,
            func: GetActiveWindow,
            stack_consumed: || 0,
        },
        Symbol {
            name: "GetLastActivePopup",
            ordinal: None,
            func: GetLastActivePopup,
            stack_consumed: || 0,
        },
        Symbol {
            name: "UpdateWindow",
            ordinal: None,
            func: UpdateWindow,
            stack_consumed: || <HWND>::stack_consumed(),
        },
        Symbol {
            name: "ShowWindow",
            ordinal: None,
            func: ShowWindow,
            stack_consumed: || <HWND>::stack_consumed() + <u32>::stack_consumed(),
        },
        Symbol {
            name: "SetFocus",
            ordinal: None,
            func: SetFocus,
            stack_consumed: || <HWND>::stack_consumed(),
        },
        Symbol {
            name: "SetCursor",
            ordinal: None,
            func: SetCursor,
            stack_consumed: || <u32>::stack_consumed(),
        },
        Symbol {
            name: "MessageBoxA",
            ordinal: None,
            func: MessageBoxA,
            stack_consumed: || {
                <HWND>::stack_consumed()
                    + <Option<&str>>::stack_consumed()
                    + <Option<&str>>::stack_consumed()
                    + <u32>::stack_consumed()
            },
        },
        Symbol {
            name: "DialogBoxParamA",
            ordinal: None,
            func: DialogBoxParamA,
            stack_consumed: || {
                <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <HWND>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
            },
        },
        Symbol {
            name: "PeekMessageA",
            ordinal: None,
            func: PeekMessageA,
            stack_consumed: || {
                <Option<&mut MSG>>::stack_consumed()
                    + <HWND>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <Result<RemoveMsg, u32>>::stack_consumed()
            },
        },
        Symbol {
            name: "GetMessageA",
            ordinal: None,
            func: GetMessageA,
            stack_consumed: || {
                <Option<&mut MSG>>::stack_consumed()
                    + <HWND>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
            },
        },
        Symbol {
            name: "WaitMessage",
            ordinal: None,
            func: WaitMessage,
            stack_consumed: || 0,
        },
        Symbol {
            name: "TranslateMessage",
            ordinal: None,
            func: TranslateMessage,
            stack_consumed: || <Option<&MSG>>::stack_consumed(),
        },
        Symbol {
            name: "DispatchMessageA",
            ordinal: None,
            func: DispatchMessageA,
            stack_consumed: || <Option<&MSG>>::stack_consumed(),
        },
        Symbol {
            name: "DefWindowProcA",
            ordinal: None,
            func: DefWindowProcA,
            stack_consumed: || {
                <HWND>::stack_consumed()
                    + <Result<WM, u32>>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
            },
        },
        Symbol {
            name: "LoadIconA",
            ordinal: None,
            func: LoadIconA,
            stack_consumed: || <u32>::stack_consumed() + <u32>::stack_consumed(),
        },
        Symbol {
            name: "LoadCursorA",
            ordinal: None,
            func: LoadCursorA,
            stack_consumed: || <u32>::stack_consumed() + <u32>::stack_consumed(),
        },
        Symbol {
            name: "ShowCursor",
            ordinal: None,
            func: ShowCursor,
            stack_consumed: || <bool>::stack_consumed(),
        },
        Symbol {
            name: "LoadImageA",
            ordinal: None,
            func: LoadImageA,
            stack_consumed: || {
                <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
            },
        },
        Symbol {
            name: "GetSystemMetrics",
            ordinal: None,
            func: GetSystemMetrics,
            stack_consumed: || <u32>::stack_consumed(),
        },
    ];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "user32.dll",
        exports: &EXPORTS,
    };
}
pub mod winmm {
    use super::*;
    use winapi::winmm::*;
    pub extern "C" fn timeSetEvent(machine: &mut Machine, esp: u32) -> u32 {
        let mut stack_offset = 4u32;
        let _uDelay = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let _uResolution = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let _lpTimeProc = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let _dwUser = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let _fuEvent = unsafe { <u32>::from_stack(machine.mem(), esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::winmm::timeSetEvent(
            machine,
            _uDelay,
            _uResolution,
            _lpTimeProc,
            _dwUser,
            _fuEvent,
        );
        result.to_raw()
    }
    const EXPORTS: [Symbol; 1usize] = [Symbol {
        name: "timeSetEvent",
        ordinal: None,
        func: timeSetEvent,
        stack_consumed: || {
            <u32>::stack_consumed()
                + <u32>::stack_consumed()
                + <u32>::stack_consumed()
                + <u32>::stack_consumed()
                + <u32>::stack_consumed()
        },
    }];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "winmm.dll",
        exports: &EXPORTS,
    };
}
