import type { THost } from '../../inventory/inventory-contract';
import type { TCredentialsGateway } from '../credentials-gateway';

export type TKeysViewProps = {
  /** To tell which hosts sign in with each key. */
  readonly hosts: THost[];
  readonly credentials?: TCredentialsGateway;
};
