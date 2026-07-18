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
import type { HardwareInfo, ResearchDoc, SearchEntry, TocItem } from "./src/lib/research/types";

const HERE = path.dirname(fileURLToPath(import.meta.url));
export const CONTENT_DIR = path.join(HERE, "content", "research");
const TOPICS_FILE = path.join(CONTENT_DIR, "topics.json");
const AUTHORS_FILE = path.join(CONTENT_DIR, "authors.json");

// The languages the site speaks, English first. Kept in sync with the client
// registry in src/i18n/index.tsx (the build side can't import the client tree).
// English is the canonical tree and the fallback for every other language.
export type Lang = "en" | "fr" | "es";

/** The URL prefix for each language (English at the root, others under
 *  /<prefix>). This is the pure-data half of the language registry; the client
 *  registry in src/i18n/index.tsx carries the same codes plus display metadata
 *  and MUST stay in sync. Kept here (not imported from src/i18n) so Node-side
 *  config — the sitemap + prerender list — need not pull the React client tree.
 *  English MUST stay first (canonical tree + fallback). */
export const LANG_PREFIXES: readonly { code: Lang; prefix: string }[] = [
  { code: "en", prefix: "" },
  { code: "fr", prefix: "fr" },
  { code: "es", prefix: "es" },
];
export const LANG_CODES: readonly Lang[] = LANG_PREFIXES.map((l) => l.code);
/** The non-English languages, in registry order — each carries a `.<lang>.mdx`
 *  sidecar convention and a `/<lang>` URL tree. */
export const TRANSLATION_LANGS: readonly Lang[] = LANG_CODES.filter((l) => l !== "en");

// A curated localized string: English is required (canonical + fallback), every
// other language is optional. A missing translation resolves to English, so a
// registry entry (a topic label, an author bio) need not be translated into
// every language at once. Mirrors useT's EN-fallback on the client.
const localized = z.object({ en: z.string() }).catchall(z.string());

// ---- Topic registry ------------------------------------------------------

const topicSchema = z.object({
  slug: z.string().regex(/^[a-z0-9-]+$/),
  order: z.number(),
  label: localized,
  description: localized,
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
  tagline: localized.optional(),
  affiliation: localized.optional(),
  bio: localized.optional(),
  links: z.array(z.object({ label: z.string(), url: z.string().url() })).default([]),
});

/** Resolve a localized string to a language, falling back to English (the
 *  canonical entry) when that language has no translation. Mirrors useT. */
export function pickLang(value: Record<string, string>, lang: Lang): string {
  return value[lang] ?? value["en"] ?? "";
}

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

// A pipeline stage of a run. Most named runs are not one algorithm but an
// ordered composition (e.g. beam-produce -> ALNS-lift -> refine). Declaring the
// stages makes the composition queryable and, via `published`, states honestly
// when a stage runs on an engine that does not yet have its own write-up here
// (the beam producer and ALNS). `engine` is a closed vocabulary so the section
// can be filtered by what each run actually uses.
const stageSchema = z.object({
  engine: z.enum([
    "beam-producer",
    "alns",
    "break-dfs",
    "exact-tail",
    "maxsat",
    "restart-tournament",
    "refinement",
    "corpus-mining",
    "clustering",
    "frame-input",
  ]),
  // One line: what this stage does in the run.
  does: z.string().min(1),
  // The learned/applied technique this stage carries, if any (a corpus prior, a
  // trap map, a rare-colour ordering). Free text.
  learns: z.string().optional(),
  // Whether the engine this stage runs on has a published write-up on the site.
  // Defaults true; set false for the still-unpublished beam producer and ALNS,
  // so a run never silently leans on a page a reader cannot open.
  published: z.boolean().default(true),
});

// The hardware an experiment ran on. Quantitative fields are the comparable
// numbers; qualitative name the kit; protocol fields say what a number means.
// `coreHours` is derived by the widget, never authored. `measured: true` is the
// standardized single-core bench; anything many-core must be false.
const hardwareSchema = z.object({
  cores: z.number().positive(),
  nodes: z.number().int().positive().optional(),
  threads: z.number().positive().optional(),
  ramGb: z.number().positive().optional(),
  gpus: z.number().int().min(0).optional(),
  cpu: z.string().optional(),
  gpu: z.string().optional(),
  accelerator: z.enum(["none", "gpu", "quantum", "fpga", "tpu"]).optional(),
  machine: z.string().optional(),
  wallClock: z.string().optional(),
  runs: z.number().int().positive().optional(),
  seedPolicy: z.string().optional(),
  measured: z.boolean().optional(),
});

const frontmatterSchema = z.object({
  title: z.string().min(1),
  description: z.string().min(1),
  // Optional short form used ONLY for the <meta name="description"> / og tag.
  // Many `description` ledes run 200–450 chars — great as the visible page
  // subtitle, but Google truncates a SERP snippet around 155–160 chars. When a
  // page sets `metaDescription`, that (tighter) string feeds the meta tag while
  // `description` stays the on-page lede. When absent, the meta tag falls back
  // to `description`, cleanly truncated at a word boundary (see metaDescription
  // in the manifest). Capped so it can't itself overflow.
  metaDescription: z.string().min(1).max(165).optional(),
  kind: z
    .enum(["finding", "experiment", "tool", "reference", "concept", "basin", "paper", "page"])
    .default("page"),
  // The *contribution* axis: what kind of research result this is, independent
  // of `kind` (which drives layout). Only `solver` results earn a row on the
  // score chart / leaderboard; everything else is a property, a decode, a
  // technique, a measurement, a dead end, or an explainer. See
  // lab/experiments/methodology.
  contribution: z
    .enum([
      "solver",
      "analysis",
      "reconstruction",
      "theory",
      "method",
      "measurement",
      "negative",
      "tool",
      "exposition",
    ])
    .optional(),
  // The *status* axis: how far through the publish pipeline a page is. Work is
  // published by pull request; the PR review sets the tier.
  //   draft  — unpublished; not normally present in the repo at all. Kept as a
  //            valid state only for pulling a page back out of publication.
  //   report — a technical report: public but not yet reviewed (badged as such).
  //   live   — a reviewed finding: public, cited, treated as ~immutable.
  status: z.enum(["draft", "report", "live"]).default("live"),
  // Optional provenance note: what this page was distilled from, so the public
  // record is traceable. Free text; never a link to anyone's private material.
  provenance: z.string().optional(),
  // Registry slug of the researcher who authored the page. Validated against
  // authors.json in scanResearchContent (below), like topics.
  author: z.string().optional(),
  // Secondary contributors, credited in the byline alongside `author` (e.g. the
  // method behind a reimplementation, or the write-up of someone else's solver).
  // Each `slug` is validated against authors.json like `author`; `role` is a
  // short free-text label ("method", "write-up", "parameter study"). This is
  // credit only: it does not change hub membership, the score chart, or nav.
  contributors: z
    .array(z.object({ slug: z.string().min(1), role: z.string().min(1) }))
    .optional(),
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
  // Month an experiment was run/first written (YYYY-MM), shown on the gallery.
  // A bare YYYY-MM parses as a string in YAML; a full date is truncated.
  date: z
    .union([
      z.string().regex(/^\d{4}-\d{2}$/, "date must be YYYY-MM"),
      z.date().transform((d) => d.toISOString().slice(0, 7)),
    ])
    .optional(),
  repro: reproSchema.optional(),
  hardware: hardwareSchema.optional(),
  // Ordered pipeline stages of a run (see stageSchema). Present on the named
  // runs, which are compositions rather than single algorithms.
  stages: z.array(stageSchema).optional(),
  sources: z.array(z.object({ label: z.string(), url: z.string().url() })).default([]),
  topics: z.array(z.string()).default([]),
  related: z.array(z.string().startsWith("/")).default([]),
  order: z.number().optional(),
  // Named sub-section within a sidebar section (e.g. "Reduce the search" inside
  // Build a solver). Groups the flat page list into the approaches-map spine.
  group: z.string().optional(),
});

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
  // A translated sidecar is "<slug>.<lang>.mdx" for any non-English registry
  // language; an "<slug>.mdx" with no language suffix is the canonical English
  // page. Built from the registry so a new language's sidecars are recognized
  // automatically.
  const sidecarRe = new RegExp(`\\.(${TRANSLATION_LANGS.join("|")})\\.mdx$`);
  const entries: RawEntry[] = [];
  for (const abs of walk(CONTENT_DIR)) {
    const rel = path.relative(CONTENT_DIR, abs).split(path.sep).join("/");
    const sidecar = sidecarRe.exec(rel);
    const lang: Lang = sidecar ? (sidecar[1] as Lang) : "en";
    // "<dir>/index.mdx" is the hub page of <dir> ("index.mdx" at the root is
    // /research itself, slug "").
    const slug = rel
      .replace(sidecarRe, "")
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
      lang,
      file: rel,
      fm: parsed.data,
      toc: extractToc(content),
      body: content,
    });
  }
  // Every translated sidecar must have an EN twin (EN is the canonical tree).
  for (const e of entries) {
    if (e.lang !== "en" && !entries.some((o) => o.lang === "en" && o.slug === e.slug)) {
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
    // Contributor slugs are validated the same way, and must be distinct from
    // each other and from the primary author (a person is credited once).
    const seen = new Set<string>();
    for (const c of e.fm.contributors ?? []) {
      if (!knownAuthors.has(c.slug)) {
        throw new Error(
          `research content: ${e.file} references unknown contributor "${c.slug}" ` +
            `(registry: content/research/authors.json)`,
        );
      }
      if (c.slug === e.fm.author) {
        throw new Error(
          `research content: ${e.file} lists "${c.slug}" as both author and contributor`,
        );
      }
      if (seen.has(c.slug)) {
        throw new Error(
          `research content: ${e.file} lists contributor "${c.slug}" more than once`,
        );
      }
      seen.add(c.slug);
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
 *  For a non-English language, untranslated pages fall back to the EN entry
 *  with translated:false. */
export function buildManifest(lang: Lang, opts?: { includeDrafts?: boolean }): ResearchDoc[] {
  const entries = scanResearchContent();
  const en = entries.filter((e) => e.lang === "en");
  const docs = en.map((e) => {
    const translation =
      lang === "en" ? undefined : entries.find((o) => o.lang === lang && o.slug === e.slug);
    const use = translation ?? e;
    const doc: ResearchDoc = {
      slug: e.slug,
      url: e.slug === "" ? "/research" : `/research/${e.slug}`,
      section: e.slug.split("/")[0] ?? "",
      title: use.fm.title,
      description: use.fm.description,
      ...(use.fm.metaDescription !== undefined
        ? { metaDescription: use.fm.metaDescription }
        : {}),
      kind: e.fm.kind,
      status: e.fm.status,
      ...(e.fm.contribution !== undefined ? { contribution: e.fm.contribution } : {}),
      ...(e.fm.author !== undefined ? { author: e.fm.author } : {}),
      ...(e.fm.contributors !== undefined ? { contributors: e.fm.contributors } : {}),
      ...(e.fm.tier !== undefined ? { tier: e.fm.tier } : {}),
      ...(e.fm.rigor !== undefined ? { rigor: e.fm.rigor } : {}),
      ...(e.fm.complexity !== undefined ? { complexity: e.fm.complexity } : {}),
      ...(e.fm.score !== undefined ? { score: e.fm.score } : {}),
      ...(use.fm.updated !== undefined ? { updated: use.fm.updated } : {}),
      ...(use.fm.date !== undefined ? { date: use.fm.date } : {}),
      ...(e.fm.repro !== undefined ? { repro: e.fm.repro } : {}),
      ...(e.fm.stages !== undefined ? { stages: e.fm.stages } : {}),
      // zod infers optional fields as `T | undefined`, which trips
      // exactOptionalPropertyTypes against HardwareInfo's `nodes?: number` etc.
      // The schema mirrors HardwareInfo exactly (and has just validated the
      // value), so assert it at this boundary rather than widen the consumer type.
      ...(e.fm.hardware !== undefined ? { hardware: e.fm.hardware as HardwareInfo } : {}),
      sources: use.fm.sources,
      topics: e.fm.topics,
      related: e.fm.related,
      ...(e.fm.order !== undefined ? { order: e.fm.order } : {}),
      ...(e.fm.group !== undefined ? { group: e.fm.group } : {}),
      ...(e.fm.provenance !== undefined ? { provenance: e.fm.provenance } : {}),
      toc: use.toc,
      translated: lang === "en" || translation !== undefined,
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
  // The per-topic hubs are crawlable pages; the /research/topics INDEX is not
  // emitted (it duplicated Overview and now redirects there), so it is left out
  // of the prerender + sitemap list. The hubs stay reachable from the left rail.
  const topicPaths = topics.map((t) => `research/topics/${t.slug}`);
  // Per-researcher hubs (the /research/people index is a real content page,
  // already covered by `pages`). Every registered author gets a prerendered
  // hub: the profile (bio + links) is the page's substance, whether or not the
  // person also authored wiki pages. Contributors who authored pages get those
  // listed too; profile-only figures (the record-holders, theorists and
  // toolmakers on the Who's-who gallery) still ship a real, crawlable page.
  const peoplePaths = researchAuthors().map((a) => `research/people/${a.slug}`);
  // The glossary is a route-generated page (like the hubs), not MDX content.
  return [...pages, ...topicPaths, ...peoplePaths, "research/glossary"];
}

/** Research paths that have a genuine rendering in the given (non-English)
 *  language, so the build prerenders a real /<lang>/research/* HTML file and the
 *  sitemap lists it. Two kinds qualify:
 *   - MDX doc pages that ship a `<slug>.<lang>.mdx` sidecar (translated:true in
 *     that language's manifest). Untranslated pages are deliberately excluded —
 *     they fall back to English client-side, and prerendering them under
 *     /<lang> would put duplicate English content on a translated URL (bad for
 *     crawlers).
 *   - The route-generated hubs (topics, people, glossary): their content comes
 *     from the localized registries (which fall back to English per field), so
 *     every one is reachable in every language and always gets a /<lang> twin. */
export function researchPagePathsFor(lang: Lang): string[] {
  const translatedDocs = buildManifest(lang)
    .filter((d) => d.translated)
    .map((d) => d.url.slice(1));
  const topicPaths = researchTopics().map((t) => `research/topics/${t.slug}`);
  const peoplePaths = researchAuthors().map((a) => `research/people/${a.slug}`);
  return [...translatedDocs, ...topicPaths, ...peoplePaths, "research/glossary"];
}

/** Basename-relative path → last-modified date (YYYY-MM-DD) for research MDX
 *  pages that declare `updated:` in frontmatter. Feeds <lastmod> in the sitemap
 *  so crawlers get a real freshness signal, computed from the same manifest the
 *  prerender list uses (no drift, no hand-maintenance). MDX pages without an
 *  `updated` date are absent from the map; the route-generated topic/person
 *  hubs are handled by researchHubLastmod() below. */
export function researchPageLastmod(): Record<string, string> {
  const out: Record<string, string> = {};
  for (const d of buildManifest("en")) {
    if (d.updated) out[d.url.slice(1)] = d.updated;
  }
  return out;
}

/** Basename-relative path → <lastmod> for the route-generated research hub
 *  pages (per-topic hubs and per-researcher hubs). These carry no frontmatter
 *  of their own, so each inherits the newest `updated` date among the pages it
 *  aggregates: a topic hub from its tagged pages, a person hub from that
 *  author's pages. Hubs that aggregate no dated page (e.g. a profile-only
 *  researcher, or the glossary, which is not derived here) stay absent, and the
 *  sitemap omits <lastmod> for them rather than inventing one. The client
 *  renderer derives the same dates via topicUpdated()/authorUpdated(). */
export function researchHubLastmod(): Record<string, string> {
  const docs = buildManifest("en");
  const newest = (subset: ResearchDoc[]): string | undefined => {
    let max: string | undefined;
    for (const d of subset) {
      if (d.updated && (max === undefined || d.updated > max)) max = d.updated;
    }
    return max;
  };
  const out: Record<string, string> = {};
  for (const t of researchTopics()) {
    const lm = newest(docs.filter((d) => d.topics.includes(t.slug)));
    if (lm) out[`research/topics/${t.slug}`] = lm;
  }
  for (const a of researchAuthors()) {
    const lm = newest(docs.filter((d) => d.author === a.slug));
    if (lm) out[`research/people/${a.slug}`] = lm;
  }
  return out;
}
