import type { TSessionEnd } from '../use-connection';

export type TSessionEndedNoticeProps = {
  readonly serverLabel: string;
  readonly end: TSessionEnd;
  readonly onReconnect: () => void;
  readonly onBack: () => void;
};
