import {
  BORDER_WIDTHS,
  COLORS,
  FONT_SIZES,
  FONT_WEIGHTS,
  HFlex,
  RADII,
  SIZES,
  SPACINGS,
} from '@faber-ui/react';
import styled from 'styled-components';

export const SessionTabBarRoot = styled(HFlex)`
  flex: none;
  gap: ${SPACINGS.XXS};
  padding: ${SPACINGS.XS} ${SPACINGS.SM} ${SPACINGS.NONE};
  overflow-x: auto;
  border-block-end: ${BORDER_WIDTHS.DEFAULT} solid ${COLORS.BORDER_DEFAULT};
  background-color: ${COLORS.SURFACE_PRIMARY};
`;

/** One tab: its select button and, for sessions, a close button, sharing one surface. */
export const SessionTab = styled(HFlex)`
  align-items: center;
  border-start-start-radius: ${RADII.MD};
  border-start-end-radius: ${RADII.MD};
  color: ${COLORS.TEXT_SECONDARY};

  &[data-active='true'] {
    background-color: ${COLORS.BACKGROUND_PRIMARY};
    color: ${COLORS.TEXT_PRIMARY};
  }
`;

export const SessionTabButton = styled.button`
  block-size: ${SIZES.MD};
  padding: ${SPACINGS.NONE} ${SPACINGS.SM};
  border: ${BORDER_WIDTHS.NONE};
  background: none;
  color: inherit;
  font-size: ${FONT_SIZES.SM};
  font-weight: ${FONT_WEIGHTS.MEDIUM};
  white-space: nowrap;
  cursor: pointer;

  &:focus-visible {
    outline: ${BORDER_WIDTHS.STRONG} solid ${COLORS.FOCUS_RING};
    border-radius: ${RADII.SM};
  }
`;
