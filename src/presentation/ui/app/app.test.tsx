import { fireEvent, render, screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';

import { App } from './app.ui';

// The app talks to the Rust core through Tauri; here the core has no saved hosts.
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn((command: string) =>
    command === 'list_hosts' || command === 'list_snippets'
      ? Promise.resolve([])
      : Promise.reject(new Error(command)),
  ),
  Channel: vi.fn(),
}));

describe('App', () => {
  it('SHOULD open on the hosts screen and invite to add the first host', async () => {
    render(<App />);

    expect(screen.getByRole('tab', { name: 'Sallygate', selected: true })).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Hosts', current: 'page' })).toBeInTheDocument();
    expect(await screen.findByText('No hosts yet')).toBeInTheDocument();
  });

  it('SHOULD switch screens from the side menu', async () => {
    render(<App />);

    fireEvent.click(screen.getByRole('button', { name: 'Snippets' }));

    expect(await screen.findByText('No snippets yet')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Snippets', current: 'page' })).toBeInTheDocument();
  });
});
