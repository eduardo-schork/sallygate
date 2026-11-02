import type { THostKey } from '../trust-contract';

export type TChangedHostKeyDialogProps = {
  readonly open: boolean;
  /** `username@address:port`, as the user typed it. */
  readonly serverLabel: string;
  readonly hostKey: THostKey | undefined;
  readonly onClose: () => void;
};
