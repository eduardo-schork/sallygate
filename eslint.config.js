import js from '@eslint/js';
import prettier from 'eslint-config-prettier';
import jsxA11y from 'eslint-plugin-jsx-a11y';
import react from 'eslint-plugin-react';
import reactHooks from 'eslint-plugin-react-hooks';
import { defineConfig, globalIgnores } from 'eslint/config';
import globals from 'globals';
import tseslint from 'typescript-eslint';

import { NO_HARDCODED_DESIGN_VALUES } from './eslint-rules/no-hardcoded-design-values.js';

const TAURI_IMPORTS = ['@tauri-apps/*'];
const UI_IMPORTS = ['react', 'react-dom', 'styled-components', '@faber-ui/*'];

function restrictImports(message, group) {
  return { 'no-restricted-imports': ['error', { patterns: [{ group, message }] }] };
}

// Layer boundaries from docs/architecture.md. Each block matches one layer, so
// the blocks never overlap and the most specific rule always applies.
const LAYER_BOUNDARIES = [
  {
    files: ['src/**/*.{ts,tsx}'],
    ignores: ['src/modules/*/infrastructure/**', 'src/modules/*/domain/**'],
    rules: restrictImports('Only infrastructure gateways may talk to Tauri.', TAURI_IMPORTS),
  },
  {
    files: ['src/modules/*/domain/**/*.{ts,tsx}'],
    rules: restrictImports('The domain layer is pure: no other layer, React, styling or Tauri.', [
      '**/application',
      '**/application/**',
      '**/infrastructure',
      '**/infrastructure/**',
      '**/presentation',
      '**/presentation/**',
      ...UI_IMPORTS,
      ...TAURI_IMPORTS,
    ]),
  },
  {
    files: ['src/modules/*/application/**/*.{ts,tsx}'],
    rules: restrictImports('The application layer depends only on the domain and its own ports.', [
      '**/infrastructure',
      '**/infrastructure/**',
      '**/presentation',
      '**/presentation/**',
      'styled-components',
      '@faber-ui/*',
      ...TAURI_IMPORTS,
    ]),
  },
  {
    files: ['src/modules/*/presentation/**/*.{ts,tsx}'],
    rules: restrictImports('Components reach the core through application hooks only.', [
      '**/infrastructure',
      '**/infrastructure/**',
      ...TAURI_IMPORTS,
    ]),
  },
];

export default defineConfig(
  globalIgnores(['**/node_modules/**', 'dist/**', 'target/**', 'src-tauri/**', 'crates/**']),
  {
    files: ['**/*.{js,mjs,cjs}'],
    extends: [js.configs.recommended],
    languageOptions: {
      globals: globals.node,
    },
  },
  {
    files: ['**/*.{ts,tsx}'],
    extends: [
      js.configs.recommended,
      tseslint.configs.strictTypeChecked,
      tseslint.configs.stylisticTypeChecked,
    ],
    languageOptions: {
      parserOptions: {
        projectService: true,
        tsconfigRootDir: import.meta.dirname,
      },
      globals: {
        ...globals.browser,
        ...globals.node,
      },
    },
    rules: {
      '@typescript-eslint/consistent-type-definitions': ['error', 'type'],
      '@typescript-eslint/consistent-type-exports': 'error',
      '@typescript-eslint/consistent-type-imports': [
        'error',
        {
          fixStyle: 'inline-type-imports',
          prefer: 'type-imports',
        },
      ],
      '@typescript-eslint/no-confusing-void-expression': [
        'error',
        {
          ignoreArrowShorthand: true,
        },
      ],
      '@typescript-eslint/naming-convention': [
        'error',
        {
          selector: 'typeAlias',
          format: ['PascalCase'],
          custom: {
            regex: '^T[A-Z]',
            match: true,
          },
        },
      ],
      'no-restricted-exports': ['error', { restrictDefaultExports: { direct: true } }],
    },
  },
  {
    files: ['vite.config.ts'],
    rules: {
      'no-restricted-exports': 'off',
    },
  },
  {
    files: ['**/*.tsx'],
    extends: [
      react.configs.flat.recommended,
      react.configs.flat['jsx-runtime'],
      reactHooks.configs.flat.recommended,
      jsxA11y.flatConfigs.recommended,
    ],
    settings: {
      react: {
        version: 'detect',
      },
    },
    rules: {
      'react/prop-types': 'off',
    },
  },
  ...LAYER_BOUNDARIES,
  {
    files: ['src/**/*.{ts,tsx}'],
    ignores: ['**/*.test.{ts,tsx}'],
    plugins: {
      '@faber-ui': {
        rules: {
          'no-hardcoded-design-values': NO_HARDCODED_DESIGN_VALUES,
        },
      },
    },
    rules: {
      '@faber-ui/no-hardcoded-design-values': 'error',
    },
  },
  prettier,
);
