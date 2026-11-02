import { Button, EmptyState, FLEX_JUSTIFIES, HFlex } from '@faber-ui/react';

import type { TSessionEndedNoticeProps } from './session-ended-notice.types';

function describeEnd({ end }: Pick<TSessionEndedNoticeProps, 'end'>): string {
  if (end.type === 'exited') {
    return end.exitStatus === 0
      ? 'The shell exited normally.'
      : `The shell exited with status ${String(end.exitStatus)}.`;
  }
  return `The connection dropped: ${end.message}.`;
}

/** Shown in place of the terminal once a session ends on its own. */
export function SessionEndedNotice({
  serverLabel,
  end,
  onReconnect,
  onClose,
}: TSessionEndedNoticeProps) {
  return (
    <EmptyState
      title={`Session on ${serverLabel} ended`}
      description={describeEnd({ end })}
      actions={
        <HFlex gap="SM" justify={FLEX_JUSTIFIES.CENTER}>
          <Button variant="outline" onClick={onClose}>
            Close tab
          </Button>
          <Button onClick={onReconnect}>Reconnect</Button>
        </HFlex>
      }
    />
  );
}
