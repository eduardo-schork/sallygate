export type TNameDialogProps = {
  readonly open: boolean;
  readonly title: string;
  /** The name the field starts with, e.g. the current name when renaming. */
  readonly initialName?: string;
  readonly submitLabel: string;
  readonly onSubmit: (name: string) => void;
  readonly onCancel: () => void;
};
