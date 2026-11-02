import { act, renderHook } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';

import type { TCommandError } from '../shared/command-error-contract';
import type { TTrustGateway } from '../trust/trust-gateway';
import type { TOpenSessionRequest, TSessionEvent } from './sessions-contract';
import type { TSessionsGateway } from './sessions-gateway';
import { useConnection } from './use-connection';

const request: TOpenSessionRequest = {
  endpoint: { address: '127.0.0.1', port: 2222, username: 'sallygate' },
  keyFilePath: '~/.ssh/id_ed25519',
  columns: 80,
  rows: 24,
};

const hostKey = {
  algorithm: 'ssh-ed25519',
  fingerprint: 'SHA256:abc',
  opensshPublicKey: 'ssh-ed25519 AAAA',
};

type TOpenSession = TSessionsGateway['openSession'];

function inMemorySessions(openSession: TOpenSession): TSessionsGateway {
  return {
    openSession,
    writeToSession: vi.fn(() => Promise.resolve()),
    resizeSession: vi.fn(() => Promise.resolve()),
    closeSession: vi.fn(() => Promise.resolve()),
  };
}

const inMemoryTrust = (): TTrustGateway => ({ trustHostKey: vi.fn(() => Promise.resolve()) });

// Tauri rejects with the plain error object the Rust command returned, not an Error.
const rejectWith = (error: TCommandError) => vi.fn<TOpenSession>().mockRejectedValue(error);

describe('useConnection', () => {
  it('SHOULD be connected when the session opens', async () => {
    const sessions = inMemorySessions(() => Promise.resolve('session-1'));
    const { result } = renderHook(() => useConnection({ sessions, trust: inMemoryTrust() }));

    await act(() => result.current.connect(request));

    expect(result.current.state).toMatchObject({ status: 'connected', sessionId: 'session-1' });
  });

  it('SHOULD ask the user about an unknown server key, then trust it and connect', async () => {
    const openSession = vi
      .fn<TOpenSession>()
      .mockRejectedValueOnce({ kind: 'unknownHostKey', message: 'unknown', hostKey })
      .mockResolvedValueOnce('session-1');
    const trust = inMemoryTrust();
    const { result } = renderHook(() =>
      useConnection({ sessions: inMemorySessions(openSession), trust }),
    );

    await act(() => result.current.connect(request));
    expect(result.current.state).toMatchObject({ status: 'confirmingHostKey', hostKey });

    await act(() => result.current.trustHostKeyAndConnect());

    expect(trust.trustHostKey).toHaveBeenCalledWith({ endpoint: request.endpoint, hostKey });
    expect(result.current.state).toMatchObject({ status: 'connected' });
  });

  it('SHOULD stop at a changed server key without trusting it', async () => {
    const trust = inMemoryTrust();
    const sessions = inMemorySessions(
      rejectWith({ kind: 'hostKeyChanged', message: 'changed', hostKey }),
    );
    const { result } = renderHook(() => useConnection({ sessions, trust }));

    await act(() => result.current.connect(request));

    expect(result.current.state).toMatchObject({ status: 'hostKeyChanged', hostKey });
    expect(trust.trustHostKey).not.toHaveBeenCalled();
  });

  it('SHOULD show other failures and keep the request for another try', async () => {
    const sessions = inMemorySessions(
      rejectWith({ kind: 'authenticationRejected', message: 'rejected' }),
    );
    const { result } = renderHook(() => useConnection({ sessions, trust: inMemoryTrust() }));

    await act(() => result.current.connect(request));

    expect(result.current.state).toMatchObject({
      status: 'failed',
      request,
      error: { kind: 'authenticationRejected' },
    });
  });

  it('SHOULD end the session when the shell exits', async () => {
    let emit: (event: TSessionEvent) => void = () => undefined;
    const sessions = inMemorySessions((_, onEvent) => {
      emit = onEvent;
      return Promise.resolve('session-1');
    });
    const { result } = renderHook(() => useConnection({ sessions, trust: inMemoryTrust() }));
    await act(() => result.current.connect(request));

    act(() => {
      emit({ type: 'exited', exitStatus: 0 });
    });

    expect(result.current.state).toMatchObject({
      status: 'ended',
      end: { type: 'exited', exitStatus: 0 },
    });
  });

  it('SHOULD forward input to the open session', async () => {
    const sessions = inMemorySessions(() => Promise.resolve('session-1'));
    const { result } = renderHook(() => useConnection({ sessions, trust: inMemoryTrust() }));
    await act(() => result.current.connect(request));

    result.current.sendInput('ls\r');

    expect(sessions.writeToSession).toHaveBeenCalledWith('session-1', 'ls\r');
  });
});
