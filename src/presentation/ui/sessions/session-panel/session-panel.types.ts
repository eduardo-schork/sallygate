import type { TSnippet } from '../../snippets/snippets-contract';
import type { TOpenSessionRequest } from '../sessions-contract';

export type TSessionPanelProps = {
  readonly request: TOpenSessionRequest;
  /** The session in front: only it shows dialogs and takes dropped files. */
  readonly isActive: boolean;
  /** On screen: the active one, or every session in split view. */
  readonly isVisible: boolean;
  /** In split view, each panel names its host and tells which one is in front. */
  readonly isSplit: boolean;
  readonly title: string;
  /** Brings this session to the front, e.g. on a click inside it in split view. */
  readonly onActivate: () => void;
  readonly onCloseTab: () => void;
  /** Shown beside the terminal; a click types one in. */
  readonly snippets: TSnippet[];
};
