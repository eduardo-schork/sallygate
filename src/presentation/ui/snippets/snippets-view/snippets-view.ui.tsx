import {
  Alert,
  AlertDialog,
  Button,
  EmptyState,
  FLEX_ALIGNS,
  FLEX_JUSTIFIES,
  HFlex,
  IconButton,
  Spinner,
  Text,
  Title,
} from '@faber-ui/react';
import { EditIcon, PlusIcon, TrashIcon } from '@faber-ui/icons';
import { useState } from 'react';

import type { TCommandError } from '../../shared/command-error-contract';
import { SnippetDialog } from '../snippet-dialog';
import type { TSaveSnippetRequest, TSnippet } from '../snippets-contract';
import {
  SnippetCard,
  SnippetCommandPreview,
  SnippetList,
  SnippetText,
  SnippetsViewRoot,
} from './snippets-view.styles';
import type { TSnippetsViewProps } from './snippets-view.types';

type TEditing = { readonly snippet?: TSnippet; readonly error?: TCommandError };

/** Saved commands: write them once, send them to any terminal from its side panel. */
export function SnippetsView({
  snippets,
  isLoading,
  loadError,
  onSave,
  onDelete,
}: TSnippetsViewProps) {
  const [editing, setEditing] = useState<TEditing>();
  const [deleting, setDeleting] = useState<TSnippet>();
  const [deleteError, setDeleteError] = useState<TCommandError>();

  const save = async (request: TSaveSnippetRequest) => {
    const error = await onSave(request);
    setEditing(error ? { ...editing, error } : undefined);
  };

  const confirmDelete = async (snippet: TSnippet) => {
    setDeleting(undefined);
    setDeleteError(await onDelete(snippet.id));
  };

  const newSnippetButton = (
    <Button startIcon={<PlusIcon />} onClick={() => setEditing({})}>
      New snippet
    </Button>
  );

  return (
    <SnippetsViewRoot gap="LG">
      <HFlex gap="SM" align={FLEX_ALIGNS.CENTER} justify={FLEX_JUSTIFIES.SPACE_BETWEEN}>
        <Title.H1>Snippets</Title.H1>
        {snippets.length > 0 ? newSnippetButton : null}
      </HFlex>

      {(loadError ?? deleteError) ? (
        <Alert color="error" role="alert">
          {(loadError ?? deleteError)?.message}
        </Alert>
      ) : null}

      {isLoading ? <Spinner label="Loading snippets" /> : null}

      {!isLoading && snippets.length === 0 && !loadError ? (
        <EmptyState
          title="No snippets yet"
          description="Save a command you type often. Then send it to any open terminal from the panel next to it."
          actions={newSnippetButton}
        />
      ) : null}

      <SnippetList gap="SM" aria-label="Snippets">
        {snippets.map((snippet) => (
          <li key={snippet.id}>
            <SnippetCard>
              <SnippetText gap="XXS">
                <Text.Strong>{snippet.name}</Text.Strong>
                <SnippetCommandPreview>{snippet.command}</SnippetCommandPreview>
              </SnippetText>
              <IconButton
                size="small"
                variant="subtle"
                color="neutral"
                aria-label={`Edit ${snippet.name}`}
                onClick={() => setEditing({ snippet })}
              >
                <EditIcon />
              </IconButton>
              <IconButton
                size="small"
                variant="subtle"
                color="neutral"
                aria-label={`Delete ${snippet.name}`}
                onClick={() => setDeleting(snippet)}
              >
                <TrashIcon />
              </IconButton>
            </SnippetCard>
          </li>
        ))}
      </SnippetList>

      {editing ? (
        <SnippetDialog
          snippet={editing.snippet}
          error={editing.error}
          onSave={(request) => void save(request)}
          onCancel={() => setEditing(undefined)}
        />
      ) : null}
      <AlertDialog
        open={deleting !== undefined}
        title={`Delete ${deleting?.name ?? 'snippet'}?`}
        confirmLabel="Delete"
        cancelLabel="Cancel"
        destructive
        onConfirm={() => {
          if (deleting) {
            void confirmDelete(deleting);
          }
        }}
        onCancel={() => setDeleting(undefined)}
      >
        <Text.P>The snippet is removed from every terminal&apos;s panel.</Text.P>
      </AlertDialog>
    </SnippetsViewRoot>
  );
}
