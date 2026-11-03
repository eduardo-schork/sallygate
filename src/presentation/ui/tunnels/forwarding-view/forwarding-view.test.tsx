import { fireEvent, render, screen, waitFor, within } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';

import type { THost } from '../../inventory/inventory-contract';
import type { TTrustGateway } from '../../trust/trust-gateway';
import type { TTunnel } from '../tunnels-contract';
import type { TTunnelsGateway } from '../tunnels-gateway';
import { ForwardingView } from './forwarding-view.ui';

const host: THost = {
  id: 'h1',
  label: 'docker',
  endpoint: { address: '127.0.0.1', port: 2222, username: 'sallygate' },
  auth: { type: 'password' },
};

const postgres: TTunnel = {
  id: 't1',
  name: 'Postgres',
  hostId: 'h1',
  kind: { type: 'local', localPort: 5432, targetAddress: 'localhost', targetPort: 5432 },
  isOn: false,
};

const hostKey = { algorithm: 'ssh-ed25519', fingerprint: 'SHA256:abc', opensshPublicKey: 'k' };

function fakeTunnels(failures: object[]): TTunnelsGateway {
  let isOn = false;
  return {
    listTunnels: () => Promise.resolve([{ ...postgres, isOn }]),
    saveTunnel: vi.fn(() => Promise.resolve()),
    deleteTunnel: vi.fn(() => Promise.resolve()),
    startTunnel: vi.fn(() => {
      const failure = failures.shift();
      if (failure) {
        // Tauri rejects with the plain error object the Rust command returned, not an Error.
        // eslint-disable-next-line @typescript-eslint/prefer-promise-reject-errors
        return Promise.reject(failure);
      }
      isOn = true;
      return Promise.resolve();
    }),
    stopTunnel: vi.fn(() => Promise.resolve()),
  };
}

const fakeTrust = (): TTrustGateway => ({
  trustHostKey: vi.fn(() => Promise.resolve()),
  listKnownHosts: () => Promise.resolve([]),
  forgetKnownHost: () => Promise.resolve(),
});

describe('ForwardingView', () => {
  it('SHOULD describe each rule and the host it goes through', async () => {
    render(<ForwardingView hosts={[host]} tunnels={fakeTunnels([])} trust={fakeTrust()} />);

    const list = await screen.findByRole('list', { name: 'Forwarding rules' });
    expect(
      await within(list).findByText('localhost:5432 to localhost:5432, via docker'),
    ).toBeInTheDocument();
  });

  it('SHOULD check the server key and ask for the password before turning a rule on', async () => {
    const tunnels = fakeTunnels([
      { kind: 'unknownHostKey', message: 'not trusted', hostKey },
      { kind: 'secretRequired', message: 'a password is needed' },
    ]);
    const trust = fakeTrust();
    render(<ForwardingView hosts={[host]} tunnels={tunnels} trust={trust} />);

    fireEvent.click(await screen.findByRole('switch', { name: 'Postgres on' }));
    fireEvent.click(await screen.findByRole('button', { name: 'Trust and connect' }));
    const password = await screen.findByLabelText('Password');
    fireEvent.change(password, { target: { value: 'secret' } });
    fireEvent.click(screen.getByRole('button', { name: 'Connect' }));

    await waitFor(() => {
      expect(tunnels.startTunnel).toHaveBeenLastCalledWith('t1', 'secret');
    });
    expect(trust.trustHostKey).toHaveBeenCalledWith({ endpoint: host.endpoint, hostKey });
    expect(await screen.findByRole('switch', { name: 'Postgres on' })).toBeChecked();
  });
});
