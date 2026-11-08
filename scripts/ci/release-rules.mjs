// Pure rules of the automatic release (no git, no files): which version comes next and how the
// version files change. Used by scripts/ci/release-plan.mjs; tested in release-rules.test.mjs.

const SEMVER = /^(\d+)\.(\d+)\.(\d+)$/;
const FEATURE = /^feat(\([^)]*\))?!?:/;
const FIX = /^fix(\([^)]*\))?!?:/;
const BREAKING = /^[a-z]+(\([^)]*\))?!:/;
const RELEASE_COMMIT = /^chore: release /;

export const NEXT_VERSION_HEADING = '## Next version';

export function isValidVersion(version) {
  return SEMVER.test(version);
}

/**
 * Next version from the subjects of the commits since the last release (Conventional Commits):
 * feat → minor, fix → patch, "!" → major (minor while the app is 0.x). Other types (docs, chore,
 * refactor, ci…) do not release. Returns null when nothing releases.
 */
export function nextVersion(current, subjects) {
  const match = SEMVER.exec(current);
  if (!match) throw new Error(`Invalid current version "${current}".`);
  const [major, minor, patch] = match.slice(1).map(Number);
  const relevant = subjects.filter((subject) => !RELEASE_COMMIT.test(subject));
  const hasBreaking = relevant.some(
    (subject) => BREAKING.test(subject) && (FEATURE.test(subject) || FIX.test(subject)),
  );
  const hasFeature = relevant.some((subject) => FEATURE.test(subject));
  const hasFix = relevant.some((subject) => FIX.test(subject));
  if (hasBreaking && major > 0) return `${major + 1}.0.0`;
  if (hasBreaking || hasFeature) return `${major}.${minor + 1}.0`;
  if (hasFix) return `${major}.${minor}.${patch + 1}`;
  return null;
}

/** The lines under a `## …` heading that starts with `heading`, up to the next `## `. */
function section(changelog, heading) {
  const lines = changelog.split('\n');
  const start = lines.findIndex((line) => line.startsWith(heading));
  if (start === -1) return [];
  const next = lines.findIndex((line, index) => index > start && line.startsWith('## '));
  return lines.slice(start + 1, next === -1 ? lines.length : next);
}

/** Items ("- …") of the "Next version" section; empty when it is missing or has no items. */
export function nextVersionItems(changelog) {
  return section(changelog, NEXT_VERSION_HEADING).filter((line) => line.startsWith('- '));
}

/** The notes of a released version, for the GitHub Release body. */
export function versionNotes(changelog, version) {
  return section(changelog, `## ${version}`).join('\n').trim();
}

/** "## Next version" (with an optional " — summary") becomes "## 0.6.0" (" — summary"). */
export function changelogWithVersion(changelog, version) {
  const lines = changelog.split('\n');
  const index = lines.findIndex((line) => line.startsWith(NEXT_VERSION_HEADING));
  if (index === -1) {
    throw new Error(
      `CHANGELOG.md has no "${NEXT_VERSION_HEADING}" section. Write the notes before releasing.`,
    );
  }
  lines[index] = `## ${version}${lines[index].slice(NEXT_VERSION_HEADING.length)}`;
  return lines.join('\n');
}

/** The `version` of `[workspace.package]` in the root Cargo.toml. */
export function cargoTomlWithVersion(cargoToml, version) {
  const updated = cargoToml.replace(
    /(\[workspace\.package\][^[]*?\nversion = ")[^"]*(")/,
    `$1${version}$2`,
  );
  if (updated === cargoToml && !cargoToml.includes(`version = "${version}"`)) {
    throw new Error('Cargo.toml has no version under [workspace.package].');
  }
  return updated;
}

/** The versions of the workspace crates (`sallygate*`) in Cargo.lock; dependencies stay. */
export function cargoLockWithVersion(cargoLock, version) {
  return cargoLock.replace(
    /(name = "sallygate(?:-[a-z]+)?"\nversion = ")[^"]*(")/g,
    `$1${version}$2`,
  );
}

export function packageJsonWithVersion(packageJson, version) {
  const manifest = JSON.parse(packageJson);
  manifest.version = version;
  return `${JSON.stringify(manifest, null, 2)}\n`;
}
