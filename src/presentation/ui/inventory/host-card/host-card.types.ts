import type { THost } from '../inventory-contract';

export type THostCardProps = {
  readonly host: THost;
  readonly onConnect: (host: THost) => void;
  readonly onEdit: (host: THost) => void;
};
