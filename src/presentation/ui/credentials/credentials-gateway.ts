import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';

import type { TGenerateSshKeyRequest, TSshKey } from './credentials-contract';

/** The SSH key commands of the Rust core. The only UI code that talks to Tauri for keys. */
export type TCredentialsGateway = {
  readonly listSshKeys: () => Promise<TSshKey[]>;
  readonly generateSshKey: (request: TGenerateSshKeyRequest) => Promise<TSshKey>;
  readonly importSshKey: (path: string) => Promise<TSshKey>;
  /** Asks for one private key file with the system's open dialog; nothing when cancelled. */
  readonly pickKeyFile: () => Promise<string | undefined>;
  readonly copyText: (text: string) => Promise<void>;
};

export const tauriCredentialsGateway: TCredentialsGateway = {
  listSshKeys: () => invoke<TSshKey[]>('list_ssh_keys'),
  generateSshKey: (request) => invoke<TSshKey>('generate_ssh_key', { request }),
  importSshKey: (path) => invoke<TSshKey>('import_ssh_key', { path }),
  pickKeyFile: async () => (await open({ multiple: false, directory: false })) ?? undefined,
  copyText: (text) => navigator.clipboard.writeText(text),
};
