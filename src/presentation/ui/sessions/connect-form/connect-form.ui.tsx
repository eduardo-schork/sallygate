import {
  Alert,
  Button,
  Field,
  FLEX_ALIGNS,
  FLEX_JUSTIFIES,
  HFlex,
  Input,
  Text,
  VFlex,
} from '@faber-ui/react';
import { type SubmitEvent, useState } from 'react';

import { CONNECTION_ERROR_COPY } from '../connection-error-copy';
import { AddressField } from './connect-form.styles';
import {
  DEFAULT_KEY_FILE_PATH,
  DEFAULT_SSH_PORT,
  INITIAL_TERMINAL_SIZE,
} from './connect-form.constants';
import type { TConnectFormProps } from './connect-form.types';

export function ConnectForm({ initialRequest, isConnecting, error, onConnect }: TConnectFormProps) {
  const [address, setAddress] = useState(initialRequest?.endpoint.address ?? '');
  const [port, setPort] = useState(String(initialRequest?.endpoint.port ?? DEFAULT_SSH_PORT));
  const [username, setUsername] = useState(initialRequest?.endpoint.username ?? '');
  const [keyFilePath, setKeyFilePath] = useState(
    initialRequest?.keyFilePath ?? DEFAULT_KEY_FILE_PATH,
  );

  const handleSubmit = (event: SubmitEvent<HTMLFormElement>) => {
    event.preventDefault();
    onConnect({
      endpoint: { address, port: Number(port), username },
      keyFilePath,
      ...INITIAL_TERMINAL_SIZE,
    });
  };

  const errorCopy = error ? CONNECTION_ERROR_COPY[error.kind] : undefined;

  return (
    <form onSubmit={handleSubmit} aria-label="Connect to a server">
      <VFlex gap="MD">
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
        {error && errorCopy ? (
          <Alert color="error" title={errorCopy.title} role="alert">
            <VFlex gap="XXS">
              <span>{errorCopy.hint}</span>
              <Text.Small>{error.message}</Text.Small>
            </VFlex>
          </Alert>
        ) : null}
        <HFlex justify={FLEX_JUSTIFIES.END}>
          <Button type="submit" loading={isConnecting}>
            Connect
          </Button>
        </HFlex>
      </VFlex>
    </form>
  );
}
