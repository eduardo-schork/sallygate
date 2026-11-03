import { BORDER_WIDTHS, COLORS, FONT_SIZES, HFlex, SPACINGS, VFlex } from '@faber-ui/react';
import styled from 'styled-components';

import { BRAND_FONT_FAMILIES } from '../../shared/brand-tokens';

export const FilesViewRoot = styled(VFlex)`
  flex: 1;
  min-block-size: 0;
  padding: ${SPACINGS.MD};
`;

export const FilesToolbar = styled(HFlex)`
  align-items: center;
  gap: ${SPACINGS.XS};

  & > button {
    flex: none;
  }
`;

/** The folder on screen, in mono: it is something you would paste into a terminal. */
export const FolderPath = styled.span`
  flex: 1;
  min-inline-size: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  font-family: ${BRAND_FONT_FAMILIES.MONO};
  font-size: ${FONT_SIZES.SM};
`;

/** The list scrolls on its own so the toolbar and transfers stay in view. */
export const FilesTableScroll = styled.div`
  flex: 1;
  min-block-size: 0;
  overflow-y: auto;

  & td,
  & th {
    white-space: nowrap;
  }

  & td:first-child {
    inline-size: 100%;
  }
`;

export const MetaCell = styled.td`
  color: ${COLORS.TEXT_SECONDARY};
  font-family: ${BRAND_FONT_FAMILIES.MONO};
  font-size: ${FONT_SIZES.XS};
`;

export const EntryName = styled(HFlex)`
  align-items: center;
  gap: ${SPACINGS.XS};
  min-inline-size: 0;
`;

export const TransfersList = styled(VFlex)`
  flex: none;
  max-block-size: 30%;
  overflow-y: auto;
  padding-block-start: ${SPACINGS.SM};
  border-block-start: ${BORDER_WIDTHS.DEFAULT} solid ${COLORS.BORDER_DEFAULT};
`;
