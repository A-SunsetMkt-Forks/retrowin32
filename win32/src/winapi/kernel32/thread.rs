use super::{KernelObject, peb_mut};
use crate::{
    Machine, System,
    winapi::{HANDLE, Str16, arena::Arena},
};
use memory::{Extensions, Mem};
use std::{rc::Rc, sync::Arc};
use win32_system::Event;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct HTHREADT;
/// state.objects[HTHREAD] maps to a Thread object.
pub type HTHREAD = HANDLE<HTHREADT>;

#[repr(C)]
struct _EXCEPTION_REGISTRATION_RECORD {
    Prev: u32,
    Handler: u32,
}
unsafe impl ::memory::Pod for _EXCEPTION_REGISTRATION_RECORD {}

#[repr(C)]
pub struct NT_TIB {
    ExceptionList: u32,
    StackBase: u32,
    StackLimit: u32,
    SubSystemTib: u32,
    FiberData: u32,
    ArbitraryUserPointer: u32,
    _Self: u32,
}
unsafe impl ::memory::Pod for NT_TIB {}

#[repr(C)]
pub struct TEB {
    pub Tib: NT_TIB,
    pub EnvironmentPointer: u32,
    pub ClientId_UniqueProcess: u32,
    pub ClientId_UniqueThread: u32,
    pub ActiveRpcHandle: u32,
    pub ThreadLocalStoragePointer: u32,
    pub Peb: u32,
    pub LastErrorValue: u32,
    pub CountOfOwnedCriticalSections: u32,
    pub CsrClientThread: u32,
    pub Win32ThreadInfo: u32,
    pub User32Reserved: [u32; 26],
    pub UserReserved: [u32; 5],
    pub WOW32Reserved: u32,
    pub CurrentLocale: u32,
    // TODO: ... there are many more fields here

    // This is at the wrong offset, but it shouldn't matter.
    pub TlsSlots: [u32; 64],
}
unsafe impl ::memory::Pod for TEB {}

pub struct Thread {
    /// Entry in kernel32.objects.
    pub handle: HTHREAD,

    /// address of TEB
    pub teb: u32,

    pub terminated: Arc<Event>,
}

/// Set up TEB, PEB, and other process info.
/// The FS register points at the TEB (thread info), which points at the PEB (process info).
fn init_teb(peb_addr: u32, thread_id: u32, arena: &mut Arena, mem: Mem) -> u32 {
    // SEH chain
    let seh_addr = arena.alloc(
        std::mem::size_of::<_EXCEPTION_REGISTRATION_RECORD>() as u32,
        4,
    );
    let seh = mem.get_aligned_ref_mut::<_EXCEPTION_REGISTRATION_RECORD>(seh_addr);
    seh.Prev = 0xFFFF_FFFF;
    seh.Handler = 0xFF5E_5EFF; // Hopefully easier to spot.

    // TEB
    let teb_addr = arena.alloc(std::cmp::max(std::mem::size_of::<TEB>() as u32, 0x100), 4);
    let teb = mem.get_aligned_ref_mut::<TEB>(teb_addr);
    teb.Tib.ExceptionList = seh_addr;
    teb.Tib._Self = teb_addr; // Confusing: it points to itself.
    teb.Peb = peb_addr;
    teb.ClientId_UniqueThread = thread_id;

    teb_addr
}

/// Information about a newly-created thread.
/// Info that persists after the thread is created is kept in Thread.
pub struct NewThread {
    pub thread: Rc<Thread>,
    /// initial esp
    pub stack_pointer: u32,
}

pub fn create_thread(machine: &mut Machine, stack_size: u32) -> NewThread {
    let handle = machine.state.kernel32.objects.reserve();

    let stack = machine.memory.mappings.alloc(
        machine.memory.imp.mem(),
        stack_size,
        format!("thread {handle:x} stack", handle = handle.to_raw()),
    );
    let stack_pointer = stack.addr + stack.size - 4;

    let mem = machine.memory.mem();
    let teb = init_teb(
        machine.state.kernel32.peb,
        handle.to_raw(),
        &mut machine.state.kernel32.arena,
        mem,
    );

    let thread = Rc::new(Thread {
        handle: HTHREAD::from_raw(handle.to_raw()),
        teb,
        terminated: Event::new(None, false, false),
    });
    machine
        .state
        .kernel32
        .objects
        .set(handle, KernelObject::Thread(thread.clone()));

    NewThread {
        thread,
        stack_pointer,
    }
}

pub fn current_thread(machine: &Machine) -> HTHREAD {
    HTHREAD::from_raw(teb(machine).ClientId_UniqueThread)
}

#[win32_derive::dllexport]
pub fn GetCurrentThread(machine: &mut Machine) -> HTHREAD {
    current_thread(machine)
}

#[win32_derive::dllexport]
pub fn GetCurrentThreadId(machine: &mut Machine) -> u32 {
    current_thread(machine).to_raw()
}

pub fn teb(machine: &Machine) -> &TEB {
    machine
        .memory
        .mem()
        .get_aligned_ref::<TEB>(machine.teb_addr())
}

pub fn teb_mut(machine: &mut Machine) -> &mut TEB {
    machine
        .memory
        .mem()
        .get_aligned_ref_mut::<TEB>(machine.teb_addr())
}

#[win32_derive::dllexport]
pub fn TlsAlloc(machine: &mut Machine) -> u32 {
    let peb = peb_mut(machine);
    let slot = peb.TlsCount;
    peb.TlsCount = slot + 1;
    slot
}

#[win32_derive::dllexport]
pub fn TlsFree(machine: &mut Machine, dwTlsIndex: u32) -> bool {
    let peb = peb_mut(machine);
    if dwTlsIndex >= peb.TlsCount {
        log::warn!("TlsFree of unknown slot {dwTlsIndex}");
        return false;
    }
    // TODO
    true
}

#[win32_derive::dllexport]
pub fn TlsSetValue(machine: &mut Machine, dwTlsIndex: u32, lpTlsValue: u32) -> bool {
    let teb = teb_mut(machine);
    teb.TlsSlots[dwTlsIndex as usize] = lpTlsValue;
    true
}

#[win32_derive::dllexport]
pub fn TlsGetValue(machine: &mut Machine, dwTlsIndex: u32) -> u32 {
    let teb = teb_mut(machine);
    teb.TlsSlots[dwTlsIndex as usize]
}

#[win32_derive::dllexport]
pub async fn CreateThread(
    machine: &mut Machine,
    lpThreadAttributes: u32,
    dwStackSize: u32,
    lpStartAddress: u32,
    lpParameter: u32,
    dwCreationFlags: u32,
    lpThreadId: u32,
) -> HTHREAD {
    let retrowin32_thread_main =
        crate::loader::get_symbol(machine, "kernel32.dll", "retrowin32_thread_main");

    let stack_size = if dwStackSize > 0 {
        dwStackSize
    } else {
        // TODO: in theory this is configured by exe header, but in practice probably doesn't matter.
        64 << 10
    };

    #[cfg(feature = "x86-emu")]
    {
        // TODO: should reuse a CPU from a previous thread that has exited
        let handle = machine.new_thread(
            true,
            stack_size,
            retrowin32_thread_main,
            &[
                0,              // return address
                lpStartAddress, // entry point
                lpParameter,    // parameter
            ],
        );
        HTHREAD::from_raw(handle)
    }

    #[cfg(not(feature = "x86-emu"))]
    {
        _ = retrowin32_thread_main;
        log::warn!("CreateThread running thread synchronously");
        machine.call_x86(lpStartAddress, vec![lpParameter]).await;
        HTHREAD::null()
    }
}

#[win32_derive::dllexport]
pub fn ExitThread(machine: &mut Machine, dwExitCode: u32) {
    #[cfg(feature = "x86-emu")]
    {
        if machine.emu.x86.cur_cpu == 0 {
            panic!("ExitThread called on main thread");
        }

        log::warn!(
            "thread on cpu {id} exiting with code {code}",
            code = dwExitCode,
            id = machine.emu.x86.cur_cpu
        );
        // TODO: free stack, other thread cleanup, set event to signal waiters, etc.
        machine.exit_thread();
    }
    #[cfg(not(feature = "x86-emu"))]
    todo!();
}

#[win32_derive::dllexport]
pub fn ResumeThread(sys: &dyn System, hThread: HTHREAD) -> u32 {
    1
}

#[win32_derive::dllexport]
pub fn TerminateThread(sys: &dyn System, hThread: HTHREAD, dwExitCode: u32) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn SetThreadDescription(
    sys: &dyn System,
    hThread: HTHREAD,
    lpThreadDescription: Option<&Str16>,
) -> bool {
    true // success
}

#[win32_derive::dllexport]
pub fn SetThreadPriority(sys: &dyn System, hThread: HTHREAD, nPriority: u32) -> bool {
    true // success
}

#[win32_derive::dllexport]
pub fn GetThreadPriority(sys: &dyn System, hThread: HTHREAD) -> i32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn SetThreadStackGuarantee(sys: &dyn System, StackSizeInBytes: Option<&mut u32>) -> bool {
    // ignore
    true // success
}
