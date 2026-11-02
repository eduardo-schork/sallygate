import { Button, HFlex, IconButton, Text } from '@faber-ui/react';
import { EditIcon, MonitorIcon } from '@faber-ui/icons';

import { formatEndpoint } from '../../shared/format-endpoint';
import { HostCardIcon, HostCardRoot, HostCardText } from './host-card.styles';
import type { THostCardProps } from './host-card.types';

/** A saved host: its name, where it points, and the two things you do with it. */
export function HostCard({ host, onConnect, onEdit }: THostCardProps) {
  return (
    <HostCardRoot forwardedAs="article" aria-label={host.label}>
      <HostCardIcon aria-hidden="true">
        <MonitorIcon />
      </HostCardIcon>
      <HostCardText>
        <Text.Strong>{host.label}</Text.Strong>
        <Text.Small>{formatEndpoint(host.endpoint)}</Text.Small>
      </HostCardText>
      <HFlex gap="XXS">
        <IconButton
          size="small"
          variant="subtle"
          color="neutral"
          aria-label={`Edit ${host.label}`}
          onClick={() => onEdit(host)}
        >
          <EditIcon />
        </IconButton>
        <Button size="small" onClick={() => onConnect(host)}>
          Connect
        </Button>
      </HFlex>
    </HostCardRoot>
  );
}
