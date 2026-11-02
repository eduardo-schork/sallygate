import type { TCommandError } from '../../shared/command-error-contract';
import type { THost, TSaveHostRequest } from '../inventory-contract';

export type THostDrawerProps = {
  readonly open: boolean;
  /** The host being edited; a new host when missing. */
  readonly host?: THost | undefined;
  readonly isSaving: boolean;
  readonly error?: TCommandError | undefined;
  readonly onSave: (request: TSaveHostRequest) => void;
  readonly onDelete: (host: THost) => void;
  readonly onClose: () => void;
};
