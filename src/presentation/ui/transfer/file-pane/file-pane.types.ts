import type { ReactNode } from 'react';

import type { TCommandError } from '../../shared/command-error-contract';
import type { TFileEntry, TFolderListing } from '../transfer-contract';

/** Which machine a pane shows. */
export type TPaneSide = 'local' | 'remote';

export type TFilePaneProps = {
  readonly side: TPaneSide;
  /** "This Mac", or the host's name. */
  readonly label: string;
  readonly folder?: TFolderListing | undefined;
  readonly isLoading: boolean;
  readonly error?: TCommandError | undefined;
  readonly onOpenFolder: (path: string) => void;
  readonly onGoUp: () => void;
  /** Buttons for this pane's header, after its name. */
  readonly toolbar?: ReactNode;
  /** The buttons at the end of an entry's row. */
  readonly renderActions: (entry: TFileEntry) => ReactNode;
  /** A file dragged from the other pane was dropped here. */
  readonly onDropFile: (entry: TFileEntry) => void;
};
