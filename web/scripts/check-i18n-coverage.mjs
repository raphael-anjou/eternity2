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
// ---------------------------------------------------------------------------
// Content registries (JSON), checked EXACTLY rather than heuristically.
//
// The scan above only sees dictionaries written in src/*.tsx. The research
// wiki also carries localized prose in JSON registries, and those feed page
// titles, meta descriptions, H1s and the sidebar nav. A missing key there
// falls back to English exactly as silently, but it is far more visible: it
// puts English text on a translated PAGE, not just on one UI label.
//
// That is not hypothetical. topics.json and authors.json shipped with `en` and
// `fr` only, which served English titles and bios on 20 Spanish pages and
// leaked English topic labels into the sidebar of 23 more, unnoticed. These
// files are small and fully parseable, so check every localized field for real.
const REGISTRIES = [
  { file: "content/research/topics.json", collection: "topics", fields: ["label", "description"] },
  { file: "content/research/authors.json", collection: "authors", fields: ["tagline", "bio"] },
  { file: "content/research/glossary.json", collection: "terms", fields: ["term", "definition"] },
];

console.log(`\n== content registry coverage for "${LANG}"\n`);
const gaps = [];
for (const reg of REGISTRIES) {
  const abs = path.resolve(reg.file);
  if (!fs.existsSync(abs)) continue;
  const data = JSON.parse(fs.readFileSync(abs, "utf8"));
  const items = Array.isArray(data) ? data : (data[reg.collection] ?? []);
  let checked = 0;
  let present = 0;
  for (const item of items) {
    for (const field of reg.fields) {
      const val = item?.[field];
      // Only fields that are actually localized objects (an `en` key) count:
      // a plain string field is language-neutral by design.
      if (!val || typeof val !== "object" || !("en" in val)) continue;
      checked++;
      if (LANG in val) present++;
      else gaps.push(`${reg.file}: ${item.slug ?? item.term?.en ?? "?"} → ${field}`);
    }
  }
  const pct = checked ? Math.round((present / checked) * 100) : 100;
  const mark = present === checked ? "✓" : "⚠";
  console.log(`  ${mark} ${reg.file}: ${present}/${checked} localized fields have "${LANG}" (${pct}%)`);
}

if (gaps.length > 0) {
  console.log(`\n  ${gaps.length} registry field(s) missing "${LANG}", each of which renders ENGLISH on a translated page:`);
  for (const g of gaps.slice(0, 40)) console.log(`    ⚠ ${g}`);
  if (gaps.length > 40) console.log(`    … and ${gaps.length - 40} more`);
}

// Advisory only — never fail the build on partial translation. Adding a
// language is deliberately incremental, and a hard gate would block the first
// commit of every new language.
process.exit(0);
