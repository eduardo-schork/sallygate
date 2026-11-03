import { fireEvent, render, screen, waitFor, within } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';

import type { TKnownHost } from '../trust-contract';
import type { TTrustGateway } from '../trust-gateway';
import { KnownHostsView } from './known-hosts-view.ui';

const docker: TKnownHost = {
  line: 2,
  hosts: '[127.0.0.1]:2222',
  hostKey: {
    algorithm: 'ssh-ed25519',
    fingerprint: 'SHA256:abc',
    opensshPublicKey: 'ssh-ed25519 AAAA',
  },
};

function fakeTrust(entries: TKnownHost[]): TTrustGateway {
  let current = [...entries];
  return {
    trustHostKey: () => Promise.resolve(),
    listKnownHosts: () => Promise.resolve([...current]),
    forgetKnownHost: vi.fn((entry: TKnownHost) => {
      current = current.filter((saved) => saved.line !== entry.line);
      return Promise.resolve();
    }),
  };
}

describe('KnownHostsView', () => {
  it('SHOULD list trusted keys and forget one after confirming', async () => {
    const hashed: TKnownHost = { line: 3, hostKey: docker.hostKey };
    const trust = fakeTrust([docker, hashed]);
    render(<KnownHostsView trust={trust} />);

    const table = await screen.findByRole('table', { name: 'Known hosts' });
    expect(within(table).getByText('[127.0.0.1]:2222')).toBeInTheDocument();
    expect(within(table).getByText('Hidden (hashed in the file)')).toBeInTheDocument();

    fireEvent.click(within(table).getByRole('button', { name: 'Forget [127.0.0.1]:2222 key' }));
    fireEvent.click(screen.getByRole('button', { name: 'Forget key' }));

    await waitFor(() => {
      expect(within(table).queryByText('[127.0.0.1]:2222')).not.toBeInTheDocument();
    });
    expect(trust.forgetKnownHost).toHaveBeenCalledWith(docker);
  });
});
