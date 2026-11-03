import '@faber-ui/react/styles.css';
import './sallygate-theme.css';

import { FONT_SIZE_SCALE, GlobalStyles } from '@faber-ui/react';
import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';

import { App } from './app.ui';
import { AppGlobalStyles } from './app.styles';

const rootElement = document.getElementById('root');

if (!rootElement) {
  throw new Error('The #root element is missing from index.html.');
}

// The terminal measures its cells once when it opens, so load its font before any session starts.
void document.fonts.load(
  `${FONT_SIZE_SCALE.SM} ${getComputedStyle(rootElement).getPropertyValue('--sg-font-mono')}`,
);

createRoot(rootElement).render(
  <StrictMode>
    <GlobalStyles />
    <AppGlobalStyles />
    <App />
  </StrictMode>,
);
