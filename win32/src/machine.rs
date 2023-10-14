use crate::{host, pe, shims::Shims, winapi};
use memory::{Mem, MemImpl};
use std::collections::HashMap;

/// Integrates the X86 CPU emulator with the Windows OS support.
pub struct Machine {
    #[cfg(feature = "x86-emu")]
    pub x86: x86::X86,
    #[cfg(feature = "x86-unicorn")]
    pub unicorn: unicorn_engine::Unicorn<'static, ()>,
    pub memory: MemImpl,
    pub host: Box<dyn host::Host>,
    pub state: winapi::State,
    pub shims: Shims,
    pub labels: HashMap<u32, String>,
}

impl Machine {
    pub fn new(host: Box<dyn host::Host>, cmdline: String) -> Self {
        #[cfg(not(feature = "x86-unicorn"))]
        let mut memory = MemImpl::default();

        #[cfg(feature = "x86-unicorn")]
        let mut memory = MemImpl::new(16 << 20);

        let mut kernel32 = winapi::kernel32::State::new(&mut memory, cmdline);

        #[cfg(feature = "x86-emu")]
        let shims = {
            kernel32 = kernel32;
            Shims::new()
        };
        #[cfg(feature = "x86-64")]
        let shims = {
            let mapping =
                kernel32
                    .mappings
                    .alloc(0x4000, "shims x64 trampoline".into(), &mut memory);
            Shims::new(
                &mut kernel32.ldt,
                mapping.addr as u64 as *mut u8,
                mapping.size,
            )
        };

        #[cfg(feature = "x86-unicorn")]
        let mut unicorn = {
            let mut unicorn = unicorn_engine::Unicorn::new(
                unicorn_engine::unicorn_const::Arch::X86,
                unicorn_engine::unicorn_const::Mode::MODE_32,
            )
            .unwrap();
            unsafe {
                unicorn
                    .mem_map_ptr(
                        0,
                        memory.len() as usize,
                        unicorn_engine::unicorn_const::Permission::ALL,
                        memory.ptr() as *mut _,
                    )
                    .unwrap();
            };
            unicorn
        };

        #[cfg(feature = "x86-unicorn")]
        let shims = {
            let mapping = kernel32
                .mappings
                .alloc(0x1000, "syscalls".into(), &mut memory);
            Shims::new(
                &mut unicorn,
                memory
                    .mem()
                    .slice(mapping.addr..mapping.addr + mapping.size),
                mapping.addr,
            )
        };

        let state = winapi::State::new(kernel32);

        Machine {
            #[cfg(feature = "x86-emu")]
            x86: x86::X86::new(),
            #[cfg(feature = "x86-unicorn")]
            unicorn,
            memory,
            host,
            state,
            shims,
            labels: HashMap::new(),
        }
    }

    pub fn mem(&self) -> Mem {
        self.memory.mem()
    }

    pub fn load_exe(
        &mut self,
        buf: &[u8],
        cmdline: String,
        relocate: bool,
    ) -> anyhow::Result<pe::LoadedAddrs> {
        pe::load_exe(self, buf, cmdline, relocate)
    }

    /// If eip points at a shim address, call the handler and update eip.
    #[cfg(feature = "x86-emu")]
    fn check_shim_call(&mut self) -> anyhow::Result<bool> {
        if self.x86.cpu.regs.eip & 0xFFFF_0000 != crate::shims_emu::SHIM_BASE {
            return Ok(false);
        }
        let crate::shims::Shim {
            func,
            stack_consumed,
            is_async,
            ..
        } = *self.shims.get(self.x86.cpu.regs.eip);
        let ret = unsafe { func(self, self.x86.cpu.regs.esp) };
        if !is_async {
            self.x86.cpu.regs.eip = self.mem().get::<u32>(self.x86.cpu.regs.esp);
            self.x86.cpu.regs.esp += stack_consumed;
            self.x86.cpu.regs.eax = ret;
        } else {
            // Async handler will manage the return address etc.
        }
        Ok(true)
    }

    // Execute one basic block.  Returns Ok(false) if we stopped early.
    #[cfg(feature = "x86-emu")]
    pub fn execute_block(&mut self) -> anyhow::Result<bool> {
        if self.check_shim_call()? {
            // Treat any shim call as a single block.
            return Ok(true);
        }
        self.x86
            .execute_block(self.memory.mem())
            .map_err(|err| anyhow::anyhow!(err))
    }

    #[cfg(feature = "x86-emu")]
    pub fn single_step(&mut self) -> anyhow::Result<()> {
        if self.check_shim_call()? {
            // Treat any shim call as a single block.
            return Ok(());
        }
        self.x86
            .single_step(self.memory.mem())
            .map_err(|err| anyhow::anyhow!(err))
    }
}
