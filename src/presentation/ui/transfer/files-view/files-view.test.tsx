import { fireEvent, render, screen, waitFor, within } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';

import type { TFileEntry } from '../transfer-contract';
import type { TTransferGateway } from '../transfer-gateway';
import { FilesView } from './files-view.ui';

function entry(path: string, kind: TFileEntry['kind'] = 'file'): TFileEntry {
  return {
    name: path.split('/').pop() ?? path,
    path,
    kind,
    sizeBytes: 2048,
    permissions: kind === 'folder' ? 0o755 : 0o644,
  };
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
    listLocalFolder: (path = '/Users/me') =>
      Promise.resolve({
        path,
        entries: path === '/Users/me' ? [entry('/Users/me/report.pdf')] : [],
      }),
    makeFolder: vi.fn(() => Promise.resolve()),
    rename: vi.fn(() => Promise.resolve()),
    delete: vi.fn(() => Promise.resolve()),
    download: vi.fn(() => Promise.resolve('/Users/me/notes.txt')),
    upload: vi.fn(() => Promise.resolve('/home/me/report.pdf')),
    pickLocalFiles: () => Promise.resolve([]),
    onFilesDropped: () => Promise.resolve(() => undefined),
    ...overrides,
  };
}

function renderView(files: TTransferGateway) {
  render(<FilesView sessionId="s1" serverLabel="me@server:22" isActive files={files} />);
}

describe('FilesView', () => {
  it('SHOULD show this Mac beside the server, with sizes and permissions', async () => {
    renderView(fakeFiles());

    const local = screen.getByRole('region', { name: 'This Mac' });
    const server = screen.getByRole('region', { name: 'me@server:22' });
    expect(await within(local).findByText('report.pdf')).toBeInTheDocument();
    expect(await within(server).findByText('notes.txt')).toBeInTheDocument();
    expect(within(server).getByText('drwxr-xr-x')).toBeInTheDocument();
    expect(within(server).getAllByText('2.0 KB')).toHaveLength(1);
  });

  it('SHOULD download into the folder open on this Mac', async () => {
    const files = fakeFiles();
    renderView(files);
    const server = screen.getByRole('region', { name: 'me@server:22' });

    fireEvent.click(
      await within(server).findByRole('button', { name: 'Download notes.txt to This Mac' }),
    );

    const transfers = await screen.findByRole('region', { name: 'Transfers' });
    expect(await within(transfers).findByText('Saved to /Users/me/notes.txt')).toBeInTheDocument();
    expect(files.download).toHaveBeenCalledWith(
      's1',
      '/home/me/notes.txt',
      'file',
      '/Users/me',
      expect.any(Function),
    );
  });

  it('SHOULD upload a file from this Mac into the server folder on screen', async () => {
    const files = fakeFiles();
    renderView(files);
    const local = screen.getByRole('region', { name: 'This Mac' });
    await screen.findByText('notes.txt');

    fireEvent.click(
      await within(local).findByRole('button', { name: 'Upload report.pdf to the server' }),
    );

    await waitFor(() => {
      expect(files.upload).toHaveBeenCalledWith(
        's1',
        '/Users/me/report.pdf',
        '/home/me',
        expect.any(Function),
      );
    });
  });

  it('SHOULD open a server folder and ask before deleting', async () => {
    const files = fakeFiles();
    renderView(files);
    const server = screen.getByRole('region', { name: 'me@server:22' });

    fireEvent.click(await within(server).findByRole('button', { name: 'Delete notes.txt' }));
    fireEvent.click(screen.getByRole('button', { name: 'Delete' }));
    await waitFor(() => {
      expect(files.delete).toHaveBeenCalledWith('s1', '/home/me/notes.txt', 'file');
    });

    fireEvent.click(within(server).getByRole('button', { name: 'apps' }));
    expect(await within(server).findByText('This folder is empty.')).toBeInTheDocument();
  });

  it('SHOULD download a whole server folder to this Mac', async () => {
    const files = fakeFiles();
    renderView(files);
    const server = screen.getByRole('region', { name: 'me@server:22' });

    fireEvent.click(
      await within(server).findByRole('button', { name: 'Download apps to This Mac' }),
    );

    await waitFor(() => {
      expect(files.download).toHaveBeenCalledWith(
        's1',
        '/home/me/apps',
        'folder',
        '/Users/me',
        expect.any(Function),
      );
    });
  });
});
