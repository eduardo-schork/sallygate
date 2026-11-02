import { invoke } from '@tauri-apps/api/core';

import type { TTrustHostKeyRequest } from './trust-contract';

/** The trust commands of the Rust core. The only UI code that talks to Tauri for trust. */
export type TTrustGateway = {
  readonly trustHostKey: (request: TTrustHostKeyRequest) => Promise<void>;
};

export const tauriTrustGateway: TTrustGateway = {
  trustHostKey: (request) => invoke('trust_host_key', { request }),
};
