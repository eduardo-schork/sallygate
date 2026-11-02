import { fireEvent, render, screen, within } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';

import type { THost } from '../inventory-contract';
import { HostsView } from './hosts-view.ui';

function host(label: string, group?: string): THost {
  return {
    id: label,
    label,
    endpoint: { address: `${label}.example.com`, port: 22, username: 'ubuntu' },
    keyFilePath: '~/.ssh/id_ed25519',
    ...(group === undefined ? {} : { group }),
  };
}

const hosts = [host('api', 'Cloud'), host('web', 'Cloud'), host('nas')];

function renderView(onConnect = vi.fn()) {
  render(
    <HostsView
      hosts={hosts}
      isLoading={false}
      onNewHost={vi.fn()}
      onConnect={onConnect}
      onEdit={vi.fn()}
    />,
  );
}

describe('HostsView', () => {
  it('SHOULD show hosts under their group, with ungrouped hosts last', () => {
    renderView();

    const cloud = screen.getByRole('region', { name: 'Cloud' });
    expect(
      within(cloud)
        .getAllByRole('article')
        .map((card) => card.ariaLabel),
    ).toEqual(['api', 'web']);
    expect(screen.getByRole('region', { name: 'No group' })).toHaveTextContent('nas');
  });

  it('SHOULD filter hosts as the user types', () => {
    renderView();

    fireEvent.change(screen.getByRole('searchbox', { name: 'Search hosts' }), {
      target: { value: 'nas' },
    });

    expect(screen.queryByRole('article', { name: 'api' })).not.toBeInTheDocument();
    expect(screen.getByRole('article', { name: 'nas' })).toBeInTheDocument();
  });

  it('SHOULD connect to the host whose card was used', () => {
    const onConnect = vi.fn();
    renderView(onConnect);

    const card = screen.getByRole('article', { name: 'web' });
    fireEvent.click(within(card).getByRole('button', { name: 'Connect' }));

    expect(onConnect).toHaveBeenCalledWith(hosts[1]);
  });
});
