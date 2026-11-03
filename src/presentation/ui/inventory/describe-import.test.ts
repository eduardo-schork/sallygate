import { describe, expect, it } from 'vitest';

import { describeImport } from './describe-import';
import type { THost } from './inventory-contract';

const host: THost = {
  id: 'host-1',
  label: 'oracle',
  endpoint: { address: '10.0.0.1', port: 22, username: 'ubuntu' },
  auth: { type: 'password' },
  group: 'SSH config',
};

describe('describeImport', () => {
  it('SHOULD count the imported and already saved hosts', () => {
    expect(describeImport({ imported: [host, host], alreadySaved: 1, invalid: [] })).toBe(
      'Imported 2 hosts into the SSH config group. 1 host already saved.',
    );
  });

  it('SHOULD say when there was nothing new', () => {
    expect(describeImport({ imported: [], alreadySaved: 3, invalid: [] })).toBe(
      'Every host in ~/.ssh/config is already saved.',
    );
    expect(describeImport({ imported: [], alreadySaved: 0, invalid: [] })).toBe(
      '~/.ssh/config has no hosts to import.',
    );
  });

  it('SHOULD name the entries it left out and why', () => {
    expect(
      describeImport({
        imported: [],
        alreadySaved: 0,
        invalid: [{ alias: 'old', reason: 'the port must not be 0' }],
      }),
    ).toBe('Left out: old (the port must not be 0).');
  });
});
