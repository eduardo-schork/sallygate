import { Button, Dialog, Field, FLEX_JUSTIFIES, HFlex, Input } from '@faber-ui/react';
import { type SubmitEvent, useId, useState } from 'react';

import type { TNameDialogProps } from './name-dialog.types';

/**
 * Asks for one file or folder name: a new folder, or a new name for an entry. Mount it when it
 * opens, so each opening starts from `initialName`.
 */
export function NameDialog({
  open,
  title,
  initialName = '',
  submitLabel,
  onSubmit,
  onCancel,
}: TNameDialogProps) {
  const formId = useId();
  const [name, setName] = useState(initialName);

  const handleSubmit = (event: SubmitEvent<HTMLFormElement>) => {
    event.preventDefault();
    const trimmed = name.trim();
    if (trimmed !== '') {
      onSubmit(trimmed);
    }
  };

  return (
    <Dialog
      open={open}
      title={title}
      closeLabel="Cancel"
      onClose={onCancel}
      footer={
        <HFlex gap="SM" justify={FLEX_JUSTIFIES.END}>
          <Button variant="outline" onClick={onCancel}>
            Cancel
          </Button>
          <Button type="submit" form={formId}>
            {submitLabel}
          </Button>
        </HFlex>
      }
    >
      <form id={formId} onSubmit={handleSubmit}>
        <Field label="Name">
          <Input
            name="name"
            value={name}
            onChange={(event) => setName(event.target.value)}
            autoComplete="off"
            spellCheck={false}
            required
          />
        </Field>
      </form>
    </Dialog>
  );
}
