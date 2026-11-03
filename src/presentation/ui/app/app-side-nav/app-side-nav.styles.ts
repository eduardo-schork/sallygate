import {
  BORDER_WIDTHS,
  CARD_PADDINGS,
  Card,
  COLORS,
  SIZES,
  SideNav,
  SPACINGS,
} from '@faber-ui/react';
import styled from 'styled-components';

export const AppSideNavRoot = styled(SideNav)`
  display: flex;
  flex: none;
  flex-direction: column;
  gap: ${SPACINGS.XXS};
  inline-size: calc(${SIZES.XXL} * 2.75);
  padding: ${SPACINGS.SM};
  border-inline-end: ${BORDER_WIDTHS.DEFAULT} solid ${COLORS.BORDER_DEFAULT};
  background-color: ${COLORS.SURFACE_PRIMARY};

  & > button {
    display: flex;
    align-items: center;
    gap: ${SPACINGS.XS};
    inline-size: 100%;
    border: none;
    background: none;
    text-align: start;
    cursor: pointer;
  }
`;

/** What is coming next, pinned to the bottom of the menu. */
export const VaultTeaser = styled(Card).attrs({ padding: CARD_PADDINGS.SMALL })`
  margin-block-start: auto;
`;
