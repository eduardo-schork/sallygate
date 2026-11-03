import { useCallback, useEffect, useState } from 'react';

import { toCommandError } from '../shared/command-error';
import type { TCommandError } from '../shared/command-error-contract';
import type { TSaveSnippetRequest, TSnippet } from './snippets-contract';
import { tauriSnippetsGateway, type TSnippetsGateway } from './snippets-gateway';

/** The saved snippets, kept in sync with the core after every change. */
export function useSnippets(gateway: TSnippetsGateway = tauriSnippetsGateway) {
  const [snippets, setSnippets] = useState<TSnippet[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<TCommandError>();

  const reload = useCallback(async () => {
    try {
      setSnippets(await gateway.listSnippets());
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

  /** Resolves with the error to show, or nothing when it worked. */
  const run = useCallback(
    async (change: () => Promise<unknown>): Promise<TCommandError | undefined> => {
      try {
        await change();
        await reload();
        return undefined;
      } catch (rejection: unknown) {
        return toCommandError(rejection);
      }
    },
    [reload],
  );

  const saveSnippet = useCallback(
    (request: TSaveSnippetRequest) => run(() => gateway.saveSnippet(request)),
    [gateway, run],
  );
  const deleteSnippet = useCallback(
    (snippetId: string) => run(() => gateway.deleteSnippet(snippetId)),
    [gateway, run],
  );

  return { snippets, isLoading, error, saveSnippet, deleteSnippet };
}
