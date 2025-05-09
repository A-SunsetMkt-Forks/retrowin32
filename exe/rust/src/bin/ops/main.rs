//! Exercise CPU operations at a low level, dumping CPU state.

#![no_main]
#![no_std]
#![windows_subsystem = "console"]

use exe::{print, println};
mod fpu;

#[inline(never)]
fn print_flags(reg: u32) {
    for (bit, name) in [
        (0, "CF"),
        (2, "PF"),
        (6, "ZF"),
        (7, "SF"),
        (10, "DF"),
        (11, "OF"),
    ] {
        if (reg >> bit) & 1 == 1 {
            print!(" {}", name);
        }
    }
}

#[inline(never)]
#[allow(asm_sub_register)] // TODO: this is just because rust-analyzer is confused
fn flags_test() {
    let mut val = 0u8;
    let mut flags: u32 = 0;
    unsafe {
        core::arch::asm!(
            "push 0",
            "popfd",
            "cmp {x}, {y}",
            "pushfd",
            "pop dword ptr [{flags}]",
            "setle [{val}]",
            x = in(reg) 1u32,
            y = in(reg) 1u32,
            flags = in(reg) &mut flags,
            val = in(reg) &mut val,
        );
    }

    print!("flags:");
    print_flags(flags);
    println!();

    if val != 0 {
        println!("setle: true");
    } else {
        println!("setle: false");
    }
}

fn bs_test() {
    let values = [0u32, 0b1, 0b110, 0b100100];
    for value in values {
        let mut bsf: u32;
        let mut bsr: u32;
        unsafe {
            core::arch::asm!(
                "bsf {bsf}, {value}",
                "bsr {bsr}, {value}",
                value = in(reg) value,
                bsf = out(reg) bsf,
                bsr = out(reg) bsr,
            );
        }
        println!("bsf {value:x}: {bsf:x}");
        println!("bsr {value:x}: {bsr:x}");
    }
}

fn tzcnt_test() {
    let values = [0u32, 0b1, 0b11000, 0b1000_0000_0000_0000];
    for value in values {
        let mut tzcnt: u32;
        unsafe {
            core::arch::asm!(
                "tzcnt {tzcnt}, {value}",
                value = in(reg) value,
                tzcnt = out(reg) tzcnt,
            );
        }
        println!("tzcnt {value:x}: {tzcnt:x}");
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn mainCRTStartup() {
    flags_test();
    fpu::test();
    bs_test();
    tzcnt_test();
}
