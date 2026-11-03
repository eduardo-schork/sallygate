import { useCallback, useEffect, useState } from 'react';

import { toCommandError } from '../shared/command-error';
import type { TCommandError } from '../shared/command-error-contract';
import type { TGenerateSshKeyRequest, TSshKey } from './credentials-contract';
import { tauriCredentialsGateway, type TCredentialsGateway } from './credentials-gateway';

/** The key pairs in `~/.ssh`, read again after each one is made or brought in. */
export function useSshKeys(credentials: TCredentialsGateway = tauriCredentialsGateway) {
  const [keys, setKeys] = useState<TSshKey[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<TCommandError>();

  const reload = useCallback(async () => {
    try {
      setKeys(await credentials.listSshKeys());
      setError(undefined);
    } catch (rejection: unknown) {
      setError(toCommandError(rejection));
    } finally {
      setIsLoading(false);
    }
  }, [credentials]);

  useEffect(() => {
    void reload();
  }, [reload]);

  /** Resolves with the error to show, or nothing when it worked. */
  const generate = useCallback(
    async (request: TGenerateSshKeyRequest): Promise<TCommandError | undefined> => {
      try {
        await credentials.generateSshKey(request);
        await reload();
        return undefined;
      } catch (rejection: unknown) {
        return toCommandError(rejection);
      }
    },
    [credentials, reload],
  );

  const importKey = useCallback(async () => {
    const path = await credentials.pickKeyFile();
    if (path === undefined) {
      return;
    }
    try {
      await credentials.importSshKey(path);
      await reload();
    } catch (rejection: unknown) {
      setError(toCommandError(rejection));
    }
  }, [credentials, reload]);

  return { keys, isLoading, error, generate, importKey, copyText: credentials.copyText };
}
