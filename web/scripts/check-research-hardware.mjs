#!/usr/bin/env node
// Hardware-disclosure auditor: every experiment page must say what it ran on.
// Reproducibility rule (see the contribute page): a page with `kind: experiment`
// must carry a `hardware:` frontmatter block naming at least the core count, so
// a measured result is never reported without the compute behind it. Hub pages
// (kind: page) and non-experiment kinds are exempt.
//
// It also sanity-checks two honesty invariants on the block:
//  - `measured: true` (the standardized single-core bench) requires cores == 1.
//  - a many-core run (cores > 1) must not claim `measured: true`.
//
// Run: node web/scripts/check-research-hardware.mjs
//
// NOT yet in the CI gate (`check:research`/`check:research:ci`): it stays a
// standalone `npm run check:research:hardware` until the 12 experiment pages
// carry their measured hardware blocks. Wire it into the gate in the same
// commit that adds those blocks, so main never goes red on undocumented runs.

import { readdirSync, readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const HERE = path.dirname(fileURLToPath(import.meta.url));
const CONTENT = path.join(HERE, "..", "content", "research");

function walk(dir) {
  return readdirSync(dir, { withFileTypes: true }).flatMap((e) => {
    const p = path.join(dir, e.name);
    return e.isDirectory() ? walk(p) : p.endsWith(".mdx") ? [p] : [];
  });
}

/** Slice the frontmatter block (between the first two `---` fences). */
function frontmatter(src) {
  const m = /^---\n([\s\S]*?)\n---/.exec(src);
  return m ? m[1] : "";
}

/** Top-level scalar value for `key:` in a YAML block (good enough for the flat
 *  frontmatter here — no nested lookups needed beyond presence of `hardware:`). */
function topLevel(fm, key) {
  const re = new RegExp(`^${key}:[ \\t]*(.*)$`, "m");
  const m = re.exec(fm);
  return m ? m[1].trim() : undefined;
}

/** A value nested one level under `hardware:` (2-space indent). */
function underHardware(fm, key) {
  const start = fm.search(/^hardware:/m);
  if (start < 0) return undefined;
  // Everything from `hardware:` to the next top-level (column-0) key.
  const rest = fm.slice(start).split("\n");
  const body = [rest[0]];
  for (let i = 1; i < rest.length; i++) {
    if (/^\S/.test(rest[i])) break; // next top-level key
    body.push(rest[i]);
  }
  const re = new RegExp(`^\\s+${key}:[ \\t]*(.*)$`, "m");
  const m = re.exec(body.join("\n"));
  return m ? m[1].trim() : undefined;
}

const problems = [];
let experiments = 0;

for (const f of walk(CONTENT)) {
  const rel = path.relative(CONTENT, f);
  const fm = frontmatter(readFileSync(f, "utf8"));
  const kind = topLevel(fm, "kind");
  if (kind !== "experiment") continue;
  experiments++;

  const hasHardware = /^hardware:/m.test(fm);
  if (!hasHardware) {
    problems.push(`${rel}: kind=experiment but no \`hardware:\` block (disclosure is mandatory)`);
    continue;
  }
  const coresRaw = underHardware(fm, "cores");
  if (coresRaw === undefined) {
    problems.push(`${rel}: \`hardware:\` block is missing \`cores:\``);
    continue;
  }
  const cores = Number(coresRaw);
  const measured = underHardware(fm, "measured") === "true";
  if (measured && cores !== 1) {
    problems.push(
      `${rel}: \`measured: true\` is the standardized single-core bench but cores=${cores} (must be 1)`,
    );
  }
  if (!measured && cores === 1) {
    // Not an error — a native single-core run is fine — but worth noting only if
    // it looks like an oversight; we stay silent to avoid nagging.
  }
}

if (problems.length) {
  console.log("== hardware-disclosure problems");
  for (const p of problems) console.log("  " + p);
}
console.log(
  `\nsummary: ${experiments} experiment page(s), ${problems.length} problem(s)`,
);
process.exit(problems.length ? 1 : 0);
