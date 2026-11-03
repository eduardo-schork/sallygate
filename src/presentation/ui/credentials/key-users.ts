import type { THost } from '../inventory/inventory-contract';
import type { TSshKey } from './credentials-contract';

/** The saved hosts that sign in with this key, whether their path uses `~` or not. */
export function keyUsers(key: TSshKey, hosts: THost[]): string[] {
  return hosts
    .filter(
      ({ auth }) =>
        auth.type === 'keyFile' &&
        (auth.path === key.path || auth.path.endsWith(`/.ssh/${key.name}`)),
    )
    .map((host) => host.label);
}
