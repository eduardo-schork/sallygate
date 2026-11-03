import { Alert, Button, Dialog, Field, FLEX_JUSTIFIES, HFlex, Input, VFlex } from '@faber-ui/react';
import { type SubmitEvent, useId, useState } from 'react';

import type { TGenerateKeyDialogProps } from './generate-key-dialog.types';

/**
 * Makes a new Ed25519 key in ~/.ssh. Mount it when it opens. The passphrase lives only in its
 * field and is cleared once sent (ADR-0015).
 */
export function GenerateKeyDialog({
  error,
  isGenerating,
  onGenerate,
  onCancel,
}: TGenerateKeyDialogProps) {
  const formId = useId();
  const [name, setName] = useState('');
  const [comment, setComment] = useState('');
  const [passphrase, setPassphrase] = useState('');

  const handleSubmit = (event: SubmitEvent<HTMLFormElement>) => {
    event.preventDefault();
    const typed = passphrase;
    setPassphrase('');
    onGenerate({ name, comment, ...(typed === '' ? {} : { passphrase: typed }) });
  };

  return (
    <Dialog
      open
      title="Generate a key"
      closeLabel="Cancel"
      onClose={onCancel}
      footer={
        <HFlex gap="SM" justify={FLEX_JUSTIFIES.END}>
          <Button variant="outline" onClick={onCancel}>
            Cancel
          </Button>
          <Button type="submit" form={formId} loading={isGenerating}>
            Generate key
          </Button>
        </HFlex>
      }
    >
      <form id={formId} onSubmit={handleSubmit}>
        <VFlex gap="MD">
          {error ? (
            <Alert color="error" role="alert">
              {error.message}
            </Alert>
          ) : null}
          <Field label="Name" description="The file in ~/.ssh. Its public half gets .pub.">
            <Input
              name="name"
              value={name}
              placeholder="id_ed25519_oracle"
              onChange={(event) => setName(event.target.value)}
              autoComplete="off"
              spellCheck={false}
              required
            />
          </Field>
          <Field
            label="Comment"
            description="Shown at the end of the public key, to tell keys apart."
          >
            <Input
              name="comment"
              value={comment}
              placeholder="you@your-mac"
              onChange={(event) => setComment(event.target.value)}
              autoComplete="off"
              spellCheck={false}
            />
          </Field>
          <Field
            label="Passphrase (optional)"
            description="Protects the key file. You type it when you connect; Sallygate never saves it."
          >
            <Input
              name="passphrase"
              type="password"
              value={passphrase}
              onChange={(event) => setPassphrase(event.target.value)}
              autoComplete="new-password"
            />
          </Field>
        </VFlex>
      </form>
    </Dialog>
  );
}
