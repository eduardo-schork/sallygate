import '@faber-ui/react/styles.css';

import { GlobalStyles } from '@faber-ui/react';
import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';

import { App } from './app.ui';
import { AppGlobalStyles } from './app.styles';

const rootElement = document.getElementById('root');

if (!rootElement) {
  throw new Error('The #root element is missing from index.html.');
}

createRoot(rootElement).render(
  <StrictMode>
    <GlobalStyles />
    <AppGlobalStyles />
    <App />
  </StrictMode>,
);
