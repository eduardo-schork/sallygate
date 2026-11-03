import { act, renderHook, waitFor } from '@testing-library/react';
import { describe, expect, it } from 'vitest';

import { snippetInput } from './snippet-input';
import type { TSaveSnippetRequest, TSnippet } from './snippets-contract';
import type { TSnippetsGateway } from './snippets-gateway';
import { useSnippets } from './use-snippets';

function inMemorySnippets(): TSnippetsGateway {
  let snippets: TSnippet[] = [];
  return {
    listSnippets: () => Promise.resolve([...snippets]),
    saveSnippet: (request: TSaveSnippetRequest) => {
      if (request.command.trim() === '') {
        return Promise.reject(new Error('the command is empty'));
      }
      const snippet = { ...request, id: request.id ?? `s${String(snippets.length + 1)}` };
      snippets = [...snippets.filter((saved) => saved.id !== snippet.id), snippet];
      return Promise.resolve(snippet);
    },
    deleteSnippet: (snippetId) => {
      snippets = snippets.filter((snippet) => snippet.id !== snippetId);
      return Promise.resolve();
    },
  };
}

// Gateways are created once per test: a new gateway on every render would reload forever.
describe('useSnippets', () => {
  it('SHOULD save, list and delete snippets', async () => {
    const gateway = inMemorySnippets();
    const { result } = renderHook(() => useSnippets(gateway));
    await waitFor(() => {
      expect(result.current.isLoading).toBe(false);
    });

    await act(async () => {
      await result.current.saveSnippet({ name: 'Disk', command: 'df -h' });
    });
    expect(result.current.snippets.map((snippet) => snippet.name)).toEqual(['Disk']);

    await act(async () => {
      await result.current.deleteSnippet('s1');
    });
    expect(result.current.snippets).toEqual([]);
  });

  it('SHOULD hand back why a save failed', async () => {
    const gateway = inMemorySnippets();
    const { result } = renderHook(() => useSnippets(gateway));

    let error: Awaited<ReturnType<typeof result.current.saveSnippet>>;
    await act(async () => {
      error = await result.current.saveSnippet({ name: 'Empty', command: ' ' });
    });

    expect(error?.message).toContain('the command is empty');
  });
});

describe('snippetInput', () => {
  it('SHOULD end every line with Enter', () => {
    expect(snippetInput('uptime')).toBe('uptime\r');
    expect(snippetInput('cd /srv\ndocker compose up -d')).toBe('cd /srv\rdocker compose up -d\r');
  });
});
