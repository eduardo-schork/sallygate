import type { TEndpoint } from '../sessions/sessions-contract';

/** `username@address:port`, the way the `ssh` command shows a destination. */
export function formatEndpoint({ username, address, port }: TEndpoint): string {
  return `${username}@${address}:${String(port)}`;
}
