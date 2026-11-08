import { useCallback, useEffect, useState } from 'react';

import { toCommandError } from '../shared/command-error';
import {
  type TAvailableUpdate,
  tauriUpdatesGateway,
  type TUpdatesGateway,
} from './updates-gateway';

export type TUpdateState =
  | { readonly status: 'none' }
  | { readonly status: 'available'; readonly update: TAvailableUpdate }
  | { readonly status: 'installing'; readonly version: string; readonly share: number }
  | { readonly status: 'failed'; readonly version: string; readonly message: string };

/** Looks for a newer version once, when the app opens, and installs it when asked. */
export function useAppUpdate(updates: TUpdatesGateway = tauriUpdatesGateway) {
  const [state, setState] = useState<TUpdateState>({ status: 'none' });

  useEffect(() => {
    let isCurrent = true;
    updates
      .checkForUpdate()
      .then((update) => {
        if (isCurrent && update) {
          setState({ status: 'available', update });
        }
      })
      // Offline, or no release yet: nothing to offer, and nothing worth interrupting for.
      .catch(() => undefined);
    return () => {
      isCurrent = false;
    };
  }, [updates]);

  const install = useCallback(async (update: TAvailableUpdate) => {
    setState({ status: 'installing', version: update.version, share: 0 });
    try {
      await update.install((share) => {
        setState({ status: 'installing', version: update.version, share });
      });
    } catch (rejection: unknown) {
      setState({
        status: 'failed',
        version: update.version,
        message: toCommandError(rejection).message,
      });
    }
  }, []);

  const dismiss = useCallback(() => {
    setState({ status: 'none' });
  }, []);

  return { state, install, dismiss };
}
