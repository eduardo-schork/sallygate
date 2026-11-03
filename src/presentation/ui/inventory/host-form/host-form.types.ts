import type { THost, TSaveHostRequest } from '../inventory-contract';

export type THostFormProps = {
  /** The host being edited; a new host when missing. */
  readonly host?: THost | undefined;
  /** The `id` of the form element, so buttons outside it can submit it. */
  readonly formId: string;
  readonly onSubmit: (request: TSaveHostRequest) => void;
  /** The saved hosts this one can go through (`ProxyJump`). */
  readonly jumpCandidates: THost[];
};
