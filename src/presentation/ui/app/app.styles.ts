import { CenterFlex, SPACINGS, VFlex } from '@faber-ui/react';
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

/** Centers whatever the screen shows when there is no terminal. */
export const AppCenter = styled(CenterFlex)`
  flex: 1;
  min-block-size: 0;
  padding: ${SPACINGS.LG};
  overflow-y: auto;
`;
