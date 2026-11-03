import type { TImportReport } from './inventory-contract';

function hostCount(count: number): string {
  return count === 1 ? '1 host' : `${String(count)} hosts`;
}

/** One sentence on what an import of `~/.ssh/config` did, for the hosts screen. */
export function describeImport({ imported, alreadySaved, invalid }: TImportReport): string {
  const parts: string[] = [];
  if (imported.length > 0) {
    parts.push(`Imported ${hostCount(imported.length)} into the SSH config group.`);
  } else if (alreadySaved > 0) {
    parts.push('Every host in ~/.ssh/config is already saved.');
  } else if (invalid.length === 0) {
    parts.push('~/.ssh/config has no hosts to import.');
  }
  if (imported.length > 0 && alreadySaved > 0) {
    parts.push(`${hostCount(alreadySaved)} already saved.`);
  }
  if (invalid.length > 0) {
    const reasons = invalid.map(({ alias, reason }) => `${alias} (${reason})`).join(', ');
    parts.push(`Left out: ${reasons}.`);
  }
  return parts.join(' ');
}
