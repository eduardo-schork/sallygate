import { CheckCircleIcon, CodeIcon, MonitorIcon } from '@faber-ui/icons';
import type { ReactNode } from 'react';

import type { TAppView } from './app-side-nav.types';

/** The side menu, top to bottom. */
export const APP_VIEWS: readonly { view: TAppView; label: string; icon: ReactNode }[] = [
  { view: 'hosts', label: 'Hosts', icon: <MonitorIcon /> },
  { view: 'snippets', label: 'Snippets', icon: <CodeIcon /> },
  { view: 'knownHosts', label: 'Known hosts', icon: <CheckCircleIcon /> },
];
