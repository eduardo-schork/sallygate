import type { THost } from '../../inventory/inventory-contract';
import type { TCommandError } from '../../shared/command-error-contract';
import type { TSaveTunnelRequest, TTunnel } from '../tunnels-contract';

export type TTunnelDialogProps = {
  /** The rule to edit; a new one when missing. */
  readonly tunnel?: TTunnel | undefined;
  readonly hosts: THost[];
  readonly error?: TCommandError | undefined;
  readonly onSave: (request: TSaveTunnelRequest) => void;
  readonly onCancel: () => void;
};
