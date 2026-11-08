import { Alert, IconButton, Spinner, Text } from '@faber-ui/react';
import { ArrowUpIcon, FileIcon, FolderIcon, LinkIcon, MonitorIcon } from '@faber-ui/icons';
import { type DragEvent, useState } from 'react';

import { formatPermissions } from '../format-permissions';
import { formatSize } from '../format-size';
import type { TFileEntry } from '../transfer-contract';
import { ENTRY_DRAG_TYPE } from './file-pane.constants';
import {
  ColumnHeads,
  EntryList,
  EntryName,
  EntryRow,
  FilePaneRoot,
  FolderButton,
  MetaText,
  PaneHeader,
  PaneLabel,
  PaneMessage,
  PaneTile,
  PaneTools,
  PathBar,
  PathText,
  RowActions,
} from './file-pane.styles';
import type { TFilePaneProps } from './file-pane.types';

const ENTRY_ICONS = {
  file: <FileIcon aria-hidden="true" />,
  folder: <FolderIcon aria-hidden="true" />,
  link: <LinkIcon aria-hidden="true" />,
} as const;

/** Files and folders copy across; links are not followed, so they stay put. */
const isCopyable = (entry: TFileEntry) => entry.kind !== 'link';

type TDraggedEntry = { readonly side: TFilePaneProps['side']; readonly entry: TFileEntry };

function readDraggedEntry(event: DragEvent): TDraggedEntry | undefined {
  const data = event.dataTransfer.getData(ENTRY_DRAG_TYPE);
  return data === '' ? undefined : (JSON.parse(data) as TDraggedEntry);
}

/**
 * One machine's files: a folder to browse, files and folders to drag to the other pane, and a
 * drop zone for what is dragged from it.
 */
export function FilePane({
  side,
  label,
  folder,
  isLoading,
  error,
  onOpenFolder,
  onGoUp,
  toolbar,
  renderActions,
  onDropEntry,
}: TFilePaneProps) {
  const [isDropTarget, setIsDropTarget] = useState(false);
  const acceptsDrag = (event: DragEvent) => event.dataTransfer.types.includes(ENTRY_DRAG_TYPE);

  const handleDrop = (event: DragEvent) => {
    setIsDropTarget(false);
    const dragged = readDraggedEntry(event);
    if (dragged && dragged.side !== side && isCopyable(dragged.entry)) {
      event.preventDefault();
      onDropEntry(dragged.entry);
    }
  };

  const startDrag = (event: DragEvent, entry: TFileEntry) => {
    event.dataTransfer.setData(ENTRY_DRAG_TYPE, JSON.stringify({ side, entry }));
    event.dataTransfer.effectAllowed = 'copy';
  };

  return (
    <FilePaneRoot
      aria-label={label}
      data-drop-target={isDropTarget}
      onDragOver={(event: DragEvent) => {
        if (acceptsDrag(event)) {
          event.preventDefault();
          setIsDropTarget(true);
        }
      }}
      onDragLeave={() => setIsDropTarget(false)}
      onDrop={handleDrop}
    >
      <PaneHeader>
        <PaneTile data-side={side} aria-hidden="true">
          <MonitorIcon />
        </PaneTile>
        <PaneLabel>{label}</PaneLabel>
        <PaneTools>{toolbar}</PaneTools>
      </PaneHeader>
      <PathBar>
        <IconButton
          size="small"
          variant="subtle"
          color="neutral"
          aria-label={`Go to the parent folder on ${label}`}
          disabled={!folder || folder.path === '/'}
          onClick={onGoUp}
        >
          <ArrowUpIcon />
        </IconButton>
        <PathText title={folder?.path}>{folder?.path ?? '…'}</PathText>
      </PathBar>
      {error ? (
        <PaneMessage>
          <Alert color="error" role="alert">
            {error.message}
          </Alert>
        </PaneMessage>
      ) : null}
      <ColumnHeads aria-hidden="true">
        <span>Name</span>
        <span>Size</span>
        <span>Permissions</span>
      </ColumnHeads>
      {isLoading && !folder ? (
        <PaneMessage>
          <Spinner label={`Loading files on ${label}`} />
        </PaneMessage>
      ) : null}
      {folder?.entries.length === 0 ? (
        <PaneMessage>
          <Text.Small>This folder is empty.</Text.Small>
        </PaneMessage>
      ) : null}
      <EntryList aria-label={`Files in ${folder?.path ?? label}`}>
        {folder?.entries.map((entry) => (
          <EntryRow
            key={entry.path}
            draggable={isCopyable(entry)}
            onDragStart={(event: DragEvent) => startDrag(event, entry)}
          >
            {entry.kind === 'folder' ? (
              <FolderButton type="button" onClick={() => onOpenFolder(entry.path)}>
                {ENTRY_ICONS.folder}
                <span>{entry.name}</span>
              </FolderButton>
            ) : (
              <EntryName>
                {ENTRY_ICONS[entry.kind]}
                <span>{entry.name}</span>
              </EntryName>
            )}
            <MetaText>{entry.kind === 'file' ? formatSize(entry.sizeBytes) : ''}</MetaText>
            <MetaText>{formatPermissions(entry.kind, entry.permissions)}</MetaText>
            <RowActions>{renderActions(entry)}</RowActions>
          </EntryRow>
        ))}
      </EntryList>
    </FilePaneRoot>
  );
}
