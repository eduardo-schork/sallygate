import type { TTransferGateway } from '../transfer-gateway';

export type TFilesViewProps = {
  readonly sessionId: string;
  /** Files dropped on the window go to this view only while it is on screen. */
  readonly isActive: boolean;
  readonly files?: TTransferGateway;
};
