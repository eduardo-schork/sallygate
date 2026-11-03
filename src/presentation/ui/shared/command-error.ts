import type { TCommandError, TCommandErrorKind } from './command-error-contract';

// A Record, so adding a kind in Rust fails the typecheck here until it is listed.
const COMMAND_ERROR_KINDS: Record<TCommandErrorKind, true> = {
  invalidInput: true,
  unknownHostKey: true,
  hostKeyChanged: true,
  secretRequired: true,
  authenticationRejected: true,
  keyFileUnreadable: true,
  unreachable: true,
  connectionFailed: true,
  knownHostsUnavailable: true,
  hostStoreUnavailable: true,
  sshConfigUnreadable: true,
  sessionNotFound: true,
  sessionFailed: true,
  filesUnavailable: true,
  fileOperationFailed: true,
};

/** Tauri rejects a failed command with the Rust `CommandError`; anything else is unexpected. */
export function isCommandError(value: unknown): value is TCommandError {
  return (
    typeof value === 'object' &&
    value !== null &&
    'kind' in value &&
    typeof value.kind === 'string' &&
    Object.hasOwn(COMMAND_ERROR_KINDS, value.kind) &&
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
