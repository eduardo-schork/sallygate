import { Alert, Button, Dialog, Field, FLEX_JUSTIFIES, HFlex, Input, VFlex } from '@faber-ui/react';
import { type SubmitEvent, useId, useState } from 'react';

import { CommandField } from './snippet-dialog.styles';
import type { TSnippetDialogProps } from './snippet-dialog.types';

/** Creates or edits one snippet. Mount it when it opens, so it starts from `snippet`. */
export function SnippetDialog({ snippet, error, onSave, onCancel }: TSnippetDialogProps) {
  const formId = useId();
  const [name, setName] = useState(snippet?.name ?? '');
  const [command, setCommand] = useState(snippet?.command ?? '');

  const handleSubmit = (event: SubmitEvent<HTMLFormElement>) => {
    event.preventDefault();
    onSave({ ...(snippet ? { id: snippet.id } : {}), name, command });
  };

  return (
    <Dialog
      open
      title={snippet ? `Edit ${snippet.name}` : 'New snippet'}
      closeLabel="Cancel"
      onClose={onCancel}
      footer={
        <HFlex gap="SM" justify={FLEX_JUSTIFIES.END}>
          <Button variant="outline" onClick={onCancel}>
            Cancel
          </Button>
          <Button type="submit" form={formId}>
            Save snippet
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
          <Field label="Name">
            <Input
              name="name"
              value={name}
              placeholder="Restart nginx"
              onChange={(event) => setName(event.target.value)}
              autoComplete="off"
              required
            />
          </Field>
          <Field
            label="Command"
            description="Sent to the terminal as typed. Each line runs in turn."
          >
            <CommandField
              name="command"
              value={command}
              rows={4}
              placeholder="sudo systemctl restart nginx"
              onChange={(event) => setCommand(event.target.value)}
              spellCheck={false}
              required
            />
          </Field>
        </VFlex>
      </form>
    </Dialog>
  );
}
