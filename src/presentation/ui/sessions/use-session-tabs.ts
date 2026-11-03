import { useCallback, useState } from 'react';

import type { TOpenSessionRequest } from './sessions-contract';

export type TSessionTab = {
  readonly id: string;
  readonly title: string;
  readonly request: TOpenSessionRequest;
};

/** The Sallygate tab (hosts and the other screens) is always first and cannot be closed. */
export const HOSTS_TAB_ID = 'hosts';

type TTabsState = {
  readonly tabs: TSessionTab[];
  readonly activeTabId: string;
  /** Whether open sessions are shown side by side instead of one at a time. */
  readonly wantsSplit: boolean;
};

/** The open session tabs and which tab is in front. */
export function useSessionTabs() {
  const [state, setState] = useState<TTabsState>({
    tabs: [],
    activeTabId: HOSTS_TAB_ID,
    wantsSplit: false,
  });

  const openTab = useCallback((title: string, request: TOpenSessionRequest) => {
    const tab = { id: crypto.randomUUID(), title, request };
    setState((current) => ({ ...current, tabs: [...current.tabs, tab], activeTabId: tab.id }));
  }, []);

  const closeTab = useCallback((tabId: string) => {
    setState((current) => {
      const index = current.tabs.findIndex((tab) => tab.id === tabId);
      const tabs = current.tabs.filter((tab) => tab.id !== tabId);
      // Closing the tab in front shows its neighbour, or the hosts screen when none is left.
      const activeTabId =
        current.activeTabId === tabId
          ? (tabs[Math.max(0, index - 1)]?.id ?? HOSTS_TAB_ID)
          : current.activeTabId;
      return { ...current, tabs, activeTabId };
    });
  }, []);

  const selectTab = useCallback((tabId: string) => {
    setState((current) => ({ ...current, activeTabId: tabId }));
  }, []);

  /** Turns split view on or off; turning it on from the Sallygate tab shows the first session. */
  const toggleSplit = useCallback(() => {
    setState((current) => ({
      ...current,
      wantsSplit: !current.wantsSplit,
      activeTabId:
        current.activeTabId === HOSTS_TAB_ID && !current.wantsSplit
          ? (current.tabs[0]?.id ?? HOSTS_TAB_ID)
          : current.activeTabId,
    }));
  }, []);

  return {
    tabs: state.tabs,
    activeTabId: state.activeTabId,
    // A split of one session is just that session.
    isSplit: state.wantsSplit && state.tabs.length > 1,
    openTab,
    closeTab,
    selectTab,
    toggleSplit,
  };
}
