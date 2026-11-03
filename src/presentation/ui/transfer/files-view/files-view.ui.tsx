import {
  Alert,
  AlertDialog,
  Button,
  FLEX_ALIGNS,
  FLEX_JUSTIFIES,
  HFlex,
  IconButton,
  Progress,
  Spinner,
  Table,
  Text,
  VFlex,
  VisuallyHidden,
} from '@faber-ui/react';
import {
  ArrowUpIcon,
  DownloadIcon,
  EditIcon,
  FileIcon,
  FolderIcon,
  LinkIcon,
  PlusIcon,
  RefreshIcon,
  TrashIcon,
  UploadIcon,
} from '@faber-ui/icons';
import { useEffect, useState } from 'react';

import { formatSize } from '../format-size';
import { NameDialog } from '../name-dialog';
import type { TRemoteEntry } from '../transfer-contract';
import { type TTransfer, useRemoteFiles } from '../use-remote-files';
import {
  EntryName,
  FilesTableScroll,
  FilesToolbar,
  FilesViewRoot,
  FolderPath,
  MetaCell,
  TransfersList,
} from './files-view.styles';
import type { TFilesViewProps } from './files-view.types';

const DATE_FORMAT = new Intl.DateTimeFormat(undefined, { dateStyle: 'medium', timeStyle: 'short' });

const ENTRY_ICONS = {
  file: <FileIcon />,
  folder: <FolderIcon />,
  link: <LinkIcon />,
} as const;

/** What the user is naming or confirming in a dialog, if anything. */
type TPrompt =
  | { readonly kind: 'newFolder' }
  | { readonly kind: 'rename'; readonly entry: TRemoteEntry }
  | { readonly kind: 'delete'; readonly entry: TRemoteEntry };

function transferStatus(transfer: TTransfer): string {
  if (transfer.status === 'failed') {
    return `Failed: ${transfer.detail ?? 'unknown error'}`;
  }
  if (transfer.status === 'done') {
    return transfer.direction === 'download'
      ? `Saved to ${transfer.detail ?? 'Downloads'}`
      : 'Uploaded';
  }
  const verb = transfer.direction === 'download' ? 'Downloading' : 'Uploading';
  return transfer.totalBytes > 0
    ? `${verb} ${formatSize(transfer.doneBytes)} of ${formatSize(transfer.totalBytes)}`
    : `${verb}…`;
}

/** The server's files for one session: browse, tidy, and copy both ways. */
export function FilesView({ sessionId, isActive, files }: TFilesViewProps) {
  const remote = useRemoteFiles(sessionId, files);
  const [prompt, setPrompt] = useState<TPrompt>();
  const { onFilesDropped, upload } = remote;

  useEffect(() => {
    if (!isActive) {
      return undefined;
    }
    const stopListening = onFilesDropped((paths) => void upload(paths));
    return () => {
      void stopListening.then((stop) => {
        stop();
      });
    };
  }, [isActive, onFilesDropped, upload]);

  const closePrompt = () => setPrompt(undefined);
  const entries = remote.folder?.entries ?? [];
  const hasFinishedTransfers = remote.transfers.some((transfer) => transfer.status !== 'running');

  return (
    <FilesViewRoot gap="SM">
      <FilesToolbar>
        <IconButton
          variant="subtle"
          color="neutral"
          aria-label="Go to the parent folder"
          disabled={remote.folder?.path === '/'}
          onClick={() => void remote.goUp()}
        >
          <ArrowUpIcon />
        </IconButton>
        <FolderPath title={remote.folder?.path}>{remote.folder?.path ?? '…'}</FolderPath>
        <IconButton
          variant="subtle"
          color="neutral"
          aria-label="Refresh"
          onClick={() => void remote.refresh()}
        >
          <RefreshIcon />
        </IconButton>
        <Button
          variant="outline"
          color="neutral"
          startIcon={<PlusIcon />}
          onClick={() => setPrompt({ kind: 'newFolder' })}
        >
          New folder
        </Button>
        <Button startIcon={<UploadIcon />} onClick={() => void remote.pickAndUpload()}>
          Upload
        </Button>
      </FilesToolbar>

      {remote.error ? (
        <Alert color="error" role="alert">
          {remote.error.message}
        </Alert>
      ) : null}

      <FilesTableScroll>
        {remote.isLoading && !remote.folder ? <Spinner label="Loading files" /> : null}
        {remote.folder && entries.length === 0 ? (
          <Text.P>This folder is empty. Drop files on the window to upload them here.</Text.P>
        ) : null}
        {entries.length > 0 ? (
          <Table aria-label={`Files in ${remote.folder?.path ?? ''}`}>
            <thead>
              <tr>
                <th scope="col">Name</th>
                <th scope="col">Size</th>
                <th scope="col">Modified</th>
                <th scope="col">
                  <VisuallyHidden>Actions</VisuallyHidden>
                </th>
              </tr>
            </thead>
            <tbody>
              {entries.map((entry) => (
                <tr key={entry.path}>
                  <td>
                    {entry.kind === 'folder' ? (
                      <Button
                        variant="subtle"
                        color="neutral"
                        size="small"
                        startIcon={ENTRY_ICONS.folder}
                        onClick={() => void remote.open(entry.path)}
                      >
                        {entry.name}
                      </Button>
                    ) : (
                      <EntryName>
                        {ENTRY_ICONS[entry.kind]}
                        <span>{entry.name}</span>
                      </EntryName>
                    )}
                  </td>
                  <MetaCell>{entry.kind === 'folder' ? '' : formatSize(entry.sizeBytes)}</MetaCell>
                  <MetaCell>
                    {entry.modifiedAt === undefined
                      ? ''
                      : DATE_FORMAT.format(new Date(entry.modifiedAt * 1000))}
                  </MetaCell>
                  <td>
                    <HFlex gap="XXS" justify={FLEX_JUSTIFIES.END}>
                      {entry.kind === 'file' ? (
                        <IconButton
                          size="small"
                          variant="subtle"
                          color="neutral"
                          aria-label={`Download ${entry.name}`}
                          onClick={() => void remote.download(entry)}
                        >
                          <DownloadIcon />
                        </IconButton>
                      ) : null}
                      <IconButton
                        size="small"
                        variant="subtle"
                        color="neutral"
                        aria-label={`Rename ${entry.name}`}
                        onClick={() => setPrompt({ kind: 'rename', entry })}
                      >
                        <EditIcon />
                      </IconButton>
                      <IconButton
                        size="small"
                        variant="subtle"
                        color="neutral"
                        aria-label={`Delete ${entry.name}`}
                        onClick={() => setPrompt({ kind: 'delete', entry })}
                      >
                        <TrashIcon />
                      </IconButton>
                    </HFlex>
                  </td>
                </tr>
              ))}
            </tbody>
          </Table>
        ) : null}
      </FilesTableScroll>

      {remote.transfers.length > 0 ? (
        <TransfersList gap="XS" as="section" aria-label="Transfers">
          <HFlex align={FLEX_ALIGNS.CENTER} justify={FLEX_JUSTIFIES.SPACE_BETWEEN}>
            <Text.Strong>Transfers</Text.Strong>
            {hasFinishedTransfers ? (
              <Button
                size="small"
                variant="subtle"
                color="neutral"
                onClick={remote.clearFinishedTransfers}
              >
                Clear finished
              </Button>
            ) : null}
          </HFlex>
          {remote.transfers.map((transfer) => (
            <VFlex key={transfer.id} gap="XXS">
              <HFlex gap="SM" justify={FLEX_JUSTIFIES.SPACE_BETWEEN}>
                <Text.Small>{transfer.name}</Text.Small>
                <Text.Small>{transferStatus(transfer)}</Text.Small>
              </HFlex>
              {transfer.status === 'running' ? (
                <Progress
                  label={`${transfer.direction === 'download' ? 'Downloading' : 'Uploading'} ${transfer.name}`}
                  value={transfer.doneBytes}
                  max={Math.max(transfer.totalBytes, 1)}
                />
              ) : null}
            </VFlex>
          ))}
        </TransfersList>
      ) : null}

      {prompt?.kind === 'newFolder' ? (
        <NameDialog
          open
          title="New folder"
          submitLabel="Create folder"
          onSubmit={(name) => {
            closePrompt();
            void remote.makeFolder(name);
          }}
          onCancel={closePrompt}
        />
      ) : null}
      {prompt?.kind === 'rename' ? (
        <NameDialog
          open
          title={`Rename ${prompt.entry.name}`}
          initialName={prompt.entry.name}
          submitLabel="Rename"
          onSubmit={(name) => {
            closePrompt();
            void remote.rename(prompt.entry, name);
          }}
          onCancel={closePrompt}
        />
      ) : null}
      <AlertDialog
        open={prompt?.kind === 'delete'}
        title={prompt?.kind === 'delete' ? `Delete ${prompt.entry.name}?` : 'Delete?'}
        confirmLabel="Delete"
        cancelLabel="Cancel"
        destructive
        onConfirm={() => {
          if (prompt?.kind === 'delete') {
            void remote.remove(prompt.entry);
          }
          closePrompt();
        }}
        onCancel={closePrompt}
      >
        <Text.P>
          {prompt?.kind === 'delete' && prompt.entry.kind === 'folder'
            ? 'The folder is removed from the server. Only empty folders can be deleted.'
            : 'The file is removed from the server. This cannot be undone.'}
        </Text.P>
      </AlertDialog>
    </FilesViewRoot>
  );
}
