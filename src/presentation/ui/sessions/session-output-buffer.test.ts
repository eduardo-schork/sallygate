import { describe, expect, it } from 'vitest';

import { createSessionOutputBuffer, decodeBase64 } from './session-output-buffer';

const bytes = (text: string) => new TextEncoder().encode(text);

describe('createSessionOutputBuffer', () => {
  it('SHOULD deliver output that arrived before the terminal attached, in order', () => {
    const buffer = createSessionOutputBuffer();
    const received: string[] = [];
    buffer.push(bytes('Welcome'));
    buffer.push(bytes('$ '));

    buffer.attach((chunk) => received.push(new TextDecoder().decode(chunk)));
    buffer.push(bytes('ls'));

    expect(received).toEqual(['Welcome', '$ ', 'ls']);
  });

  it('SHOULD hold output again after the terminal detaches', () => {
    const buffer = createSessionOutputBuffer();
    const first: Uint8Array[] = [];
    const detach = buffer.attach((chunk) => first.push(chunk));

    detach();
    buffer.push(bytes('later'));
    const second: Uint8Array[] = [];
    buffer.attach((chunk) => second.push(chunk));

    expect(first).toEqual([]);
    expect(second).toEqual([bytes('later')]);
  });
});

describe('decodeBase64', () => {
  it('SHOULD return the exact bytes, including escape codes and UTF-8', () => {
    // Compared as plain arrays: jsdom and Node each have their own Uint8Array class.
    expect(Array.from(decodeBase64(btoa('\u001b[32mok')))).toEqual(
      Array.from(bytes('\u001b[32mok')),
    );
    expect(new TextDecoder().decode(decodeBase64('w6k='))).toBe('é');
  });
});
