// Node-side scanner for the research wiki content (web/content/research).
// Single source of truth for what MDX pages exist: the Vite plugin builds the
// client manifest from it, and sitemap.config.ts derives the prerender +
// sitemap paths from it. Frontmatter is zod-validated and fails the build
// loudly — a page with broken metadata never ships silently.
//
// File conventions:
//   content/research/<slug>.mdx      → English page at /research/<slug>
//   content/research/<slug>.fr.mdx   → French translation (optional; missing
//                                      translations fall back to EN with a
//                                      notice, so FR never 404s)

import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import matter from "gray-matter";
import GithubSlugger from "github-slugger";
import { z } from "zod";
import type { ResearchDoc, SearchEntry, TocItem } from "./src/lib/research/types";

const HERE = path.dirname(fileURLToPath(import.meta.url));
export const CONTENT_DIR = path.join(HERE, "content", "research");
const TOPICS_FILE = path.join(CONTENT_DIR, "topics.json");
const AUTHORS_FILE = path.join(CONTENT_DIR, "authors.json");

// ---- Topic registry ------------------------------------------------------

const topicSchema = z.object({
  slug: z.string().regex(/^[a-z0-9-]+$/),
  order: z.number(),
  label: z.object({ en: z.string(), fr: z.string() }),
  description: z.object({ en: z.string(), fr: z.string() }),
});

export type TopicDef = z.infer<typeof topicSchema>;

let topicsCache: TopicDef[] | null = null;

/** The curated topic registry (content/research/topics.json), validated. */
export function researchTopics(fresh = false): TopicDef[] {
  if (topicsCache && !fresh) return topicsCache;
  if (!fs.existsSync(TOPICS_FILE)) return (topicsCache = []);
  const raw: unknown = JSON.parse(fs.readFileSync(TOPICS_FILE, "utf8"));
  const parsed = z.object({ topics: z.array(topicSchema) }).safeParse(raw);
  if (!parsed.success) {
    throw new Error(
      `research content: invalid topics.json:\n` +
        parsed.error.issues.map((i) => `  ${i.path.join(".")}: ${i.message}`).join("\n"),
    );
  }
  topicsCache = [...parsed.data.topics].sort((a, b) => a.order - b.order);
  return topicsCache;
}

// ---- Author registry -----------------------------------------------------

const authorSchema = z.object({
  slug: z.string().regex(/^[a-z0-9-]+$/),
  name: z.string().min(1),
  tagline: z.object({ en: z.string(), fr: z.string() }).optional(),
  affiliation: z.object({ en: z.string(), fr: z.string() }).optional(),
  bio: z.object({ en: z.string(), fr: z.string() }).optional(),
  links: z.array(z.object({ label: z.string(), url: z.string().url() })).default([]),
});

export type AuthorDef = z.infer<typeof authorSchema>;

let authorsCache: AuthorDef[] | null = null;

/** The curated author registry (content/research/authors.json), validated.
 *  Profiles only; the list of each person's pages is derived from the `author`
 *  frontmatter field, never stored here. */
export function researchAuthors(fresh = false): AuthorDef[] {
  if (authorsCache && !fresh) return authorsCache;
  if (!fs.existsSync(AUTHORS_FILE)) return (authorsCache = []);
  const raw: unknown = JSON.parse(fs.readFileSync(AUTHORS_FILE, "utf8"));
  const parsed = z.object({ authors: z.array(authorSchema) }).safeParse(raw);
  if (!parsed.success) {
    throw new Error(
      `research content: invalid authors.json:\n` +
        parsed.error.issues.map((i) => `  ${i.path.join(".")}: ${i.message}`).join("\n"),
    );
  }
  authorsCache = [...parsed.data.authors].sort((a, b) => a.name.localeCompare(b.name));
  return authorsCache;
}

const reproSchema = z.object({
  kind: z.enum(["exact", "seeded", "stochastic", "heavy", "prose"]),
  cmd: z.string().optional(),
  topic: z.string().optional(),
});

const complexitySchema = z.object({
  time: z.string().optional(),
  space: z.string().optional(),
  note: z.string().optional(),
});

const frontmatterSchema = z.object({
  title: z.string().min(1),
  description: z.string().min(1),
  kind: z
    .enum(["finding", "experiment", "tool", "reference", "concept", "basin", "paper", "page"])
    .default("page"),
  status: z.enum(["live", "draft"]).default("live"),
  // Registry slug of the researcher who authored the page. Validated against
  // authors.json in scanResearchContent (below), like topics.
  author: z.string().optional(),
  tier: z.number().int().min(1).max(3).optional(),
  rigor: z.enum(["proven", "measured", "conjectured"]).optional(),
  complexity: complexitySchema.optional(),
  score: z.number().int().optional(),
  // YAML parses a bare 2026-07-01 as a Date; accept both, normalize to string.
  updated: z
    .union([
      z.string().regex(/^\d{4}-\d{2}-\d{2}$/, "updated must be YYYY-MM-DD"),
      z.date().transform((d) => d.toISOString().slice(0, 10)),
    ])
    .optional(),
  repro: reproSchema.optional(),
  sources: z.array(z.object({ label: z.string(), url: z.string().url() })).default([]),
  topics: z.array(z.string()).default([]),
  related: z.array(z.string().startsWith("/")).default([]),
  order: z.number().optional(),
  // Named sub-section within a sidebar section (e.g. "Reduce the search" inside
  // Build a solver). Groups the flat page list into the approaches-map spine.
  group: z.string().optional(),
});

export type Lang = "en" | "fr";

interface RawEntry {
  slug: string;
  lang: Lang;
  file: string; // content-dir-relative
  fm: z.infer<typeof frontmatterSchema>;
  toc: TocItem[];
  body: string; // markdown body (frontmatter stripped) — used by the search index
}

/** Extract h2/h3 headings from markdown source, skipping fenced code blocks. */
function extractToc(body: string): TocItem[] {
  const slugger = new GithubSlugger();
  const toc: TocItem[] = [];
  let inFence = false;
  for (const line of body.split("\n")) {
    if (/^\s*(```|~~~)/.test(line)) {
      inFence = !inFence;
      continue;
    }
    if (inFence) continue;
    const m = /^(#{2,3})\s+(.+?)\s*#*\s*$/.exec(line);
    if (!m) continue;
    // Strip inline markdown (emphasis, code, links) to get the plain text
    // rehype-slug will see. Keep headings simple in content.
    const text = (m[2] ?? "")
      .replace(/\[([^\]]*)\]\([^)]*\)/g, "$1")
      .replace(/[*_`]/g, "")
      .trim();
    toc.push({ depth: (m[1] ?? "##").length, text, id: slugger.slug(text) });
  }
  return toc;
}

function walk(dir: string): string[] {
  if (!fs.existsSync(dir)) return [];
  return fs.readdirSync(dir, { withFileTypes: true }).flatMap((e) => {
    const p = path.join(dir, e.name);
    if (e.isDirectory()) return walk(p);
    return e.name.endsWith(".mdx") ? [p] : [];
  });
}

let cache: RawEntry[] | null = null;

/** Scan + validate all research MDX files. Cached per process; the Vite plugin
 *  busts the cache on content changes. */
export function scanResearchContent(fresh = false): RawEntry[] {
  if (cache && !fresh) return cache;
  const entries: RawEntry[] = [];
  for (const abs of walk(CONTENT_DIR)) {
    const rel = path.relative(CONTENT_DIR, abs).split(path.sep).join("/");
    const isFr = rel.endsWith(".fr.mdx");
    // "<dir>/index.mdx" is the hub page of <dir> ("index.mdx" at the root is
    // /research itself, slug "").
    const slug = rel
      .replace(/\.fr\.mdx$/, "")
      .replace(/\.mdx$/, "")
      .replace(/(^|\/)index$/, "$1")
      .replace(/\/$/, "");
    const { data, content } = matter(fs.readFileSync(abs, "utf8"));
    const parsed = frontmatterSchema.safeParse(data);
    if (!parsed.success) {
      throw new Error(
        `research content: invalid frontmatter in ${rel}:\n` +
          parsed.error.issues.map((i) => `  ${i.path.join(".")}: ${i.message}`).join("\n"),
      );
    }
    entries.push({
      slug,
      lang: isFr ? "fr" : "en",
      file: rel,
      fm: parsed.data,
      toc: extractToc(content),
      body: content,
    });
  }
  // Every FR file must have an EN twin (EN is the canonical tree).
  for (const e of entries) {
    if (e.lang === "fr" && !entries.some((o) => o.lang === "en" && o.slug === e.slug)) {
      throw new Error(`research content: ${e.file} has no English counterpart (${e.slug}.mdx)`);
    }
  }
  // The topics/ and people/ URL prefixes are reserved for auto-generated hub
  // pages (topic hubs and researcher hubs).
  for (const e of entries) {
    if (e.slug === "topics" || e.slug.startsWith("topics/")) {
      throw new Error(
        `research content: ${e.file} — the "topics/" slug prefix is reserved for topic hubs`,
      );
    }
    // "/research/people" itself is a real content page (the community gallery);
    // only the sub-paths are reserved for auto-generated per-researcher hubs.
    if (e.slug.startsWith("people/")) {
      throw new Error(
        `research content: ${e.file} — the "people/<slug>" prefix is reserved for researcher hubs`,
      );
    }
  }
  // Frontmatter author must exist in the registry (catches typos).
  const knownAuthors = new Set(researchAuthors(fresh).map((a) => a.slug));
  for (const e of entries) {
    if (e.fm.author !== undefined && !knownAuthors.has(e.fm.author)) {
      throw new Error(
        `research content: ${e.file} references unknown author "${e.fm.author}" ` +
          `(registry: content/research/authors.json)`,
      );
    }
  }
  // Frontmatter topics must exist in the registry (catches typos).
  const known = new Set(researchTopics(fresh).map((t) => t.slug));
  for (const e of entries) {
    for (const s of e.fm.topics) {
      if (!known.has(s)) {
        throw new Error(
          `research content: ${e.file} references unknown topic "${s}" ` +
            `(registry: content/research/topics.json)`,
        );
      }
    }
  }
  cache = entries;
  return entries;
}

/** Canonical order of a doc within its section. */
function sortKey(d: ResearchDoc): [number, string] {
  return [d.order ?? 1e9, d.title];
}

/** Build the per-language manifest the client consumes.
 *  For FR, untranslated pages fall back to the EN entry with translated:false. */
export function buildManifest(lang: Lang, opts?: { includeDrafts?: boolean }): ResearchDoc[] {
  const entries = scanResearchContent();
  const en = entries.filter((e) => e.lang === "en");
  const docs = en.map((e) => {
    const fr = lang === "fr" ? entries.find((o) => o.lang === "fr" && o.slug === e.slug) : undefined;
    const use = fr ?? e;
    const doc: ResearchDoc = {
      slug: e.slug,
      url: e.slug === "" ? "/research" : `/research/${e.slug}`,
      section: e.slug.split("/")[0] ?? "",
      title: use.fm.title,
      description: use.fm.description,
      kind: e.fm.kind,
      status: e.fm.status,
      ...(e.fm.author !== undefined ? { author: e.fm.author } : {}),
      ...(e.fm.tier !== undefined ? { tier: e.fm.tier } : {}),
      ...(e.fm.rigor !== undefined ? { rigor: e.fm.rigor } : {}),
      ...(e.fm.complexity !== undefined ? { complexity: e.fm.complexity } : {}),
      ...(e.fm.score !== undefined ? { score: e.fm.score } : {}),
      ...(use.fm.updated !== undefined ? { updated: use.fm.updated } : {}),
      ...(e.fm.repro !== undefined ? { repro: e.fm.repro } : {}),
      sources: use.fm.sources,
      topics: e.fm.topics,
      related: e.fm.related,
      ...(e.fm.order !== undefined ? { order: e.fm.order } : {}),
      ...(e.fm.group !== undefined ? { group: e.fm.group } : {}),
      toc: use.toc,
      translated: lang === "en" || fr !== undefined,
      file: use.file,
    };
    return doc;
  });
  const visible = opts?.includeDrafts ? docs : docs.filter((d) => d.status !== "draft");
  return visible.sort((a, b) => {
    const [ao, at] = sortKey(a);
    const [bo, bt] = sortKey(b);
    return a.section === b.section
      ? ao - bo || at.localeCompare(bt)
      : a.section.localeCompare(b.section);
  });
}

/** Reduce MDX source to plain searchable text: drop frontmatter (already
 *  stripped), ESM imports/exports, JSX tags, MDX expressions, code-fence
 *  markers, markdown syntax and math delimiters — keep the words. */
function plainText(body: string): string {
  return (
    body
      .replace(/^(import|export)\s[^\n]*$/gm, "")
      .replace(/```[^\n]*/g, " ")
      .replace(/<[^>]+>/g, " ")
      .replace(/\{[^}]*\}/g, " ")
      .replace(/!\[([^\]]*)\]\([^)]*\)/g, "$1")
      .replace(/\[([^\]]*)\]\([^)]*\)/g, "$1")
      .replace(/^#{1,6}\s+/gm, "")
      .replace(/[*_`~]|\$\$?/g, "")
      .replace(/\s+/g, " ")
      .trim()
  );
}

/** Full-text search entries for all publishable MDX pages, one language. */
export function searchEntries(lang: Lang): SearchEntry[] {
  const entries = scanResearchContent();
  return buildManifest(lang).map((d) => {
    const raw = entries.find((e) => e.file === d.file);
    return {
      url: d.url,
      title: d.title,
      description: d.description,
      kind: d.kind,
      text: raw ? plainText(raw.body) : "",
    };
  });
}

/** Site paths (EN, prefix-free) of all publishable MDX pages plus the
 *  auto-generated topic hub pages — feeds the prerender list and sitemap.
 *  Drafts are excluded (they would 404 in prod). */
export function researchPagePaths(): string[] {
  const pages = buildManifest("en").map((d) => d.url.slice(1));
  const topics = researchTopics();
  const topicPaths =
    topics.length > 0
      ? ["research/topics", ...topics.map((t) => `research/topics/${t.slug}`)]
      : [];
  // Per-researcher hubs (the /research/people index is a real content page,
  // already covered by `pages`). Every registered author gets a prerendered
  // hub: the profile (bio + links) is the page's substance, whether or not the
  // person also authored wiki pages. Contributors who authored pages get those
  // listed too; profile-only figures (the record-holders, theorists and
  // toolmakers on the Who's-who gallery) still ship a real, crawlable page.
  const peoplePaths = researchAuthors().map((a) => `research/people/${a.slug}`);
  return [...pages, ...topicPaths, ...peoplePaths];
}
