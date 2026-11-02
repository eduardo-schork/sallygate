import type { TSessionOutputBuffer } from '../session-output-buffer';

export type TTerminalViewProps = {
  readonly output: TSessionOutputBuffer;
  /** What the user typed or pasted, as text. */
  readonly onInput: (data: string) => void;
  /** The terminal grid changed to fit its box. */
  readonly onResize: (columns: number, rows: number) => void;
};
