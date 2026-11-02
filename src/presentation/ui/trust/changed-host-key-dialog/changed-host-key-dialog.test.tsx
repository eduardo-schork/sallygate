import { render, screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';

import { ChangedHostKeyDialog } from './changed-host-key-dialog.ui';

describe('ChangedHostKeyDialog', () => {
  it('SHOULD warn about the changed key and offer no way to trust it', () => {
    render(
      <ChangedHostKeyDialog
        open
        serverLabel="sallygate@127.0.0.1:2222"
        hostKey={{ algorithm: 'ssh-ed25519', fingerprint: 'SHA256:new', opensshPublicKey: '' }}
        onClose={vi.fn()}
      />,
    );

    expect(screen.getByText('Sallygate did not connect')).toBeInTheDocument();
    expect(screen.getByText('SHA256:new')).toBeInTheDocument();
    expect(screen.queryByRole('button', { name: /trust/i })).not.toBeInTheDocument();
  });
});
