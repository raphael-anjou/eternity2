#!/usr/bin/env node
// Style auditor: flags LLM-writing artifacts in the research wiki prose.
//  - em dashes (—) outside code fences and outside quoted material
//  - a banned-phrase list of recurring tics (EN + FR)
// Verbatim quotes are exempt: text inside "..." / “...” / « ... » on the
// same line is masked before counting.
//
// Run: node web/scripts/check-research-style.mjs [--list]

import { readdirSync, readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const HERE = path.dirname(fileURLToPath(import.meta.url));
const CONTENT = path.join(HERE, "..", "content", "research");
const LIST = process.argv.includes("--list");

const BANNED = [
  /\bhonest(?:ly)?\b/i, // the wiki's verbal crutch; keep only where load-bearing
  /\bSpoiler:/i,
  /\bdeep[- ]dive\b/i,
  /\bdelve\b/i,
  /\bthe beauty of\b/i,
  /\bnot just [^.]{3,40}, (?:it'?s|but)\b/i,
  /\bcrucially\b/i,
  /\bIn essence\b/i,
];

function walk(dir) {
  return readdirSync(dir, { withFileTypes: true }).flatMap((e) => {
    const p = path.join(dir, e.name);
    return e.isDirectory() ? walk(p) : p.endsWith(".mdx") ? [p] : [];
  });
}

let totalDash = 0;
let totalBanned = 0;
const perFile = [];

for (const f of walk(CONTENT)) {
  const rel = path.relative(CONTENT, f);
  const lines = readFileSync(f, "utf8").split("\n");
  let inFence = false;
  let dashes = 0;
  const banned = [];
  lines.forEach((line, i) => {
    if (/^\s*(```|~~~)/.test(line)) {
      inFence = !inFence;
      return;
    }
    if (inFence) return;
    // mask quoted spans (verbatim source material is exempt)
    const masked = line
      .replace(/"[^"]*"/g, '""')
      .replace(/“[^”]*”/g, "“”")
      .replace(/«[^»]*»/g, "«»");
    const d = (masked.match(/—/g) || []).length;
    dashes += d;
    if (LIST && d > 0) console.log(`${rel}:${i + 1}: ${line.trim().slice(0, 100)}`);
    for (const re of BANNED) {
      if (re.test(masked)) banned.push(`${rel}:${i + 1}: ${re} :: ${line.trim().slice(0, 80)}`);
    }
  });
  totalDash += dashes;
  totalBanned += banned.length;
  if (dashes > 0 || banned.length > 0) perFile.push([rel, dashes, banned]);
}

perFile.sort((a, b) => b[1] - a[1]);
console.log("== em dashes (outside code/quotes) per file — top 20");
for (const [rel, d] of perFile.slice(0, 20)) console.log(`  ${String(d).padStart(4)}  ${rel}`);
console.log("\n== banned-phrase hits");
for (const [, , banned] of perFile) for (const b of banned) console.log("  " + b);

console.log(`\nsummary: ${totalDash} em dashes, ${totalBanned} banned-phrase hits`);
process.exit(totalDash > 0 || totalBanned > 0 ? 1 : 0);
