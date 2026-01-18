import js from '@eslint/js';
import typescript from '@typescript-eslint/eslint-plugin';
import typescriptParser from '@typescript-eslint/parser';
import globals from 'globals';

export default [
  js.configs.recommended,
  {
    files: ['**/*.{ts,tsx,js,jsx}'],
    languageOptions: {
      ecmaVersion: 2022,
      sourceType: 'module',
      globals: {
        ...globals.browser,
        ...globals.node,
        ...globals.es2022,
        'invoke': 'readonly',
        'getCurrentWindow': 'readonly'
      },
      parser: typescriptParser
    },
    plugins: {
      '@typescript-eslint': typescript
    },
    rules: {
      ...typescript.configs.recommended.rules,
      '@typescript-eslint/no-unused-vars': ['error', { argsIgnorePattern: '^_' }],
      '@typescript-eslint/no-explicit-any': 'warn',
      'no-console': 'warn',
      'prefer-const': 'error'
    }
  },
  {
    ignores: [
      'build/',
      '.svelte-kit/',
      'dist/',
      'node_modules/',
      'src-tauri/target/',
      '*.config.js',
      '*.config.ts',
      '**/*.svelte'
    ]
  }
];