import { useCallback, useEffect, useRef, useState } from 'react';

import { toCommandError } from '../shared/command-error';
import type { TCommandError } from '../shared/command-error-contract';
import { baseName, parentFolder } from './remote-path';
import type { TFileEntry, TFolderListing } from './transfer-contract';
import { tauriTransferGateway, type TTransferGateway } from './transfer-gateway';

export type TTransfer = {
  readonly id: number;
  readonly name: string;
  readonly direction: 'download' | 'upload';
  readonly doneBytes: number;
  readonly totalBytes: number;
  readonly status: 'running' | 'done' | 'failed';
  /** Where a finished download landed, or why a transfer failed. */
  readonly detail?: string;
};

/** The files of one open session: the folder on screen, the changes to it and the transfers. */
export function useRemoteFiles(sessionId: string, files: TTransferGateway = tauriTransferGateway) {
  const [folder, setFolder] = useState<TFolderListing>();
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<TCommandError>();
  const [transfers, setTransfers] = useState<TTransfer[]>([]);
  const nextTransferId = useRef(1);
  // Read by callbacks that outlive a render, such as a long upload finishing.
  const folderPath = useRef<string>(undefined);

  const open = useCallback(
    async (path?: string) => {
      setIsLoading(true);
      try {
        const opened = await files.listFolder(sessionId, path);
        folderPath.current = opened.path;
        setFolder(opened);
        setError(undefined);
      } catch (rejection: unknown) {
        setError(toCommandError(rejection));
      } finally {
        setIsLoading(false);
      }
    },
    [files, sessionId],
  );

  useEffect(() => {
    void open();
  }, [open]);

  const refresh = useCallback(() => open(folderPath.current), [open]);

  /** Runs a change, then shows the folder again; a failure is shown instead. */
  const change = useCallback(
    async (run: () => Promise<unknown>) => {
      try {
        await run();
        await refresh();
      } catch (rejection: unknown) {
        setError(toCommandError(rejection));
      }
    },
    [refresh],
  );

  const updateTransfer = (id: number, update: Partial<TTransfer>) => {
    setTransfers((current) =>
      current.map((transfer) => (transfer.id === id ? { ...transfer, ...update } : transfer)),
    );
  };

  const track = useCallback(
    async (
      name: string,
      direction: TTransfer['direction'],
      run: (
        onProgress: (progress: { doneBytes: number; totalBytes: number }) => void,
      ) => Promise<string>,
    ) => {
      const id = nextTransferId.current;
      nextTransferId.current += 1;
      setTransfers((current) => [
        ...current,
        { id, name, direction, doneBytes: 0, totalBytes: 0, status: 'running' },
      ]);
      try {
        const landedAt = await run((progress) => updateTransfer(id, progress));
        updateTransfer(id, { status: 'done', detail: landedAt });
      } catch (rejection: unknown) {
        updateTransfer(id, { status: 'failed', detail: toCommandError(rejection).message });
      }
    },
    [],
  );

  /** Copies a remote file into a folder on this computer. */
  const download = useCallback(
    (entry: TFileEntry, localFolder: string) =>
      track(entry.name, 'download', (onProgress) =>
        files.download(sessionId, entry.path, entry.kind, localFolder, onProgress),
      ),
    [files, sessionId, track],
  );

  const upload = useCallback(
    async (localPaths: string[]) => {
      const target = folderPath.current;
      if (target === undefined) {
        return;
      }
      await Promise.all(
        localPaths.map((localPath) =>
          track(baseName(localPath), 'upload', (onProgress) =>
            files.upload(sessionId, localPath, target, onProgress),
          ),
        ),
      );
      if (folderPath.current === target) {
        await refresh();
      }
    },
    [files, refresh, sessionId, track],
  );

  const pickAndUpload = useCallback(async () => {
    await upload(await files.pickLocalFiles());
  }, [files, upload]);

  return {
    folder,
    isLoading,
    error,
    transfers,
    open,
    refresh,
    goUp: () => (folder ? open(parentFolder(folder.path)) : Promise.resolve()),
    makeFolder: (name: string) =>
      change(() => files.makeFolder(sessionId, folderPath.current ?? '/', name)),
    rename: (entry: TFileEntry, newName: string) =>
      change(() => files.rename(sessionId, entry.path, newName)),
    remove: (entry: TFileEntry) => change(() => files.delete(sessionId, entry.path, entry.kind)),
    download,
    upload,
    pickAndUpload,
    clearFinishedTransfers: () =>
      setTransfers((current) => current.filter((transfer) => transfer.status === 'running')),
    onFilesDropped: files.onFilesDropped,
  };
}
