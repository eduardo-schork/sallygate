import type { TCommandError } from '../../shared/command-error-contract';
import type { THost } from '../inventory-contract';
import type { TImportResult } from '../use-hosts';

export type THostsViewProps = {
  readonly hosts: THost[];
  readonly isLoading: boolean;
  readonly loadError?: TCommandError | undefined;
  readonly onNewHost: () => void;
  readonly onConnect: (host: THost) => void;
  readonly onEdit: (host: THost) => void;
  readonly onImportSshConfig: () => Promise<TImportResult>;
};
