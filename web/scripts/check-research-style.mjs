#!/usr/bin/env node
// Style auditor: flags LLM-writing artifacts in the research wiki prose.
//  - em dashes (—) outside code fences and outside quoted material
//  - a banned-phrase list of recurring tics (EN + FR)
//  - em dashes and the banned-phrase list anywhere in web/src/**/*.{ts,tsx,json}
//    (strings and comments), with the same quotation exemption plus a
//    `style-allow-em-dash` marker
// Verbatim quotes are exempt in the body: text inside "..." / “...” / « ... »
// on the same line is masked before counting. Frontmatter is not: its quoted
// strings (title, description, ...) are the site's own text, so only curly
// quotes and guillemets mask there.
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
  let fmState = lines[0] === "---" ? 1 : 0; // 1 = inside frontmatter, 2 = past it
  let dashes = 0;
  const banned = [];
  lines.forEach((line, i) => {
    const inFront = fmState === 1 && i > 0 && line !== "---";
    if (fmState === 1 && i > 0 && line === "---") fmState = 2;
    if (/^\s*(```|~~~)/.test(line)) {
      inFence = !inFence;
      return;
    }
    if (inFence) return;
    if (/^\s*>/.test(line)) return; // blockquotes hold verbatim source material
    // mask quoted spans (verbatim source material is exempt)
    const masked = (inFront ? line : line.replace(/"[^"]*"/g, '""'))
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

// Second pass: the site's source (web/src/**/*.{ts,tsx,json}). Its strings
// reach the built pages (labels, captions, table cells, metadata), so the same
// no-em-dash rule applies to every line, strings and comments alike. Exempt:
// spans inside curly quotes or guillemets on one line (verbatim quotations,
// as in the MDX pass), and a line whose own text or the line just above
// carries the marker `style-allow-em-dash` (with a reason) for a quotation
// that cannot sit in curly quotes or for code that must match the character.
const SRC = path.join(HERE, "..", "src");
function walkSrc(dir) {
  return readdirSync(dir, { withFileTypes: true }).flatMap((e) => {
    const p = path.join(dir, e.name);
    if (e.isDirectory()) return e.name === "node_modules" ? [] : walkSrc(p);
    return /\.(ts|tsx|json)$/.test(e.name) ? [p] : [];
  });
}
let srcDash = 0;
let srcBanned = 0;
const srcHits = [];
const srcBannedHits = [];
for (const f of walkSrc(SRC)) {
  const rel = path.relative(path.join(HERE, ".."), f);
  const lines = readFileSync(f, "utf8").split("\n");
  lines.forEach((line, i) => {
    if (line.includes("style-allow-em-dash") || (i > 0 && lines[i - 1].includes("style-allow-em-dash"))) return;
    const masked = line.replace(/“[^”]*”/g, "“”").replace(/«[^»]*»/g, "«»");
    for (const re of BANNED) {
      if (re.test(masked)) {
        srcBanned++;
        srcBannedHits.push(`${rel}:${i + 1}: ${re} :: ${line.trim().slice(0, 80)}`);
      }
    }
    if (!line.includes("\u2014")) return;
    const d = (masked.match(/\u2014/g) || []).length;
    if (d > 0) {
      srcDash += d;
      srcHits.push(`${rel}:${i + 1}: ${line.trim().slice(0, 100)}`);
    }
  });
}

perFile.sort((a, b) => b[1] - a[1]);
console.log("== em dashes (outside code/quotes) per file, top 20");
for (const [rel, d] of perFile.slice(0, 20)) console.log(`  ${String(d).padStart(4)}  ${rel}`);
console.log("\n== banned-phrase hits");
for (const [, , banned] of perFile) for (const b of banned) console.log("  " + b);

console.log("\n== banned-phrase hits in web/src (ts, tsx, json)");
for (const h of srcBannedHits) console.log("  " + h);

console.log("\n== em dashes in web/src (ts, tsx, json)");
for (const h of srcHits.slice(0, LIST ? srcHits.length : 40)) console.log("  " + h);
if (!LIST && srcHits.length > 40) console.log(`  ... ${srcHits.length - 40} more (run with --list)`);

console.log(
  `\nsummary: ${totalDash} em dashes, ${totalBanned} banned-phrase hits, ${srcDash} em dashes in web/src, ${srcBanned} banned-phrase hits in web/src`,
);
process.exit(totalDash > 0 || totalBanned > 0 || srcDash > 0 || srcBanned > 0 ? 1 : 0);
