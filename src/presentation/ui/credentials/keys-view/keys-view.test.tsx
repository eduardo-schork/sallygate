import { fireEvent, render, screen, waitFor, within } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';

import type { TGenerateSshKeyRequest, TSshKey } from '../credentials-contract';
import type { TCredentialsGateway } from '../credentials-gateway';
import { KeysView } from './keys-view.ui';

const oracle: TSshKey = {
  name: 'oracle.key',
  path: '~/.ssh/oracle.key',
  algorithm: 'ssh-ed25519',
  fingerprint: 'SHA256:abc',
  publicKey: 'ssh-ed25519 AAAA me@mac',
  isEncrypted: true,
};

function fakeCredentials(): TCredentialsGateway {
  let keys = [oracle];
  return {
    listSshKeys: () => Promise.resolve([...keys]),
    generateSshKey: vi.fn((request: TGenerateSshKeyRequest) => {
      const key = {
        ...oracle,
        name: request.name,
        path: `~/.ssh/${request.name}`,
        isEncrypted: false,
      };
      keys = [...keys, key];
      return Promise.resolve(key);
    }),
    importSshKey: vi.fn(() => Promise.resolve(oracle)),
    pickKeyFile: () => Promise.resolve(undefined),
    copyText: vi.fn(() => Promise.resolve()),
  };
}

const hosts = [
  {
    id: 'h1',
    label: 'Oracle VPN',
    endpoint: { address: 'x', port: 22, username: 'ubuntu' },
    auth: { type: 'keyFile' as const, path: '~/.ssh/oracle.key' },
  },
];

describe('KeysView', () => {
  it('SHOULD show each key with the hosts that use it and copy its public key', async () => {
    const credentials = fakeCredentials();
    render(<KeysView hosts={hosts} credentials={credentials} />);

    const list = await screen.findByRole('list', { name: 'SSH keys' });
    expect(await within(list).findByText('ed25519, used by Oracle VPN')).toBeInTheDocument();
    expect(within(list).getByText('Passphrase')).toBeInTheDocument();

    fireEvent.click(
      within(list).getByRole('button', { name: 'Copy the public key of oracle.key' }),
    );

    expect(await within(list).findByText('Copied')).toBeInTheDocument();
    expect(credentials.copyText).toHaveBeenCalledWith('ssh-ed25519 AAAA me@mac');
  });

  it('SHOULD generate a key without sending an empty passphrase', async () => {
    const credentials = fakeCredentials();
    render(<KeysView hosts={[]} credentials={credentials} />);

    fireEvent.click(screen.getByRole('button', { name: 'Generate key' }));
    fireEvent.change(screen.getByRole('textbox', { name: /Name/u }), {
      target: { value: 'id_new' },
    });
    const dialog = screen.getByRole('dialog');
    fireEvent.click(within(dialog).getByRole('button', { name: 'Generate key' }));

    await waitFor(() => {
      expect(credentials.generateSshKey).toHaveBeenCalledWith({ name: 'id_new', comment: '' });
    });
    expect(await screen.findByText('id_new')).toBeInTheDocument();
  });
});
