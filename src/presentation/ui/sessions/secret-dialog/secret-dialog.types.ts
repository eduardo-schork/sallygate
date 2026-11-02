import type { TAuth } from '../sessions-contract';

export type TSecretDialogProps = {
  readonly open: boolean;
  /** `username@address:port`, as the user typed it. */
  readonly serverLabel: string;
  /** Decides whether to ask for a password or a key passphrase. */
  readonly auth: TAuth;
  readonly onSubmit: (secret: string) => void;
  readonly onCancel: () => void;
};
