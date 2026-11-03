import { useState } from 'react';

import { KeysView } from '../credentials/keys-view';
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
import { ForwardingView } from '../tunnels/forwarding-view';
import { useSnippets } from '../snippets/use-snippets';
import { AppSideNav, type TAppView } from './app-side-nav';
import { AppBody, AppContent, AppRoot, HomeView, SessionArea } from './app.styles';

// The host stays while the drawer closes, so its title and fields do not flip mid-animation.
type TDrawerState = { readonly isOpen: boolean; readonly host?: THost | undefined };

/** Two sessions side by side, four in a square, then three columns. */
function splitColumns(sessionCount: number): number {
  return sessionCount <= 4 ? 2 : 3;
}

/** The window: tabs on top; below, the side menu and a screen, a session or a grid of sessions. */
export function App() {
  const { hosts, isLoading, loadError, saveHost, deleteHost, importSshConfig } = useHosts();
  const { tabs, activeTabId, isSplit, openTab, closeTab, selectTab, toggleSplit } =
    useSessionTabs();
  const isHomeInFront = activeTabId === HOSTS_TAB_ID;
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
    const jumpHost = hosts.find((saved) => saved.id === host.jumpHostId);
    const jump = jumpHost ? { endpoint: jumpHost.endpoint, auth: jumpHost.auth } : undefined;
    openTab(host.label, createSessionRequest(host.endpoint, host.auth, jump));
  };

  return (
    <AppRoot>
      <SessionTabBar
        tabs={tabs}
        activeTabId={activeTabId}
        onSelect={selectTab}
        onClose={closeTab}
        isSplit={isSplit}
        onToggleSplit={toggleSplit}
      />
      <AppBody>
        <AppSideNav
          current={isHomeInFront ? view : undefined}
          onNavigate={(next) => {
            setView(next);
            selectTab(HOSTS_TAB_ID);
          }}
        />
        <AppContent>
          <HomeView hidden={!isHomeInFront}>
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
            {view === 'keys' ? <KeysView hosts={hosts} /> : null}
            {view === 'forwarding' ? <ForwardingView hosts={hosts} /> : null}
            {view === 'knownHosts' ? <KnownHostsView /> : null}
          </HomeView>
          <SessionArea
            hidden={isHomeInFront}
            data-columns={isSplit ? splitColumns(tabs.length) : undefined}
            style={isSplit ? { '--session-columns': splitColumns(tabs.length) } : undefined}
          >
            {tabs.map((tab) => (
              <SessionPanel
                key={tab.id}
                request={tab.request}
                title={tab.title}
                isActive={activeTabId === tab.id}
                isVisible={isSplit || activeTabId === tab.id}
                isSplit={isSplit}
                onActivate={() => selectTab(tab.id)}
                snippets={snippets.snippets}
                onCloseTab={() => closeTab(tab.id)}
              />
            ))}
          </SessionArea>
        </AppContent>
      </AppBody>
      <HostDrawer
        open={drawer.isOpen}
        host={drawer.host}
        isSaving={isSaving}
        error={drawerError}
        onSave={(request) => void handleSave(request)}
        onDelete={(host) => void handleDelete(host)}
        onClose={() => setDrawer({ isOpen: false })}
        hosts={hosts}
      />
    </AppRoot>
  );
}
