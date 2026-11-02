import { fireEvent, render, screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';

import { SecretDialog } from './secret-dialog.ui';

describe('SecretDialog', () => {
  it('SHOULD send the typed password and clear the field', () => {
    const onSubmit = vi.fn();
    render(
      <SecretDialog
        open
        serverLabel="me@home.lan:22"
        auth={{ type: 'password' }}
        onSubmit={onSubmit}
        onCancel={vi.fn()}
      />,
    );

    const field = screen.getByLabelText('Password');
    fireEvent.change(field, { target: { value: 'hunter2' } });
    fireEvent.click(screen.getByRole('button', { name: 'Connect' }));

    expect(onSubmit).toHaveBeenCalledWith('hunter2');
    expect(field).toHaveValue('');
  });

  it('SHOULD ask for the passphrase of a key file', () => {
    render(
      <SecretDialog
        open
        serverLabel="ubuntu@vps:22"
        auth={{ type: 'keyFile', path: '~/.ssh/id_ed25519' }}
        onSubmit={vi.fn()}
        onCancel={vi.fn()}
      />,
    );

    expect(screen.getByLabelText('Passphrase')).toBeInTheDocument();
  });
});
