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
  topics: string[];
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
  topics?: string[];
}

// Mirrors the current hub-card order. Shrinks to empty during the migration.
// Topic tags let the topic hubs cover legacy pages before they migrate.
const STATIC_LEAVES: StaticLeaf[] = [
  // Why it's hard (hub order)
  { url: "/research/why/prune-vs-speed", seoKey: "prune-vs-speed", kind: "finding", order: 10, topics: ["speed", "search-space", "backtracking"] },
  { url: "/research/why/walls-and-methods", seoKey: "walls-and-methods", kind: "finding", order: 20, topics: ["structure"] },
  { url: "/research/why/complex-theory", seoKey: "complex-theory", kind: "finding", order: 30, topics: ["structure", "backtracking"] },
  { url: "/research/why/phase-transition", seoKey: "phase-transition", kind: "finding", order: 40, topics: ["structure"] },
  { url: "/research/why/rigidity-wall", seoKey: "rigidity-wall", kind: "finding", order: 50, topics: ["structure", "local-search", "exact-methods"] },
  { url: "/research/why/sigma-cycles", seoKey: "sigma-cycles", kind: "finding", order: 60, topics: ["structure", "local-search"] },
  { url: "/research/why/mismatch-geometry", seoKey: "mismatch-geometry", kind: "finding", order: 70, topics: ["structure", "records"] },
  { url: "/research/why/forbidden-patterns", seoKey: "forbidden-patterns", kind: "finding", order: 80, topics: ["structure", "search-space"] },
  { url: "/research/why/no-forced-moves", seoKey: "no-forced-moves", kind: "finding", order: 90, topics: ["structure", "search-space"] },
  { url: "/research/why/piece-theft", seoKey: "piece-theft", kind: "finding", order: 100, topics: ["structure", "search-space"] },
  { url: "/research/why/rare-color-geography", seoKey: "rare-color-geography", kind: "finding", order: 110, topics: ["structure"] },
  // border-balance: order 120 — first MDX pilot, lives in the manifest.
  { url: "/research/why/entropy-area-law", seoKey: "entropy-area-law", kind: "finding", order: 130, topics: ["structure"] },

  // Build a solver (hub order)
  { url: "/research/reference", seoKey: "reference", kind: "reference", order: 10, topics: ["backtracking"] },
  { url: "/research/papers", seoKey: "papers", kind: "paper", order: 20 },
  { url: "/research/records", seoKey: "records", kind: "reference", order: 30, topics: ["records"] },
  { url: "/research/build/solvers", seoKey: "solvers", kind: "tool", order: 40, topics: ["backtracking", "speed"] },
  { url: "/research/build/dead-ends", seoKey: "dead-ends", kind: "reference", order: 50, topics: ["search-space", "learning", "exact-methods"] },
  { url: "/research/build/run-it-yourself", seoKey: "run-it-yourself", kind: "tool", order: 60 },

  // The lab notebook
  { url: "/research/lab/findings", seoKey: "findings", kind: "finding", order: 10, topics: ["structure"] },
  { url: "/research/lab/experiments", seoKey: "experiments", kind: "invention", order: 20 },
  { url: "/research/lab/experiments/palimpsest", seoKey: "exp-palimpsest", kind: "invention", order: 10, topics: ["local-search", "learning", "records"] },
  { url: "/research/lab/experiments/keyring", seoKey: "exp-keyring", kind: "invention", order: 20, topics: ["construction", "learning"] },
  { url: "/research/lab/experiments/prior", seoKey: "exp-prior", kind: "invention", order: 30, topics: ["construction", "learning"] },
  { url: "/research/lab/experiments/replay", seoKey: "exp-replay", kind: "invention", order: 40, topics: ["records", "backtracking"] },
  { url: "/research/lab/experiments/gauntlet", seoKey: "exp-gauntlet", kind: "invention", order: 50, topics: ["construction"] },
  { url: "/research/lab/experiments/cloister", seoKey: "exp-cloister", kind: "invention", order: 60, topics: ["local-search", "backtracking"] },
  { url: "/research/lab/experiments/midden", seoKey: "exp-midden", kind: "invention", order: 70, topics: ["local-search"] },
  { url: "/research/lab/experiments/ladder", seoKey: "exp-ladder", kind: "invention", order: 80, topics: ["local-search"] },
  { url: "/research/lab/experiments/lodestone", seoKey: "exp-lodestone", kind: "invention", order: 90, topics: ["construction", "search-space"] },
  { url: "/research/lab/experiments/mosaic", seoKey: "exp-mosaic", kind: "invention", order: 100, topics: ["exact-methods"] },
  { url: "/research/lab/experiments/bandsaw", seoKey: "exp-bandsaw", kind: "invention", order: 110, topics: ["exact-methods"] },
  { url: "/research/lab/experiments/staged", seoKey: "exp-staged", kind: "invention", order: 120, topics: ["construction"] },
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
      topics: leaf.topics ?? [],
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
      topics: d.topics,
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

/** Every research page as a flat list (sections in order, depth-first). */
export function allNavItems(lang: Lang): NavItem[] {
  return researchNav(lang).flatMap((s) => s.items.flatMap((i) => [i, ...i.children]));
}

/** Reading order within one section — drives prev/next (the sidebar is scoped
 *  per section, so page-to-page flow stays inside it too). */
export function sectionReadingOrder(section: NavSection): NavItem[] {
  return section.items.flatMap((i) => [i, ...i.children]);
}

/** The section a research URL belongs to (handles the re-homed flat pages). */
export function findSection(lang: Lang, url: string): NavSection | undefined {
  const sections = researchNav(lang);
  const direct = sections.find((s) =>
    sectionReadingOrder(s).some((i) => i.url === url),
  );
  return direct ?? sections.find((s) => url === s.url || url.startsWith(s.url + "/"));
}

/** Pages tagged with a topic, in global reading order. */
export function topicMembers(lang: Lang, slug: string): NavItem[] {
  return allNavItems(lang).filter((i) => i.topics.includes(slug));
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
