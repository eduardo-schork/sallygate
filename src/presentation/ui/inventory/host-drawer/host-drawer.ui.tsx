import {
  Alert,
  AlertDialog,
  Button,
  Drawer,
  DRAWER_SIDES,
  FLEX_JUSTIFIES,
  HFlex,
  VFlex,
} from '@faber-ui/react';
import { useId, useState } from 'react';

import { HostForm } from '../host-form';
import type { THostDrawerProps } from './host-drawer.types';

/** Side panel to add a host or change a saved one, like the Termius host editor. */
export function HostDrawer({
  open,
  host,
  isSaving,
  error,
  onSave,
  onDelete,
  onClose,
}: THostDrawerProps) {
  const formId = useId();
  const [isConfirmingDelete, setIsConfirmingDelete] = useState(false);

  return (
    <>
      <Drawer
        open={open}
        onClose={onClose}
        side={DRAWER_SIDES.END}
        title={host ? `Edit ${host.label}` : 'New host'}
        closeLabel="Close"
        footer={
          <HFlex gap="SM" justify={FLEX_JUSTIFIES.SPACE_BETWEEN}>
            {host ? (
              <Button variant="subtle" color="accent" onClick={() => setIsConfirmingDelete(true)}>
                Delete host
              </Button>
            ) : (
              <span />
            )}
            <HFlex gap="SM">
              <Button variant="outline" onClick={onClose}>
                Cancel
              </Button>
              <Button type="submit" form={formId} loading={isSaving}>
                Save host
              </Button>
            </HFlex>
          </HFlex>
        }
      >
        <VFlex gap="MD">
          {/* A new key per host resets the form's fields when another host is opened. */}
          <HostForm key={host?.id ?? 'new'} host={host} formId={formId} onSubmit={onSave} />
          {error ? (
            <Alert color="error" title="The host was not saved" role="alert">
              {error.message}
            </Alert>
          ) : null}
        </VFlex>
      </Drawer>
      {host ? (
        <AlertDialog
          open={isConfirmingDelete}
          title={`Delete ${host.label}?`}
          confirmLabel="Delete host"
          cancelLabel="Keep it"
          destructive
          onConfirm={() => {
            setIsConfirmingDelete(false);
            onDelete(host);
          }}
          onCancel={() => setIsConfirmingDelete(false)}
        >
          Sallygate forgets how to reach this server. Open sessions stay open, and the server itself
          is not touched.
        </AlertDialog>
      ) : null}
    </>
  );
}
