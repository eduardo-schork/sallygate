import type { TCommandError, TCommandErrorKind } from './command-error-contract';

const COMMAND_ERROR_KINDS = new Set<string>([
  'invalidInput',
  'unknownHostKey',
  'hostKeyChanged',
  'authenticationRejected',
  'keyFileUnreadable',
  'unreachable',
  'connectionFailed',
  'knownHostsUnavailable',
  'sessionNotFound',
  'sessionFailed',
] satisfies TCommandErrorKind[]);

/** Tauri rejects a failed command with the Rust `CommandError`; anything else is unexpected. */
export function isCommandError(value: unknown): value is TCommandError {
  return (
    typeof value === 'object' &&
    value !== null &&
    'kind' in value &&
    typeof value.kind === 'string' &&
    COMMAND_ERROR_KINDS.has(value.kind) &&
    'message' in value &&
    typeof value.message === 'string'
  );
}

export function toCommandError(value: unknown): TCommandError {
  if (isCommandError(value)) {
    return value;
  }
  return { kind: 'connectionFailed', message: String(value) };
}
