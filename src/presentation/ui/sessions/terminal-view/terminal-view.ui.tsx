import '@xterm/xterm/css/xterm.css';

import { FitAddon } from '@xterm/addon-fit';
import { Terminal } from '@xterm/xterm';
import { useEffect, useRef } from 'react';

import {
  createTerminalTheme,
  resolveTerminalFontFamily,
  TERMINAL_FONT_SIZE,
} from './terminal-theme';
import { TERMINAL_SCROLLBACK_LINES } from './terminal-view.constants';
import { TerminalViewRoot } from './terminal-view.styles';
import type { TTerminalViewProps } from './terminal-view.types';

/** An interactive terminal that shows a session's output and sends back what the user types. */
export function TerminalView({ output, onInput, onResize }: TTerminalViewProps) {
  const containerRef = useRef<HTMLDivElement>(null);
  // Kept in refs so new callbacks do not tear down and rebuild the terminal.
  const onInputRef = useRef(onInput);
  const onResizeRef = useRef(onResize);
  useEffect(() => {
    onInputRef.current = onInput;
    onResizeRef.current = onResize;
  });

  useEffect(() => {
    const container = containerRef.current;
    if (!container) {
      return undefined;
    }

    const terminal = new Terminal({
      cursorBlink: true,
      fontFamily: resolveTerminalFontFamily(),
      fontSize: TERMINAL_FONT_SIZE,
      scrollback: TERMINAL_SCROLLBACK_LINES,
      theme: createTerminalTheme(),
    });
    const fitAddon = new FitAddon();
    terminal.loadAddon(fitAddon);
    terminal.open(container);

    const fitToContainer = () => {
      // A hidden tab has no size; fitting it would shrink the remote terminal to nothing.
      if (container.clientWidth === 0 || container.clientHeight === 0) {
        return;
      }
      fitAddon.fit();
      onResizeRef.current(terminal.cols, terminal.rows);
    };
    fitToContainer();
    terminal.focus();

    const inputSubscription = terminal.onData((data) => onInputRef.current(data));
    const detachOutput = output.attach((bytes) => terminal.write(bytes));
    const resizeObserver = new ResizeObserver(fitToContainer);
    resizeObserver.observe(container);

    return () => {
      resizeObserver.disconnect();
      detachOutput();
      inputSubscription.dispose();
      terminal.dispose();
    };
  }, [output]);

  return <TerminalViewRoot ref={containerRef} aria-label="Terminal" role="application" />;
}
