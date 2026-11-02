import { useCallback, useEffect, useState } from 'react';

import { toCommandError } from '../shared/command-error';
import type { TCommandError } from '../shared/command-error-contract';
import type { THost, TSaveHostRequest } from './inventory-contract';
import { tauriInventoryGateway, type TInventoryGateway } from './inventory-gateway';

type TSaveResult = { readonly host: THost } | { readonly error: TCommandError };

/** The saved hosts, kept in sync with the core after every change. */
export function useHosts(inventory: TInventoryGateway = tauriInventoryGateway) {
  const [hosts, setHosts] = useState<THost[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [loadError, setLoadError] = useState<TCommandError>();

  const reload = useCallback(async () => {
    try {
      setHosts(await inventory.listHosts());
      setLoadError(undefined);
    } catch (rejection: unknown) {
      setLoadError(toCommandError(rejection));
    } finally {
      setIsLoading(false);
    }
  }, [inventory]);

  useEffect(() => {
    void reload();
  }, [reload]);

  const saveHost = useCallback(
    async (request: TSaveHostRequest): Promise<TSaveResult> => {
      try {
        const host = await inventory.saveHost(request);
        await reload();
        return { host };
      } catch (rejection: unknown) {
        return { error: toCommandError(rejection) };
      }
    },
    [inventory, reload],
  );

  const deleteHost = useCallback(
    async (hostId: string): Promise<TCommandError | undefined> => {
      try {
        await inventory.deleteHost(hostId);
        await reload();
        return undefined;
      } catch (rejection: unknown) {
        return toCommandError(rejection);
      }
    },
    [inventory, reload],
  );

  return { hosts, isLoading, loadError, saveHost, deleteHost };
}
