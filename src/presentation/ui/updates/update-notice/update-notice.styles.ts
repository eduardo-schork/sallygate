import { BORDER_WIDTHS, COLORS, FONT_SIZES, HFlex, SIZES, SPACINGS } from '@faber-ui/react';
import styled from 'styled-components';

/** A thin bar under the tabs: noticeable, never in the way of a terminal. */
export const UpdateNoticeRoot = styled(HFlex)`
  flex: none;
  align-items: center;
  gap: ${SPACINGS.SM};
  padding: ${SPACINGS.XS} ${SPACINGS.MD};
  border-block-end: ${BORDER_WIDTHS.DEFAULT} solid ${COLORS.BORDER_DEFAULT};
  background-color: ${COLORS.SURFACE_PRIMARY};
  font-size: ${FONT_SIZES.SM};

  & > progress {
    inline-size: calc(${SIZES.XXL} * 2);
  }
`;

export const UpdateNoticeActions = styled(HFlex)`
  align-items: center;
  gap: ${SPACINGS.XS};
  margin-inline-start: auto;
`;
