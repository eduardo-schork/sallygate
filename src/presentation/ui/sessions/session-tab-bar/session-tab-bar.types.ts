import type { TSessionTab } from '../use-session-tabs';

export type TSessionTabBarProps = {
  readonly tabs: TSessionTab[];
  readonly activeTabId: string;
  readonly onSelect: (tabId: string) => void;
  readonly onClose: (tabId: string) => void;
};
