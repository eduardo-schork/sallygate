import { describe, expect, it } from 'vitest';

import {
  cargoLockWithVersion,
  cargoTomlWithVersion,
  changelogWithVersion,
  nextVersion,
  nextVersionItems,
  packageJsonWithVersion,
  versionNotes,
} from './release-rules.mjs';

describe('nextVersion', () => {
  it('raises the minor for a feature and the patch for a fix', () => {
    expect(nextVersion('0.5.0', ['fix: typo', 'feat(ui): split view', 'docs: plan'])).toBe('0.6.0');
    expect(nextVersion('0.5.0', ['fix(sessions): resize', 'chore: deps'])).toBe('0.5.1');
  });

  it('does not release for docs, chores, ci or its own release commit', () => {
    expect(nextVersion('0.5.0', ['docs: plan', 'ci: cache', 'refactor: names'])).toBeNull();
    expect(nextVersion('0.6.0', ['chore: release 0.6.0'])).toBeNull();
    expect(nextVersion('0.5.0', [])).toBeNull();
  });

  it('treats a breaking change as minor while the app is 0.x, major afterwards', () => {
    expect(nextVersion('0.5.0', ['feat!: new hosts format'])).toBe('0.6.0');
    expect(nextVersion('1.2.3', ['fix!: drop old files'])).toBe('2.0.0');
  });

  it('ignores words that only look like a type', () => {
    expect(nextVersion('0.5.0', ['featured list tweak', 'fixup wording'])).toBeNull();
  });
});

describe('changelog', () => {
  const changelog = [
    '# Changelog',
    '',
    '## Next version',
    '',
    '- Split view',
    '',
    '## 0.1.0',
    '',
    '- First',
    '',
  ].join('\n');

  it('turns the next version heading into the version and reads its notes back', () => {
    const released = changelogWithVersion(changelog, '0.2.0');
    expect(released).toContain('## 0.2.0\n\n- Split view');
    expect(released).not.toContain('Next version');
    expect(versionNotes(released, '0.2.0')).toBe('- Split view');
  });

  it('lists the items of the next version only, and refuses to release without them', () => {
    expect(nextVersionItems(changelog)).toEqual(['- Split view']);
    expect(nextVersionItems('# Changelog\n\n## 0.1.0\n\n- First\n')).toEqual([]);
    expect(() => changelogWithVersion('# Changelog\n\n## 0.1.0\n', '0.2.0')).toThrow(
      /Next version/,
    );
  });
});

describe('version files', () => {
  it('sets the workspace version in Cargo.toml only', () => {
    const cargoToml =
      '[workspace]\nmembers = []\n\n[workspace.package]\nversion = "0.1.0-dev"\nedition = "2024"\n\n[dependencies]\nfoo = { version = "1" }\n';
    const updated = cargoTomlWithVersion(cargoToml, '0.2.0');
    expect(updated).toContain('[workspace.package]\nversion = "0.2.0"');
    expect(updated).toContain('foo = { version = "1" }');
  });

  it('sets the workspace crates in Cargo.lock and leaves dependencies alone', () => {
    const lock =
      '[[package]]\nname = "sallygate-domain"\nversion = "0.1.0-dev"\n\n[[package]]\nname = "serde"\nversion = "1.0.0"\n\n[[package]]\nname = "sallygate"\nversion = "0.1.0-dev"\n';
    const updated = cargoLockWithVersion(lock, '0.2.0');
    expect(updated).toContain('name = "sallygate-domain"\nversion = "0.2.0"');
    expect(updated).toContain('name = "sallygate"\nversion = "0.2.0"');
    expect(updated).toContain('name = "serde"\nversion = "1.0.0"');
  });

  it('sets the version in package.json', () => {
    const manifest = JSON.stringify({ name: 'sallygate', version: '0.1.0-dev' }, null, 2);
    expect(JSON.parse(packageJsonWithVersion(manifest, '0.2.0')).version).toBe('0.2.0');
  });
});
