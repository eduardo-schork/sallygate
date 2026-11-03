import { Card, CARD_PADDINGS, FONT_SIZES, SPACINGS, VFlex } from '@faber-ui/react';
import styled from 'styled-components';

import { BRAND_FONT_FAMILIES } from '../../shared/brand-tokens';

export const SnippetsViewRoot = styled(VFlex)`
  flex: 1;
  min-block-size: 0;
  padding: ${SPACINGS.LG};
  overflow-y: auto;
`;

export const SnippetCard = styled(Card).attrs({ padding: CARD_PADDINGS.SMALL })`
  display: flex;
  align-items: center;
  gap: ${SPACINGS.SM};
`;

export const SnippetText = styled(VFlex)`
  flex: 1;
  min-inline-size: 0;
`;

/** The command in mono, cut after two lines: the dialog shows it whole. */
export const SnippetCommandPreview = styled.code`
  display: -webkit-box;
  overflow: hidden;
  -webkit-box-orient: vertical;
  -webkit-line-clamp: 2;
  font-family: ${BRAND_FONT_FAMILIES.MONO};
  font-size: ${FONT_SIZES.XS};
  white-space: pre-wrap;
  word-break: break-all;
`;

export const SnippetList = styled(VFlex).attrs({ forwardedAs: 'ul' })`
  margin: 0;
  padding: 0;
  list-style: none;
`;
