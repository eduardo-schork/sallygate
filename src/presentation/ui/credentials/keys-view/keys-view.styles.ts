import {
  Card,
  CARD_PADDINGS,
  COLORS,
  FONT_SIZES,
  HFlex,
  RADII,
  SIZES,
  SPACINGS,
  VFlex,
} from '@faber-ui/react';
import styled from 'styled-components';

import { BRAND_FONT_FAMILIES } from '../../shared/brand-tokens';

export const KeysViewRoot = styled(VFlex)`
  flex: 1;
  min-block-size: 0;
  padding: ${SPACINGS.LG};
  overflow-y: auto;
`;

export const KeyList = styled(VFlex).attrs({ forwardedAs: 'ul' })`
  margin: 0;
  padding: 0;
  list-style: none;
`;

export const KeyCard = styled(Card).attrs({ padding: CARD_PADDINGS.SMALL })`
  display: flex;
  align-items: center;
  gap: ${SPACINGS.SM};
`;

/** The key mark: a square in the brand color, like the host cards. */
export const KeyIcon = styled(HFlex)`
  flex: none;
  align-items: center;
  justify-content: center;
  inline-size: ${SIZES.MD};
  block-size: ${SIZES.MD};
  border-radius: ${RADII.MD};
  background-color: ${COLORS.PRIMARY};
  color: ${COLORS.ON_PRIMARY};
`;

export const KeyText = styled(VFlex)`
  flex: 1;
  min-inline-size: 0;
`;

export const KeyFingerprint = styled.code`
  overflow: hidden;
  color: ${COLORS.TEXT_SECONDARY};
  font-family: ${BRAND_FONT_FAMILIES.MONO};
  font-size: ${FONT_SIZES.XS};
  text-overflow: ellipsis;
  white-space: nowrap;
`;
