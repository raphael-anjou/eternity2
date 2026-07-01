// The research wiki navigation model: the sidebar tree, breadcrumbs, and
// prev/next all derive from one ordered union of
//   1. MDX pages from the manifest (the target state), and
//   2. STATIC_LEAVES — the TSX pages that haven't migrated yet (transitional;
//      titles come from seo.ts so nothing is duplicated). Each entry deleted
//      as its page moves to web/content/research.

import type { Lang } from "@/i18n";
import { pageTitle } from "@/seo";
import { researchDocs } from "./manifest";
import type { ResearchKind } from "./types";

export interface NavItem {
  url: string;
  title: string;
  kind: ResearchKind;
  /** Backed by an MDX page (docs shell) vs a legacy TSX page. */
  mdx: boolean;
  translated: boolean;
  children: NavItem[];
}

export interface NavSection {
  key: string;
  url: string;
  label: string;
  items: NavItem[];
}

const SECTION_LABELS: Record<string, { en: string; fr: string }> = {
  why: { en: "Why it's hard", fr: "Pourquoi c'est dur" },
  build: { en: "Build a solver", fr: "Écrire un solveur" },
  lab: { en: "The lab notebook", fr: "Le carnet de labo" },
};

/** Order of the sidebar sections. */
const SECTION_ORDER = ["why", "build", "lab"] as const;

/** Flat research pages that belong to a section other than their first URL
 *  segment (the Build door re-homes the reference pages). */
const SECTION_OF: Record<string, string> = {
  reference: "build",
  papers: "build",
  records: "build",
};

interface StaticLeaf {
  url: string;
  seoKey: string;
  kind: ResearchKind;
  order: number;
}

// Mirrors the current hub-card order. Shrinks to empty during the migration.
const STATIC_LEAVES: StaticLeaf[] = [
  // Why it's hard (hub order)
  { url: "/research/why/prune-vs-speed", seoKey: "prune-vs-speed", kind: "finding", order: 10 },
  { url: "/research/why/walls-and-methods", seoKey: "walls-and-methods", kind: "finding", order: 20 },
  { url: "/research/why/complex-theory", seoKey: "complex-theory", kind: "finding", order: 30 },
  { url: "/research/why/phase-transition", seoKey: "phase-transition", kind: "finding", order: 40 },
  { url: "/research/why/rigidity-wall", seoKey: "rigidity-wall", kind: "finding", order: 50 },
  { url: "/research/why/sigma-cycles", seoKey: "sigma-cycles", kind: "finding", order: 60 },
  { url: "/research/why/mismatch-geometry", seoKey: "mismatch-geometry", kind: "finding", order: 70 },
  { url: "/research/why/forbidden-patterns", seoKey: "forbidden-patterns", kind: "finding", order: 80 },
  { url: "/research/why/no-forced-moves", seoKey: "no-forced-moves", kind: "finding", order: 90 },
  { url: "/research/why/piece-theft", seoKey: "piece-theft", kind: "finding", order: 100 },
  { url: "/research/why/rare-color-geography", seoKey: "rare-color-geography", kind: "finding", order: 110 },
  // border-balance: order 120 — first MDX pilot, lives in the manifest.
  { url: "/research/why/entropy-area-law", seoKey: "entropy-area-law", kind: "finding", order: 130 },

  // Build a solver (hub order)
  { url: "/research/reference", seoKey: "reference", kind: "reference", order: 10 },
  { url: "/research/papers", seoKey: "papers", kind: "paper", order: 20 },
  { url: "/research/records", seoKey: "records", kind: "reference", order: 30 },
  { url: "/research/build/solvers", seoKey: "solvers", kind: "tool", order: 40 },
  { url: "/research/build/dead-ends", seoKey: "dead-ends", kind: "reference", order: 50 },
  { url: "/research/build/run-it-yourself", seoKey: "run-it-yourself", kind: "tool", order: 60 },

  // The lab notebook
  { url: "/research/lab/findings", seoKey: "findings", kind: "finding", order: 10 },
  { url: "/research/lab/experiments", seoKey: "experiments", kind: "invention", order: 20 },
  { url: "/research/lab/experiments/palimpsest", seoKey: "exp-palimpsest", kind: "invention", order: 10 },
  { url: "/research/lab/experiments/keyring", seoKey: "exp-keyring", kind: "invention", order: 20 },
  { url: "/research/lab/experiments/prior", seoKey: "exp-prior", kind: "invention", order: 30 },
  { url: "/research/lab/experiments/replay", seoKey: "exp-replay", kind: "invention", order: 40 },
  { url: "/research/lab/experiments/gauntlet", seoKey: "exp-gauntlet", kind: "invention", order: 50 },
  { url: "/research/lab/experiments/cloister", seoKey: "exp-cloister", kind: "invention", order: 60 },
  { url: "/research/lab/experiments/midden", seoKey: "exp-midden", kind: "invention", order: 70 },
  { url: "/research/lab/experiments/ladder", seoKey: "exp-ladder", kind: "invention", order: 80 },
  { url: "/research/lab/experiments/lodestone", seoKey: "exp-lodestone", kind: "invention", order: 90 },
  { url: "/research/lab/experiments/mosaic", seoKey: "exp-mosaic", kind: "invention", order: 100 },
  { url: "/research/lab/experiments/bandsaw", seoKey: "exp-bandsaw", kind: "invention", order: 110 },
  { url: "/research/lab/experiments/staged", seoKey: "exp-staged", kind: "invention", order: 120 },
  { url: "/research/lab/experiments/log", seoKey: "experiments-log", kind: "tool", order: 130 },
];

function sectionOf(url: string): string {
  const seg = url.split("/")[2] ?? "";
  return SECTION_OF[seg] ?? seg;
}

/** Build the full sidebar tree for a language. */
export function researchNav(lang: Lang): NavSection[] {
  type Flat = NavItem & { order: number };
  const flat: Flat[] = [];

  for (const leaf of STATIC_LEAVES) {
    flat.push({
      url: leaf.url,
      title: pageTitle(leaf.seoKey, lang),
      kind: leaf.kind,
      mdx: false,
      translated: true,
      children: [],
      order: leaf.order,
    });
  }
  for (const d of researchDocs(lang)) {
    flat.push({
      url: d.url,
      title: d.title,
      kind: d.kind,
      mdx: true,
      translated: d.translated,
      children: [],
      order: d.order ?? 1e9,
    });
  }

  return SECTION_ORDER.map((key) => {
    const label = SECTION_LABELS[key];
    const inSection = flat.filter((i) => sectionOf(i.url) === key);
    // Nest one level: an item is a child of the section item whose URL is its
    // parent path (e.g. …/experiments/palimpsest under …/experiments).
    const byUrl = new Map(inSection.map((i) => [i.url, i]));
    const roots: Flat[] = [];
    for (const item of inSection) {
      const parentUrl = item.url.slice(0, item.url.lastIndexOf("/"));
      const parent = byUrl.get(parentUrl);
      if (parent) parent.children.push(item);
      else roots.push(item);
    }
    const byOrder = (a: Flat, b: Flat) =>
      a.order - b.order || a.title.localeCompare(b.title);
    roots.sort(byOrder);
    for (const r of roots) (r.children as Flat[]).sort(byOrder);
    return {
      key,
      url: `/research/${key}`,
      label: label ? label[lang] : key,
      items: roots,
    };
  });
}

/** Reading order across the whole wiki (sections, depth-first) — drives
 *  prev/next. Hub pages are not in the list; only leaves. */
export function researchReadingOrder(lang: Lang): NavItem[] {
  return researchNav(lang).flatMap((s) => s.items.flatMap((i) => [i, ...i.children]));
}

/** Localized labels for page kinds (badges, related rail). */
export function kindLabel(kind: ResearchKind, lang: Lang): string {
  const L: Record<ResearchKind, { en: string; fr: string }> = {
    finding: { en: "finding", fr: "résultat" },
    invention: { en: "invention", fr: "invention" },
    tool: { en: "tool", fr: "outil" },
    reference: { en: "reference", fr: "référence" },
    concept: { en: "concept", fr: "concept" },
    basin: { en: "board study", fr: "étude de plateau" },
    paper: { en: "papers", fr: "articles" },
    page: { en: "page", fr: "page" },
  };
  return L[kind][lang];
}

/** Dot color per kind — matches the related-rail convention. */
export const KIND_DOT: Record<ResearchKind, string> = {
  finding: "bg-violet-400",
  invention: "bg-emerald-400",
  tool: "bg-amber-400",
  reference: "bg-stone-400",
  concept: "bg-sky-400",
  basin: "bg-sky-400",
  paper: "bg-stone-400",
  page: "bg-stone-400",
};
