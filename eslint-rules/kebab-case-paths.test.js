import { describe, expect, it } from 'vitest';

import { findKebabCasePathViolations } from './kebab-case-paths.js';

describe('findKebabCasePathViolations', () => {
  it('SHOULD accept project conventions and ecosystem exceptions', () => {
    const violations = findKebabCasePathViolations([
      'packages/react/src/components/icon-button/icon-button.ui.tsx',
      'packages/react/src/components/icon-button/icon-button.stories.tsx',
      'apps/storybook/.storybook/preview.ts',
      'apps/storybook/browser-tests/overlays.browser.ts',
      'docs/component-guidelines.md',
      'packages/react/package.json',
      'README.md',
      'AGENTS.md',
      'packages/react/CHANGELOG.md',
    ]);

    expect(violations).toEqual([]);
  });

  it.each([
    ['PascalCase file', 'packages/react/src/components/button/Button.tsx', 'Button.tsx'],
    [
      'PascalCase directory',
      'packages/react/src/components/IconButton/icon-button.ui.tsx',
      'IconButton',
    ],
    ['camelCase file', 'packages/utilities/src/cssVariable.ts', 'cssVariable.ts'],
    ['camelCase directory', 'packages/tokens/src/fontSizes/font-sizes.ts', 'fontSizes'],
  ])('rejects a %s', (_description, path, invalidSegment) => {
    expect(findKebabCasePathViolations([path])).toEqual([
      expect.objectContaining({ path, segment: invalidSegment }),
    ]);
  });
});
