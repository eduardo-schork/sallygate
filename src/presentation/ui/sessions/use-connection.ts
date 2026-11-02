import { useCallback, useEffect, useRef, useState } from 'react';

import { toCommandError } from '../shared/command-error';
import type { TCommandError } from '../shared/command-error-contract';
import { tauriTrustGateway, type TTrustGateway } from '../trust/trust-gateway';
import type { THostKey } from '../trust/trust-contract';
import {
  createSessionOutputBuffer,
  decodeBase64,
  type TSessionOutputBuffer,
} from './session-output-buffer';
import type { TOpenSessionRequest, TSessionEvent } from './sessions-contract';
import { tauriSessionsGateway, type TSessionsGateway } from './sessions-gateway';

export type TSessionEnd = Exclude<TSessionEvent, { type: 'output' }>;

export type TConnectionState =
  | { readonly status: 'idle'; readonly request: TOpenSessionRequest }
  | { readonly status: 'connecting'; readonly request: TOpenSessionRequest }
  | {
      readonly status: 'confirmingHostKey';
      readonly request: TOpenSessionRequest;
      readonly hostKey: THostKey;
    }
  | {
      readonly status: 'hostKeyChanged';
      readonly request: TOpenSessionRequest;
      readonly hostKey: THostKey;
    }
  | { readonly status: 'askingSecret'; readonly request: TOpenSessionRequest }
  | {
      readonly status: 'failed';
      readonly request: TOpenSessionRequest;
      readonly error: TCommandError;
    }
  | {
      readonly status: 'connected';
      readonly request: TOpenSessionRequest;
      readonly sessionId: string;
      readonly output: TSessionOutputBuffer;
    }
  | { readonly status: 'ended'; readonly request: TOpenSessionRequest; readonly end: TSessionEnd };

type TConnectionGateways = {
  readonly sessions?: TSessionsGateway;
  readonly trust?: TTrustGateway;
};

/**
 * One terminal connection: opening it, the server key decision, its output and its end.
 * The session closes when the component using this hook unmounts (its tab closes).
 * A password or passphrase goes to the core with one request and is never kept (ADR-0015).
 */
export function useConnection(
  initialRequest: TOpenSessionRequest,
  { sessions = tauriSessionsGateway, trust = tauriTrustGateway }: TConnectionGateways = {},
) {
  const [state, setState] = useState<TConnectionState>({
    status: 'idle',
    request: initialRequest,
  });
  const openSessionId = useRef<string | undefined>(undefined);
  const isMounted = useRef(true);

  useEffect(() => {
    isMounted.current = true;
    return () => {
      isMounted.current = false;
      if (openSessionId.current) {
        void sessions.closeSession(openSessionId.current).catch(() => undefined);
        openSessionId.current = undefined;
      }
    };
  }, [sessions]);

  const connect = useCallback(
    async (request: TOpenSessionRequest, secret?: string) => {
      setState({ status: 'connecting', request });
      const output = createSessionOutputBuffer();
      // An object, so the event handler and the code below share the flag.
      const progress = { hasEnded: false };

      const onEvent = (event: TSessionEvent) => {
        if (event.type === 'output') {
          output.push(decodeBase64(event.data));
          return;
        }
        progress.hasEnded = true;
        openSessionId.current = undefined;
        setState({ status: 'ended', request, end: event });
      };

      try {
        const sessionId = await sessions.openSession(
          secret === undefined ? request : { ...request, secret },
          onEvent,
        );
        if (!isMounted.current) {
          // The tab closed while connecting: nobody will use this session.
          void sessions.closeSession(sessionId).catch(() => undefined);
          return;
        }
        if (!progress.hasEnded) {
          openSessionId.current = sessionId;
          setState({ status: 'connected', request, sessionId, output });
        }
      } catch (rejection: unknown) {
        const error = toCommandError(rejection);
        if (error.kind === 'unknownHostKey' && error.hostKey) {
          setState({ status: 'confirmingHostKey', request, hostKey: error.hostKey });
        } else if (error.kind === 'secretRequired') {
          setState({ status: 'askingSecret', request });
        } else if (error.kind === 'hostKeyChanged' && error.hostKey) {
          setState({ status: 'hostKeyChanged', request, hostKey: error.hostKey });
        } else {
          setState({ status: 'failed', request, error });
        }
      }
    },
    [sessions],
  );

  const trustHostKeyAndConnect = useCallback(async () => {
    if (state.status !== 'confirmingHostKey') {
      return;
    }
    const { request, hostKey } = state;
    setState({ status: 'connecting', request });
    try {
      await trust.trustHostKey({ endpoint: request.endpoint, hostKey });
    } catch (rejection: unknown) {
      setState({ status: 'failed', request, error: toCommandError(rejection) });
      return;
    }
    await connect(request);
  }, [connect, state, trust]);

  const connectWithSecret = useCallback(
    async (secret: string) => {
      if (state.status === 'askingSecret') {
        await connect(state.request, secret);
      }
    },
    [connect, state],
  );

  /** Leaves a pending decision or failure without connecting. */
  const dismiss = useCallback(() => {
    setState((current) => ({ status: 'idle', request: current.request }));
  }, []);

  const sessionId = state.status === 'connected' ? state.sessionId : undefined;

  // Input and resizes are fire-and-forget: if they fail, the session has ended, and its
  // end event is what updates the screen.

  const sendInput = useCallback(
    (data: string) => {
      if (sessionId) {
        void sessions.writeToSession(sessionId, data).catch(() => undefined);
      }
    },
    [sessionId, sessions],
  );

  const resize = useCallback(
    (columns: number, rows: number) => {
      if (sessionId) {
        void sessions.resizeSession(sessionId, columns, rows).catch(() => undefined);
      }
    },
    [sessionId, sessions],
  );

  return {
    state,
    connect,
    trustHostKeyAndConnect,
    connectWithSecret,
    dismiss,
    sendInput,
    resize,
  };
}
