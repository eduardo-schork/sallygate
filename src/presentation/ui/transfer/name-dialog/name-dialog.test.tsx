import { fireEvent, render, screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';

import { NameDialog } from './name-dialog.ui';

describe('NameDialog', () => {
  it('SHOULD start from the given name and send the trimmed new one', () => {
    const onSubmit = vi.fn();
    render(
      <NameDialog
        open
        title="Rename notes.txt"
        initialName="notes.txt"
        submitLabel="Rename"
        onSubmit={onSubmit}
        onCancel={vi.fn()}
      />,
    );
    const field = screen.getByRole('textbox', { name: 'Name' });
    expect(field).toHaveValue('notes.txt');

    fireEvent.change(field, { target: { value: '  todo.txt ' } });
    fireEvent.click(screen.getByRole('button', { name: 'Rename' }));

    expect(onSubmit).toHaveBeenCalledWith('todo.txt');
  });
});
