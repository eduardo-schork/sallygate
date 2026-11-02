import { useCallback, useState } from 'react';

import type { TOpenSessionRequest } from './sessions-contract';

export type TSessionTab = {
  readonly id: string;
  readonly title: string;
  readonly request: TOpenSessionRequest;
};

/** The hosts screen is always the first tab and cannot be closed. */
export const HOSTS_TAB_ID = 'hosts';

type TTabsState = { readonly tabs: TSessionTab[]; readonly activeTabId: string };

/** The open session tabs and which tab is in front. */
export function useSessionTabs() {
  const [state, setState] = useState<TTabsState>({ tabs: [], activeTabId: HOSTS_TAB_ID });

  const openTab = useCallback((title: string, request: TOpenSessionRequest) => {
    const tab = { id: crypto.randomUUID(), title, request };
    setState((current) => ({ tabs: [...current.tabs, tab], activeTabId: tab.id }));
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
      return { tabs, activeTabId };
    });
  }, []);

  const selectTab = useCallback((tabId: string) => {
    setState((current) => ({ ...current, activeTabId: tabId }));
  }, []);

  return { tabs: state.tabs, activeTabId: state.activeTabId, openTab, closeTab, selectTab };
}
