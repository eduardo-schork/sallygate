import { execFileSync } from 'node:child_process';

import { findKebabCasePathViolations } from '../eslint-rules/kebab-case-paths.js';

// Rust code follows snake_case, so only the TypeScript side is checked.
const KEBAB_CASE_ROOTS = ['src/presentation/ui/', 'scripts/', 'eslint-rules/'];

const gitOutput = execFileSync(
  'git',
  ['ls-files', '--cached', '--others', '--exclude-standard', '-z'],
  { encoding: 'utf8' },
);
const checkedPaths = gitOutput
  .split('\0')
  .filter((path) => KEBAB_CASE_ROOTS.some((root) => path.startsWith(root)));
const violations = findKebabCasePathViolations(checkedPaths);

if (violations.length > 0) {
  for (const violation of violations) {
    console.error(
      `${violation.path}: ${violation.type} segment "${violation.segment}" must use kebab case.`,
    );
  }

  process.exitCode = 1;
}
