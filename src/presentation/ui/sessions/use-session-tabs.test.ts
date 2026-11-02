import { act, renderHook } from '@testing-library/react';
import { describe, expect, it } from 'vitest';

import type { TOpenSessionRequest } from './sessions-contract';
import { HOSTS_TAB_ID, useSessionTabs } from './use-session-tabs';

const request: TOpenSessionRequest = {
  endpoint: { address: '127.0.0.1', port: 2222, username: 'sallygate' },
  auth: { type: 'keyFile', path: '~/.ssh/id_ed25519' },
  columns: 80,
  rows: 24,
};

describe('useSessionTabs', () => {
  it('SHOULD start on the hosts screen', () => {
    const { result } = renderHook(() => useSessionTabs());

    expect(result.current.activeTabId).toBe(HOSTS_TAB_ID);
    expect(result.current.tabs).toEqual([]);
  });

  it('SHOULD bring a new tab to the front', () => {
    const { result } = renderHook(() => useSessionTabs());

    act(() => {
      result.current.openTab('web', request);
    });

    expect(result.current.tabs.map((tab) => tab.title)).toEqual(['web']);
    expect(result.current.activeTabId).toBe(result.current.tabs[0]?.id);
  });

  it('SHOULD show the previous tab after closing the one in front', () => {
    const { result } = renderHook(() => useSessionTabs());
    act(() => {
      result.current.openTab('web', request);
      result.current.openTab('db', request);
    });
    const [web, db] = result.current.tabs;

    act(() => {
      result.current.closeTab(db?.id ?? '');
    });

    expect(result.current.activeTabId).toBe(web?.id);
  });

  it('SHOULD go back to the hosts screen after closing the last tab', () => {
    const { result } = renderHook(() => useSessionTabs());
    act(() => {
      result.current.openTab('web', request);
    });

    act(() => {
      result.current.closeTab(result.current.tabs[0]?.id ?? '');
    });

    expect(result.current.activeTabId).toBe(HOSTS_TAB_ID);
  });
});
