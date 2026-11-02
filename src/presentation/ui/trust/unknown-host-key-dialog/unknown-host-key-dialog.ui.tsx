import { AlertDialog, CodeBlock, Text, VFlex } from '@faber-ui/react';

import type { TUnknownHostKeyDialogProps } from './unknown-host-key-dialog.types';

/** First connection to a server: the user compares its fingerprint before trusting it. */
export function UnknownHostKeyDialog({
  open,
  serverLabel,
  hostKey,
  onTrust,
  onCancel,
}: TUnknownHostKeyDialogProps) {
  return (
    <AlertDialog
      open={open}
      title="Trust this server?"
      confirmLabel="Trust and connect"
      cancelLabel="Cancel"
      onConfirm={onTrust}
      onCancel={onCancel}
    >
      <VFlex gap="SM">
        <Text.P>
          This is the first connection to {serverLabel}. Compare this fingerprint with the one the
          server shows for its own key before you trust it.
        </Text.P>
        {hostKey ? (
          <CodeBlock label={hostKey.algorithm} code={hostKey.fingerprint} copyLabel="Copy" />
        ) : null}
        <Text.Small>
          On the server, <Text.Code>ssh-keygen -lf /etc/ssh/ssh_host_ed25519_key.pub</Text.Code>{' '}
          prints it.
        </Text.Small>
      </VFlex>
    </AlertDialog>
  );
}
