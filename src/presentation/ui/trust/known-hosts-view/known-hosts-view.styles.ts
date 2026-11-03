import { COLORS, FONT_SIZES, SPACINGS, VFlex } from '@faber-ui/react';
import styled from 'styled-components';

import { BRAND_FONT_FAMILIES } from '../../shared/brand-tokens';

export const KnownHostsViewRoot = styled(VFlex)`
  flex: 1;
  min-block-size: 0;
  padding: ${SPACINGS.LG};
  overflow-y: auto;

  & td:last-child {
    text-align: end;
  }
`;

/** Servers and fingerprints are compared character by character, so they are mono. */
export const MonoCell = styled.td`
  font-family: ${BRAND_FONT_FAMILIES.MONO};
  font-size: ${FONT_SIZES.XS};
  word-break: break-all;
`;

export const MutedCell = styled.td`
  color: ${COLORS.TEXT_SECONDARY};
  font-size: ${FONT_SIZES.SM};
`;
