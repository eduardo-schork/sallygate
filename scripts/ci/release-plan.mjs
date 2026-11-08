// Automatic release helper for GitHub Actions (.github/workflows/release.yml).
//   node scripts/ci/release-plan.mjs plan            → decides whether this push releases, and the version
//   node scripts/ci/release-plan.mjs apply <version> → writes the version to Cargo.toml, Cargo.lock,
//                                                       package.json and CHANGELOG.md
//   node scripts/ci/release-plan.mjs notes <version> → prints that version's CHANGELOG notes
//   node scripts/ci/release-plan.mjs updater-manifest <version> <signature file>
//                                                     → prints the latest.json for the app's updater
// No token is used here: this script only reads git history and writes local files.
import { execFileSync } from 'node:child_process';
import { appendFileSync, readFileSync, writeFileSync } from 'node:fs';

import {
  cargoLockWithVersion,
  cargoTomlWithVersion,
  changelogWithVersion,
  isValidVersion,
  NEXT_VERSION_HEADING,
  nextVersion,
  nextVersionItems,
  packageJsonWithVersion,
  updaterManifest,
  versionNotes,
} from './release-rules.mjs';

/** Before the first release, versions count from here: the first feature makes it 0.1.0. */
const FIRST_VERSION_BASE = '0.0.0';

function git(...args) {
  // stderr is kept out of the log: "no tag yet" is an expected answer, not an error.
  return execFileSync('git', args, { encoding: 'utf8', stdio: ['ignore', 'pipe', 'pipe'] }).trim();
}

function lastTag() {
  try {
    return git('describe', '--tags', '--abbrev=0', '--match', 'v*');
  } catch {
    return null;
  }
}

function subjectsSince(tag) {
  const output = git('log', '--format=%s', tag ? `${tag}..HEAD` : 'HEAD');
  return output ? output.split('\n') : [];
}

/** Outputs for the next jobs; printed too, so the run log shows the decision. */
function output(values) {
  for (const [key, value] of Object.entries(values)) {
    console.log(`${key}=${value}`);
    if (process.env.GITHUB_OUTPUT) appendFileSync(process.env.GITHUB_OUTPUT, `${key}=${value}\n`);
  }
}

function plan() {
  const tag = lastTag();
  const current = tag ? tag.slice(1) : FIRST_VERSION_BASE;
  const subjects = subjectsSince(tag);
  const version = nextVersion(current, subjects);
  console.log(`Last release ${tag ?? '(none)'}, ${String(subjects.length)} commit(s) since.`);
  if (!version) {
    console.log('No feat or fix commit since the last release: nothing to release.');
    output({ release: 'false', version: '' });
    return;
  }
  // Fails here, before any build, when the notes were not written.
  if (nextVersionItems(readFileSync('CHANGELOG.md', 'utf8')).length === 0) {
    throw new Error(
      `Version ${version} is due, but CHANGELOG.md has no "${NEXT_VERSION_HEADING}" section with notes ("- …").`,
    );
  }
  output({ release: 'true', version });
}

function update(file, change) {
  writeFileSync(file, change(readFileSync(file, 'utf8')));
}

function apply(version) {
  if (!isValidVersion(version ?? '')) throw new Error(`Invalid version "${version}".`);
  update('CHANGELOG.md', (text) => changelogWithVersion(text, version));
  update('Cargo.toml', (text) => cargoTomlWithVersion(text, version));
  update('Cargo.lock', (text) => cargoLockWithVersion(text, version));
  update('package.json', (text) => packageJsonWithVersion(text, version));
  console.log(
    `Version ${version} written to Cargo.toml, Cargo.lock, package.json and CHANGELOG.md.`,
  );
}

/** Where every release's update package is downloaded from. */
const RELEASES_URL = 'https://github.com/eduardo-schork/sallygate/releases/download';
const UPDATE_PACKAGE = 'Sallygate.app.tar.gz';

function manifest(version, signatureFile) {
  if (!isValidVersion(version ?? '') || !signatureFile) {
    throw new Error('Usage: release-plan.mjs updater-manifest <version> <signature file>');
  }
  console.log(
    updaterManifest({
      version,
      notes: versionNotes(readFileSync('CHANGELOG.md', 'utf8'), version),
      publishedAt: new Date().toISOString(),
      signature: readFileSync(signatureFile, 'utf8'),
      url: `${RELEASES_URL}/v${version}/${UPDATE_PACKAGE}`,
    }).trimEnd(),
  );
}

function notes(version) {
  console.log(versionNotes(readFileSync('CHANGELOG.md', 'utf8'), version));
}

const [command, argument, extra] = process.argv.slice(2);
try {
  if (command === 'plan') plan();
  else if (command === 'apply') apply(argument);
  else if (command === 'notes') notes(argument);
  else if (command === 'updater-manifest') manifest(argument, extra);
  else {
    throw new Error(
      'Usage: release-plan.mjs plan | apply <version> | notes <version> | updater-manifest <version> <signature file>',
    );
  }
} catch (error) {
  console.error(`✖ ${error.message}`);
  process.exit(1);
}
