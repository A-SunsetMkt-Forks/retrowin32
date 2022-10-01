import * as preact from 'preact';
import { h } from 'preact';
import { Breakpoint } from './break';
import { Code } from './code';
import { Mappings } from './mappings';
import { Memory } from './memory';
import { Registers } from './registers';
import { Stack } from './stack';
import * as wasm from './wasm/wasm';

async function loadExe(path: string): Promise<ArrayBuffer> {
  return await (await fetch(path)).arrayBuffer();
}

// Matches 'pub type JsHost' in lib.rs.
interface JsHost {
  exit(code: number): void;
  write(buf: Uint8Array): number;
}

class VM implements JsHost {
  x86: wasm.X86;
  decoder = new TextDecoder();
  breakpoints = new Map<number, Breakpoint>();
  exitCode: number | undefined = undefined;

  constructor(exe: ArrayBuffer) {
    // new Uint8Array(exe: TypedArray) creates a uint8 view onto the buffer, no copies.
    // But then passing the buffer to Rust must copy the array into the WASM heap...
    this.x86 = wasm.load_exe(this, new Uint8Array(exe));
  }

  step() {
    this.x86.step();
    if (this.exitCode !== undefined) return false;
    const bp = this.breakpoints.get(this.x86.eip);
    if (bp) {
      if (bp.temporary) {
        this.breakpoints.delete(bp.addr);
      }
      return false;
    }
    return true;
  }

  mappings(): wasm.Mapping[] {
    return JSON.parse(this.x86.mappings_json()) as wasm.Mapping[];
  }
  disassemble(addr: number): wasm.Instruction[] {
    // Note: disassemble_json() may cause allocations, invalidating any existing .memory()!
    return JSON.parse(this.x86.disassemble_json(addr)) as wasm.Instruction[];
  }

  exit(code: number) {
    console.warn('exited with code', code);
    this.exitCode = code;
  }
  write(buf: Uint8Array): number {
    console.log(this.decoder.decode(buf));
    return buf.length;
  }
}

namespace Page {
  export interface Props {
    vm: VM;
  }
  export interface State {
    memBase: number;
    memHighlight?: number;
  }
}
class Page extends preact.Component<Page.Props, Page.State> {
  state: Page.State = { memBase: 0x40_1000 };

  updateAfter(f: () => void) {
    try {
      f();
    } finally {
      this.forceUpdate();
    }
  }

  step() {
    this.updateAfter(() => this.props.vm.step());
  }

  run() {
    this.updateAfter(() => {
      for (;;) {
        if (!this.props.vm.step()) break;
      }
    });
  }

  runTo(addr: number) {
    this.props.vm.breakpoints.set(addr, { addr, temporary: true });
    this.run();
  }

  render() {
    // Note: disassemble_json() may cause allocations, invalidating any existing .memory()!
    const instrs = this.props.vm.disassemble(this.props.vm.x86.eip);
    return (
      <main>
        <button
          onClick={() => this.run()}
        >
          run
        </button>
        <button
          onClick={() => {
            this.props.vm.step();
            this.forceUpdate();
          }}
        >
          step
        </button>
        <button
          onClick={() => this.runTo(instrs[1].addr)}
        >
          step over
        </button>
        <div style={{ display: 'flex' }}>
          <Code
            instrs={instrs}
            highlightMemory={(addr) => this.setState({ memHighlight: addr })}
            showMemory={(memBase) => this.setState({ memBase })}
            runTo={(addr: number) => this.runTo(addr)}
          />
          <div style={{ width: '12ex' }} />
          <Registers regs={this.props.vm.x86} />
        </div>
        <div style={{ display: 'flex' }}>
          <div>
            <Memory
              mem={this.props.vm.x86.memory()}
              base={this.state.memBase}
              highlight={this.state.memHighlight}
              jumpTo={(addr) => this.setState({ memBase: addr })}
            />
            <Mappings mappings={this.props.vm.mappings()} />
          </div>
          <div style={{ width: '12ex' }} />
          <Stack x86={this.props.vm.x86} />
        </div>
      </main>
    );
  }
}

async function main() {
  const path = document.location.search.substring(1);
  if (!path) throw new Error('expected ?path in URL');
  const exe = await loadExe(path);
  await wasm.default(new URL('wasm/wasm_bg.wasm', document.location.href));

  const vm = new VM(exe);
  console.log(vm.mappings());

  preact.render(<Page vm={vm} />, document.body);
}

main();
