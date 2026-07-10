// Shared types for the research wiki content pipeline. Used on both sides:
// the Node-side scanner (content.config.ts, the Vite plugin) and the client
// (docs shell, search). Keep this file dependency-free.

export type ResearchKind =
  | "finding"
  | "experiment"
  | "tool"
  | "reference"
  | "concept"
  | "basin"
  | "paper"
  | "page";

export type ReproKind = "exact" | "seeded" | "stochastic" | "heavy" | "prose";

export interface TocItem {
  /** Heading depth: 2 or 3 (h2/h3). */
  depth: number;
  /** Plain heading text. */
  text: string;
  /** Anchor id — matches what rehype-slug assigns. */
  id: string;
}

export interface ReproInfo {
  kind: ReproKind;
  /** Copy-pasteable reproduce command (e.g. `just research-<topic>`). */
  cmd?: string | undefined;
  /** research/topics/<id> this result was computed from. */
  topic?: string | undefined;
}

/** One entry of the full-text search index (per language). */
export interface SearchEntry {
  url: string;
  title: string;
  description: string;
  kind: string;
  /** Plain body text (empty for pages that only index title/description). */
  text: string;
}

/** A topic (big cross-cutting category), labels resolved to one language. */
export interface Topic {
  slug: string;
  label: string;
  description: string;
}

/** A researcher who has authored pages on the wiki, resolved to one language.
 *  The registry (content/research/authors.json) holds the profile; the *list*
 *  of a person's pages is derived from every doc's `author` field, never
 *  hand-maintained — the same way topic membership is derived from `topics`. */
export interface Author {
  /** URL slug, e.g. "raphael-anjou" → /research/people/raphael-anjou. */
  slug: string;
  /** Display name as it should appear in bylines and the hub header. */
  name: string;
  /** One-line role/summary, e.g. "Independent researcher". */
  tagline?: string;
  /** A short prose bio (one language). */
  bio?: string;
  /** Optional affiliation line (institution, group, "independent"). */
  affiliation?: string;
  /** Outbound links (personal site, GitHub, groups.io handle, ORCID…). */
  links: { label: string; url: string }[];
}

/** One research wiki page, in one language. */
export interface ResearchDoc {
  /** Content path without extension/lang, e.g. "why/border-balance". */
  slug: string;
  /** Site path (language-neutral), e.g. "/research/why/border-balance". */
  url: string;
  /** First slug segment, e.g. "why" — the sidebar section. */
  section: string;
  title: string;
  description: string;
  kind: ResearchKind;
  status: "live" | "draft";
  /** Registry slug of the page's author (drives the byline + researcher hub).
   *  Undefined for structural/community pages with no single researcher. */
  author?: string;
  /** 1 = flagship, 2 = finding, 3 = supporting. */
  tier?: number;
  /** Matched-edges score, for inventions/basins. */
  score?: number;
  /** ISO date of last substantive update. */
  updated?: string;
  repro?: ReproInfo;
  /** External resources supporting the page's claims (groups.io posts,
   *  papers, community pages) — every claim links its evidence. */
  sources: { label: string; url: string }[];
  /** Cross-cutting topic slugs (speed, search-space, dfs, gpu, quantum, …). */
  topics: string[];
  /** Site paths of related pages (drives the related rail). */
  related: string[];
  /** Sort key within its section (lower first; ties by title). */
  order?: number;
  toc: TocItem[];
  /** False in the fr manifest when no .fr.mdx exists (page falls back to EN). */
  translated: boolean;
  /** Content-dir-relative file backing this entry, e.g. "why/border-balance.fr.mdx". */
  file: string;
}
