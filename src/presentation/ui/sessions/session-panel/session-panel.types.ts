import type { TSnippet } from '../../snippets/snippets-contract';
import type { TOpenSessionRequest } from '../sessions-contract';

export type TSessionPanelProps = {
  readonly request: TOpenSessionRequest;
  /** Only the tab in front shows dialogs; the others keep running behind it. */
  readonly isActive: boolean;
  readonly onCloseTab: () => void;
  /** Shown beside the terminal; a click types one in. */
  readonly snippets: TSnippet[];
};
