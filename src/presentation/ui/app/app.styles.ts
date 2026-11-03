import { BORDER_WIDTHS, COLORS, HFlex, VFlex } from '@faber-ui/react';
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

/** Below the tab bar: the side menu, always there, and the tab's content. */
export const AppBody = styled(HFlex)`
  flex: 1;
  min-block-size: 0;
`;

export const AppContent = styled(VFlex)`
  flex: 1;
  min-inline-size: 0;
`;

/** A screen of the Sallygate tab; hidden, not unmounted, while a session is in front. */
export const HomeView = styled(VFlex)`
  flex: 1;
  min-block-size: 0;

  &[hidden] {
    display: none;
  }
`;

/** The sessions: one at a time, or a grid of all of them in split view. */
export const SessionArea = styled(VFlex)`
  flex: 1;
  min-block-size: 0;

  &[hidden] {
    display: none;
  }

  &[data-columns] {
    display: grid;
    grid-template-columns: repeat(var(--session-columns), minmax(0, 1fr));
    grid-auto-rows: minmax(0, 1fr);
    gap: ${BORDER_WIDTHS.DEFAULT};
    background-color: ${COLORS.BORDER_DEFAULT};
  }
`;
