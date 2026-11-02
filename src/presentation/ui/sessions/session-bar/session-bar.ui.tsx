import { Button, Text } from '@faber-ui/react';

import { SessionBarRoot } from './session-bar.styles';
import type { TSessionBarProps } from './session-bar.types';

/** Names the server the terminal is connected to and closes the session. */
export function SessionBar({ serverLabel, onClose }: TSessionBarProps) {
  return (
    <SessionBarRoot as="header">
      <Text.Strong>{serverLabel}</Text.Strong>
      <Button size="small" variant="subtle" onClick={onClose}>
        Close session
      </Button>
    </SessionBarRoot>
  );
}
