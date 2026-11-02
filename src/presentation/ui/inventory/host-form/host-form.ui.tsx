import { Field, FLEX_ALIGNS, HFlex, Input, VFlex } from '@faber-ui/react';
import { type SubmitEvent, useState } from 'react';

import { DEFAULT_KEY_FILE_PATH, DEFAULT_SSH_PORT } from './host-form.constants';
import { AddressField } from './host-form.styles';
import type { THostFormProps } from './host-form.types';

/** Everything Sallygate needs to reach a server. Validation happens in the core. */
export function HostForm({ host, formId, onSubmit }: THostFormProps) {
  const [label, setLabel] = useState(host?.label ?? '');
  const [group, setGroup] = useState(host?.group ?? '');
  const [address, setAddress] = useState(host?.endpoint.address ?? '');
  const [port, setPort] = useState(String(host?.endpoint.port ?? DEFAULT_SSH_PORT));
  const [username, setUsername] = useState(host?.endpoint.username ?? '');
  const [keyFilePath, setKeyFilePath] = useState(host?.keyFilePath ?? DEFAULT_KEY_FILE_PATH);

  const handleSubmit = (event: SubmitEvent<HTMLFormElement>) => {
    event.preventDefault();
    onSubmit({
      ...(host ? { id: host.id } : {}),
      label,
      group,
      endpoint: { address, port: Number(port), username },
      keyFilePath,
    });
  };

  return (
    <form id={formId} onSubmit={handleSubmit} aria-label={host ? 'Edit host' : 'New host'}>
      <VFlex gap="MD">
        <Field label="Name" description="How the host appears in your list">
          <Input
            name="label"
            value={label}
            onChange={(event) => setLabel(event.target.value)}
            autoComplete="off"
            required
          />
        </Field>
        <Field label="Group" description="Optional, like Cloud or Home">
          <Input
            name="group"
            value={group}
            onChange={(event) => setGroup(event.target.value)}
            autoComplete="off"
          />
        </Field>
        <HFlex gap="SM" align={FLEX_ALIGNS.END}>
          <AddressField label="Address" description="Hostname or IP address">
            <Input
              name="address"
              value={address}
              onChange={(event) => setAddress(event.target.value)}
              autoComplete="off"
              spellCheck={false}
              required
            />
          </AddressField>
          <Field label="Port" description="Usually 22">
            <Input
              name="port"
              type="number"
              min={1}
              max={65535}
              value={port}
              onChange={(event) => setPort(event.target.value)}
              required
            />
          </Field>
        </HFlex>
        <Field label="Username">
          <Input
            name="username"
            value={username}
            onChange={(event) => setUsername(event.target.value)}
            autoComplete="off"
            spellCheck={false}
            required
          />
        </Field>
        <Field
          label="Private key"
          description="A key file without a passphrase, for example ~/.ssh/id_ed25519"
        >
          <Input
            name="keyFilePath"
            value={keyFilePath}
            onChange={(event) => setKeyFilePath(event.target.value)}
            autoComplete="off"
            spellCheck={false}
            required
          />
        </Field>
      </VFlex>
    </form>
  );
}
