import { useState } from 'react';

import { HostDrawer } from '../inventory/host-drawer';
import { HostsView } from '../inventory/hosts-view';
import type { THost, TSaveHostRequest } from '../inventory/inventory-contract';
import { useHosts } from '../inventory/use-hosts';
import { SessionPanel } from '../sessions/session-panel';
import { createSessionRequest } from '../sessions/session-request';
import { SessionTabBar } from '../sessions/session-tab-bar';
import { HOSTS_TAB_ID, useSessionTabs } from '../sessions/use-session-tabs';
import type { TCommandError } from '../shared/command-error-contract';
import { AppRoot, HostsPanel } from './app.styles';

type TDrawerState =
  { readonly isOpen: false } | { readonly isOpen: true; readonly host?: THost | undefined };

/** The window: tabs on top, the hosts screen or a session below. */
export function App() {
  const { hosts, isLoading, loadError, saveHost, deleteHost } = useHosts();
  const { tabs, activeTabId, openTab, closeTab, selectTab } = useSessionTabs();
  const [drawer, setDrawer] = useState<TDrawerState>({ isOpen: false });
  const [isSaving, setIsSaving] = useState(false);
  const [drawerError, setDrawerError] = useState<TCommandError>();

  const openDrawer = (host?: THost) => {
    setDrawerError(undefined);
    setDrawer({ isOpen: true, host });
  };

  const handleSave = async (request: TSaveHostRequest) => {
    setIsSaving(true);
    const result = await saveHost(request);
    setIsSaving(false);
    if ('error' in result) {
      setDrawerError(result.error);
      return;
    }
    setDrawer({ isOpen: false });
  };

  const handleDelete = async (host: THost) => {
    const error = await deleteHost(host.id);
    if (error) {
      setDrawerError(error);
      return;
    }
    setDrawer({ isOpen: false });
  };

  const connectTo = (host: THost) => {
    openTab(host.label, createSessionRequest(host.endpoint, host.keyFilePath));
  };

  return (
    <AppRoot>
      <SessionTabBar
        tabs={tabs}
        activeTabId={activeTabId}
        onSelect={selectTab}
        onClose={closeTab}
      />
      <HostsPanel hidden={activeTabId !== HOSTS_TAB_ID}>
        <HostsView
          hosts={hosts}
          isLoading={isLoading}
          loadError={loadError}
          onNewHost={() => openDrawer()}
          onConnect={connectTo}
          onEdit={openDrawer}
        />
      </HostsPanel>
      {tabs.map((tab) => (
        <SessionPanel
          key={tab.id}
          request={tab.request}
          isActive={activeTabId === tab.id}
          onCloseTab={() => closeTab(tab.id)}
        />
      ))}
      <HostDrawer
        open={drawer.isOpen}
        host={drawer.isOpen ? drawer.host : undefined}
        isSaving={isSaving}
        error={drawerError}
        onSave={(request) => void handleSave(request)}
        onDelete={(host) => void handleDelete(host)}
        onClose={() => setDrawer({ isOpen: false })}
      />
    </AppRoot>
  );
}
