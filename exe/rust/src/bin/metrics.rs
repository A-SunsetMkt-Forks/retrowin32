//! Dump GetSystemMetrics() values.

#![no_main]
#![no_std]
#![windows_subsystem = "console"]

use exe::{print::print, println};
use windows_sys::Win32::UI::WindowsAndMessaging::GetSystemMetrics;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mainCRTStartup() {
    print(b"GetSystemMetrics():\r\n");
    for i in 0..100 {
        let metric = unsafe { GetSystemMetrics(i) };
        println!("{} => {}", i, metric);
    }
}
