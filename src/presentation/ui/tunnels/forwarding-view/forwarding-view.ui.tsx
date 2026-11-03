import {
  Alert,
  AlertDialog,
  Badge,
  Button,
  EmptyState,
  FLEX_ALIGNS,
  FLEX_JUSTIFIES,
  HFlex,
  IconButton,
  Spinner,
  Switch,
  Text,
  Title,
} from '@faber-ui/react';
import { EditIcon, PlusIcon, TrashIcon } from '@faber-ui/icons';
import { useState } from 'react';

import type { THost } from '../../inventory/inventory-contract';
import { CONNECTION_ERROR_COPY } from '../../sessions/connection-error-copy';
import { SecretDialog } from '../../sessions/secret-dialog';
import type { TCommandError } from '../../shared/command-error-contract';
import { formatEndpoint } from '../../shared/format-endpoint';
import type { THostKey } from '../../trust/trust-contract';
import { UnknownHostKeyDialog } from '../../trust/unknown-host-key-dialog';
import { describeTunnel } from '../describe-tunnel';
import { TunnelDialog } from '../tunnel-dialog';
import type { TSaveTunnelRequest, TTunnel } from '../tunnels-contract';
import { useTunnels } from '../use-tunnels';
import {
  ForwardingViewRoot,
  RuleCard,
  RuleList,
  RuleRoute,
  RuleText,
} from './forwarding-view.styles';
import type { TForwardingViewProps } from './forwarding-view.types';

/** A dialog in front of the list, if any. */
type TPrompt =
  | { readonly kind: 'edit'; readonly tunnel?: TTunnel; readonly error?: TCommandError }
  | { readonly kind: 'delete'; readonly tunnel: TTunnel }
  | { readonly kind: 'secret'; readonly tunnel: TTunnel; readonly host: THost }
  | {
      readonly kind: 'hostKey';
      readonly tunnel: TTunnel;
      readonly host: THost;
      readonly hostKey: THostKey;
    };

/** Forwarding rules: use a service on a server as if it ran on this Mac. */
export function ForwardingView({ hosts, tunnels: gateway, trust }: TForwardingViewProps) {
  const { tunnels, isLoading, error, save, remove, turnOn, turnOff, trustHostKey } = useTunnels(
    gateway,
    trust,
  );
  const [prompt, setPrompt] = useState<TPrompt>();
  const [failure, setFailure] = useState<TCommandError>();
  const closePrompt = () => setPrompt(undefined);
  const hostOf = (tunnel: TTunnel) => hosts.find((host) => host.id === tunnel.hostId);

  /** Turns a rule on, asking for a password or a server key check when the core needs one. */
  const start = async (tunnel: TTunnel, secret?: string) => {
    setFailure(undefined);
    const startError = await turnOn(tunnel.id, secret);
    const host = hostOf(tunnel);
    if (!startError) {
      return;
    }
    if (host && startError.kind === 'secretRequired') {
      setPrompt({ kind: 'secret', tunnel, host });
    } else if (host && startError.kind === 'unknownHostKey' && startError.hostKey) {
      setPrompt({ kind: 'hostKey', tunnel, host, hostKey: startError.hostKey });
    } else {
      setFailure(startError);
    }
  };

  const trustAndStart = async (tunnel: TTunnel, host: THost, hostKey: THostKey) => {
    closePrompt();
    await trustHostKey(host.endpoint, hostKey);
    await start(tunnel);
  };

  const saveRule = async (request: TSaveTunnelRequest, tunnel?: TTunnel) => {
    const saveError = await save(request);
    setPrompt(
      saveError ? { kind: 'edit', error: saveError, ...(tunnel ? { tunnel } : {}) } : undefined,
    );
  };

  const newRuleButton = (
    <Button startIcon={<PlusIcon />} onClick={() => setPrompt({ kind: 'edit' })}>
      New rule
    </Button>
  );

  return (
    <ForwardingViewRoot gap="LG">
      <HFlex gap="SM" align={FLEX_ALIGNS.CENTER} justify={FLEX_JUSTIFIES.SPACE_BETWEEN}>
        <Title.H1>Port forwarding</Title.H1>
        {hosts.length > 0 && tunnels.length > 0 ? newRuleButton : null}
      </HFlex>
      <Text.P>
        Use a service on a server as if it ran on this Mac, without opening a port in its firewall.
        Rules listen on this Mac only.
      </Text.P>

      {(error ?? failure) ? (
        <Alert
          color="error"
          role="alert"
          title={failure ? CONNECTION_ERROR_COPY[failure.kind].title : undefined}
        >
          {(failure ?? error)?.message}
        </Alert>
      ) : null}
      {isLoading ? <Spinner label="Loading rules" /> : null}
      {!isLoading && hosts.length === 0 ? (
        <EmptyState
          title="Save a host first"
          description="A rule sends traffic through one of your saved hosts."
        />
      ) : null}
      {!isLoading && hosts.length > 0 && tunnels.length === 0 && !error ? (
        <EmptyState
          title="No rules yet"
          description="Reach a database or dashboard that only the server can see."
          actions={newRuleButton}
        />
      ) : null}

      <RuleList gap="SM" aria-label="Forwarding rules">
        {tunnels.map((tunnel) => {
          const host = hostOf(tunnel);
          return (
            <li key={tunnel.id}>
              <RuleCard>
                <Badge color={tunnel.kind.type === 'local' ? 'primary' : 'neutral'}>
                  {tunnel.kind.type === 'local' ? 'Local' : 'SOCKS'}
                </Badge>
                <RuleText gap="XXS">
                  <Text.Strong>{tunnel.name}</Text.Strong>
                  <RuleRoute>
                    {describeTunnel(tunnel.kind, host?.label ?? 'a deleted host')}
                  </RuleRoute>
                </RuleText>
                <Switch
                  label={tunnel.isOn ? 'On' : 'Off'}
                  aria-label={`${tunnel.name} on`}
                  checked={tunnel.isOn}
                  onChange={() => void (tunnel.isOn ? turnOff(tunnel.id) : start(tunnel))}
                />
                <IconButton
                  size="small"
                  variant="subtle"
                  color="neutral"
                  aria-label={`Edit ${tunnel.name}`}
                  onClick={() => setPrompt({ kind: 'edit', tunnel })}
                >
                  <EditIcon />
                </IconButton>
                <IconButton
                  size="small"
                  variant="subtle"
                  color="neutral"
                  aria-label={`Delete ${tunnel.name}`}
                  onClick={() => setPrompt({ kind: 'delete', tunnel })}
                >
                  <TrashIcon />
                </IconButton>
              </RuleCard>
            </li>
          );
        })}
      </RuleList>

      {prompt?.kind === 'edit' ? (
        <TunnelDialog
          tunnel={prompt.tunnel}
          hosts={hosts}
          error={prompt.error}
          onSave={(request) => void saveRule(request, prompt.tunnel)}
          onCancel={closePrompt}
        />
      ) : null}
      <AlertDialog
        open={prompt?.kind === 'delete'}
        title={`Delete ${prompt?.kind === 'delete' ? prompt.tunnel.name : 'rule'}?`}
        confirmLabel="Delete"
        cancelLabel="Cancel"
        destructive
        onConfirm={() => {
          if (prompt?.kind === 'delete') {
            void remove(prompt.tunnel.id);
          }
          closePrompt();
        }}
        onCancel={closePrompt}
      >
        <Text.P>The rule is turned off and removed.</Text.P>
      </AlertDialog>
      {prompt?.kind === 'secret' ? (
        <SecretDialog
          open
          serverLabel={formatEndpoint(prompt.host.endpoint)}
          auth={prompt.host.auth}
          onSubmit={(secret) => {
            closePrompt();
            void start(prompt.tunnel, secret);
          }}
          onCancel={closePrompt}
        />
      ) : null}
      <UnknownHostKeyDialog
        open={prompt?.kind === 'hostKey'}
        serverLabel={prompt?.kind === 'hostKey' ? formatEndpoint(prompt.host.endpoint) : ''}
        hostKey={prompt?.kind === 'hostKey' ? prompt.hostKey : undefined}
        onTrust={() => {
          if (prompt?.kind === 'hostKey') {
            void trustAndStart(prompt.tunnel, prompt.host, prompt.hostKey);
          }
        }}
        onCancel={closePrompt}
      />
    </ForwardingViewRoot>
  );
}
