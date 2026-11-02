import { formatEndpoint } from '../shared/format-endpoint';
import type { THost } from './inventory-contract';

export type THostGroup = { readonly name: string | undefined; readonly hosts: THost[] };

/** Keeps the core's order and splits it into groups; hosts without a group come last. */
export function groupHosts(hosts: THost[]): THostGroup[] {
  const groups: THostGroup[] = [];
  for (const host of hosts) {
    const lastGroup = groups.at(-1);
    if (lastGroup && lastGroup.name === host.group) {
      lastGroup.hosts.push(host);
    } else {
      groups.push({ name: host.group, hosts: [host] });
    }
  }
  return groups;
}

/** Matches the search text against the name, group and `username@address:port`. */
export function matchesSearch(host: THost, search: string): boolean {
  const query = search.trim().toLowerCase();
  if (query === '') {
    return true;
  }
  return [host.label, host.group ?? '', formatEndpoint(host.endpoint)].some((text) =>
    text.toLowerCase().includes(query),
  );
}
