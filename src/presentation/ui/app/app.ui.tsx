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
import { SnippetsView } from '../snippets/snippets-view';
import { KnownHostsView } from '../trust/known-hosts-view';
import { useSnippets } from '../snippets/use-snippets';
import { AppSideNav, type TAppView } from './app-side-nav';
import { AppRoot, HomePanel, HomeView } from './app.styles';

// The host stays while the drawer closes, so its title and fields do not flip mid-animation.
type TDrawerState = { readonly isOpen: boolean; readonly host?: THost | undefined };

/** The window: tabs on top; below, the Sallygate tab (side menu and a screen) or a session. */
export function App() {
  const { hosts, isLoading, loadError, saveHost, deleteHost, importSshConfig } = useHosts();
  const { tabs, activeTabId, openTab, closeTab, selectTab } = useSessionTabs();
  const snippets = useSnippets();
  const [view, setView] = useState<TAppView>('hosts');
  const [drawer, setDrawer] = useState<TDrawerState>({ isOpen: false });
  const [isSaving, setIsSaving] = useState(false);
  const [drawerError, setDrawerError] = useState<TCommandError>();

  const closeDrawer = () => {
    setDrawer((current) => ({ ...current, isOpen: false }));
  };

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
    closeDrawer();
  };

  const handleDelete = async (host: THost) => {
    const error = await deleteHost(host.id);
    if (error) {
      setDrawerError(error);
      return;
    }
    closeDrawer();
  };

  const connectTo = (host: THost) => {
    openTab(host.label, createSessionRequest(host.endpoint, host.auth));
  };

  return (
    <AppRoot>
      <SessionTabBar
        tabs={tabs}
        activeTabId={activeTabId}
        onSelect={selectTab}
        onClose={closeTab}
      />
      <HomePanel hidden={activeTabId !== HOSTS_TAB_ID}>
        <AppSideNav current={view} onNavigate={setView} />
        <HomeView>
          {view === 'hosts' ? (
            <HostsView
              hosts={hosts}
              isLoading={isLoading}
              loadError={loadError}
              onNewHost={() => openDrawer()}
              onConnect={connectTo}
              onEdit={openDrawer}
              onImportSshConfig={importSshConfig}
            />
          ) : null}
          {view === 'snippets' ? (
            <SnippetsView
              snippets={snippets.snippets}
              isLoading={snippets.isLoading}
              loadError={snippets.error}
              onSave={snippets.saveSnippet}
              onDelete={snippets.deleteSnippet}
            />
          ) : null}
          {view === 'knownHosts' ? <KnownHostsView /> : null}
        </HomeView>
      </HomePanel>
      {tabs.map((tab) => (
        <SessionPanel
          key={tab.id}
          request={tab.request}
          isActive={activeTabId === tab.id}
          snippets={snippets.snippets}
          onCloseTab={() => closeTab(tab.id)}
        />
      ))}
      <HostDrawer
        open={drawer.isOpen}
        host={drawer.host}
        isSaving={isSaving}
        error={drawerError}
        onSave={(request) => void handleSave(request)}
        onDelete={(host) => void handleDelete(host)}
        onClose={() => setDrawer({ isOpen: false })}
      />
    </AppRoot>
  );
}
