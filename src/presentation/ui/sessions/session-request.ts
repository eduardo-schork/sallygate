import type { TAuth, TEndpoint, TJumpHost, TOpenSessionRequest } from './sessions-contract';

/** The terminal is resized to fit its tab right after the session opens. */
const INITIAL_TERMINAL_SIZE = { columns: 80, rows: 24 } as const;

export function createSessionRequest(
  endpoint: TEndpoint,
  auth: TAuth,
  jump?: TJumpHost,
): TOpenSessionRequest {
  return { endpoint, auth, ...(jump ? { jump } : {}), ...INITIAL_TERMINAL_SIZE };
}
