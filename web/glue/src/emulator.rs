use crate::{debugger, host::JsHost};
use wasm_bindgen::prelude::*;

type JsResult<T> = Result<T, JsError>;
fn err_from_anyhow(err: anyhow::Error) -> JsError {
    JsError::new(&err.to_string())
}

#[wasm_bindgen]
pub struct Emulator {
    machine: win32::Machine,
}

#[wasm_bindgen]
pub enum Status {
    Running,
    Blocked,
    Error,
    DebugBreak,
    Exit,
}

#[wasm_bindgen]
pub enum Register {
    EAX,
    ECX,
    EDX,
    EBX,
    ESP,
    EBP,
    ESI,
    EDI,
}

impl From<Register> for x86::Register {
    fn from(reg: Register) -> Self {
        match reg {
            Register::EAX => x86::Register::EAX,
            Register::ECX => x86::Register::ECX,
            Register::EDX => x86::Register::EDX,
            Register::EBX => x86::Register::EBX,
            Register::ESP => x86::Register::ESP,
            Register::EBP => x86::Register::EBP,
            Register::ESI => x86::Register::ESI,
            Register::EDI => x86::Register::EDI,
        }
    }
}

#[wasm_bindgen]
impl Emulator {
    pub fn set_external_dlls(&mut self, dlls: Vec<String>) {
        self.machine.set_external_dlls(dlls);
    }

    pub fn load_exe(&mut self, buf: &[u8], cmdline: String, relocate: bool) -> JsResult<()> {
        self.machine
            .load_exe(buf, cmdline, if relocate { Some(None) } else { None })
            .map_err(err_from_anyhow)?;
        Ok(())
    }

    pub fn labels(&self) -> JsResult<String> {
        let str = serde_json::to_string(&self.machine.labels)?;
        Ok(str)
    }

    pub fn memory(&self) -> js_sys::DataView {
        let mem = js_sys::WebAssembly::Memory::from(wasm_bindgen::memory());
        let buf = js_sys::ArrayBuffer::from(mem.buffer());
        let ofs = self.machine.memory.imp.as_ptr() as usize;
        js_sys::DataView::new(&buf, ofs, buf.byte_length() as usize - ofs)
    }

    #[wasm_bindgen(getter)]
    pub fn eip(&self) -> u32 {
        self.machine.emu.x86.cpu().regs.eip
    }
    #[wasm_bindgen(setter)]
    pub fn set_eip(&mut self, value: u32) {
        self.machine
            .emu
            .x86
            .cpu_mut()
            .jmp(self.machine.memory.mem(), value);
    }

    pub fn reg(&self, reg: Register) -> u32 {
        self.machine.emu.x86.cpu().regs.get32(reg.into())
    }
    pub fn set_reg(&mut self, reg: Register, value: u32) {
        self.machine.emu.x86.cpu_mut().regs.set32(reg.into(), value);
    }

    #[wasm_bindgen(getter)]
    pub fn exit_code(&self) -> u32 {
        match self.machine.status {
            win32::Status::Exit(code) => code,
            _ => 0,
        }
    }

    pub fn regs(&self) -> debugger::Registers {
        debugger::Registers::from_x86(&self.machine.emu.x86.cpu())
    }

    #[wasm_bindgen(getter)]
    pub fn instr_count(&self) -> usize {
        self.machine.emu.x86.instr_count
    }

    pub fn disassemble_json(&self, addr: u32, limit: usize) -> String {
        serde_json::to_string(&x86::debug::disassemble(self.machine.mem(), addr, limit))
            .unwrap_throw()
    }

    pub fn unblock(&mut self) {
        self.machine.unblock_all();
    }

    /// Run code until at least count instructions have run.
    /// This exists to avoid many round-trips from JS to Rust in the execution loop.
    pub fn run(&mut self, count: usize) -> JsResult<Status> {
        if count == 1 {
            self.machine.single_step();
        } else {
            // Note that instr_count overflows at 4b, but we don't expect to run
            // 4b instructions in a single run() invocation.
            let start = self.machine.emu.x86.instr_count;
            while self.machine.emu.x86.instr_count.wrapping_sub(start) < count {
                if !self.machine.run() {
                    break;
                }
            }
        }

        Ok(match &self.machine.status {
            win32::Status::Running => Status::Running,
            win32::Status::Blocked => Status::Blocked,
            win32::Status::Error { message } => return Err(JsError::new(message)),
            win32::Status::DebugBreak => Status::DebugBreak,
            win32::Status::Exit(_code) => {
                // TODO: use exit code
                Status::Exit
            }
        })
    }

    pub fn breakpoint_add(&mut self, addr: u32) {
        self.machine.add_breakpoint(addr);
    }
    pub fn breakpoint_clear(&mut self, addr: u32) {
        self.machine.clear_breakpoint(addr);
    }

    pub fn mappings_json(&self) -> String {
        serde_json::to_string(&self.machine.memory.mappings.vec()).unwrap_throw()
    }

    pub fn set_tracing_scheme(&self, scheme: &str) {
        win32::winapi::trace::set_scheme(scheme);
    }

    pub fn direct_draw_surfaces(&self) -> Vec<JsValue> {
        debugger::surfaces_from_machine(&self.machine)
    }
}

#[wasm_bindgen]
pub fn new_emulator(host: JsHost) -> Emulator {
    crate::log::init(host.clone().unchecked_into());
    let machine = win32::Machine::new(Box::new(host));
    Emulator { machine }
}
