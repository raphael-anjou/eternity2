import js from '@eslint/js'
import globals from 'globals'
import reactHooks from 'eslint-plugin-react-hooks'
import reactRefresh from 'eslint-plugin-react-refresh'
import tseslint from 'typescript-eslint'
import { defineConfig, globalIgnores } from 'eslint/config'

export default defineConfig([
  globalIgnores(['dist', 'build', '.react-router']),
  {
    files: ['**/*.{ts,tsx}'],
    extends: [
      js.configs.recommended,
      tseslint.configs.recommended,
      reactHooks.configs.flat.recommended,
      reactRefresh.configs.vite,
    ],
    languageOptions: {
      globals: globals.browser,
    },
  },
  {
    // React Router framework-mode route modules legitimately export `meta`,
    // `links`, `loader`, etc. alongside the default component — the Fast
    // Refresh "only export components" rule does not apply to them.
    //
    // shadcn/ui primitives co-locate their cva `*Variants` builders with the
    // component (the canonical shadcn layout), and src/i18n is a
    // provider+hooks+helpers module rather than a Fast-Refresh component file.
    // Neither is a real hot-reload hazard, so the rule is off for them too.
    files: [
      'src/root.tsx',
      'src/routes.ts',
      'src/layout.tsx',
      'src/pages/**/*.tsx',
      'src/components/ui/**/*.tsx',
      'src/i18n/**/*.tsx',
    ],
    rules: {
      'react-refresh/only-export-components': 'off',
    },
  },
])
