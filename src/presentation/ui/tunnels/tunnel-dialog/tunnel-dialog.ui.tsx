import {
  Alert,
  Button,
  Dialog,
  Field,
  FLEX_JUSTIFIES,
  HFlex,
  Input,
  Segment,
  SegmentedControl,
  Select,
  VFlex,
} from '@faber-ui/react';
import { type SubmitEvent, useId, useState } from 'react';

import type { TTunnelKind } from '../tunnels-contract';
import type { TTunnelDialogProps } from './tunnel-dialog.types';

type TKindType = TTunnelKind['type'];

/** Creates or edits one forwarding rule. Mount it when it opens, so it starts from `tunnel`. */
export function TunnelDialog({ tunnel, hosts, error, onSave, onCancel }: TTunnelDialogProps) {
  const formId = useId();
  const kind = tunnel?.kind;
  const [name, setName] = useState(tunnel?.name ?? '');
  const [hostId, setHostId] = useState(tunnel?.hostId ?? hosts[0]?.id ?? '');
  const [type, setType] = useState<TKindType>(kind?.type ?? 'local');
  const [localPort, setLocalPort] = useState(String(kind?.localPort ?? ''));
  const [targetAddress, setTargetAddress] = useState(
    kind?.type === 'local' ? kind.targetAddress : 'localhost',
  );
  const [targetPort, setTargetPort] = useState(
    kind?.type === 'local' ? String(kind.targetPort) : '',
  );

  const handleSubmit = (event: SubmitEvent<HTMLFormElement>) => {
    event.preventDefault();
    const port = Number(localPort);
    onSave({
      ...(tunnel ? { id: tunnel.id } : {}),
      name,
      hostId,
      kind:
        type === 'local'
          ? { type, localPort: port, targetAddress, targetPort: Number(targetPort) }
          : { type, localPort: port },
    });
  };

  return (
    <Dialog
      open
      title={tunnel ? `Edit ${tunnel.name}` : 'New rule'}
      closeLabel="Cancel"
      onClose={onCancel}
      footer={
        <HFlex gap="SM" justify={FLEX_JUSTIFIES.END}>
          <Button variant="outline" onClick={onCancel}>
            Cancel
          </Button>
          <Button type="submit" form={formId}>
            Save rule
          </Button>
        </HFlex>
      }
    >
      <form id={formId} onSubmit={handleSubmit}>
        <VFlex gap="MD">
          {error ? (
            <Alert color="error" role="alert">
              {error.message}
            </Alert>
          ) : null}
          <Field label="Name">
            <Input
              name="name"
              value={name}
              placeholder="Postgres on oracle"
              onChange={(event) => setName(event.target.value)}
              autoComplete="off"
              required
            />
          </Field>
          <Field label="Through host" description="The saved host the traffic goes through.">
            <Select
              name="hostId"
              value={hostId}
              onChange={(event) => setHostId(event.target.value)}
              required
            >
              {hosts.map((host) => (
                <option key={host.id} value={host.id}>
                  {host.label}
                </option>
              ))}
            </Select>
          </Field>
          <SegmentedControl label="Kind">
            <Segment
              name="kind"
              value="local"
              checked={type === 'local'}
              onChange={() => setType('local')}
            >
              A service on the server
            </Segment>
            <Segment
              name="kind"
              value="dynamic"
              checked={type === 'dynamic'}
              onChange={() => setType('dynamic')}
            >
              SOCKS proxy
            </Segment>
          </SegmentedControl>
          <Field
            label="Port on this Mac"
            description={
              type === 'local'
                ? 'Open localhost on this port to reach the service.'
                : 'Point your browser or app at a SOCKS5 proxy on localhost and this port.'
            }
          >
            <Input
              name="localPort"
              inputMode="numeric"
              value={localPort}
              placeholder={type === 'local' ? '5432' : '1080'}
              onChange={(event) => setLocalPort(event.target.value)}
              required
            />
          </Field>
          {type === 'local' ? (
            <HFlex gap="SM">
              <Field label="Service address" description="As the server sees it.">
                <Input
                  name="targetAddress"
                  value={targetAddress}
                  onChange={(event) => setTargetAddress(event.target.value)}
                  autoComplete="off"
                  spellCheck={false}
                  required
                />
              </Field>
              <Field label="Service port">
                <Input
                  name="targetPort"
                  inputMode="numeric"
                  value={targetPort}
                  placeholder="5432"
                  onChange={(event) => setTargetPort(event.target.value)}
                  required
                />
              </Field>
            </HFlex>
          ) : null}
        </VFlex>
      </form>
    </Dialog>
  );
}
