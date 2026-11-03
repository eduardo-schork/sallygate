import { BORDER_WIDTHS, COLORS, FONT_SIZES, RADII, SIZES, SPACINGS, VFlex } from '@faber-ui/react';
import styled from 'styled-components';

import { BRAND_FONT_FAMILIES } from '../../shared/brand-tokens';

export const SnippetPanelRoot = styled(VFlex).attrs({ forwardedAs: 'aside' })`
  flex: none;
  inline-size: calc(${SIZES.XXL} * 3.5);
  padding: ${SPACINGS.SM};
  overflow-y: auto;
  border-inline-start: ${BORDER_WIDTHS.DEFAULT} solid ${COLORS.BORDER_DEFAULT};
  background-color: ${COLORS.SURFACE_PRIMARY};
`;

/** A whole snippet is one button: its name, then its command in mono. */
export const SnippetRunButton = styled.button`
  display: flex;
  flex-direction: column;
  gap: ${SPACINGS.XXS};
  inline-size: 100%;
  padding: ${SPACINGS.XS} ${SPACINGS.SM};
  border: ${BORDER_WIDTHS.DEFAULT} solid ${COLORS.BORDER_DEFAULT};
  border-radius: ${RADII.MD};
  background: none;
  color: ${COLORS.TEXT_PRIMARY};
  text-align: start;
  cursor: pointer;

  &:hover {
    background-color: ${COLORS.DISABLED_BACKGROUND};
  }

  &:focus-visible {
    outline: ${BORDER_WIDTHS.STRONG} solid ${COLORS.FOCUS_RING};
    outline-offset: ${BORDER_WIDTHS.STRONG};
  }
`;

export const SnippetRunCommand = styled.code`
  overflow: hidden;
  color: ${COLORS.TEXT_SECONDARY};
  font-family: ${BRAND_FONT_FAMILIES.MONO};
  font-size: ${FONT_SIZES.XS};
  text-overflow: ellipsis;
  white-space: nowrap;
`;

export const SnippetRunList = styled(VFlex).attrs({ forwardedAs: 'ul' })`
  margin: 0;
  padding: 0;
  list-style: none;
`;
