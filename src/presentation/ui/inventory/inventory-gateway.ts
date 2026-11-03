import { invoke } from '@tauri-apps/api/core';

import type { THost, TImportReport, TSaveHostRequest } from './inventory-contract';

/** The saved-hosts commands of the Rust core. The only UI code that talks to Tauri for hosts. */
export type TInventoryGateway = {
  readonly listHosts: () => Promise<THost[]>;
  readonly saveHost: (request: TSaveHostRequest) => Promise<THost>;
  readonly deleteHost: (hostId: string) => Promise<void>;
  /** Saves the hosts from `~/.ssh/config` that are not saved yet. */
  readonly importSshConfig: () => Promise<TImportReport>;
};

export const tauriInventoryGateway: TInventoryGateway = {
  listHosts: () => invoke<THost[]>('list_hosts'),
  saveHost: (request) => invoke<THost>('save_host', { request }),
  deleteHost: (hostId) => invoke('delete_host', { hostId }),
  importSshConfig: () => invoke<TImportReport>('import_ssh_config'),
};
