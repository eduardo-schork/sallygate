import { COLORS, SPACINGS } from '@faber-ui/react';
import styled from 'styled-components';

/** The terminal takes all the room it is given; xterm.js fits the grid inside it. */
export const TerminalViewRoot = styled.div`
  flex: 1;
  min-block-size: 0;
  padding: ${SPACINGS.SM};
  background-color: ${COLORS.BACKGROUND_PRIMARY};

  .xterm {
    block-size: 100%;
  }
`;
