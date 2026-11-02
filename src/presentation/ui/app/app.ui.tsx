import { EmptyState } from '@faber-ui/react';

import { AppRoot } from './app.styles';

export function App() {
  return (
    <AppRoot>
      <EmptyState title="No hosts yet" description="Saved servers will show up here." />
    </AppRoot>
  );
}
