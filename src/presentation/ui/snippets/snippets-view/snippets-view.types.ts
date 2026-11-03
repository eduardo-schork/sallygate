import type { TCommandError } from '../../shared/command-error-contract';
import type { TSaveSnippetRequest, TSnippet } from '../snippets-contract';

export type TSnippetsViewProps = {
  readonly snippets: TSnippet[];
  readonly isLoading: boolean;
  readonly loadError?: TCommandError | undefined;
  /** Resolves with the error to show, or nothing when it worked. */
  readonly onSave: (request: TSaveSnippetRequest) => Promise<TCommandError | undefined>;
  readonly onDelete: (snippetId: string) => Promise<TCommandError | undefined>;
};
