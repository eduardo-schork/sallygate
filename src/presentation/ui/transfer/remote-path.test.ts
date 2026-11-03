import { describe, expect, it } from 'vitest';

import { formatSize } from './format-size';
import { baseName, parentFolder } from './remote-path';

describe('parentFolder', () => {
  it('SHOULD go one folder up and stop at the root', () => {
    expect(parentFolder('/home/me/apps')).toBe('/home/me');
    expect(parentFolder('/home/me/')).toBe('/home');
    expect(parentFolder('/home')).toBe('/');
    expect(parentFolder('/')).toBe('/');
  });
});

describe('baseName', () => {
  it('SHOULD take the last part of a path', () => {
    expect(baseName('/Users/me/report.pdf')).toBe('report.pdf');
    expect(baseName('C:\\files\\notes.txt')).toBe('notes.txt');
  });
});

describe('formatSize', () => {
  it('SHOULD pick a readable unit', () => {
    expect(formatSize(512)).toBe('512 B');
    expect(formatSize(1536)).toBe('1.5 KB');
    expect(formatSize(25 * 1024 * 1024)).toBe('25 MB');
  });
});
