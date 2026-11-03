import {
  Alert,
  Button,
  EmptyState,
  FLEX_ALIGNS,
  FLEX_JUSTIFIES,
  HFlex,
  Segment,
  SegmentedControl,
  Spinner,
  Text,
  VFlex,
} from '@faber-ui/react';
import { useEffect, useRef, useState } from 'react';

import { formatEndpoint } from '../../shared/format-endpoint';
import { SnippetPanel } from '../../snippets/snippet-panel';
import { snippetInput } from '../../snippets/snippet-input';
import { FilesView } from '../../transfer/files-view';
import { ChangedHostKeyDialog } from '../../trust/changed-host-key-dialog';
import { UnknownHostKeyDialog } from '../../trust/unknown-host-key-dialog';
import { CONNECTION_ERROR_COPY } from '../connection-error-copy';
import { SecretDialog } from '../secret-dialog';
import { SessionEndedNotice } from '../session-ended-notice';
import { TerminalView } from '../terminal-view';
import { useConnection } from '../use-connection';
import {
  SessionPanelMessage,
  SessionPanelRoot,
  SessionView,
  SessionViewBar,
} from './session-panel.styles';
import type { TSessionPanelProps } from './session-panel.types';

/** One tab's session: it connects when the tab opens and closes when the tab closes. */
export function SessionPanel({ request, isActive, onCloseTab, snippets }: TSessionPanelProps) {
  const { state, connect, trustHostKeyAndConnect, connectWithSecret, dismiss, sendInput, resize } =
    useConnection(request);
  const serverLabel = formatEndpoint(request.endpoint);
  const [view, setView] = useState<'terminal' | 'files'>('terminal');
  // The file browser opens its SFTP channel only once the user first asks for files.
  const [hasOpenedFiles, setHasOpenedFiles] = useState(false);
  const [isSnippetPanelOpen, setIsSnippetPanelOpen] = useState(true);
  const showView = (next: 'terminal' | 'files') => {
    setView(next);
    if (next === 'files') {
      setHasOpenedFiles(true);
    }
  };

  // Connect once when the tab opens; React may run effects twice in development.
  const hasStarted = useRef(false);
  useEffect(() => {
    if (!hasStarted.current) {
      hasStarted.current = true;
      void connect(request);
    }
  }, [connect, request]);

  const closeTabActions = (primaryLabel: string, onPrimary: () => void) => (
    <HFlex gap="SM" justify={FLEX_JUSTIFIES.CENTER}>
      <Button variant="outline" onClick={onCloseTab}>
        Close tab
      </Button>
      <Button onClick={onPrimary}>{primaryLabel}</Button>
    </HFlex>
  );

  return (
    <SessionPanelRoot hidden={!isActive}>
      {state.status === 'connected' ? (
        <>
          <SessionViewBar>
            <SegmentedControl label="Show" labelHidden>
              <Segment
                name={`view-${state.sessionId}`}
                value="terminal"
                checked={view === 'terminal'}
                onChange={() => showView('terminal')}
              >
                Terminal
              </Segment>
              <Segment
                name={`view-${state.sessionId}`}
                value="files"
                checked={view === 'files'}
                onChange={() => showView('files')}
              >
                Files
              </Segment>
            </SegmentedControl>
            {view === 'terminal' ? (
              <Button
                size="small"
                variant="subtle"
                color="neutral"
                aria-pressed={isSnippetPanelOpen}
                onClick={() => setIsSnippetPanelOpen((isOpen) => !isOpen)}
              >
                {isSnippetPanelOpen ? 'Hide snippets' : 'Show snippets'}
              </Button>
            ) : null}
          </SessionViewBar>
          <SessionView hidden={view !== 'terminal'}>
            <TerminalView output={state.output} onInput={sendInput} onResize={resize} />
            {isSnippetPanelOpen ? (
              <SnippetPanel
                snippets={snippets}
                onRun={(snippet) => sendInput(snippetInput(snippet.command))}
              />
            ) : null}
          </SessionView>
          {hasOpenedFiles ? (
            <SessionView hidden={view !== 'files'}>
              <FilesView
                sessionId={state.sessionId}
                serverLabel={serverLabel}
                isActive={isActive && view === 'files'}
              />
            </SessionView>
          ) : null}
        </>
      ) : null}

      {state.status === 'connecting' ? (
        <SessionPanelMessage>
          <VFlex gap="SM" align={FLEX_ALIGNS.CENTER} role="status">
            <Spinner decorative />
            <Text.P>Connecting to {serverLabel}…</Text.P>
          </VFlex>
        </SessionPanelMessage>
      ) : null}

      {state.status === 'failed' ? (
        <SessionPanelMessage>
          <VFlex gap="MD">
            <Alert color="error" title={CONNECTION_ERROR_COPY[state.error.kind].title} role="alert">
              <VFlex gap="XXS">
                <span>{CONNECTION_ERROR_COPY[state.error.kind].hint}</span>
                <Text.Small>{state.error.message}</Text.Small>
              </VFlex>
            </Alert>
            {closeTabActions('Try again', () => void connect(request))}
          </VFlex>
        </SessionPanelMessage>
      ) : null}

      {state.status === 'idle' ||
      state.status === 'confirmingHostKey' ||
      state.status === 'askingSecret' ||
      state.status === 'hostKeyChanged' ? (
        <SessionPanelMessage>
          <EmptyState
            title={`Not connected to ${serverLabel}`}
            description="The session did not open."
            actions={closeTabActions('Connect', () => void connect(request))}
          />
        </SessionPanelMessage>
      ) : null}

      {state.status === 'ended' ? (
        <SessionPanelMessage>
          <SessionEndedNotice
            serverLabel={serverLabel}
            end={state.end}
            onReconnect={() => void connect(request)}
            onClose={onCloseTab}
          />
        </SessionPanelMessage>
      ) : null}

      <UnknownHostKeyDialog
        open={isActive && state.status === 'confirmingHostKey'}
        serverLabel={serverLabel}
        hostKey={state.status === 'confirmingHostKey' ? state.hostKey : undefined}
        onTrust={() => void trustHostKeyAndConnect()}
        onCancel={dismiss}
      />
      <SecretDialog
        open={isActive && state.status === 'askingSecret'}
        serverLabel={serverLabel}
        auth={request.auth}
        onSubmit={(secret) => void connectWithSecret(secret)}
        onCancel={dismiss}
      />
      <ChangedHostKeyDialog
        open={isActive && state.status === 'hostKeyChanged'}
        serverLabel={serverLabel}
        hostKey={state.status === 'hostKeyChanged' ? state.hostKey : undefined}
        onClose={dismiss}
      />
    </SessionPanelRoot>
  );
}
