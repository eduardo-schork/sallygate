import { BORDER_WIDTHS, CenterFlex, COLORS, HFlex, SPACINGS, VFlex } from '@faber-ui/react';
import styled from 'styled-components';

/** Hidden tabs stay mounted so their terminals keep running. */
export const SessionPanelRoot = styled(VFlex)`
  flex: 1;
  min-block-size: 0;

  &[hidden] {
    display: none;
  }
`;

export const SessionPanelMessage = styled(CenterFlex)`
  flex: 1;
  padding: ${SPACINGS.LG};
`;

/** Switches a connected tab between its terminal and its files. */
export const SessionViewBar = styled(HFlex)`
  flex: none;
  padding: ${SPACINGS.XS} ${SPACINGS.MD};
  border-block-end: ${BORDER_WIDTHS.DEFAULT} solid ${COLORS.BORDER_DEFAULT};
`;

/** The terminal or the files; the one not shown stays mounted so it keeps its state. */
export const SessionView = styled(VFlex)`
  flex: 1;
  min-block-size: 0;

  &[hidden] {
    display: none;
  }
`;
