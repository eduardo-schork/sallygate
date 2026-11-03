import { useCallback, useEffect, useState } from 'react';

import { toCommandError } from '../shared/command-error';
import type { TCommandError } from '../shared/command-error-contract';
import type { TEndpoint } from '../sessions/sessions-contract';
import type { THostKey } from '../trust/trust-contract';
import { tauriTrustGateway, type TTrustGateway } from '../trust/trust-gateway';
import type { TSaveTunnelRequest, TTunnel } from './tunnels-contract';
import { tauriTunnelsGateway, type TTunnelsGateway } from './tunnels-gateway';

/** The forwarding rules and whether each is on, read again after every change. */
export function useTunnels(
  gateway: TTunnelsGateway = tauriTunnelsGateway,
  trust: TTrustGateway = tauriTrustGateway,
) {
  const [tunnels, setTunnels] = useState<TTunnel[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<TCommandError>();

  const reload = useCallback(async () => {
    try {
      setTunnels(await gateway.listTunnels());
      setError(undefined);
    } catch (rejection: unknown) {
      setError(toCommandError(rejection));
    } finally {
      setIsLoading(false);
    }
  }, [gateway]);

  useEffect(() => {
    void reload();
  }, [reload]);

  /** Runs a change and reloads; resolves with the error to show, or nothing when it worked. */
  const run = useCallback(
    async (change: () => Promise<unknown>): Promise<TCommandError | undefined> => {
      try {
        await change();
        return undefined;
      } catch (rejection: unknown) {
        return toCommandError(rejection);
      } finally {
        await reload();
      }
    },
    [reload],
  );

  return {
    tunnels,
    isLoading,
    error,
    save: (request: TSaveTunnelRequest) => run(() => gateway.saveTunnel(request)),
    remove: (tunnelId: string) => run(() => gateway.deleteTunnel(tunnelId)),
    turnOn: (tunnelId: string, secret?: string) => run(() => gateway.startTunnel(tunnelId, secret)),
    turnOff: (tunnelId: string) => run(() => gateway.stopTunnel(tunnelId)),
    /** Records the server key the user checked, before turning a rule on again. */
    trustHostKey: (endpoint: TEndpoint, hostKey: THostKey) =>
      trust.trustHostKey({ endpoint, hostKey }),
  };
}
