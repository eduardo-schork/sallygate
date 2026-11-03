import { act, renderHook, waitFor } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';

import type { TRemoteEntry, TRemoteFolder } from './transfer-contract';
import type { TTransferGateway } from './transfer-gateway';
import { useRemoteFiles } from './use-remote-files';

function entry(path: string, kind: TRemoteEntry['kind'] = 'file'): TRemoteEntry {
  return { name: path.split('/').pop() ?? path, path, kind, sizeBytes: 10 };
}

function fakeFiles(): TTransferGateway & { readonly calls: string[] } {
  const calls: string[] = [];
  const listing: Record<string, TRemoteEntry[]> = {
    '/home/me': [entry('/home/me/apps', 'folder'), entry('/home/me/notes.txt')],
    '/home': [entry('/home/me', 'folder')],
  };
  return {
    calls,
    listFolder: (_sessionId, path = '/home/me') =>
      Promise.resolve<TRemoteFolder>({ path, entries: listing[path] ?? [] }),
    makeFolder: (_sessionId, parent, name) => {
      calls.push(`make ${parent} ${name}`);
      return Promise.resolve();
    },
    rename: () => Promise.resolve(),
    delete: (_sessionId, path) => {
      calls.push(`delete ${path}`);
      return Promise.resolve();
    },
    download: (_sessionId, remotePath, onProgress) => {
      onProgress({ doneBytes: 5, totalBytes: 10 });
      return Promise.resolve(`/Users/me/Downloads/${remotePath.split('/').pop() ?? ''}`);
    },
    upload: (_sessionId, localPath, remoteFolder) => {
      calls.push(`upload ${localPath} ${remoteFolder}`);
      return Promise.resolve(`${remoteFolder}/x`);
    },
    pickLocalFiles: () => Promise.resolve(['/Users/me/a.txt']),
    onFilesDropped: vi.fn(() => Promise.resolve(() => undefined)),
  };
}

async function renderFiles(files: TTransferGateway) {
  const hook = renderHook(() => useRemoteFiles('session-1', files));
  await waitFor(() => {
    expect(hook.result.current.isLoading).toBe(false);
  });
  return hook;
}

// Gateways are created once per test: a new gateway on every render would reload forever.
describe('useRemoteFiles', () => {
  it('SHOULD open the login folder and go up from it', async () => {
    const { result } = await renderFiles(fakeFiles());
    expect(result.current.folder?.path).toBe('/home/me');

    await act(() => result.current.goUp());

    expect(result.current.folder?.path).toBe('/home');
  });

  it('SHOULD make folders in the folder on screen', async () => {
    const files = fakeFiles();
    const { result } = await renderFiles(files);

    await act(() => result.current.makeFolder('logs'));

    expect(files.calls).toEqual(['make /home/me logs']);
  });

  it('SHOULD track a download until it lands', async () => {
    const { result } = await renderFiles(fakeFiles());

    await act(() => result.current.download(entry('/home/me/notes.txt')));

    expect(result.current.transfers).toEqual([
      expect.objectContaining({
        name: 'notes.txt',
        direction: 'download',
        status: 'done',
        detail: '/Users/me/Downloads/notes.txt',
        doneBytes: 5,
      }),
    ]);
  });

  it('SHOULD upload picked files into the folder on screen', async () => {
    const files = fakeFiles();
    const { result } = await renderFiles(files);

    await act(() => result.current.pickAndUpload());

    expect(files.calls).toEqual(['upload /Users/me/a.txt /home/me']);
    expect(result.current.transfers[0]).toMatchObject({ name: 'a.txt', status: 'done' });
  });

  it('SHOULD show why a change failed', async () => {
    const files = fakeFiles();
    const { result } = await renderFiles({
      ...files,
      delete: () => Promise.reject(new Error('permission denied')),
    });

    await act(() => result.current.remove(entry('/home/me/notes.txt')));

    expect(result.current.error?.message).toContain('permission denied');
  });
});
