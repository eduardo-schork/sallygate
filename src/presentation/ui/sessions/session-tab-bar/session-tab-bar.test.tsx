import { fireEvent, render, screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';

import type { TOpenSessionRequest } from '../sessions-contract';
import { HOSTS_TAB_ID } from '../use-session-tabs';
import { SessionTabBar } from './session-tab-bar.ui';

const request: TOpenSessionRequest = {
  endpoint: { address: '127.0.0.1', port: 2222, username: 'sallygate' },
  auth: { type: 'password' },
  columns: 80,
  rows: 24,
};

function renderBar(tabCount: number, onToggleSplit = vi.fn()) {
  const tabs = Array.from({ length: tabCount }, (_, index) => ({
    id: `t${String(index)}`,
    title: `server-${String(index)}`,
    request,
  }));
  render(
    <SessionTabBar
      tabs={tabs}
      activeTabId={HOSTS_TAB_ID}
      onSelect={vi.fn()}
      onClose={vi.fn()}
      isSplit={false}
      onToggleSplit={onToggleSplit}
    />,
  );
}

describe('SessionTabBar', () => {
  it('SHOULD offer split view once two sessions are open', () => {
    const onToggleSplit = vi.fn();
    renderBar(2, onToggleSplit);

    fireEvent.click(screen.getByRole('button', { name: 'Split view' }));

    expect(onToggleSplit).toHaveBeenCalledOnce();
    expect(screen.getByRole('tablist', { name: 'Open tabs' })).toBeInTheDocument();
  });

  it('SHOULD not split a single session', () => {
    renderBar(1);

    expect(screen.getByRole('button', { name: 'Split view' })).toBeDisabled();
  });
});
