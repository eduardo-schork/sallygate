import { Text } from '@faber-ui/react';

import {
  SnippetPanelRoot,
  SnippetRunButton,
  SnippetRunCommand,
  SnippetRunList,
} from './snippet-panel.styles';
import type { TSnippetPanelProps } from './snippet-panel.types';

/** The saved snippets beside a terminal; a click types one into it and presses Enter. */
export function SnippetPanel({ snippets, onRun }: TSnippetPanelProps) {
  return (
    <SnippetPanelRoot gap="SM" aria-label="Snippets">
      <Text.Strong>Snippets</Text.Strong>
      {snippets.length === 0 ? (
        <Text.Small>No snippets yet. Save one in Snippets, in the side menu.</Text.Small>
      ) : (
        <SnippetRunList gap="XS">
          {snippets.map((snippet) => (
            <li key={snippet.id}>
              <SnippetRunButton
                type="button"
                title={`Run: ${snippet.command}`}
                onClick={() => onRun(snippet)}
              >
                <Text.Small>{snippet.name}</Text.Small>
                <SnippetRunCommand>{snippet.command.split('\n')[0]}</SnippetRunCommand>
              </SnippetRunButton>
            </li>
          ))}
        </SnippetRunList>
      )}
    </SnippetPanelRoot>
  );
}
