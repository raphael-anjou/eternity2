#!/usr/bin/env node
// Data-quality auditor for the research wiki's citations.
//
//  1. Every groups.io message link is validated against the LOCAL archive
//     (v2/community-exports/messages.jsonl): does the msg_num exist? The
//     report prints author/date/subject so a reviewer can judge whether the
//     cited message plausibly supports the claim.
//  2. Every other external URL is probed (HEAD, then GET on 405) with a
//     timeout; 403/405/429 count as "blocked (manual check)", not dead.
//  3. Pages of kind finding/concept/reference with NO sources at all are
//     flagged as missing-sources candidates.
//
// Run: node web/scripts/check-research-citations.mjs [--no-fetch]

import { readdirSync, readFileSync, existsSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const HERE = path.dirname(fileURLToPath(import.meta.url));
const WEB = path.join(HERE, "..");
const CONTENT = path.join(WEB, "content", "research");
const ARCHIVE = path.resolve(WEB, "../../v2/community-exports/messages.jsonl");
const NO_FETCH = process.argv.includes("--no-fetch");

function walk(dir) {
  return readdirSync(dir, { withFileTypes: true }).flatMap((e) => {
    const p = path.join(dir, e.name);
    return e.isDirectory() ? walk(p) : p.endsWith(".mdx") ? [p] : [];
  });
}

// ---- load archive index ----------------------------------------------------
const archive = new Map();
if (existsSync(ARCHIVE)) {
  for (const line of readFileSync(ARCHIVE, "utf8").split("\n")) {
    if (!line.trim()) continue;
    try {
      const m = JSON.parse(line);
      archive.set(Number(m.msg_num), {
        author: m.name,
        date: (m.created || "").slice(0, 10),
        subject: m.subject,
      });
    } catch {
      /* skip malformed lines */
    }
  }
}
console.log(`archive index: ${archive.size} messages\n`);

// ---- scan content ----------------------------------------------------------
const files = walk(CONTENT);
const msgCites = new Map(); // msgNum -> [files]
const extUrls = new Map(); // url -> [files]
const noSources = [];

for (const f of files) {
  const rel = path.relative(CONTENT, f);
  const src = readFileSync(f, "utf8");
  for (const m of src.matchAll(/groups\.io\/g\/eternity2\/message\/(\d+)/g)) {
    const n = Number(m[1]);
    if (!msgCites.has(n)) msgCites.set(n, []);
    if (!msgCites.get(n).includes(rel)) msgCites.get(n).push(rel);
  }
  for (const m of src.matchAll(/https?:\/\/[^\s"'<>\]]+/g)) {
    let url = m[0].replace(/[.,;:*_]+$/, "");
    // markdown wraps URLs in (...) — strip trailing ')' beyond the balance
    while (url.endsWith(")") && (url.match(/\(/g) || []).length < (url.match(/\)/g) || []).length) {
      url = url.slice(0, -1);
    }
    url = url.replace(/[.,;:*_]+$/, "");
    if (url.includes("groups.io/g/eternity2")) continue;
    if (url.includes("eternity2.dev")) continue;
    if (!extUrls.has(url)) extUrls.set(url, []);
    if (!extUrls.get(url).includes(rel)) extUrls.get(url).push(rel);
  }
  // missing sources check (EN files only; FR mirrors EN)
  if (!rel.endsWith(".fr.mdx")) {
    const fm = src.split("---")[1] ?? "";
    const kind = /kind:\s*(\w+)/.exec(fm)?.[1];
    const hasSources = /sources:\s*\n\s+-/.test(fm);
    const hasRepro = /repro:\s*\n/.test(fm); // computed-from link renders from repro.topic
    const hasInlineCite =
      /groups\.io\/g\/eternity2\/message|doi\.org|arxiv\.org|github\.com/.test(src);
    if (["finding", "concept", "reference"].includes(kind) && !hasSources && !hasRepro && !hasInlineCite) {
      noSources.push(rel);
    }
  }
}

// ---- 1. archive validation -------------------------------------------------
let missing = 0;
const POST_EXPORT_OK = new Set([11905]); // provided firsthand by the user; post-dates the export
console.log(`== groups.io citations: ${msgCites.size} distinct messages cited`);
for (const [n, where] of [...msgCites.entries()].sort((a, b) => a[0] - b[0])) {
  if (!archive.has(n)) {
    if (POST_EXPORT_OK.has(n)) continue;
    console.log(`  MISSING msg ${n}  (cited in: ${where.join(", ")})`);
    missing++;
  }
}
if (missing === 0) console.log("  all cited messages exist in the archive ✓");

// ---- 2. external URL probing -----------------------------------------------
let dead = 0;
if (!NO_FETCH) {
  console.log(`\n== external URLs: ${extUrls.size} distinct`);
  const urls = [...extUrls.keys()];
  const results = [];
  const CONCURRENCY = 4;
  async function probe(url) {
    const ctrl = new AbortController();
    const t = setTimeout(() => ctrl.abort(), 12000);
    try {
      let res = await fetch(url, { method: "HEAD", redirect: "follow", signal: ctrl.signal });
      if (res.status === 405 || res.status === 501) {
        res = await fetch(url, { method: "GET", redirect: "follow", signal: ctrl.signal });
      }
      return res.status;
    } catch {
      return 0;
    } finally {
      clearTimeout(t);
    }
  }
  let i = 0;
  await Promise.all(
    Array.from({ length: CONCURRENCY }, async () => {
      while (i < urls.length) {
        const url = urls[i++];
        const status = await probe(url);
        results.push([url, status]);
      }
    }),
  );
  for (const [url, status] of results.sort()) {
    if (status >= 200 && status < 400) continue;
    const blocked = [401, 402, 403, 405, 429, 999].includes(status);
    const tag = blocked ? "BLOCKED(manual)" : status === 0 ? "UNREACHABLE" : `HTTP ${status}`;
    if (!blocked) dead++;
    console.log(`  ${tag}  ${url}\n           in: ${extUrls.get(url).join(", ")}`);
  }
  if (dead === 0) console.log("  no dead external URLs (blocked ones need manual review) ✓");
}

// ---- 3. missing sources ----------------------------------------------------
console.log(`\n== pages with no sources at all (kind finding/concept/reference):`);
if (noSources.length === 0) console.log("  none ✓");
for (const f of noSources) console.log(`  ${f}`);

console.log(
  `\nsummary: ${files.length} files, ${msgCites.size} msg citations (${missing} missing), ` +
    `${extUrls.size} external urls (${dead} dead), ${noSources.length} no-source pages`,
);
process.exit(missing > 0 || dead > 0 ? 1 : 0);
