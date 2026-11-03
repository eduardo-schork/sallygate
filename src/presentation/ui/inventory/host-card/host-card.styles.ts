import { Card, CARD_PADDINGS, COLORS, HFlex, RADII, SIZES, SPACINGS, Text } from '@faber-ui/react';
import styled from 'styled-components';

import { BRAND_FONT_FAMILIES } from '../../shared/brand-tokens';

export const HostCardRoot = styled(Card).attrs({ padding: CARD_PADDINGS.SMALL })`
  display: flex;
  align-items: center;
  gap: ${SPACINGS.SM};
`;

/** The server mark: a square in the brand color, like the prototype. */
export const HostCardIcon = styled(HFlex)`
  flex: none;
  align-items: center;
  justify-content: center;
  inline-size: ${SIZES.MD};
  block-size: ${SIZES.MD};
  border-radius: ${RADII.MD};
  background-color: ${COLORS.PRIMARY};
  color: ${COLORS.ON_PRIMARY};
`;

export const HostCardText = styled.div`
  flex: 1;
  min-inline-size: 0;

  & > * {
    display: block;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
`;

/** Where the host points, in mono: something you would type in a terminal. */
export const HostCardEndpoint = styled(Text.Small)`
  font-family: ${BRAND_FONT_FAMILIES.MONO};
`;
