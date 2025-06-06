use super::{HEVENT, Thread, command_line, init::init_peb, loader::Module};
use crate::loader;
use std::{collections::HashMap, rc::Rc, sync::Arc};
use win32_system::{Event, System, generic_get_state, memory::Memory};
use win32_winapi::{HANDLE, HMODULE, Handles};

/// Objects identified by kernel handles, all of which can be passed to Wait* functions.
pub enum KernelObject {
    Event(Arc<Event>),
    Thread(Rc<Thread>),
}

impl Clone for KernelObject {
    fn clone(&self) -> Self {
        match self {
            KernelObject::Event(ev) => KernelObject::Event(ev.clone()),
            KernelObject::Thread(th) => KernelObject::Thread(th.clone()),
        }
    }
}

type KernelObjects = Handles<HANDLE<()>, KernelObject>;
pub trait KernelObjectsMethods {
    fn get_event(&self, handle: HEVENT) -> Option<&Event>;
}
impl KernelObjectsMethods for KernelObjects {
    fn get_event(&self, handle: HEVENT) -> Option<&Event> {
        match self.get_raw(handle.to_raw()) {
            Some(KernelObject::Event(ev)) => Some(ev),
            _ => None,
        }
    }
}

/// State held via the sys.get_state() interface.
#[derive(Default)]
pub struct State {
    /// If true, debug break when entering the exe entry point.
    pub break_on_startup: bool,

    /// Address image was loaded at.
    pub image_base: u32,

    /// Address of PEB (process information exposed to executable).
    pub peb: u32,

    /// Process command line.
    pub cmdline: command_line::State,

    pub modules: HashMap<HMODULE, Module>,

    // There is a collection of handle types that are all from the same key space,
    // because they can be passed to the various Wait functions.
    pub objects: Handles<HANDLE<()>, KernelObject>,
}

impl State {
    pub fn init_process(&mut self, memory: &Memory, retrowin32_syscall: &[u8], cmdline: String) {
        let cmdline = command_line::State::new(cmdline);
        let syscall_addr = init_peb(self, memory, retrowin32_syscall, cmdline);
        let module = retrowin32_dll_module(syscall_addr);
        self.modules
            .insert(HMODULE::from_raw(module.image_base), module);
    }
}

#[inline]
pub fn get_state(sys: &dyn System) -> std::cell::RefMut<State> {
    generic_get_state::<State>(sys)
}

/// Return the Module for the magic retrowin32.dll module, which
/// provides the retrowin32_syscall function needed for x86<->win32 calls.
fn retrowin32_dll_module(addr: u32) -> Module {
    let names = HashMap::from([("retrowin32_syscall".into(), addr)]);
    let exports = loader::Exports {
        names,
        ..Default::default()
    };
    Module {
        name: "retrowin32.dll".into(),
        // use a non-zero base address so it doesn't register as the null HMODULE
        image_base: 1,
        exports,
        ..Default::default() // rest of fields unused
    }
}
