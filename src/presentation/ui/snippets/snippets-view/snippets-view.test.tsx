import { fireEvent, render, screen, waitFor, within } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';

import { SnippetsView } from './snippets-view.ui';

const snippets = [{ id: 's1', name: 'Disk', command: 'df -h' }];

describe('SnippetsView', () => {
  it('SHOULD save a new snippet from the dialog', async () => {
    const onSave = vi.fn().mockResolvedValue(undefined);
    render(<SnippetsView snippets={[]} isLoading={false} onSave={onSave} onDelete={vi.fn()} />);

    fireEvent.click(screen.getByRole('button', { name: 'New snippet' }));
    fireEvent.change(screen.getByRole('textbox', { name: 'Name' }), {
      target: { value: 'Uptime' },
    });
    fireEvent.change(screen.getByRole('textbox', { name: /Command/u }), {
      target: { value: 'uptime' },
    });
    fireEvent.click(screen.getByRole('button', { name: 'Save snippet' }));

    await waitFor(() => {
      expect(onSave).toHaveBeenCalledWith({ name: 'Uptime', command: 'uptime' });
    });
  });

  it('SHOULD keep the dialog open with the reason when saving fails', async () => {
    const onSave = vi
      .fn()
      .mockResolvedValue({ kind: 'invalidInput', message: 'the command is empty' });
    render(
      <SnippetsView snippets={snippets} isLoading={false} onSave={onSave} onDelete={vi.fn()} />,
    );

    fireEvent.click(screen.getByRole('button', { name: 'Edit Disk' }));
    fireEvent.click(screen.getByRole('button', { name: 'Save snippet' }));

    expect(await screen.findByRole('alert')).toHaveTextContent('the command is empty');
    expect(onSave).toHaveBeenCalledWith({ id: 's1', name: 'Disk', command: 'df -h' });
  });

  it('SHOULD ask before deleting', async () => {
    const onDelete = vi.fn().mockResolvedValue(undefined);
    render(
      <SnippetsView snippets={snippets} isLoading={false} onSave={vi.fn()} onDelete={onDelete} />,
    );

    const list = screen.getByRole('list', { name: 'Snippets' });
    fireEvent.click(within(list).getByRole('button', { name: 'Delete Disk' }));
    fireEvent.click(screen.getByRole('button', { name: 'Delete' }));

    await waitFor(() => {
      expect(onDelete).toHaveBeenCalledWith('s1');
    });
  });
});
