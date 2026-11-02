import { CenterFlex, SPACINGS, VFlex } from '@faber-ui/react';
import styled from 'styled-components';

/** Hidden tabs stay mounted so their terminals keep running. */
export const SessionPanelRoot = styled(VFlex)`
  flex: 1;
  min-block-size: 0;

  &[hidden] {
    display: none;
  }
`;

export const SessionPanelMessage = styled(CenterFlex)`
  flex: 1;
  padding: ${SPACINGS.LG};
`;
