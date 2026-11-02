import {
  Alert,
  Button,
  EmptyState,
  FLEX_ALIGNS,
  FLEX_JUSTIFIES,
  FLEX_WRAPS,
  HFlex,
  Spinner,
  Text,
  Title,
  VFlex,
} from '@faber-ui/react';
import { PlusIcon } from '@faber-ui/icons';
import { useState } from 'react';

import { groupHosts, matchesSearch } from '../group-hosts';
import { HostCard } from '../host-card';
import { HostGrid, HostsSearch, HostsToolbar, HostsViewRoot } from './hosts-view.styles';
import type { THostsViewProps } from './hosts-view.types';

/** The saved hosts, grouped, with search and a way to add more. */
export function HostsView({
  hosts,
  isLoading,
  loadError,
  onNewHost,
  onConnect,
  onEdit,
}: THostsViewProps) {
  const [search, setSearch] = useState('');
  const groups = groupHosts(hosts.filter((host) => matchesSearch(host, search)));

  const newHostButton = (
    <Button startIcon={<PlusIcon />} onClick={onNewHost}>
      New host
    </Button>
  );

  return (
    <HostsViewRoot gap="LG">
      <HFlex
        gap="SM"
        align={FLEX_ALIGNS.CENTER}
        justify={FLEX_JUSTIFIES.SPACE_BETWEEN}
        wrap={FLEX_WRAPS.WRAP}
      >
        <Title.H1>Hosts</Title.H1>
        <HostsToolbar>
          <HostsSearch
            type="search"
            aria-label="Search hosts"
            placeholder="Search by name, group or address"
            value={search}
            onChange={(event) => setSearch(event.target.value)}
          />
          {newHostButton}
        </HostsToolbar>
      </HFlex>

      {loadError ? (
        <Alert color="error" title="Could not load your hosts" role="alert">
          {loadError.message}
        </Alert>
      ) : null}

      {isLoading ? <Spinner label="Loading hosts" /> : null}

      {!isLoading && hosts.length === 0 && !loadError ? (
        <EmptyState
          title="No hosts yet"
          description="Save a server once, then open a terminal on it with one click."
          actions={newHostButton}
        />
      ) : null}

      {hosts.length > 0 && groups.length === 0 ? (
        <Text.P>No host matches “{search}”.</Text.P>
      ) : null}

      {groups.map((group) => (
        <VFlex key={group.name ?? ''} gap="SM" as="section" aria-label={group.name ?? 'No group'}>
          <Title.H2>
            {group.name ?? 'No group'} <Text.Span>({group.hosts.length})</Text.Span>
          </Title.H2>
          <HostGrid>
            {group.hosts.map((host) => (
              <HostCard key={host.id} host={host} onConnect={onConnect} onEdit={onEdit} />
            ))}
          </HostGrid>
        </VFlex>
      ))}
    </HostsViewRoot>
  );
}
