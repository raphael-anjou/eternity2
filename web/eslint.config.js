import js from '@eslint/js'
import globals from 'globals'
import reactHooks from 'eslint-plugin-react-hooks'
import reactRefresh from 'eslint-plugin-react-refresh'
import tseslint from 'typescript-eslint'
import { defineConfig, globalIgnores } from 'eslint/config'

export default defineConfig([
  // The generated wasm-bindgen glue is machine-authored (and full of `any` by
  // design); never lint it. Everything else is hand-written and must comply.
  globalIgnores(['dist', 'build', '.react-router', 'src/engine/pkg']),
  {
    files: ['**/*.{ts,tsx}'],
    extends: [
      js.configs.recommended,
      // Type-checked rules (not just syntactic): this is what actually
      // enforces TypeScript discipline — no implicit/explicit `any` leaking
      // through, no unsafe member access on untyped values, etc.
      tseslint.configs.recommendedTypeChecked,
      reactHooks.configs.flat.recommended,
      reactRefresh.configs.vite,
    ],
    languageOptions: {
      globals: globals.browser,
      parserOptions: {
        // Wire the type-checked rules to the project's tsconfigs.
        projectService: true,
        tsconfigRootDir: import.meta.dirname,
      },
    },
    rules: {
      // Ban the escape hatches that would let untyped code back in. These turn
      // the type system from "advisory" into "enforced".
      '@typescript-eslint/no-explicit-any': 'error',
      '@typescript-eslint/no-non-null-assertion': 'error',
      '@typescript-eslint/ban-ts-comment': [
        'error',
        { 'ts-ignore': true, 'ts-expect-error': 'allow-with-description', 'ts-nocheck': true },
      ],
      '@typescript-eslint/consistent-type-imports': 'error',
      // Surface floating promises and misused promises (common React footguns).
      '@typescript-eslint/no-floating-promises': 'error',
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
