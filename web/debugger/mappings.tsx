import * as preact from 'preact';
import * as wasm from '../glue/pkg/glue';
import { MemoryView, Number } from './memory';
import { hex } from './util';

namespace Mappings {
  export interface Props extends MemoryView {
    mappings: wasm.Mapping[];
    highlight?: number;
  }
}
export class Mappings extends preact.Component<Mappings.Props> {
  render() {
    const rows = this.props.mappings.map(mapping => {
      let className: string | undefined;
      const highlight = this.props.highlight;
      if (highlight !== undefined && highlight >= mapping.addr && highlight < (mapping.addr + mapping.size)) {
        className = 'highlight';
      }
      return (
        <tr class={className}>
          <td>
            <code>
              <Number digits={8} {...this.props}>{mapping.addr}</Number>
            </code>
          </td>
          <td style={{ textAlign: 'right' }}>
            <code>{hex(mapping.size)}</code>
          </td>
          <td>{mapping.module}</td>
          <td>{mapping.desc}</td>
        </tr>
      );
    });
    return (
      <div style={{ flex: 1, minHeight: 0, display: 'flex' }}>
        <table style={{ width: '100%', display: 'block', overflow: 'auto' }}>
          <thead>
            <tr>
              <th>addr</th>
              <th>size</th>
              <th>module</th>
              <th>desc</th>
            </tr>
          </thead>
          <tbody>{rows}</tbody>
        </table>
      </div>
    );
  }
}
