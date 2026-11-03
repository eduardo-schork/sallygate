import { useCallback, useEffect, useState } from 'react';

import { toCommandError } from '../shared/command-error';
import type { TCommandError } from '../shared/command-error-contract';
import { parentFolder } from './remote-path';
import type { TFolderListing } from './transfer-contract';
import { tauriTransferGateway, type TTransferGateway } from './transfer-gateway';

/** The folder on this computer the files view shows: what to upload, where downloads land. */
export function useLocalFiles(files: TTransferGateway = tauriTransferGateway) {
  const [folder, setFolder] = useState<TFolderListing>();
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<TCommandError>();

  const open = useCallback(
    async (path?: string) => {
      setIsLoading(true);
      try {
        setFolder(await files.listLocalFolder(path));
        setError(undefined);
      } catch (rejection: unknown) {
        setError(toCommandError(rejection));
      } finally {
        setIsLoading(false);
      }
    },
    [files],
  );

  useEffect(() => {
    void open();
  }, [open]);

  return {
    folder,
    isLoading,
    error,
    open,
    refresh: () => open(folder?.path),
    goUp: () => (folder ? open(parentFolder(folder.path)) : Promise.resolve()),
  };
}
