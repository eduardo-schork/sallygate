import type { TCommandErrorKind } from '../shared/command-error-contract';

type TErrorCopy = { readonly title: string; readonly hint: string };

/** What each failure means for the user and what to try next. */
export const CONNECTION_ERROR_COPY: Record<TCommandErrorKind, TErrorCopy> = {
  invalidInput: {
    title: 'Check the connection details',
    hint: 'One of the fields above is not valid.',
  },
  unknownHostKey: {
    title: 'The server is not trusted yet',
    hint: 'Connect again to review its fingerprint.',
  },
  hostKeyChanged: {
    title: 'The server key changed',
    hint: 'Sallygate did not connect. Remove the old key from ~/.ssh/known_hosts only if you know why it changed.',
  },
  secretRequired: {
    title: 'A password is needed',
    hint: 'This host logs in with a password or a key passphrase.',
  },
  authenticationRejected: {
    title: 'The server rejected the login',
    hint: 'Check the username, and that this key is listed in ~/.ssh/authorized_keys on the server.',
  },
  keyFileUnreadable: {
    title: 'Could not read the private key',
    hint: 'Check the path and, if the key has one, the passphrase.',
  },
  unreachable: {
    title: 'Could not reach the server',
    hint: 'Check the address and port, and that the server is online.',
  },
  connectionFailed: {
    title: 'The connection failed',
    hint: 'Try again. If it keeps failing, this is what went wrong:',
  },
  knownHostsUnavailable: {
    title: 'Could not use ~/.ssh/known_hosts',
    hint: 'Check that the file can be read and written.',
  },
  hostStoreUnavailable: {
    title: 'Could not use the saved hosts',
    hint: 'Sallygate keeps them in its data folder; check that it can be read and written.',
  },
  sessionNotFound: {
    title: 'The session is no longer open',
    hint: 'Connect again to open a new one.',
  },
  sessionFailed: {
    title: 'The session stopped responding',
    hint: 'Connect again to open a new one.',
  },
};
