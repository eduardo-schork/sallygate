import { fireEvent, render, screen, waitFor, within } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';

import type { TRemoteEntry } from '../transfer-contract';
import type { TTransferGateway } from '../transfer-gateway';
import { FilesView } from './files-view.ui';

function entry(path: string, kind: TRemoteEntry['kind'] = 'file'): TRemoteEntry {
  return { name: path.split('/').pop() ?? path, path, kind, sizeBytes: 2048 };
}

function fakeFiles(overrides: Partial<TTransferGateway> = {}): TTransferGateway {
  return {
    listFolder: (_sessionId, path = '/home/me') =>
      Promise.resolve({
        path,
        entries:
          path === '/home/me'
            ? [entry('/home/me/apps', 'folder'), entry('/home/me/notes.txt')]
            : [],
      }),
    makeFolder: vi.fn(() => Promise.resolve()),
    rename: vi.fn(() => Promise.resolve()),
    delete: vi.fn(() => Promise.resolve()),
    download: vi.fn(() => Promise.resolve('/Users/me/Downloads/notes.txt')),
    upload: vi.fn(() => Promise.resolve('/home/me/a.txt')),
    pickLocalFiles: () => Promise.resolve([]),
    onFilesDropped: () => Promise.resolve(() => undefined),
    ...overrides,
  };
}

describe('FilesView', () => {
  it('SHOULD list the login folder and open a folder', async () => {
    render(<FilesView sessionId="s1" isActive files={fakeFiles()} />);

    const table = await screen.findByRole('table', { name: 'Files in /home/me' });
    expect(within(table).getByText('notes.txt')).toBeInTheDocument();
    expect(within(table).getByText('2.0 KB')).toBeInTheDocument();

    fireEvent.click(within(table).getByRole('button', { name: 'apps' }));

    expect(await screen.findByText(/This folder is empty/u)).toBeInTheDocument();
  });

  it('SHOULD download a file and say where it landed', async () => {
    render(<FilesView sessionId="s1" isActive files={fakeFiles()} />);

    fireEvent.click(await screen.findByRole('button', { name: 'Download notes.txt' }));

    const transfers = await screen.findByRole('region', { name: 'Transfers' });
    expect(
      await within(transfers).findByText('Saved to /Users/me/Downloads/notes.txt'),
    ).toBeInTheDocument();
  });

  it('SHOULD ask before deleting', async () => {
    const files = fakeFiles();
    render(<FilesView sessionId="s1" isActive files={files} />);

    fireEvent.click(await screen.findByRole('button', { name: 'Delete notes.txt' }));
    expect(files.delete).not.toHaveBeenCalled();
    fireEvent.click(screen.getByRole('button', { name: 'Delete' }));

    await waitFor(() => {
      expect(files.delete).toHaveBeenCalledWith('s1', '/home/me/notes.txt', 'file');
    });
  });

  it('SHOULD upload files dropped on the window while it is on screen', async () => {
    let drop: ((paths: string[]) => void) | undefined;
    const files = fakeFiles({
      onFilesDropped: (onDrop) => {
        drop = onDrop;
        return Promise.resolve(() => undefined);
      },
    });
    render(<FilesView sessionId="s1" isActive files={files} />);
    await screen.findByRole('table');

    drop?.(['/Users/me/a.txt']);

    await waitFor(() => {
      expect(files.upload).toHaveBeenCalledWith(
        's1',
        '/Users/me/a.txt',
        '/home/me',
        expect.any(Function),
      );
    });
  });
});
