import { Textarea } from '@faber-ui/react';
import styled from 'styled-components';

import { BRAND_FONT_FAMILIES } from '../../shared/brand-tokens';

/** Commands are typed in mono, as they will appear in the terminal. */
export const CommandField = styled(Textarea)`
  font-family: ${BRAND_FONT_FAMILIES.MONO};
`;
