import {
  AlertDialog,
  Button,
  FLEX_ALIGNS,
  FLEX_JUSTIFIES,
  HFlex,
  IconButton,
  Progress,
  Text,
} from '@faber-ui/react';
import {
  ArrowRightIcon,
  DownloadIcon,
  EditIcon,
  PlusIcon,
  RefreshIcon,
  TrashIcon,
  UploadIcon,
} from '@faber-ui/icons';
import { useEffect, useState } from 'react';

import { FilePane } from '../file-pane';
import { formatSize } from '../format-size';
import { NameDialog } from '../name-dialog';
import type { TFileEntry } from '../transfer-contract';
import { useLocalFiles } from '../use-local-files';
import { type TTransfer, useRemoteFiles } from '../use-remote-files';
import {
  FilesViewRoot,
  Panes,
  TransferBar,
  TransferPercent,
  TransferRow,
  TransferStatus,
} from './files-view.styles';
import type { TFilesViewProps } from './files-view.types';

const LOCAL_LABEL = 'This Mac';

/** What the user is naming or confirming in a dialog, if anything. */
type TPrompt =
  | { readonly kind: 'newFolder' }
  | { readonly kind: 'rename'; readonly entry: TFileEntry }
  | { readonly kind: 'delete'; readonly entry: TFileEntry };

function transferStatus(transfer: TTransfer): string {
  if (transfer.status === 'failed') {
    return `Failed: ${transfer.detail ?? 'unknown error'}`;
  }
  if (transfer.status === 'done') {
    return transfer.direction === 'download'
      ? `Saved to ${transfer.detail ?? LOCAL_LABEL}`
      : `Uploaded to ${transfer.detail ?? 'the server'}`;
  }
  const verb = transfer.direction === 'download' ? 'Downloading' : 'Uploading';
  return transfer.totalBytes > 0
    ? `${verb} ${formatSize(transfer.doneBytes)} of ${formatSize(transfer.totalBytes)}`
    : `${verb}…`;
}

function percent(transfer: TTransfer): string {
  const share = transfer.totalBytes > 0 ? transfer.doneBytes / transfer.totalBytes : 0;
  return `${String(Math.round(share * 100))}%`;
}

/** This Mac beside the server: browse both, and drag files across to copy them. */
export function FilesView({ sessionId, serverLabel, isActive, files }: TFilesViewProps) {
  const local = useLocalFiles(files);
  const remote = useRemoteFiles(sessionId, files);
  const [prompt, setPrompt] = useState<TPrompt>();
  const { onFilesDropped, upload } = remote;
  const localFolder = local.folder?.path;

  // Files dropped from Finder go to the server folder on screen.
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

  const download = async (entry: TFileEntry) => {
    if (localFolder === undefined) {
      return;
    }
    await remote.download(entry, localFolder);
    await local.refresh();
  };

  const closePrompt = () => setPrompt(undefined);
  const hasFinishedTransfers = remote.transfers.some((transfer) => transfer.status !== 'running');
  const iconButton = { size: 'small', variant: 'subtle', color: 'neutral' } as const;

  return (
    <FilesViewRoot>
      <Panes>
        <FilePane
          side="local"
          label={LOCAL_LABEL}
          folder={local.folder}
          isLoading={local.isLoading}
          error={local.error}
          onOpenFolder={(path) => void local.open(path)}
          onGoUp={() => void local.goUp()}
          toolbar={
            <IconButton
              {...iconButton}
              aria-label={`Refresh ${LOCAL_LABEL}`}
              onClick={() => void local.refresh()}
            >
              <RefreshIcon />
            </IconButton>
          }
          renderActions={(entry) =>
            entry.kind === 'file' ? (
              <IconButton
                {...iconButton}
                aria-label={`Upload ${entry.name} to the server`}
                onClick={() => void remote.upload([entry.path])}
              >
                <ArrowRightIcon />
              </IconButton>
            ) : null
          }
          onDropFile={(entry) => void download(entry)}
        />
        <FilePane
          side="remote"
          label={serverLabel}
          folder={remote.folder}
          isLoading={remote.isLoading}
          error={remote.error}
          onOpenFolder={(path) => void remote.open(path)}
          onGoUp={() => void remote.goUp()}
          toolbar={
            <>
              <IconButton
                {...iconButton}
                aria-label="Refresh the server"
                onClick={() => void remote.refresh()}
              >
                <RefreshIcon />
              </IconButton>
              <IconButton
                {...iconButton}
                aria-label="New folder on the server"
                onClick={() => setPrompt({ kind: 'newFolder' })}
              >
                <PlusIcon />
              </IconButton>
              <Button
                size="small"
                startIcon={<UploadIcon />}
                onClick={() => void remote.pickAndUpload()}
              >
                Upload
              </Button>
            </>
          }
          renderActions={(entry) => (
            <>
              {entry.kind === 'file' ? (
                <IconButton
                  {...iconButton}
                  aria-label={`Download ${entry.name} to ${LOCAL_LABEL}`}
                  onClick={() => void download(entry)}
                >
                  <DownloadIcon />
                </IconButton>
              ) : null}
              <IconButton
                {...iconButton}
                aria-label={`Rename ${entry.name}`}
                onClick={() => setPrompt({ kind: 'rename', entry })}
              >
                <EditIcon />
              </IconButton>
              <IconButton
                {...iconButton}
                aria-label={`Delete ${entry.name}`}
                onClick={() => setPrompt({ kind: 'delete', entry })}
              >
                <TrashIcon />
              </IconButton>
            </>
          )}
          onDropFile={(entry) => void remote.upload([entry.path])}
        />
      </Panes>

      {remote.transfers.length > 0 ? (
        <TransferBar gap="XS" as="section" aria-label="Transfers">
          {remote.transfers.map((transfer) => (
            <TransferRow key={transfer.id}>
              <Text.Strong>{transfer.name}</Text.Strong>
              <TransferStatus>{transferStatus(transfer)}</TransferStatus>
              {transfer.status === 'running' ? (
                <>
                  <Progress
                    label={`${transfer.direction === 'download' ? 'Downloading' : 'Uploading'} ${transfer.name}`}
                    value={transfer.doneBytes}
                    max={Math.max(transfer.totalBytes, 1)}
                  />
                  <TransferPercent>{percent(transfer)}</TransferPercent>
                </>
              ) : null}
            </TransferRow>
          ))}
          {hasFinishedTransfers ? (
            <HFlex align={FLEX_ALIGNS.CENTER} justify={FLEX_JUSTIFIES.END}>
              <Button
                size="small"
                variant="subtle"
                color="neutral"
                onClick={remote.clearFinishedTransfers}
              >
                Clear finished
              </Button>
            </HFlex>
          ) : null}
        </TransferBar>
      ) : null}

      {prompt?.kind === 'newFolder' ? (
        <NameDialog
          open
          title="New folder on the server"
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
