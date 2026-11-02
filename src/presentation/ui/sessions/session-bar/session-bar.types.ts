export type TSessionBarProps = {
  /** `username@address:port` of the open session. */
  readonly serverLabel: string;
  readonly onClose: () => void;
};
