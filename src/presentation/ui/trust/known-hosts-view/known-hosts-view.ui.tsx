import {
  Alert,
  AlertDialog,
  Button,
  EmptyState,
  Spinner,
  Table,
  Text,
  Title,
  VisuallyHidden,
} from '@faber-ui/react';
import { useState } from 'react';

import type { TKnownHost } from '../trust-contract';
import { useKnownHosts } from '../use-known-hosts';
import { KnownHostsViewRoot, MonoCell, MutedCell } from './known-hosts-view.styles';
import type { TKnownHostsViewProps } from './known-hosts-view.types';

const HASHED_HOSTS = 'Hidden (hashed in the file)';

/** The server keys the user trusts, shared with the `ssh` command; forget one to be asked again. */
export function KnownHostsView({ trust }: TKnownHostsViewProps) {
  const { knownHosts, isLoading, error, forget } = useKnownHosts(trust);
  const [forgetting, setForgetting] = useState<TKnownHost>();

  return (
    <KnownHostsViewRoot gap="LG">
      <Title.H1>Known hosts</Title.H1>
      <Text.P>
        Servers whose keys you trusted, from <Text.Code>~/.ssh/known_hosts</Text.Code>, the same
        list the <Text.Code>ssh</Text.Code> command uses. Forget a key when a server was rebuilt:
        the next connection asks you to check it again.
      </Text.P>

      {error ? (
        <Alert color="error" role="alert">
          {error.message}
        </Alert>
      ) : null}
      {isLoading ? <Spinner label="Loading known hosts" /> : null}
      {!isLoading && knownHosts.length === 0 && !error ? (
        <EmptyState
          title="No trusted servers yet"
          description="When you trust a server on its first connection, it shows up here."
        />
      ) : null}

      {knownHosts.length > 0 ? (
        <Table aria-label="Known hosts">
          <thead>
            <tr>
              <th scope="col">Server</th>
              <th scope="col">Key</th>
              <th scope="col">Fingerprint</th>
              <th scope="col">
                <VisuallyHidden>Actions</VisuallyHidden>
              </th>
            </tr>
          </thead>
          <tbody>
            {knownHosts.map((entry) => (
              <tr key={`${String(entry.line)}-${entry.hostKey.fingerprint}`}>
                <MonoCell>{entry.hosts ?? HASHED_HOSTS}</MonoCell>
                <MutedCell>{entry.hostKey.algorithm}</MutedCell>
                <MonoCell>{entry.hostKey.fingerprint}</MonoCell>
                <td>
                  <Button
                    size="small"
                    variant="subtle"
                    color="neutral"
                    aria-label={`Forget ${entry.hosts ?? 'hashed server'} key`}
                    onClick={() => setForgetting(entry)}
                  >
                    Forget
                  </Button>
                </td>
              </tr>
            ))}
          </tbody>
        </Table>
      ) : null}

      <AlertDialog
        open={forgetting !== undefined}
        title={`Forget the key of ${forgetting?.hosts ?? 'this server'}?`}
        confirmLabel="Forget key"
        cancelLabel="Cancel"
        destructive
        onConfirm={() => {
          if (forgetting) {
            void forget(forgetting);
          }
          setForgetting(undefined);
        }}
        onCancel={() => setForgetting(undefined)}
      >
        <Text.P>
          The line is removed from ~/.ssh/known_hosts. The next connection, from Sallygate or the
          ssh command, asks you to check the server&apos;s fingerprint again.
        </Text.P>
      </AlertDialog>
    </KnownHostsViewRoot>
  );
}
