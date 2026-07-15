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

/** The *contribution* axis: what kind of research result a page is, independent
 *  of `kind` (which drives layout). Only `solver` results earn a row on the
 *  score chart / leaderboard. See lab/experiments/methodology for the full taxonomy.
 *  solver — produces a competitive board by searching (a real search score);
 *  analysis — proves/computes a property of an existing board or the instance;
 *  reconstruction — decodes/rebuilds community work to extract an insight;
 *  theory — a mathematical property, law, or impossibility proof;
 *  method — a technique described for reuse, not a scored run;
 *  measurement — a benchmark or empirical observation about solvers/instances;
 *  negative — a rigorously-run dead end (first-class, not a footnote);
 *  tool — a software artifact; exposition — an explainer for an audience. */
export type ContributionKind =
  | "solver"
  | "analysis"
  | "reconstruction"
  | "theory"
  | "method"
  | "measurement"
  | "negative"
  | "tool"
  | "exposition";

/** How far through the publish pipeline a page is (work is published by PR; the
 *  PR review sets the tier). `draft` = unpublished (not normally in the repo);
 *  `report` = a public technical report, not yet reviewed (badged); `live` = a
 *  reviewed finding, cited and ~immutable. */
export type StatusTier = "draft" | "report" | "live";

/** How firmly a page's central claim is established — a scannable credibility
 *  signal (rendered as a badge). "proven" = a formal or exhaustive proof/
 *  certificate; "measured" = an empirical result on this project's engine;
 *  "conjectured" = a hypothesis or literature reading not yet established here. */
export type RigorKind = "proven" | "measured" | "conjectured";

/** Algorithmic cost of the method a page describes, for the researcher who
 *  wants the complexity at a glance. Strings are KaTeX-free plain text
 *  (e.g. "O(e·d²) time, O(e·d) space"); `note` adds the one-line caveat. */
export interface ComplexityInfo {
  time?: string | undefined;
  space?: string | undefined;
  note?: string | undefined;
}

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

/** The class of compute a run leaned on — kept small and explicit so the
 *  widget can badge it. `none` = plain CPU; the rest name the accelerator that
 *  did the heavy lifting. Quantum/FPGA/TPU are in the vocabulary from the start
 *  so a future run needs no schema migration. */
export type AcceleratorKind = "none" | "gpu" | "quantum" | "fpga" | "tpu";

/** The hardware an experiment actually ran on — mandatory on experiment pages
 *  so every measured result is comparable and reproducible. Split three ways:
 *  the *quantitative* fields are the comparable numbers; the *qualitative*
 *  fields name the exact kit; the *protocol* fields say what a single number
 *  means. `coreHours` is DERIVED by the widget (cores × wall-clock hours), the
 *  honest compute-cost figure that lets a 1-core bench and a 400-core cluster
 *  run sit in the same table. */
export interface HardwareInfo {
  // Quantitative — the comparable numbers.
  /** Total logical cores used (summed across nodes for a cluster). */
  cores: number;
  /** Machines the run spanned; omit or 1 for a single box. */
  nodes?: number;
  /** OS threads spawned — differs from cores when oversubscribed. */
  threads?: number;
  /** Total RAM available to the run, in GiB (summed across nodes). */
  ramGb?: number;
  /** GPU count; 0 for CPU-only. */
  gpus?: number;
  // Qualitative — the exact kit and its provenance.
  /** CPU brand/model, e.g. "Apple M2 Pro", "AMD EPYC 7742". */
  cpu?: string;
  /** GPU/accelerator model when present, e.g. "NVIDIA RTX 4090". */
  gpu?: string;
  /** Which class of accelerator carried the work. */
  accelerator?: AcceleratorKind;
  /** Human label for the box or cluster, e.g. "MacBook Pro 14 (2023)",
   *  "8× EPYC node cluster". */
  machine?: string;
  // Protocol — what makes the number mean something.
  /** Budget per run and repeats as prose, e.g. "10 × 60 s", "10 h". */
  wallClock?: string;
  /** How many independent runs the reported figures aggregate. */
  runs?: number;
  /** How the start was varied, e.g. "randomized corner permutation per run". */
  seedPolicy?: string;
  /** True only for the standardized single-core bench (1 core / fixed budget),
   *  which makes the run cross-comparable; false for a native run recorded for
   *  provenance. A many-core run can never be true. */
  measured?: boolean;
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
  /** Optional short override for the <meta description>/og tag; `description`
   *  stays the on-page lede. Absent → meta falls back to a truncated
   *  `description`. See metaDescriptionFor(). */
  metaDescription?: string;
  kind: ResearchKind;
  status: StatusTier;
  /** What kind of research contribution this is (drives score-chart membership:
   *  only `solver` appears). Undefined on structural/hub pages. */
  contribution?: ContributionKind;
  /** Registry slug of the page's author (drives the byline + researcher hub).
   *  Undefined for structural/community pages with no single researcher. */
  author?: string;
  /** 1 = flagship, 2 = finding, 3 = supporting. */
  tier?: number;
  /** How firmly the central claim is established (badge). */
  rigor?: RigorKind;
  /** Algorithmic cost of the method (badge / small block). */
  complexity?: ComplexityInfo;
  /** Matched-edges score, for inventions/basins. */
  score?: number;
  /** ISO date of last substantive update. */
  updated?: string;
  /** Month an experiment was run / first written (YYYY-MM), for the gallery. */
  date?: string;
  repro?: ReproInfo;
  /** The hardware a measured run leaned on (mandatory on experiment pages). */
  hardware?: HardwareInfo;
  /** External resources supporting the page's claims (groups.io posts,
   *  papers, community pages) — every claim links its evidence. */
  sources: { label: string; url: string }[];
  /** Cross-cutting topic slugs (speed, search-space, dfs, gpu, quantum, …). */
  topics: string[];
  /** Site paths of related pages (drives the related rail). */
  related: string[];
  /** Sort key within its section (lower first; ties by title). */
  order?: number;
  /** Named sub-section within a sidebar section (the approaches-map spine). */
  group?: string;
  /** Optional provenance note: what this page was distilled from. Never a link
   *  to private material. */
  provenance?: string;
  toc: TocItem[];
  /** False in the fr manifest when no .fr.mdx exists (page falls back to EN). */
  translated: boolean;
  /** Content-dir-relative file backing this entry, e.g. "why/border-balance.fr.mdx". */
  file: string;
}
