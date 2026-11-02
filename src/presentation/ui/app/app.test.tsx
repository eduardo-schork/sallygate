import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';

import { App } from './app.ui';

describe('App', () => {
  it('SHOULD start on the connect form', () => {
    render(<App />);

    expect(screen.getByRole('heading', { name: 'Connect to a server' })).toBeInTheDocument();
    expect(screen.getByRole('form', { name: 'Connect to a server' })).toBeInTheDocument();
    expect(screen.getByLabelText('Private key')).toHaveValue('~/.ssh/id_ed25519');
  });
});
