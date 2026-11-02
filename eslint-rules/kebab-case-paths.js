const ALLOWED_FILE_NAMES = new Set([
  '.bun-version',
  '.editorconfig',
  '.env.example',
  '.gitignore',
  '.prettierignore',
  'AGENTS.md',
  'CHANGELOG.md',
  'LICENSE',
  'OFL.txt',
  'PROJECT.md',
  'README.md',
  'SECURITY.md',
  'bun.lock',
  'eslint.config.js',
  'package.json',
  'prettier.config.mjs',
  'tsconfig.base.json',
  'tsconfig.build.json',
  'tsconfig.json',
  'turbo.json',
  'vite.config.ts',
  'vitest.config.ts',
  'vitest.setup.ts',
]);

const KEBAB_CASE_SEGMENT_PATTERN = /^[a-z0-9]+(?:-[a-z0-9]+)*$/u;
const HIDDEN_DIRECTORY_PATTERN = /^\.[a-z0-9]+(?:-[a-z0-9]+)*$/u;
const KEBAB_CASE_FILE_PATTERN =
  /^[a-z0-9]+(?:-[a-z0-9]+)*(?:\.(?:browser|build|config|constants|d|stories|styles|test|types|ui))*\.[a-z0-9]+$/u;

export function findKebabCasePathViolations(paths) {
  const violations = [];

  for (const originalPath of paths) {
    const normalizedPath = originalPath.replaceAll('\\', '/').replace(/^\.\//u, '');
    const segments = normalizedPath.split('/');
    const fileName = segments.pop();

    for (const directoryName of segments) {
      if (
        !KEBAB_CASE_SEGMENT_PATTERN.test(directoryName) &&
        !HIDDEN_DIRECTORY_PATTERN.test(directoryName)
      ) {
        violations.push({ path: normalizedPath, segment: directoryName, type: 'directory' });
      }
    }

    if (fileName && !ALLOWED_FILE_NAMES.has(fileName) && !KEBAB_CASE_FILE_PATTERN.test(fileName)) {
      violations.push({ path: normalizedPath, segment: fileName, type: 'file' });
    }
  }

  return violations;
}
