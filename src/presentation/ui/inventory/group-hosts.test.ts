import { describe, expect, it } from 'vitest';

import { groupHosts, matchesSearch } from './group-hosts';
import type { THost } from './inventory-contract';

function host(label: string, group?: string): THost {
  return {
    id: label,
    label,
    endpoint: { address: `${label}.example.com`, port: 22, username: 'ubuntu' },
    auth: { type: 'keyFile', path: '~/.ssh/id_ed25519' },
    ...(group === undefined ? {} : { group }),
  };
}

describe('groupHosts', () => {
  it('SHOULD keep the order and split consecutive hosts by group', () => {
    const groups = groupHosts([host('api', 'Cloud'), host('web', 'Cloud'), host('nas')]);

    expect(groups.map((group) => [group.name, group.hosts.map((item) => item.label)])).toEqual([
      ['Cloud', ['api', 'web']],
      [undefined, ['nas']],
    ]);
  });
});

describe('matchesSearch', () => {
  it('SHOULD match the name, the group or the address, ignoring case', () => {
    expect(matchesSearch(host('api', 'Cloud'), 'API')).toBe(true);
    expect(matchesSearch(host('api', 'Cloud'), 'cloud')).toBe(true);
    expect(matchesSearch(host('api', 'Cloud'), 'ubuntu@api.example')).toBe(true);
    expect(matchesSearch(host('api', 'Cloud'), 'nas')).toBe(false);
  });

  it('SHOULD match everything when the search is blank', () => {
    expect(matchesSearch(host('api'), '  ')).toBe(true);
  });
});
