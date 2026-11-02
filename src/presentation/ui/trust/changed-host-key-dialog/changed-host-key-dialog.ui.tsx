import { Alert, Button, CodeBlock, Dialog, Text, VFlex } from '@faber-ui/react';

import type { TChangedHostKeyDialogProps } from './changed-host-key-dialog.types';

/**
 * The server presented a different key than the one trusted before. Sallygate refuses to
 * connect and offers no shortcut around it: the user must fix it on purpose.
 */
export function ChangedHostKeyDialog({
  open,
  serverLabel,
  hostKey,
  onClose,
}: TChangedHostKeyDialogProps) {
  return (
    <Dialog
      open={open}
      onClose={onClose}
      title="This server's key changed"
      closeLabel="Close"
      footer={<Button onClick={onClose}>Close</Button>}
    >
      <VFlex gap="SM">
        <Alert color="error" title="Sallygate did not connect">
          Someone may be impersonating {serverLabel}.
        </Alert>
        {hostKey ? (
          <CodeBlock
            label={`New ${hostKey.algorithm} key`}
            code={hostKey.fingerprint}
            copyLabel="Copy"
          />
        ) : null}
        <Text.P>
          If you reinstalled the server or replaced its keys yourself, remove its old line from{' '}
          <Text.Code>~/.ssh/known_hosts</Text.Code> and connect again to review the new key.
        </Text.P>
      </VFlex>
    </Dialog>
  );
}
