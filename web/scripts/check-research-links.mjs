#!/usr/bin/env node
// Validate every internal link in the research wiki content: markdown links,
// href= attributes in JSX islands, and frontmatter `related` paths must all
// resolve to a real page (MDX slug, topic hub, or a known non-research route).
// Run: node web/scripts/check-research-links.mjs  (or `just research-wiki-check`)

import { readdirSync, readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const HERE = path.dirname(fileURLToPath(import.meta.url));
const WEB = path.join(HERE, "..");
const CONTENT = path.join(WEB, "content", "research");

// Non-research routes that content may link to (from sitemap PAGE_PATHS).
const sitemap = readFileSync(path.join(WEB, "sitemap.config.ts"), "utf8");
const staticPaths = new Set(
  [...sitemap.matchAll(/^\s*"([^"]*)",\s*$/gm)].map((m) => "/" + m[1]).map((p) => (p === "/" ? "/" : p)),
);

function walk(dir) {
  return readdirSync(dir, { withFileTypes: true }).flatMap((e) => {
    const p = path.join(dir, e.name);
    return e.isDirectory() ? walk(p) : p.endsWith(".mdx") ? [p] : [];
  });
}

const files = walk(CONTENT);

// Known research URLs: from MDX slugs + topic hubs + auto-generated researcher
// hubs (both hub families are derived from registries, not backed by MDX).
const known = new Set(staticPaths);
known.add("/research");
known.add("/research/topics");
known.add("/research/people");
known.add("/research/glossary");
// Route-generated pages added by the generated-routes wave (no MDX backs them):
// the reproduce index, the by-contribution index, and one hub per contribution
// kind. Kept in sync with GENERATED_ROUTE_PATHS in content.config.ts.
known.add("/research/build/reproduce");
known.add("/research/lab/experiments/by-contribution");
for (const k of [
  "solver",
  "analysis",
  "reconstruction",
  "theory",
  "method",
  "measurement",
  "tool",
  "exposition",
  "negative",
]) {
  known.add(`/research/lab/experiments/by-contribution/${k}`);
}
for (const f of files) {
  const rel = path.relative(CONTENT, f).split(path.sep).join("/");
  const slug = rel.replace(/\.fr\.mdx$/, "").replace(/\.mdx$/, "").replace(/(^|\/)index$/, "$1").replace(/\/$/, "");
  known.add(slug === "" ? "/research" : `/research/${slug}`);
}
const topicsJson = JSON.parse(readFileSync(path.join(CONTENT, "topics.json"), "utf8"));
for (const t of topicsJson.topics) known.add(`/research/topics/${t.slug}`);
const authorsJson = JSON.parse(readFileSync(path.join(CONTENT, "authors.json"), "utf8"));
for (const a of authorsJson.authors) known.add(`/research/people/${a.slug}`);

let bad = 0;
for (const f of files) {
  const src = readFileSync(f, "utf8");
  const rel = path.relative(WEB, f);
  const links = [
    ...[...src.matchAll(/\]\((\/[^)\s#?]*)[^)]*\)/g)].map((m) => m[1]),
    ...[...src.matchAll(/(?:href|to)=["'{]+(\/[^"'}\s#?]*)/g)].map((m) => m[1]),
    ...[...src.matchAll(/^\s*-\s*(\/research\/[^\s#?]*)\s*$/gm)].map((m) => m[1]),
  ];
  for (const l of links) {
    const clean = l.replace(/\/$/, "");
    // /viewer?... style app links: path part must be known.
    if (!known.has(clean)) {
      console.error(`DEAD LINK  ${rel}  →  ${l}`);
      bad++;
    }
  }
}

// The glossary is a JSON registry, not MDX; validate each term's `see` link too.
const glossaryPath = path.join(CONTENT, "glossary.json");
let glossaryTerms = 0;
try {
  const glossary = JSON.parse(readFileSync(glossaryPath, "utf8"));
  for (const term of glossary.terms ?? []) {
    glossaryTerms++;
    if (!term.see) continue;
    const clean = String(term.see).replace(/\/$/, "");
    if (!known.has(clean)) {
      console.error(`DEAD LINK  glossary.json (${term.term})  →  ${term.see}`);
      bad++;
    }
  }
} catch {
  /* no glossary yet — fine */
}

if (bad > 0) {
  console.error(`\n${bad} dead internal link(s).`);
  process.exit(1);
}
console.log(
  `research wiki links OK — ${files.length} files, ${glossaryTerms} glossary terms, ${known.size} known routes`,
);
