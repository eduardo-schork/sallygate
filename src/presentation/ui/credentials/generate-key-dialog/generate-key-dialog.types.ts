import type { TCommandError } from '../../shared/command-error-contract';
import type { TGenerateSshKeyRequest } from '../credentials-contract';

export type TGenerateKeyDialogProps = {
  readonly error?: TCommandError | undefined;
  readonly isGenerating: boolean;
  readonly onGenerate: (request: TGenerateSshKeyRequest) => void;
  readonly onCancel: () => void;
};
