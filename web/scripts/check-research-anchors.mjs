#!/usr/bin/env node
// Validate that every in-page (#slug) and cross-page (/research/x#slug) anchor
// link in the research MDX resolves to a real heading. The repo's own
// check-research-links.mjs strips #fragments, so broken anchors pass CI silently
// — this fills that gap. Uses the SAME github-slugger the app's rehype-slug and
// content.config extractToc use, so computed slugs match runtime IDs exactly.
//
// Usage: node check-anchors.mjs [--<lang>-only]   (run from web/)
//        e.g. --fr-only, --es-only

import fs from "node:fs";
import path from "node:path";
import GithubSlugger from "github-slugger";

const CONTENT = path.resolve("content/research");
// Non-English languages that ship translated sidecars (mirrors the language
// registry in content.config.ts; a literal here to keep the script dep-free).
const TRANSLATION_LANGS = ["fr", "es"];
const SIDECAR_RE = new RegExp(`\\.(${TRANSLATION_LANGS.join("|")})\\.mdx$`);
// --<lang>-only restricts the check to one language's sidecars.
const onlyLang = TRANSLATION_LANGS.find((l) => process.argv.includes(`--${l}-only`)) ?? null;

/** The language of a content file from its name: a `.<lang>.mdx` sidecar, else
 *  English. */
function fileLang(file) {
  const m = SIDECAR_RE.exec(file);
  return m ? m[1] : "en";
}

function walk(dir) {
  return fs.readdirSync(dir, { withFileTypes: true }).flatMap((e) => {
    const p = path.join(dir, e.name);
    if (e.isDirectory()) return walk(p);
    return e.name.endsWith(".mdx") ? [p] : [];
  });
}

// Strip frontmatter + code fences, then collect heading slugs the way the app does.
function headingSlugs(src) {
  const body = src.replace(/^---\n[\s\S]*?\n---\n/, "");
  const slugger = new GithubSlugger();
  const slugs = new Set();
  let inFence = false;
  for (const line of body.split("\n")) {
    if (/^\s*(```|~~~)/.test(line)) { inFence = !inFence; continue; }
    if (inFence) continue;
    const m = /^(#{1,6})\s+(.+?)\s*#*\s*$/.exec(line);
    if (!m) continue;
    const text = m[2]
      .replace(/\[([^\]]*)\]\([^)]*\)/g, "$1")
      .replace(/[*_`]/g, "")
      .trim();
    slugs.add(slugger.slug(text));
  }
  return slugs;
}

// slug -> Set of heading slugs (page URL keyed by content slug)
const files = walk(CONTENT).filter((f) => !onlyLang || fileLang(f) === onlyLang);
const allFiles = walk(CONTENT);

// Build a map: page slug (language-neutral) -> heading slugs, per language.
function pageSlug(file) {
  const rel = path.relative(CONTENT, file).split(path.sep).join("/");
  return rel.replace(SIDECAR_RE, "").replace(/\.mdx$/, "").replace(/(^|\/)index$/, "$1").replace(/\/$/, "");
}
const headingsByUrlLang = new Map(); // `${lang}:${url}` -> Set<slug>
for (const file of allFiles) {
  const s = pageSlug(file);
  const url = s === "" ? "/research" : `/research/${s}`;
  headingsByUrlLang.set(`${fileLang(file)}:${url}`, headingSlugs(fs.readFileSync(file, "utf8")));
}

let broken = 0;
for (const file of files) {
  const lang = fileLang(file);
  const src = fs.readFileSync(file, "utf8");
  const selfUrl = (() => { const s = pageSlug(file); return s === "" ? "/research" : `/research/${s}`; })();
  const rel = path.relative(CONTENT, file).split(path.sep).join("/");
  // same-page (#slug) and cross-page (/research/x#slug)
  const anchors = [
    ...[...src.matchAll(/\]\(#([^)]+)\)/g)].map((m) => ({ url: selfUrl, frag: m[1] })),
    ...[...src.matchAll(/\]\((\/research\/[a-z0-9/-]*)#([^)]+)\)/g)].map((m) => ({ url: m[1], frag: m[2] })),
    ...[...src.matchAll(/\](\(\/research)#([^)]+)\)/g)].map((m) => ({ url: "/research", frag: m[2] })),
  ];
  for (const a of anchors) {
    const set = headingsByUrlLang.get(`${lang}:${a.url}`) ?? headingsByUrlLang.get(`en:${a.url}`);
    if (!set) { console.log(`? ${rel}: anchor target page not found: ${a.url}#${a.frag}`); broken++; continue; }
    if (!set.has(a.frag)) {
      console.log(`✗ ${rel}: broken anchor ${a.url}#${a.frag}`);
      broken++;
    }
  }
}
if (broken === 0) console.log(`anchors OK — ${files.length} files checked, all #fragments resolve`);
else { console.log(`\n${broken} broken anchor(s)`); process.exit(1); }
