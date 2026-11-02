import type { TCommandError } from '../../shared/command-error-contract';
import type { TOpenSessionRequest } from '../sessions-contract';

export type TConnectFormProps = {
  /** Fills the form, e.g. with the last attempt so it can be retried or fixed. */
  readonly initialRequest?: TOpenSessionRequest | undefined;
  readonly isConnecting: boolean;
  readonly error?: TCommandError | undefined;
  readonly onConnect: (request: TOpenSessionRequest) => void;
};
