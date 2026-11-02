import { invoke } from '@tauri-apps/api/core';

import type { THost, TSaveHostRequest } from './inventory-contract';

/** The saved-hosts commands of the Rust core. The only UI code that talks to Tauri for hosts. */
export type TInventoryGateway = {
  readonly listHosts: () => Promise<THost[]>;
  readonly saveHost: (request: TSaveHostRequest) => Promise<THost>;
  readonly deleteHost: (hostId: string) => Promise<void>;
};

export const tauriInventoryGateway: TInventoryGateway = {
  listHosts: () => invoke<THost[]>('list_hosts'),
  saveHost: (request) => invoke<THost>('save_host', { request }),
  deleteHost: (hostId) => invoke('delete_host', { hostId }),
};
