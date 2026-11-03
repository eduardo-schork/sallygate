import { invoke } from '@tauri-apps/api/core';

import type { TKnownHost, TTrustHostKeyRequest } from './trust-contract';

/** The trust commands of the Rust core. The only UI code that talks to Tauri for trust. */
export type TTrustGateway = {
  readonly trustHostKey: (request: TTrustHostKeyRequest) => Promise<void>;
  readonly listKnownHosts: () => Promise<TKnownHost[]>;
  readonly forgetKnownHost: (entry: TKnownHost) => Promise<void>;
};

export const tauriTrustGateway: TTrustGateway = {
  trustHostKey: (request) => invoke('trust_host_key', { request }),
  listKnownHosts: () => invoke<TKnownHost[]>('list_known_hosts'),
  forgetKnownHost: (entry) => invoke('forget_known_host', { entry }),
};
