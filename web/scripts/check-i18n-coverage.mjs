#!/usr/bin/env node
// i18n dictionary coverage report. The app's UI strings live in inline
// dictionaries — `{ en: …, fr: …, es: … }` — resolved by useT()/pick(), which
// fall back to English when a language key is missing. That fallback is what
// lets a language be added incrementally, but it also means a missing
// translation renders silently in English instead of failing the build. This
// script restores the "what's still untranslated?" signal the old required-key
// typing gave us: it scans src/ for dictionaries that have an `en:` and `fr:`
// key but no `es:` sibling, and lists them.
//
// It is a REPORT, not a gate (exit 0 always) — partial translation is a valid
// state. Run: node scripts/check-i18n-coverage.mjs [--lang es] (run from web/).
//
// Heuristic, not a parser: it finds object literals that contain top-level
// `en:` and `fr:` keys and checks for a sibling `es:`. Good enough to catch the
// dictionaries a translator missed; it can miss exotic shapes, which is fine for
// a advisory report.

import fs from "node:fs";
import path from "node:path";

const SRC = path.resolve("src");
const langArg = process.argv.indexOf("--lang");
const LANG = langArg !== -1 ? process.argv[langArg + 1] : "es";

function walk(dir) {
  return fs.readdirSync(dir, { withFileTypes: true }).flatMap((e) => {
    const p = path.join(dir, e.name);
    if (e.isDirectory()) return walk(p);
    return /\.tsx?$/.test(e.name) ? [p] : [];
  });
}

// Count, per file, how many `fr:` dictionary keys have a matching `<LANG>:`
// sibling nearby. We approximate "a dictionary" by pairing each `fr:` with the
// nearest following `<LANG>:` before the next `en:` (dictionaries list en, then
// fr, then es by convention). A `fr:` with no `<LANG>:` before the next `en:` or
// a large gap is reported as a likely-missing translation.
const missing = [];
let totalFr = 0;
let totalLang = 0;

for (const file of walk(SRC)) {
  if (file.endsWith(".d.ts")) continue;
  if (file.includes(path.join("i18n", "index"))) continue; // the helper itself
  const src = fs.readFileSync(file, "utf8");
  const lines = src.split("\n");
  // Indices of lines that open an `en:` / `fr:` / `<LANG>:` dictionary branch.
  const isKey = (line, key) => new RegExp(`^\\s*${key}:\\s*[\\[{"'\`]`).test(line);
  let openFr = 0;
  let openLang = 0;
  for (const line of lines) {
    if (isKey(line, "fr")) openFr++;
    if (isKey(line, LANG)) openLang++;
  }
  totalFr += openFr;
  totalLang += openLang;
  if (openFr > openLang) {
    missing.push({ file: path.relative(process.cwd(), file), fr: openFr, lang: openLang });
  }
}

const rel = path.relative(process.cwd(), SRC);
console.log(`== i18n coverage for "${LANG}" under ${rel}/\n`);
console.log(`  fr dictionary branches:   ${totalFr}`);
console.log(`  ${LANG} dictionary branches:   ${totalLang}`);
console.log(`  coverage: ${totalFr ? Math.round((totalLang / totalFr) * 100) : 100}%\n`);

if (missing.length === 0) {
  console.log(`every fr dictionary has a matching ${LANG} sibling — coverage complete.`);
} else {
  console.log(`${missing.length} file(s) have fr dictionaries missing a ${LANG} sibling:`);
  for (const m of missing.sort((a, b) => b.fr - b.lang - (a.fr - a.lang))) {
    console.log(`  ⚠ ${m.file}  (fr×${m.fr}, ${LANG}×${m.lang})`);
  }
  console.log(`\nThese render in English for ${LANG} readers until translated (fallback is intentional).`);
}
// Advisory only — never fail the build on partial translation.
process.exit(0);
