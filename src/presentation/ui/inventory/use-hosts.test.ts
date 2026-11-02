import { act, renderHook, waitFor } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';

import type { THost, TSaveHostRequest } from './inventory-contract';
import type { TInventoryGateway } from './inventory-gateway';
import { useHosts } from './use-hosts';

function inMemoryInventory(initialHosts: THost[] = []): TInventoryGateway {
  let hosts = [...initialHosts];
  return {
    listHosts: () => Promise.resolve([...hosts]),
    saveHost: (request: TSaveHostRequest) => {
      const host: THost = { ...request, id: request.id ?? `host-${String(hosts.length + 1)}` };
      hosts = [...hosts.filter((saved) => saved.id !== host.id), host];
      return Promise.resolve(host);
    },
    deleteHost: (hostId) => {
      hosts = hosts.filter((host) => host.id !== hostId);
      return Promise.resolve();
    },
  };
}

const newHost: TSaveHostRequest = {
  label: 'web',
  endpoint: { address: '10.0.0.5', port: 22, username: 'ubuntu' },
  keyFilePath: '~/.ssh/id_ed25519',
};

// Gateways are created once per test: a new gateway on every render would reload forever.
describe('useHosts', () => {
  it('SHOULD load the saved hosts', async () => {
    const saved: THost = { ...newHost, id: 'host-1' };
    const inventory = inMemoryInventory([saved]);
    const { result } = renderHook(() => useHosts(inventory));

    await waitFor(() => {
      expect(result.current.isLoading).toBe(false);
    });

    expect(result.current.hosts).toEqual([saved]);
  });

  it('SHOULD show a host right after saving it', async () => {
    const inventory = inMemoryInventory();
    const { result } = renderHook(() => useHosts(inventory));
    await waitFor(() => {
      expect(result.current.isLoading).toBe(false);
    });

    await act(() => result.current.saveHost(newHost));

    expect(result.current.hosts.map((host) => host.label)).toEqual(['web']);
  });

  it('SHOULD drop a host right after deleting it', async () => {
    const inventory = inMemoryInventory([{ ...newHost, id: 'host-1' }]);
    const { result } = renderHook(() => useHosts(inventory));
    await waitFor(() => {
      expect(result.current.hosts).toHaveLength(1);
    });

    await act(() => result.current.deleteHost('host-1'));

    expect(result.current.hosts).toEqual([]);
  });

  it('SHOULD report why saving failed', async () => {
    const inventory: TInventoryGateway = {
      ...inMemoryInventory(),
      // Tauri rejects with the plain error object the Rust command returned.
      saveHost: vi.fn<TInventoryGateway['saveHost']>().mockRejectedValue({
        kind: 'invalidInput',
        message: 'the host name is empty',
      }),
    };
    const { result } = renderHook(() => useHosts(inventory));

    const outcome = await act(() => result.current.saveHost(newHost));

    expect(outcome).toEqual({ error: { kind: 'invalidInput', message: 'the host name is empty' } });
  });
});
