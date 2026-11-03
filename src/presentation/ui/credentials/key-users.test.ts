import { describe, expect, it } from 'vitest';

import type { THost } from '../inventory/inventory-contract';
import type { TSshKey } from './credentials-contract';
import { keyUsers } from './key-users';

const key: TSshKey = {
  name: 'id_ed25519',
  path: '~/.ssh/id_ed25519',
  algorithm: 'ssh-ed25519',
  fingerprint: 'SHA256:abc',
  publicKey: 'ssh-ed25519 AAAA',
  isEncrypted: false,
};

function host(label: string, auth: THost['auth']): THost {
  return { id: label, label, endpoint: { address: 'x', port: 22, username: 'u' }, auth };
}

describe('keyUsers', () => {
  it('SHOULD match hosts by either form of the key path', () => {
    const hosts = [
      host('tilde', { type: 'keyFile', path: '~/.ssh/id_ed25519' }),
      host('absolute', { type: 'keyFile', path: '/Users/me/.ssh/id_ed25519' }),
      host('other', { type: 'keyFile', path: '~/.ssh/id_ed25519_work' }),
      host('password', { type: 'password' }),
    ];
    expect(keyUsers(key, hosts)).toEqual(['tilde', 'absolute']);
  });
});
