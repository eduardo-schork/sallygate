import {
  BORDER_WIDTHS,
  COLORS,
  FONT_SIZES,
  FONT_WEIGHTS,
  HFlex,
  RADII,
  SIZES,
  SPACINGS,
  VFlex,
} from '@faber-ui/react';
import styled from 'styled-components';

import { BRAND_FONT_FAMILIES } from '../../shared/brand-tokens';

/** Name, size, permissions, actions: the same grid for the header and every row. */
const COLUMNS = `minmax(0, 1fr) ${SIZES.XXL} calc(${SIZES.XXL} * 1.4) calc(${SIZES.XL} * 1.5)`;

export const FilePaneRoot = styled(VFlex).attrs({ forwardedAs: 'section' })`
  min-inline-size: 0;
  min-block-size: 0;
  background-color: ${COLORS.BACKGROUND_PRIMARY};

  &[data-drop-target='true'] {
    outline: ${BORDER_WIDTHS.STRONG} solid ${COLORS.FOCUS_RING};
    outline-offset: calc(-1 * ${BORDER_WIDTHS.STRONG});
  }
`;

export const PaneHeader = styled(HFlex)`
  flex: none;
  align-items: center;
  gap: ${SPACINGS.XS};
  padding: ${SPACINGS.SM} ${SPACINGS.MD};
  border-block-end: ${BORDER_WIDTHS.DEFAULT} solid ${COLORS.BORDER_DEFAULT};
`;

/** The machine mark: neutral for this Mac, the brand color for the server. */
export const PaneTile = styled(HFlex)`
  flex: none;
  align-items: center;
  justify-content: center;
  inline-size: ${SIZES.SM};
  block-size: ${SIZES.SM};
  border-radius: ${RADII.MD};
  background-color: ${COLORS.BORDER_STRONG};
  color: ${COLORS.ON_PRIMARY};

  &[data-side='remote'] {
    background-color: ${COLORS.PRIMARY};
  }
`;

export const PaneLabel = styled.span`
  overflow: hidden;
  font-weight: ${FONT_WEIGHTS.SEMIBOLD};
  text-overflow: ellipsis;
  white-space: nowrap;
`;

export const PaneTools = styled(HFlex)`
  flex: none;
  align-items: center;
  gap: ${SPACINGS.XXS};
  margin-inline-start: auto;
`;

export const PathBar = styled(HFlex)`
  flex: none;
  align-items: center;
  gap: ${SPACINGS.XXS};
  padding: ${SPACINGS.XS} ${SPACINGS.MD};
  border-block-end: ${BORDER_WIDTHS.DEFAULT} solid ${COLORS.BORDER_DEFAULT};
`;

/** The folder in mono: something you could paste into a terminal. */
export const PathText = styled.code`
  flex: 1;
  min-inline-size: 0;
  overflow: hidden;
  padding: ${SPACINGS.XXS} ${SPACINGS.XS};
  border-radius: ${RADII.MD};
  background-color: ${COLORS.SURFACE_PRIMARY};
  font-family: ${BRAND_FONT_FAMILIES.MONO};
  font-size: ${FONT_SIZES.SM};
  text-overflow: ellipsis;
  white-space: nowrap;
`;

export const ColumnHeads = styled.div`
  display: grid;
  flex: none;
  grid-template-columns: ${COLUMNS};
  padding: ${SPACINGS.XS} ${SPACINGS.MD};
  color: ${COLORS.TEXT_SECONDARY};
  font-size: ${FONT_SIZES.XS};
  font-weight: ${FONT_WEIGHTS.SEMIBOLD};
`;

export const EntryList = styled.ul`
  flex: 1;
  min-block-size: 0;
  margin: 0;
  padding: 0;
  overflow-y: auto;
  list-style: none;
`;

export const EntryRow = styled.li`
  display: grid;
  grid-template-columns: ${COLUMNS};
  align-items: center;
  min-block-size: ${SIZES.MD};
  padding: 0 ${SPACINGS.MD};

  &:hover {
    background-color: ${COLORS.SURFACE_PRIMARY};
  }

  &[draggable='true'] {
    cursor: grab;
  }
`;

export const EntryName = styled(HFlex)`
  align-items: center;
  gap: ${SPACINGS.XS};
  min-inline-size: 0;

  & > span {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
`;

/** A folder's name opens it; it looks like the other names, not like a button. */
export const FolderButton = styled.button`
  display: flex;
  align-items: center;
  gap: ${SPACINGS.XS};
  min-inline-size: 0;
  padding: 0;
  border: none;
  background: none;
  color: ${COLORS.TEXT_PRIMARY};
  font: inherit;
  font-weight: ${FONT_WEIGHTS.SEMIBOLD};
  text-align: start;
  cursor: pointer;

  & > span {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  &:hover > span {
    text-decoration: underline;
  }

  &:focus-visible {
    outline: ${BORDER_WIDTHS.STRONG} solid ${COLORS.FOCUS_RING};
    outline-offset: ${BORDER_WIDTHS.STRONG};
  }
`;

export const MetaText = styled.span`
  color: ${COLORS.TEXT_SECONDARY};
  font-family: ${BRAND_FONT_FAMILIES.MONO};
  font-size: ${FONT_SIZES.XS};
  /* JetBrains Mono joins "--" into a dash, which hides the letters of a permission. */
  font-variant-ligatures: none;
`;

export const RowActions = styled(HFlex)`
  justify-content: flex-end;
  gap: ${SPACINGS.NONE};
`;

export const PaneMessage = styled.div`
  padding: ${SPACINGS.MD};
`;
