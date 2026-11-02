import { Linter } from 'eslint';
import { describe, expect, it } from 'vitest';

import { NO_HARDCODED_DESIGN_VALUES } from './no-hardcoded-design-values.js';

const RULE_ID = '@faber-ui/no-hardcoded-design-values';

function lint(code) {
  const linter = new Linter();

  return linter.verify(code, {
    languageOptions: {
      ecmaVersion: 'latest',
      sourceType: 'module',
    },
    plugins: {
      '@faber-ui': {
        rules: {
          'no-hardcoded-design-values': NO_HARDCODED_DESIGN_VALUES,
        },
      },
    },
    rules: {
      [RULE_ID]: 'error',
    },
  });
}

describe('no-hardcoded-design-values', () => {
  it.each([
    ['color', '#ffffff'],
    ['HSL color', 'hsl(95 46% 26%)'],
    ['pixel dimension', '2px'],
    ['relative dimension', '0.5em'],
    ['duration', '150ms'],
    ['rotation', '360deg'],
  ])('rejects a hardcoded %s', (_description, value) => {
    const messages = lint(`const styles = css\`outline-offset: ${value};\`;`);

    expect(messages).toEqual([
      expect.objectContaining({
        messageId: 'hardcoded',
        ruleId: RULE_ID,
        severity: 2,
      }),
    ]);
  });

  it.each([
    ['font-weight', 600],
    ['line-height', 1.5],
    ['opacity', 0.5],
    ['z-index', 10],
  ])('rejects a hardcoded unitless %s', (property, value) => {
    const messages = lint(`const styles = css\`${property}: ${value};\`;`);

    expect(messages).toEqual([
      expect.objectContaining({
        messageId: 'hardcoded',
        ruleId: RULE_ID,
        severity: 2,
      }),
    ]);
  });

  it('SHOULD accept token interpolation and universal CSS mechanics', () => {
    const messages = lint(
      'const styles = css`width: 100%; min-width: 0; height: auto; color: transparent; font: inherit; padding: ${SPACINGS.MD};`;',
    );

    expect(messages).toEqual([]);
  });
});
