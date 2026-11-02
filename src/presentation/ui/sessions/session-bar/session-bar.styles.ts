import {
  BORDER_WIDTHS,
  COLORS,
  FLEX_ALIGNS,
  FLEX_JUSTIFIES,
  HFlex,
  SPACINGS,
} from '@faber-ui/react';
import styled from 'styled-components';

export const SessionBarRoot = styled(HFlex).attrs({
  align: FLEX_ALIGNS.CENTER,
  justify: FLEX_JUSTIFIES.SPACE_BETWEEN,
})`
  padding: ${SPACINGS.XS} ${SPACINGS.MD};
  border-block-end: ${BORDER_WIDTHS.DEFAULT} solid ${COLORS.BORDER_DEFAULT};
  background-color: ${COLORS.SURFACE_PRIMARY};
`;
