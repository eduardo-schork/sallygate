import { fireEvent, render, screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';

import { UnknownHostKeyDialog } from './unknown-host-key-dialog.ui';

const hostKey = {
  algorithm: 'ssh-ed25519',
  fingerprint: 'SHA256:6ZHFhJ1NWossUqeghPe7RJ7zSEwg2qtgTFNkH4htT4Q',
  opensshPublicKey: 'ssh-ed25519 AAAA',
};

describe('UnknownHostKeyDialog', () => {
  it('SHOULD show the fingerprint to compare and trust only on confirm', () => {
    const onTrust = vi.fn();
    render(
      <UnknownHostKeyDialog
        open
        serverLabel="sallygate@127.0.0.1:2222"
        hostKey={hostKey}
        onTrust={onTrust}
        onCancel={vi.fn()}
      />,
    );

    expect(screen.getByText(hostKey.fingerprint)).toBeInTheDocument();
    expect(onTrust).not.toHaveBeenCalled();

    fireEvent.click(screen.getByRole('button', { name: 'Trust and connect' }));

    expect(onTrust).toHaveBeenCalledOnce();
  });
});
