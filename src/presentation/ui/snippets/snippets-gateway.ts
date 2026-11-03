import { invoke } from '@tauri-apps/api/core';

import type { TSaveSnippetRequest, TSnippet } from './snippets-contract';

/** The snippet commands of the Rust core. The only UI code that talks to Tauri for snippets. */
export type TSnippetsGateway = {
  readonly listSnippets: () => Promise<TSnippet[]>;
  readonly saveSnippet: (request: TSaveSnippetRequest) => Promise<TSnippet>;
  readonly deleteSnippet: (snippetId: string) => Promise<void>;
};

export const tauriSnippetsGateway: TSnippetsGateway = {
  listSnippets: () => invoke<TSnippet[]>('list_snippets'),
  saveSnippet: (request) => invoke<TSnippet>('save_snippet', { request }),
  deleteSnippet: (snippetId) => invoke('delete_snippet', { snippetId }),
};
