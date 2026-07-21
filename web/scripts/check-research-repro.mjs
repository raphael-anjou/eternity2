#!/usr/bin/env node
// Reproducibility-presence auditor: every result-bearing page must say how it
// can be reproduced or re-verified. Reproducibility rule (see the contribute
// page): a page whose `kind` is a claim about the puzzle — `concept`, `finding`
// or `experiment` — must carry a `repro:` frontmatter block. That block is the
// page's answer to "how do I check this?", and its `kind` field ranges from a
// re-runnable command (`exact`/`seeded`/`stochastic`/`heavy`) down to `prose`,
// which states plainly that the page is expository and reproduces no number of
// its own (the measured figures it cites live on the linked experiment pages).
//
// Hub/index pages (`kind: page`), references, tools, papers and basins are
// exempt: they either carry no standalone claim or are indexes of pages that do.
//
// Only the English source page is audited; the parity check
// (check-research-parity.mjs) already gates that each `.fr`/`.es` sidecar
// mirrors the EN `repro` block, so auditing the twins here would be redundant.
//
// Run: node web/scripts/check-research-repro.mjs
//
// Part of the CI gate: wired into `check:research` and `check:research:ci` (and
// therefore the deploy workflow's `quality` job), alongside the hardware
// auditor it is modeled on.

import { readdirSync, readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const HERE = path.dirname(fileURLToPath(import.meta.url));
const CONTENT = path.join(HERE, "..", "content", "research");

// Kinds that make a standalone claim about the puzzle and therefore owe the
// reader a reproduction path. Everything else (page, reference, tool, paper,
// basin) is exempt.
const CLAIM_KINDS = new Set(["concept", "finding", "experiment"]);

function walk(dir) {
  return readdirSync(dir, { withFileTypes: true }).flatMap((e) => {
    const p = path.join(dir, e.name);
    return e.isDirectory() ? walk(p) : p.endsWith(".mdx") ? [p] : [];
  });
}

/** Only the English source page carries the canonical frontmatter; the `.fr`
 *  and `.es` sidecars are checked for parity by check-research-parity.mjs. */
function isEnglishSource(file) {
  const base = path.basename(file);
  return !/\.(fr|es)\.mdx$/.test(base);
}

/** Slice the frontmatter block (between the first two `---` fences). */
function frontmatter(src) {
  const m = /^---\n([\s\S]*?)\n---/.exec(src);
  return m ? m[1] : "";
}

/** Top-level scalar value for `key:` in a flat YAML block (presence is enough
 *  for `kind:` and `repro:` here — no nested lookups needed). */
function topLevel(fm, key) {
  const re = new RegExp(`^${key}:[ \\t]*(.*)$`, "m");
  const m = re.exec(fm);
  return m ? m[1].trim() : undefined;
}

const problems = [];
let audited = 0;

for (const f of walk(CONTENT)) {
  if (!isEnglishSource(f)) continue;
  const rel = path.relative(CONTENT, f);
  const fm = frontmatter(readFileSync(f, "utf8"));
  const kind = topLevel(fm, "kind");
  if (!CLAIM_KINDS.has(kind)) continue;
  audited++;

  const hasRepro = /^repro:/m.test(fm);
  if (!hasRepro) {
    problems.push(
      `${rel}: kind=${kind} but no \`repro:\` block (a reproduction path is mandatory; use \`repro: { kind: prose }\` for an expository page that reproduces no number of its own)`,
    );
  }
}

if (problems.length) {
  console.log("== reproducibility-presence problems");
  for (const p of problems) console.log("  " + p);
}
console.log(
  `\nsummary: ${audited} claim page(s) audited, ${problems.length} problem(s)`,
);
process.exit(problems.length ? 1 : 0);
