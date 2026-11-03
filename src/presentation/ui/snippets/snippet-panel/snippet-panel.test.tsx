import { fireEvent, render, screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';

import { SnippetPanel } from './snippet-panel.ui';

describe('SnippetPanel', () => {
  it('SHOULD run the snippet that is clicked', () => {
    const onRun = vi.fn();
    const disk = { id: 's1', name: 'Disk', command: 'df -h' };
    render(<SnippetPanel snippets={[disk]} onRun={onRun} />);

    fireEvent.click(screen.getByRole('button', { name: /Disk/u }));

    expect(onRun).toHaveBeenCalledWith(disk);
  });
});
