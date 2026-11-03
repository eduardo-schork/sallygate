import type { THost } from '../../inventory/inventory-contract';
import type { TTrustGateway } from '../../trust/trust-gateway';
import type { TTunnelsGateway } from '../tunnels-gateway';

export type TForwardingViewProps = {
  /** Rules go through saved hosts. */
  readonly hosts: THost[];
  readonly tunnels?: TTunnelsGateway;
  readonly trust?: TTrustGateway;
};
