import { fireEvent, render, screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';

import type { TUpdatesGateway } from '../updates-gateway';
import { UpdateNotice } from './update-notice.ui';

describe('UpdateNotice', () => {
  it('SHOULD say nothing when the app is up to date or offline', async () => {
    const updates: TUpdatesGateway = { checkForUpdate: () => Promise.reject(new Error('offline')) };
    render(<UpdateNotice updates={updates} />);

    await Promise.resolve();

    expect(screen.queryByRole('status', { name: 'App update' })).not.toBeInTheDocument();
  });

  it('SHOULD offer a newer version and install it when asked', async () => {
    const install = vi.fn(() => Promise.resolve());
    const updates: TUpdatesGateway = {
      checkForUpdate: () => Promise.resolve({ version: '0.2.0', install }),
    };
    render(<UpdateNotice updates={updates} />);

    expect(await screen.findByText(/Sallygate 0\.2\.0 is available/u)).toBeInTheDocument();
    fireEvent.click(screen.getByRole('button', { name: 'Restart to update' }));

    expect(install).toHaveBeenCalledOnce();
  });

  it('SHOULD say why an update failed', async () => {
    const updates: TUpdatesGateway = {
      checkForUpdate: () =>
        Promise.resolve({
          version: '0.2.0',
          install: () => Promise.reject(new Error('signature mismatch')),
        }),
    };
    render(<UpdateNotice updates={updates} />);

    fireEvent.click(await screen.findByRole('button', { name: 'Restart to update' }));

    expect(
      await screen.findByText(/Could not update to 0\.2\.0: .*signature mismatch/u),
    ).toBeInTheDocument();
  });
});
