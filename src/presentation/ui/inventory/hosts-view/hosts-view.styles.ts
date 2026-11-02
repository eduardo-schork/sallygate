import { HFlex, Input, SIZES, SPACINGS, VFlex } from '@faber-ui/react';
import styled from 'styled-components';

export const HostsViewRoot = styled(VFlex)`
  flex: 1;
  min-block-size: 0;
  padding: ${SPACINGS.LG};
  overflow-y: auto;
`;

/** Search and the add button; the button never shrinks below its label. */
export const HostsToolbar = styled(HFlex)`
  align-items: center;
  gap: ${SPACINGS.SM};

  & > button {
    flex: none;
  }
`;

export const HostsSearch = styled(Input)`
  inline-size: calc(${SIZES.XXL} * 3);
`;

/** Cards flow into as many columns as fit, each at least a readable width. */
export const HostGrid = styled.div`
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(min(100%, calc(${SIZES.XXL} * 4)), 1fr));
  gap: ${SPACINGS.SM};
`;
