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
