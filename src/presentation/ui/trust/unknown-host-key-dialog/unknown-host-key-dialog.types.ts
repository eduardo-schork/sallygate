import type { THostKey } from '../trust-contract';

export type TUnknownHostKeyDialogProps = {
  readonly open: boolean;
  /** `username@address:port`, as the user typed it. */
  readonly serverLabel: string;
  readonly hostKey: THostKey | undefined;
  readonly onTrust: () => void;
  readonly onCancel: () => void;
};
