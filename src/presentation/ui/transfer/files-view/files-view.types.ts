import type { TTransferGateway } from '../transfer-gateway';

export type TFilesViewProps = {
  readonly sessionId: string;
  /** How the server is named in its pane, like `ubuntu@10.0.0.5:22`. */
  readonly serverLabel: string;
  /** Files dropped on the window go to this view only while it is on screen. */
  readonly isActive: boolean;
  readonly files?: TTransferGateway;
};
