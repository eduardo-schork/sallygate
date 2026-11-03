import type { TEntryKind } from './transfer-contract';

const KIND_LETTER: Record<TEntryKind, string> = { file: '-', folder: 'd', link: 'l' };
const BITS = 'rwx';

/** `ls -l` style permissions, like `drwxr-xr-x`; empty when the mode is unknown. */
export function formatPermissions(kind: TEntryKind, mode?: number): string {
  if (mode === undefined) {
    return '';
  }
  let text = KIND_LETTER[kind];
  for (let shift = 8; shift >= 0; shift -= 1) {
    text += (mode >> shift) & 1 ? BITS.charAt((8 - shift) % 3) : '-';
  }
  return text;
}
