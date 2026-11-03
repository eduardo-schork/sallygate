import { Card, CARD_PADDINGS, FONT_SIZES, SPACINGS, VFlex } from '@faber-ui/react';
import styled from 'styled-components';

import { BRAND_FONT_FAMILIES } from '../../shared/brand-tokens';

export const ForwardingViewRoot = styled(VFlex)`
  flex: 1;
  min-block-size: 0;
  padding: ${SPACINGS.LG};
  overflow-y: auto;
`;

export const RuleList = styled(VFlex).attrs({ forwardedAs: 'ul' })`
  margin: 0;
  padding: 0;
  list-style: none;
`;

export const RuleCard = styled(Card).attrs({ padding: CARD_PADDINGS.SMALL })`
  display: flex;
  align-items: center;
  gap: ${SPACINGS.SM};
`;

export const RuleText = styled(VFlex)`
  flex: 1;
  min-inline-size: 0;
`;

/** Ports and addresses in mono: they are what you type into a client. */
export const RuleRoute = styled.span`
  font-family: ${BRAND_FONT_FAMILIES.MONO};
  font-size: ${FONT_SIZES.XS};
`;
