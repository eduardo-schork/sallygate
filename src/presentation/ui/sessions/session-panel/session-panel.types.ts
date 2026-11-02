import type { TOpenSessionRequest } from '../sessions-contract';

export type TSessionPanelProps = {
  readonly request: TOpenSessionRequest;
  /** Only the tab in front shows dialogs; the others keep running behind it. */
  readonly isActive: boolean;
  readonly onCloseTab: () => void;
};
