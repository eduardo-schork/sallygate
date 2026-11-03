import type { TCommandError } from '../../shared/command-error-contract';
import type { TSaveSnippetRequest, TSnippet } from '../snippets-contract';

export type TSnippetDialogProps = {
  /** The snippet to edit; a new one when missing. */
  readonly snippet?: TSnippet | undefined;
  readonly error?: TCommandError | undefined;
  readonly onSave: (request: TSaveSnippetRequest) => void;
  readonly onCancel: () => void;
};
