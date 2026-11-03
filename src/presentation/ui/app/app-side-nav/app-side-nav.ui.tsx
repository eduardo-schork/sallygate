import { HFlex, NavLink, Text, VFlex } from '@faber-ui/react';
import { LockIcon } from '@faber-ui/icons';

import { APP_VIEWS } from './app-side-nav.constants';
import { AppSideNavRoot, VaultTeaser } from './app-side-nav.styles';
import type { TAppSideNavProps } from './app-side-nav.types';

/** The side menu of the Sallygate tab: one entry per screen, and what comes next. */
export function AppSideNav({ current, onNavigate }: TAppSideNavProps) {
  return (
    <AppSideNavRoot aria-label="Sallygate">
      {APP_VIEWS.map(({ view, label, icon }) => (
        <NavLink
          key={view}
          as="button"
          type="button"
          current={current === view}
          onClick={() => onNavigate(view)}
        >
          {icon}
          {label}
        </NavLink>
      ))}
      <VaultTeaser>
        <VFlex gap="XXS">
          <HFlex gap="XS" align="center">
            <LockIcon aria-hidden="true" />
            <Text.Strong>Vault</Text.Strong>
          </HFlex>
          <Text.Small>Coming later: your keys and passwords behind one master password.</Text.Small>
        </VFlex>
      </VaultTeaser>
    </AppSideNavRoot>
  );
}
