import { Button, IconButton } from '@faber-ui/react';
import { CloseIcon, GridIcon } from '@faber-ui/icons';

import { HOSTS_TAB_ID } from '../use-session-tabs';
import {
  SessionTab,
  SessionTabBarRoot,
  SessionTabButton,
  SessionTabBarTools,
  SessionTabList,
} from './session-tab-bar.styles';
import type { TSessionTabBarProps } from './session-tab-bar.types';

/** The Sallygate tab (hosts, snippets…) and one tab per open session, like browser tabs. */
export function SessionTabBar({
  tabs,
  activeTabId,
  onSelect,
  onClose,
  isSplit,
  onToggleSplit,
}: TSessionTabBarProps) {
  return (
    <SessionTabBarRoot>
      <SessionTabList role="tablist" aria-label="Open tabs">
        <SessionTab data-active={activeTabId === HOSTS_TAB_ID}>
          <SessionTabButton
            role="tab"
            aria-selected={activeTabId === HOSTS_TAB_ID}
            onClick={() => onSelect(HOSTS_TAB_ID)}
          >
            Sallygate
          </SessionTabButton>
        </SessionTab>
        {tabs.map((tab) => (
          <SessionTab key={tab.id} data-active={activeTabId === tab.id}>
            <SessionTabButton
              role="tab"
              aria-selected={activeTabId === tab.id}
              onClick={() => onSelect(tab.id)}
            >
              {tab.title}
            </SessionTabButton>
            <IconButton
              size="small"
              variant="subtle"
              color="neutral"
              aria-label={`Close ${tab.title}`}
              onClick={() => onClose(tab.id)}
            >
              <CloseIcon />
            </IconButton>
          </SessionTab>
        ))}
      </SessionTabList>
      <SessionTabBarTools>
        <Button
          size="small"
          variant={isSplit ? 'light' : 'subtle'}
          color="neutral"
          startIcon={<GridIcon />}
          aria-pressed={isSplit}
          disabled={tabs.length < 2}
          title={tabs.length < 2 ? 'Open two sessions to see them side by side' : undefined}
          onClick={onToggleSplit}
        >
          Split view
        </Button>
      </SessionTabBarTools>
    </SessionTabBarRoot>
  );
}
