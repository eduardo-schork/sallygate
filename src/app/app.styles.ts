import { CenterFlex } from '@faber-ui/react';
import styled, { createGlobalStyle } from 'styled-components';

// A desktop window has no page scroll: the app always fills it.
export const AppGlobalStyles = createGlobalStyle`
  html,
  body,
  #root {
    block-size: 100%;
  }
`;

// `forwardedAs` keeps CenterFlex and only swaps the element it renders; `as` would replace CenterFlex.
export const AppRoot = styled(CenterFlex).attrs({ forwardedAs: 'main' })`
  block-size: 100%;
`;
