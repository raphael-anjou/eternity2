#!/usr/bin/env node
// Scan research/topics/*/article.md, parse each file's YAML frontmatter, and
// emit research/index.json — the registry the website reads. Run after adding or
// editing a topic:
//
//   node research/build-index.mjs
//
// Validates that each topic's frontmatter `id` matches its directory name (so
// the registry can never reference a missing directory) and that any declared
// result / site data files exist on disk. Exits non-zero on any problem
// (usable as a CI gate).
//
// Frontmatter is parsed by a tiny dependency-free reader supporting exactly the
// subset this schema uses: top-level scalars, nested maps (one level), block
// sequences of scalars, and block sequences of maps. No external YAML dep, so
// the static site build stays dependency-free — the site reads index.json, not
// the frontmatter.

import { readdirSync, readFileSync, writeFileSync, existsSync, statSync } from "node:fs";
import { join, dirname } from "node:path";
import { fileURLToPath } from "node:url";

const ROOT = dirname(fileURLToPath(import.meta.url));
const TOPICS = join(ROOT, "topics");

// ── minimal frontmatter / YAML-subset parser ───────────────────────────────
function extractFrontmatter(text) {
  const m = text.match(/^---\r?\n([\s\S]*?)\r?\n---\r?\n?/);
  if (!m) return null;
  return m[1];
}

function scalar(v) {
  v = v.trim();
  if (v === "" || v === "null" || v === "~") return null;
  if (v === "true") return true;
  if (v === "false") return false;
  if (/^-?\d+$/.test(v)) return Number(v);
  if ((v.startsWith('"') && v.endsWith('"')) || (v.startsWith("'") && v.endsWith("'"))) {
    return v.slice(1, -1);
  }
  return v;
}

// Parse the subset. Returns a plain object. Throws on shapes outside the subset.
function parseFrontmatter(src) {
  // Drop comment-only and blank lines; track indentation.
  const lines = src
    .split(/\r?\n/)
    .filter((l) => l.trim() !== "" && !/^\s*#/.test(l));

  const root = {};
  let i = 0;

  function indentOf(l) {
    return l.match(/^ */)[0].length;
  }

  function parseBlock(baseIndent) {
    // Returns either an object (mapping) or array (sequence) at this indent.
    const isSeq = lines[i] !== undefined && lines[i].slice(baseIndent).startsWith("- ");
    if (isSeq) {
      const arr = [];
      while (i < lines.length && indentOf(lines[i]) === baseIndent && lines[i].slice(baseIndent).startsWith("- ")) {
        const rest = lines[i].slice(baseIndent + 2);
        if (rest.includes(":") && !/^https?:/.test(rest)) {
          // sequence of maps: first key inline, remaining keys indented under it
          const obj = {};
          const [k, ...vp] = rest.split(":");
          obj[k.trim()] = scalar(vp.join(":"));
          i++;
          const childIndent = baseIndent + 2;
          while (i < lines.length && indentOf(lines[i]) === childIndent && !lines[i].slice(childIndent).startsWith("- ")) {
            const line = lines[i].slice(childIndent);
            const ci = line.indexOf(":");
            obj[line.slice(0, ci).trim()] = scalar(line.slice(ci + 1));
            i++;
          }
          arr.push(obj);
        } else {
          arr.push(scalar(rest));
          i++;
        }
      }
      return arr;
    }
    // mapping
    const obj = {};
    while (i < lines.length && indentOf(lines[i]) === baseIndent) {
      const line = lines[i].slice(baseIndent);
      const ci = line.indexOf(":");
      if (ci === -1) throw new Error(`bad frontmatter line: ${lines[i]}`);
      const key = line.slice(0, ci).trim();
      const after = line.slice(ci + 1).trim();
      i++;
      if (after === "") {
        // nested block (map or sequence) at deeper indent
        if (i < lines.length && indentOf(lines[i]) > baseIndent) {
          obj[key] = parseBlock(indentOf(lines[i]));
        } else {
          obj[key] = null;
        }
      } else {
        obj[key] = scalar(after);
      }
    }
    return obj;
  }

  Object.assign(root, parseBlock(0));
  return root;
}

// ── scan topics ─────────────────────────────────────────────────────────────
const errors = [];
const topics = [];

if (!existsSync(TOPICS)) {
  console.error("no topics/ directory yet — nothing to index");
  writeFileSync(join(ROOT, "index.json"), JSON.stringify({ count: 0, topics: [] }, null, 2) + "\n");
  process.exit(0);
}

for (const dir of readdirSync(TOPICS).sort()) {
  const topicDir = join(TOPICS, dir);
  if (!statSync(topicDir).isDirectory()) continue;
  const articlePath = join(topicDir, "article.md");
  if (!existsSync(articlePath)) {
    errors.push(`${dir}: missing article.md`);
    continue;
  }
  const fm = extractFrontmatter(readFileSync(articlePath, "utf8"));
  if (!fm) {
    errors.push(`${dir}: article.md has no YAML frontmatter (--- block)`);
    continue;
  }
  let meta;
  try {
    meta = parseFrontmatter(fm);
  } catch (e) {
    errors.push(`${dir}: frontmatter parse error — ${e.message}`);
    continue;
  }

  if (meta.id !== dir) {
    errors.push(`${dir}: frontmatter id "${meta.id}" must equal the directory name "${dir}"`);
  }
  for (const field of ["title", "summary", "status", "created"]) {
    if (!meta[field]) errors.push(`${dir}: frontmatter missing required field "${field}"`);
  }
  if (meta.status && !["draft", "published"].includes(meta.status)) {
    errors.push(`${dir}: status must be "draft" or "published" (got "${meta.status}")`);
  }
  for (const r of meta.results ?? []) {
    if (r.path && !existsSync(join(topicDir, r.path))) {
      errors.push(`${dir}: declared result file "${r.path}" does not exist`);
    }
  }
  if (meta.site?.render && meta.site?.dataFile && !existsSync(join(topicDir, meta.site.dataFile))) {
    errors.push(`${dir}: site.dataFile "${meta.site.dataFile}" does not exist`);
  }

  topics.push({
    id: meta.id,
    title: meta.title,
    summary: meta.summary,
    status: meta.status,
    created: meta.created,
    updated: meta.updated ?? meta.created,
    contributors: meta.contributors ?? [],
    tags: meta.tags ?? [],
    sources: meta.sources ?? [],
    site: meta.site ?? { render: false },
    path: `research/topics/${dir}`,
    article: `research/topics/${dir}/article.md`,
  });
}

if (errors.length) {
  console.error("research index build FAILED:");
  for (const e of errors) console.error("  - " + e);
  process.exit(1);
}

const index = { count: topics.length, topics };
writeFileSync(join(ROOT, "index.json"), JSON.stringify(index, null, 2) + "\n");
console.log(`research index OK — ${topics.length} topic(s)`);
