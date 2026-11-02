import { fireEvent, render, screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';

import { HostForm } from './host-form.ui';

describe('HostForm', () => {
  it('SHOULD build a save request from what the user typed', () => {
    const onSubmit = vi.fn();
    render(<HostForm formId="host-form" onSubmit={onSubmit} />);

    fireEvent.change(screen.getByLabelText('Name'), { target: { value: 'oracle-vps' } });
    fireEvent.change(screen.getByLabelText('Group'), { target: { value: 'Cloud' } });
    fireEvent.change(screen.getByLabelText('Address'), { target: { value: '10.0.0.5' } });
    fireEvent.change(screen.getByLabelText('Username'), { target: { value: 'ubuntu' } });
    fireEvent.submit(screen.getByRole('form', { name: 'New host' }));

    expect(onSubmit).toHaveBeenCalledWith({
      label: 'oracle-vps',
      group: 'Cloud',
      endpoint: { address: '10.0.0.5', port: 22, username: 'ubuntu' },
      keyFilePath: '~/.ssh/id_ed25519',
    });
  });

  it('SHOULD keep the id when editing a saved host', () => {
    const onSubmit = vi.fn();
    render(
      <HostForm
        formId="host-form"
        host={{
          id: 'host-1',
          label: 'web',
          endpoint: { address: 'web.example.com', port: 2222, username: 'deploy' },
          keyFilePath: '~/.ssh/deploy',
        }}
        onSubmit={onSubmit}
      />,
    );

    fireEvent.submit(screen.getByRole('form', { name: 'Edit host' }));

    expect(onSubmit).toHaveBeenCalledWith(expect.objectContaining({ id: 'host-1', label: 'web' }));
  });
});
