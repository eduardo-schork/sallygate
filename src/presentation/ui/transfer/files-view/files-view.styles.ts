import { BORDER_WIDTHS, COLORS, FONT_SIZES, HFlex, SIZES, SPACINGS, VFlex } from '@faber-ui/react';
import styled from 'styled-components';

export const FilesViewRoot = styled(VFlex)`
  flex: 1;
  min-block-size: 0;
`;

/** Two panes side by side; the 1px gap shows the border color as their divider. */
export const Panes = styled.div`
  display: grid;
  flex: 1;
  min-block-size: 0;
  grid-template-columns: repeat(auto-fit, minmax(calc(${SIZES.XXL} * 5), 1fr));
  gap: ${BORDER_WIDTHS.DEFAULT};
  background-color: ${COLORS.BORDER_DEFAULT};
`;

export const TransferBar = styled(VFlex)`
  flex: none;
  max-block-size: 30%;
  padding: ${SPACINGS.SM} ${SPACINGS.MD};
  overflow-y: auto;
  border-block-start: ${BORDER_WIDTHS.DEFAULT} solid ${COLORS.BORDER_DEFAULT};
  background-color: ${COLORS.SURFACE_PRIMARY};
`;

export const TransferRow = styled(HFlex)`
  flex-wrap: wrap;
  align-items: center;
  gap: ${SPACINGS.SM};

  & > progress {
    flex: 1;
    min-inline-size: calc(${SIZES.XXL} * 1.5);
  }
`;

export const TransferStatus = styled.span`
  color: ${COLORS.TEXT_SECONDARY};
  font-size: ${FONT_SIZES.SM};
`;

export const TransferPercent = styled.span`
  font-variant-numeric: tabular-nums;
`;
