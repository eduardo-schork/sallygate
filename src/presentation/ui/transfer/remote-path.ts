/** The folder that holds `path`; the root stays the root. */
export function parentFolder(path: string): string {
  const trimmed = path.replace(/\/+$/u, '');
  const cut = trimmed.lastIndexOf('/');
  return cut <= 0 ? '/' : trimmed.slice(0, cut);
}

/** The last part of a local or remote path. */
export function baseName(path: string): string {
  return (
    path
      .replace(/[/\\]+$/u, '')
      .split(/[/\\]/u)
      .pop() ?? path
  );
}
