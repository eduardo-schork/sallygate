import { Channel, invoke } from '@tauri-apps/api/core';

import type { TOpenSessionRequest, TSessionEvent } from './sessions-contract';

/** The sessions commands of the Rust core. The only UI code that talks to Tauri for sessions. */
export type TSessionsGateway = {
  readonly openSession: (
    request: TOpenSessionRequest,
    onEvent: (event: TSessionEvent) => void,
  ) => Promise<string>;
  readonly writeToSession: (sessionId: string, data: string) => Promise<void>;
  readonly resizeSession: (sessionId: string, columns: number, rows: number) => Promise<void>;
  readonly closeSession: (sessionId: string) => Promise<void>;
};

export const tauriSessionsGateway: TSessionsGateway = {
  openSession: (request, onEvent) => {
    const onEventChannel = new Channel<TSessionEvent>();
    onEventChannel.onmessage = onEvent;
    return invoke<string>('open_session', { request, onEvent: onEventChannel });
  },
  writeToSession: (sessionId, data) => invoke('write_to_session', { sessionId, data }),
  resizeSession: (sessionId, columns, rows) =>
    invoke('resize_session', { sessionId, columns, rows }),
  closeSession: (sessionId) => invoke('close_session', { sessionId }),
};
