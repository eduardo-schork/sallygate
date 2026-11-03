import { invoke } from '@tauri-apps/api/core';

import type { TSaveTunnelRequest, TTunnel } from './tunnels-contract';

/** The port forwarding commands of the Rust core. The only UI code that talks to Tauri for them. */
export type TTunnelsGateway = {
  readonly listTunnels: () => Promise<TTunnel[]>;
  readonly saveTunnel: (request: TSaveTunnelRequest) => Promise<void>;
  readonly deleteTunnel: (tunnelId: string) => Promise<void>;
  /** The secret, when the host needs one, is sent once and not kept (ADR-0015). */
  readonly startTunnel: (tunnelId: string, secret?: string) => Promise<void>;
  readonly stopTunnel: (tunnelId: string) => Promise<void>;
};

export const tauriTunnelsGateway: TTunnelsGateway = {
  listTunnels: () => invoke<TTunnel[]>('list_tunnels'),
  saveTunnel: (request) => invoke('save_tunnel', { request }),
  deleteTunnel: (tunnelId) => invoke('delete_tunnel', { tunnelId }),
  startTunnel: (tunnelId, secret) => invoke('start_tunnel', { tunnelId, secret }),
  stopTunnel: (tunnelId) => invoke('stop_tunnel', { tunnelId }),
};
