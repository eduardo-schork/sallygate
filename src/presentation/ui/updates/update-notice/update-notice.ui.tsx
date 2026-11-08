import { Button, Progress, Text } from '@faber-ui/react';

import { useAppUpdate } from '../use-app-update';
import { UpdateNoticeActions, UpdateNoticeRoot } from './update-notice.styles';
import type { TUpdateNoticeProps } from './update-notice.types';

/** Offers a newer Sallygate when there is one; installing restarts the app. */
export function UpdateNotice({ updates }: TUpdateNoticeProps) {
  const { state, install, dismiss } = useAppUpdate(updates);

  if (state.status === 'none') {
    return null;
  }

  return (
    <UpdateNoticeRoot role="status" aria-label="App update">
      {state.status === 'available' ? (
        <>
          <Text.Span>
            Sallygate {state.update.version} is available. Open sessions close when it restarts.
          </Text.Span>
          <UpdateNoticeActions>
            <Button size="small" variant="subtle" color="neutral" onClick={dismiss}>
              Later
            </Button>
            <Button size="small" onClick={() => void install(state.update)}>
              Restart to update
            </Button>
          </UpdateNoticeActions>
        </>
      ) : null}
      {state.status === 'installing' ? (
        <>
          <Text.Span>Downloading Sallygate {state.version}…</Text.Span>
          <Progress label={`Downloading Sallygate ${state.version}`} value={state.share} max={1} />
        </>
      ) : null}
      {state.status === 'failed' ? (
        <>
          <Text.Span>
            Could not update to {state.version}: {state.message}
          </Text.Span>
          <UpdateNoticeActions>
            <Button size="small" variant="subtle" color="neutral" onClick={dismiss}>
              Dismiss
            </Button>
          </UpdateNoticeActions>
        </>
      ) : null}
    </UpdateNoticeRoot>
  );
}
