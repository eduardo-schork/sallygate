import {
  BORDER_WIDTHS,
  CenterFlex,
  COLORS,
  FONT_WEIGHTS,
  HFlex,
  SPACINGS,
  VFlex,
} from '@faber-ui/react';
import styled from 'styled-components';

/** Hidden tabs stay mounted so their terminals keep running. */
export const SessionPanelRoot = styled(VFlex)`
  flex: 1;
  min-block-size: 0;
  min-inline-size: 0;
  background-color: ${COLORS.BACKGROUND_PRIMARY};

  &[hidden] {
    display: none;
  }

  /* In split view the session in front is outlined, like a focused window. */
  &[data-split='true'][data-active='true'] {
    outline: ${BORDER_WIDTHS.STRONG} solid ${COLORS.PRIMARY};
    outline-offset: calc(-1 * ${BORDER_WIDTHS.STRONG});
  }
`;

export const SessionTitle = styled.span`
  overflow: hidden;
  font-weight: ${FONT_WEIGHTS.SEMIBOLD};
  text-overflow: ellipsis;
  white-space: nowrap;
`;

export const SessionPanelMessage = styled(CenterFlex)`
  flex: 1;
  padding: ${SPACINGS.LG};
`;

/** Switches a connected tab between its terminal and its files. */
export const SessionViewBar = styled(HFlex)`
  flex: none;
  align-items: center;
  gap: ${SPACINGS.SM};

  & > :last-child {
    margin-inline-start: auto;
  }
  padding: ${SPACINGS.XS} ${SPACINGS.MD};
  border-block-end: ${BORDER_WIDTHS.DEFAULT} solid ${COLORS.BORDER_DEFAULT};
`;

/** The terminal or the files; the one not shown stays mounted so it keeps its state. */
export const SessionView = styled(HFlex)`
  flex: 1;
  min-block-size: 0;

  &[hidden] {
    display: none;
  }
`;
