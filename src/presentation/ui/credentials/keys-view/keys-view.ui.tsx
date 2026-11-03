import {
  Alert,
  Badge,
  Button,
  EmptyState,
  FLEX_ALIGNS,
  FLEX_JUSTIFIES,
  HFlex,
  Spinner,
  Text,
  Title,
} from '@faber-ui/react';
import { CheckIcon, CopyIcon, LockIcon, PlusIcon } from '@faber-ui/icons';
import { useState } from 'react';

import type { TCommandError } from '../../shared/command-error-contract';
import type { TGenerateSshKeyRequest, TSshKey } from '../credentials-contract';
import { GenerateKeyDialog } from '../generate-key-dialog';
import { keyUsers } from '../key-users';
import { useSshKeys } from '../use-ssh-keys';
import {
  KeyCard,
  KeyFingerprint,
  KeyIcon,
  KeyList,
  KeyText,
  KeysViewRoot,
} from './keys-view.styles';
import type { TKeysViewProps } from './keys-view.types';

type TGenerating = { readonly isGenerating: boolean; readonly error?: TCommandError };

function usedByText(key: TSshKey, users: string[]): string {
  const algorithm = key.algorithm.replace(/^ssh-/u, '');
  return users.length === 0
    ? `${algorithm}, not used by a saved host`
    : `${algorithm}, used by ${users.join(', ')}`;
}

/** The key pairs in ~/.ssh: make one, bring one in, copy a public key to a server. */
export function KeysView({ hosts, credentials }: TKeysViewProps) {
  const { keys, isLoading, error, generate, importKey, copyText } = useSshKeys(credentials);
  const [generating, setGenerating] = useState<TGenerating>();
  const [copiedName, setCopiedName] = useState<string>();

  const handleGenerate = async (request: TGenerateSshKeyRequest) => {
    setGenerating({ isGenerating: true });
    const failure = await generate(request);
    setGenerating(failure ? { isGenerating: false, error: failure } : undefined);
  };

  const copyPublicKey = async (key: TSshKey) => {
    await copyText(key.publicKey);
    setCopiedName(key.name);
  };

  return (
    <KeysViewRoot gap="LG">
      <HFlex gap="SM" align={FLEX_ALIGNS.CENTER} justify={FLEX_JUSTIFIES.SPACE_BETWEEN}>
        <Title.H1>SSH keys</Title.H1>
        <HFlex gap="SM">
          <Button variant="outline" color="neutral" onClick={() => void importKey()}>
            Import
          </Button>
          <Button startIcon={<PlusIcon />} onClick={() => setGenerating({ isGenerating: false })}>
            Generate key
          </Button>
        </HFlex>
      </HFlex>
      <Text.P>
        The keys in <Text.Code>~/.ssh</Text.Code>. To log in with one, copy its public key into the
        server&apos;s <Text.Code>~/.ssh/authorized_keys</Text.Code>.
      </Text.P>

      {error ? (
        <Alert color="error" role="alert">
          {error.message}
        </Alert>
      ) : null}
      {isLoading ? <Spinner label="Loading keys" /> : null}
      {!isLoading && keys.length === 0 && !error ? (
        <EmptyState
          title="No keys in ~/.ssh yet"
          description="Generate one here, or import a private key file you already have."
        />
      ) : null}

      <KeyList gap="SM" aria-label="SSH keys">
        {keys.map((key) => (
          <li key={key.name}>
            <KeyCard>
              <KeyIcon aria-hidden="true">
                <LockIcon />
              </KeyIcon>
              <KeyText gap="XXS">
                <HFlex gap="XS" align={FLEX_ALIGNS.CENTER}>
                  <Text.Strong>{key.name}</Text.Strong>
                  {key.isEncrypted ? <Badge color="neutral">Passphrase</Badge> : null}
                </HFlex>
                <Text.Small>{usedByText(key, keyUsers(key, hosts))}</Text.Small>
                <KeyFingerprint>{key.fingerprint}</KeyFingerprint>
              </KeyText>
              <Button
                size="small"
                variant="outline"
                color="neutral"
                startIcon={copiedName === key.name ? <CheckIcon /> : <CopyIcon />}
                aria-label={`Copy the public key of ${key.name}`}
                onClick={() => void copyPublicKey(key)}
              >
                {copiedName === key.name ? 'Copied' : 'Copy public key'}
              </Button>
            </KeyCard>
          </li>
        ))}
      </KeyList>

      {generating ? (
        <GenerateKeyDialog
          isGenerating={generating.isGenerating}
          error={generating.error}
          onGenerate={(request) => void handleGenerate(request)}
          onCancel={() => setGenerating(undefined)}
        />
      ) : null}
    </KeysViewRoot>
  );
}
