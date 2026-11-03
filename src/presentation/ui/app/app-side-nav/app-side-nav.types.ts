/** The screens of the Sallygate tab, one per side menu entry. */
export type TAppView = 'hosts' | 'keys' | 'forwarding' | 'snippets' | 'knownHosts';

export type TAppSideNavProps = {
  readonly current: TAppView;
  readonly onNavigate: (view: TAppView) => void;
};
