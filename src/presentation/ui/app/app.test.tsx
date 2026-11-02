import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';

import { App } from './app.ui';

describe('App', () => {
  it('SHOULD render the empty host list inside the main landmark', () => {
    render(<App />);

    expect(screen.getByRole('main')).toHaveTextContent('No hosts yet');
  });
});
