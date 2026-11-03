import { Channel, invoke } from '@tauri-apps/api/core';
import { getCurrentWebview } from '@tauri-apps/api/webview';
import { open } from '@tauri-apps/plugin-dialog';

import type { TEntryKind, TRemoteFolder, TTransferProgress } from './transfer-contract';

type TOnProgress = (progress: TTransferProgress) => void;

function progressChannel(onProgress: TOnProgress): Channel<TTransferProgress> {
  const channel = new Channel<TTransferProgress>();
  channel.onmessage = onProgress;
  return channel;
}

/** A session's files and the local files to send. The only UI code that talks to Tauri for files. */
export type TTransferGateway = {
  /** The login folder when `path` is missing. */
  readonly listFolder: (sessionId: string, path?: string) => Promise<TRemoteFolder>;
  readonly makeFolder: (sessionId: string, parent: string, name: string) => Promise<void>;
  readonly rename: (sessionId: string, path: string, newName: string) => Promise<void>;
  readonly delete: (sessionId: string, path: string, kind: TEntryKind) => Promise<void>;
  /** Resolves with where the file landed in the Downloads folder. */
  readonly download: (
    sessionId: string,
    remotePath: string,
    onProgress: TOnProgress,
  ) => Promise<string>;
  /** Resolves with the remote path. */
  readonly upload: (
    sessionId: string,
    localPath: string,
    remoteFolder: string,
    onProgress: TOnProgress,
  ) => Promise<string>;
  /** Asks the user for local files with the system's open dialog; empty when cancelled. */
  readonly pickLocalFiles: () => Promise<string[]>;
  /** Calls `onDrop` with the paths of files dropped on the window; returns how to stop. */
  readonly onFilesDropped: (onDrop: (paths: string[]) => void) => Promise<() => void>;
};

export const tauriTransferGateway: TTransferGateway = {
  listFolder: (sessionId, path) => invoke<TRemoteFolder>('list_remote_folder', { sessionId, path }),
  makeFolder: (sessionId, parent, name) =>
    invoke('make_remote_folder', { sessionId, parent, name }),
  rename: (sessionId, path, newName) => invoke('rename_remote_entry', { sessionId, path, newName }),
  delete: (sessionId, path, kind) => invoke('delete_remote_entry', { sessionId, path, kind }),
  download: (sessionId, remotePath, onProgress) =>
    invoke<string>('download_remote_file', {
      sessionId,
      remotePath,
      onProgress: progressChannel(onProgress),
    }),
  upload: (sessionId, localPath, remoteFolder, onProgress) =>
    invoke<string>('upload_local_file', {
      sessionId,
      localPath,
      remoteFolder,
      onProgress: progressChannel(onProgress),
    }),
  pickLocalFiles: async () => (await open({ multiple: true, directory: false })) ?? [],
  onFilesDropped: (onDrop) =>
    getCurrentWebview().onDragDropEvent((event) => {
      if (event.payload.type === 'drop') {
        onDrop(event.payload.paths);
      }
    }),
};
