import { useCallback, useEffect, useState } from 'react';

import { toCommandError } from '../shared/command-error';
import type { TCommandError } from '../shared/command-error-contract';
import type { TKnownHost } from './trust-contract';
import { tauriTrustGateway, type TTrustGateway } from './trust-gateway';

/** The trusted server keys, read again after each one is forgotten. */
export function useKnownHosts(trust: TTrustGateway = tauriTrustGateway) {
  const [knownHosts, setKnownHosts] = useState<TKnownHost[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<TCommandError>();

  const reload = useCallback(async () => {
    try {
      setKnownHosts(await trust.listKnownHosts());
      setError(undefined);
    } catch (rejection: unknown) {
      setError(toCommandError(rejection));
    } finally {
      setIsLoading(false);
    }
  }, [trust]);

  useEffect(() => {
    void reload();
  }, [reload]);

  const forget = useCallback(
    async (entry: TKnownHost) => {
      try {
        await trust.forgetKnownHost(entry);
      } catch (rejection: unknown) {
        setError(toCommandError(rejection));
      }
      // Reloaded either way: a moved entry means the list on screen is stale.
      await reload();
    },
    [reload, trust],
  );

  return { knownHosts, isLoading, error, forget };
}
