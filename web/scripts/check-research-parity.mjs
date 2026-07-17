#!/usr/bin/env node
// EN/FR translation-parity auditor. For every `<slug>.fr.mdx`, compare it to its
// English twin `<slug>.mdx` and flag structural drift a translation must not
// introduce — the failures the per-file translate-verify pass can miss because
// it never sees both files side by side at build time:
//
//  - machine frontmatter must match exactly (kind, contribution, rigor, tier,
//    score, order, status, author, topics, related, group, repro, hardware,
//    date). Only title/description/metaDescription/sources[].label are allowed
//    to differ (those are the translated fields).
//  - the ESM import lines must match (a missing island import = a broken page).
//  - the set of JSX component tags used in the body must match (a dropped or
//    renamed <Component> means missing/duplicated content).
//  - the heading count must match (a dropped heading loses content and can
//    strand an anchor; check-research-anchors validates resolution, this
//    validates that the headings still exist).
//
// It also WARNS (non-fatal) when the FR `updated:` is older than the EN twin's,
// i.e. the source changed after the translation — a staleness signal.
//
// Part of the CI gate via check:research / check:research:ci.
// Usage: node web/scripts/check-research-parity.mjs

import fs from "node:fs";
import path from "node:path";
import matter from "gray-matter";

const CONTENT = path.resolve("content/research");

// Frontmatter keys that are TRANSLATED, so they are allowed to differ between
// the EN and FR twin. Everything else must match exactly.
//
// `complexity` is fully reader-facing: .time / .space / .note all render as
// human-readable text (type comment: "KaTeX-free plain text" for humans), and
// in practice they carry prose fragments ("one MIP solve per region", "binary
// assignment variables"), not just formulas — so the whole block is translated.
const TRANSLATABLE_KEYS = new Set([
  "title",
  "description",
  "metaDescription",
  "sources",
  "complexity",
]);

// `stages[]` MIXES identifiers (engine, published — must match) with reader-facing
// prose (does, learns — translated), so it is compared entry-by-entry with the
// prose sub-fields blanked out. (Other object-valued keys are pure machine data
// and compared whole.)
const PROSE_SUBFIELDS = {
  stages: ["does", "learns"],
};

/** Return a copy of a frontmatter value with its known prose sub-fields removed,
 *  so the comparison checks only the machine parts. */
function normalizeForCompare(key, value) {
  const prose = PROSE_SUBFIELDS[key];
  if (!prose || value == null) return value;
  const strip = (obj) => {
    if (obj == null || typeof obj !== "object") return obj;
    const out = Array.isArray(obj) ? [...obj] : { ...obj };
    for (const f of prose) delete out[f];
    return out;
  };
  return Array.isArray(value) ? value.map(strip) : strip(value);
}

function walk(dir) {
  return fs.readdirSync(dir, { withFileTypes: true }).flatMap((e) => {
    const p = path.join(dir, e.name);
    if (e.isDirectory()) return walk(p);
    return e.name.endsWith(".mdx") ? [p] : [];
  });
}

/** Import lines at the top of the MDX (verbatim, order-insensitive). */
function importLines(body) {
  return body
    .split("\n")
    .filter((l) => /^\s*import\s/.test(l))
    .map((l) => l.trim())
    .sort();
}

/** Multiset of JSX component tag names used in the body ( <Foo …> / <Foo/> ),
 *  capitalised tags only (components, not lowercase HTML). Counts occurrences so
 *  a dropped or duplicated island is caught. */
function componentTags(body) {
  const counts = new Map();
  for (const m of body.matchAll(/<([A-Z][A-Za-z0-9]*)[\s/>]/g)) {
    counts.set(m[1], (counts.get(m[1]) ?? 0) + 1);
  }
  return counts;
}

/** Number of markdown headings (##..###### and #), outside code fences. */
function headingCount(body) {
  let inFence = false;
  let n = 0;
  for (const line of body.split("\n")) {
    if (/^\s*(```|~~~)/.test(line)) {
      inFence = !inFence;
      continue;
    }
    if (!inFence && /^#{1,6}\s+\S/.test(line)) n++;
  }
  return n;
}

function stripImportsAndExports(body) {
  // For the heading/tag scan, ignore the ESM plumbing block (imports + inline
  // `export function …` component definitions) so a translated body's headings
  // are compared, not the shared code.
  return body;
}

function eqDeep(a, b) {
  return JSON.stringify(a) === JSON.stringify(b);
}

function multisetEq(a, b) {
  if (a.size !== b.size) return false;
  for (const [k, v] of a) if (b.get(k) !== v) return false;
  return true;
}

const problems = [];
const warnings = [];

const frFiles = walk(CONTENT).filter((f) => f.endsWith(".fr.mdx"));

for (const frPath of frFiles) {
  const rel = path.relative(CONTENT, frPath).split(path.sep).join("/");
  const enPath = frPath.replace(/\.fr\.mdx$/, ".mdx");
  if (!fs.existsSync(enPath)) {
    // check-research/content.config already enforces the EN twin exists; guard anyway.
    problems.push(`${rel}: no English twin (${path.basename(enPath)})`);
    continue;
  }
  const en = matter(fs.readFileSync(enPath, "utf8"));
  const fr = matter(fs.readFileSync(frPath, "utf8"));

  // 1) Machine frontmatter must match on every non-translatable key.
  const keys = new Set([...Object.keys(en.data), ...Object.keys(fr.data)]);
  for (const k of keys) {
    if (TRANSLATABLE_KEYS.has(k)) continue;
    const enV = normalizeForCompare(k, en.data[k]);
    const frV = normalizeForCompare(k, fr.data[k]);
    if (!eqDeep(enV, frV)) {
      problems.push(
        `${rel}: frontmatter "${k}" differs from EN on a non-prose field — must match ` +
          `(EN=${JSON.stringify(enV)} FR=${JSON.stringify(frV)})`,
      );
    }
  }
  // sources[].label is translated, but the url list must match exactly.
  if (Array.isArray(en.data.sources) || Array.isArray(fr.data.sources)) {
    const enUrls = (en.data.sources ?? []).map((s) => s.url);
    const frUrls = (fr.data.sources ?? []).map((s) => s.url);
    if (!eqDeep(enUrls, frUrls)) {
      problems.push(`${rel}: sources[].url list differs from EN (labels may differ, URLs must not)`);
    }
  }

  // 2) Import lines must match.
  const enImports = importLines(en.content);
  const frImports = importLines(fr.content);
  if (!eqDeep(enImports, frImports)) {
    const missing = enImports.filter((l) => !frImports.includes(l));
    const extra = frImports.filter((l) => !enImports.includes(l));
    problems.push(
      `${rel}: import lines differ from EN` +
        (missing.length ? ` — missing: ${missing.join(" | ")}` : "") +
        (extra.length ? ` — extra: ${extra.join(" | ")}` : ""),
    );
  }

  // 3) JSX component-tag multiset must match.
  const enTags = componentTags(stripImportsAndExports(en.content));
  const frTags = componentTags(stripImportsAndExports(fr.content));
  if (!multisetEq(enTags, frTags)) {
    const diff = [];
    for (const k of new Set([...enTags.keys(), ...frTags.keys()])) {
      const a = enTags.get(k) ?? 0;
      const b = frTags.get(k) ?? 0;
      if (a !== b) diff.push(`${k} EN×${a} FR×${b}`);
    }
    problems.push(`${rel}: JSX component usage differs from EN — ${diff.join(", ")}`);
  }

  // 4) Heading count must match.
  const enH = headingCount(en.content);
  const frH = headingCount(fr.content);
  if (enH !== frH) {
    problems.push(`${rel}: heading count differs from EN (EN=${enH} FR=${frH})`);
  }

  // 5) Staleness warning: FR updated older than EN updated.
  const enU = en.data.updated;
  const frU = fr.data.updated;
  if (typeof enU === "string" && typeof frU === "string" && frU < enU) {
    warnings.push(`${rel}: FR updated ${frU} is older than EN ${enU} — translation may be stale`);
  }
}

console.log(`== EN/FR translation parity — ${frFiles.length} translated page(s)\n`);
if (warnings.length) {
  console.log("warnings (non-fatal):");
  for (const w of warnings) console.log(`  ⚠ ${w}`);
  console.log("");
}
if (problems.length) {
  for (const p of problems) console.log(`✗ ${p}`);
  console.log(`\n${problems.length} parity problem(s)`);
  process.exit(1);
}
console.log(`parity OK — every .fr.mdx matches its English twin's structure`);
