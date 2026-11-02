import { Button, Dialog, Field, FLEX_JUSTIFIES, HFlex, Input, Text, VFlex } from '@faber-ui/react';
import { type SubmitEvent, useId, useState } from 'react';

import type { TSecretDialogProps } from './secret-dialog.types';

/**
 * Asks for a password or key passphrase for one connection. The value lives only in this
 * field and is cleared as soon as it is sent or the dialog closes (ADR-0015).
 */
export function SecretDialog({ open, serverLabel, auth, onSubmit, onCancel }: TSecretDialogProps) {
  const formId = useId();
  const [secret, setSecret] = useState('');
  const isPassword = auth.type === 'password';

  const handleSubmit = (event: SubmitEvent<HTMLFormElement>) => {
    event.preventDefault();
    const value = secret;
    setSecret('');
    onSubmit(value);
  };

  const handleCancel = () => {
    setSecret('');
    onCancel();
  };

  return (
    <Dialog
      open={open}
      title={isPassword ? 'Enter the password' : 'Enter the key passphrase'}
      closeLabel="Cancel"
      onClose={handleCancel}
      footer={
        <HFlex gap="SM" justify={FLEX_JUSTIFIES.END}>
          <Button variant="outline" onClick={handleCancel}>
            Cancel
          </Button>
          <Button type="submit" form={formId}>
            Connect
          </Button>
        </HFlex>
      }
    >
      <form id={formId} onSubmit={handleSubmit}>
        <VFlex gap="SM">
          <Text.P>
            {isPassword
              ? `To sign in to ${serverLabel}.`
              : `To unlock ${auth.path} for ${serverLabel}.`}{' '}
            It is used for this connection only and never saved.
          </Text.P>
          <Field label={isPassword ? 'Password' : 'Passphrase'}>
            <Input
              name="secret"
              type="password"
              value={secret}
              onChange={(event) => setSecret(event.target.value)}
              autoComplete="off"
              required
            />
          </Field>
        </VFlex>
      </form>
    </Dialog>
  );
}
