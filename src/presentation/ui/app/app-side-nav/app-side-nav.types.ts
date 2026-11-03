/** The screens of the Sallygate tab, one per side menu entry. */
export type TAppView = 'hosts' | 'keys' | 'forwarding' | 'snippets' | 'knownHosts';

export type TAppSideNavProps = {
  /** Missing while a session is in front: no screen of the menu is shown then. */
  readonly current: TAppView | undefined;
  readonly onNavigate: (view: TAppView) => void;
};
