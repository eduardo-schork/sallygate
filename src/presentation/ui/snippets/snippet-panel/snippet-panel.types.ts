import type { TSnippet } from '../snippets-contract';

export type TSnippetPanelProps = {
  readonly snippets: TSnippet[];
  /** Sends the snippet to the terminal next to the panel. */
  readonly onRun: (snippet: TSnippet) => void;
};
