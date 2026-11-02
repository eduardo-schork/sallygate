import { VFlex } from '@faber-ui/react';
import styled, { createGlobalStyle } from 'styled-components';

// A desktop window has no page scroll: the app always fills it.
export const AppGlobalStyles = createGlobalStyle`
  html,
  body,
  #root {
    block-size: 100%;
  }
`;

// `forwardedAs` keeps VFlex and only swaps the element it renders; `as` would replace VFlex.
export const AppRoot = styled(VFlex).attrs({ forwardedAs: 'main' })`
  block-size: 100%;
`;

/** The hosts screen; hidden, not unmounted, while a session tab is in front. */
export const HostsPanel = styled(VFlex)`
  flex: 1;
  min-block-size: 0;

  &[hidden] {
    display: none;
  }
`;
