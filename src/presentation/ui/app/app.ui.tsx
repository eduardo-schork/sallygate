import { Container, Text, Title, VFlex } from '@faber-ui/react';

import { ConnectForm } from '../sessions/connect-form';
import { formatEndpoint } from '../sessions/format-endpoint';
import { SessionBar } from '../sessions/session-bar';
import { SessionEndedNotice } from '../sessions/session-ended-notice';
import { TerminalView } from '../sessions/terminal-view';
import { useConnection } from '../sessions/use-connection';
import { ChangedHostKeyDialog } from '../trust/changed-host-key-dialog';
import { UnknownHostKeyDialog } from '../trust/unknown-host-key-dialog';
import { AppCenter, AppRoot } from './app.styles';

export function App() {
  const { state, connect, trustHostKeyAndConnect, backToForm, closeSession, sendInput, resize } =
    useConnection();

  if (state.status === 'connected') {
    return (
      <AppRoot>
        <SessionBar
          serverLabel={formatEndpoint(state.request.endpoint)}
          onClose={() => void closeSession()}
        />
        <TerminalView output={state.output} onInput={sendInput} onResize={resize} />
      </AppRoot>
    );
  }

  if (state.status === 'ended') {
    return (
      <AppRoot>
        <AppCenter>
          <SessionEndedNotice
            serverLabel={formatEndpoint(state.request.endpoint)}
            end={state.end}
            onReconnect={() => void connect(state.request)}
            onBack={backToForm}
          />
        </AppCenter>
      </AppRoot>
    );
  }

  const request = state.status === 'idle' ? state.lastRequest : state.request;
  const serverLabel = request ? formatEndpoint(request.endpoint) : '';

  return (
    <AppRoot>
      <AppCenter>
        <Container size="SMALL">
          <VFlex gap="LG">
            <VFlex gap="XS">
              <Title.H1>Connect to a server</Title.H1>
              <Text.Lead>Open a terminal on any server you can reach over SSH.</Text.Lead>
            </VFlex>
            <ConnectForm
              initialRequest={request}
              isConnecting={state.status === 'connecting'}
              error={state.status === 'failed' ? state.error : undefined}
              onConnect={(newRequest) => void connect(newRequest)}
            />
          </VFlex>
        </Container>
      </AppCenter>
      <UnknownHostKeyDialog
        open={state.status === 'confirmingHostKey'}
        serverLabel={serverLabel}
        hostKey={state.status === 'confirmingHostKey' ? state.hostKey : undefined}
        onTrust={() => void trustHostKeyAndConnect()}
        onCancel={backToForm}
      />
      <ChangedHostKeyDialog
        open={state.status === 'hostKeyChanged'}
        serverLabel={serverLabel}
        hostKey={state.status === 'hostKeyChanged' ? state.hostKey : undefined}
        onClose={backToForm}
      />
    </AppRoot>
  );
}
