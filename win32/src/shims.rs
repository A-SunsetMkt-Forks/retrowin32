//! "Shims" are my word for the mechanism for x86 -> retrowin32 (and back) calls.
//!
//! In the simple case, we register Rust functions like kernel32.dll!ExitProcess
//! to associate with a special invalid x86 address.  If the x86 ever jumps to such an
//! address, we forward the call to the registered shim handler.
//!
//! The win32_derive::dllexport attribute on our shim functions wraps them with
//! a prologue/epilogue that does the required stack manipulation to read
//! arguments off the x86 stack and transform them into Rust types, so the Rust
//! functions can act as if they're just being called from Rust.
//!
//! The complex case is when our Rust function needs to call back into x86
//! code.  See `future.rs` for more.

use crate::Machine;

#[cfg(feature = "cpuemu")]
pub use crate::shims_cpuemu::{async_call, become_async, Shims};
#[cfg(not(feature = "cpuemu"))]
pub use crate::shims_raw::{async_call, become_async, Shims};

pub type Handler = unsafe extern "C" fn(&mut Machine, u32) -> u32;

#[derive(Clone)]
pub struct Shim {
    pub name: &'static str,
    pub func: Handler,
    pub stack_consumed: Option<u32>,
}
