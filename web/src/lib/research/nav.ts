// The research wiki navigation model: the sidebar tree, breadcrumbs, prev/next
// and topic membership all derive from the content manifest — the MDX files
// under web/content/research are the single source of truth.

import type { Lang } from "@/i18n";
import { researchDocs } from "./manifest";
import type { ResearchKind } from "./types";

export interface NavItem {
  url: string;
  title: string;
  description: string;
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
  // The open notebook: structural findings and named search experiments, each
  // credited to its author by a byline (see the `author` frontmatter field and
  // the per-researcher hubs at /research/people/<slug>). Deliberately named for
  // the work, not one person — it scales to any contributor.
  lab: { en: "The lab", fr: "Le laboratoire" },
};

/** Order of the sidebar sections. */
const SECTION_ORDER = ["why", "build", "lab"] as const;

/** Flat research pages that belong to a section other than their first URL
 *  segment (the Build door re-homes the reference pages). */
const SECTION_OF: Record<string, string> = {
  reference: "build",
  papers: "build",
  records: "build",
  history: "build",
  contribute: "build",
  people: "build",
};

function sectionOf(url: string): string {
  const seg = url.split("/")[2] ?? "";
  return SECTION_OF[seg] ?? seg;
}

/** Build the full sidebar tree for a language. */
export function researchNav(lang: Lang): NavSection[] {
  type Flat = NavItem & { order: number };
  const flat: Flat[] = [];

  for (const d of researchDocs(lang)) {
    flat.push({
      url: d.url,
      title: d.title,
      description: d.description,
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
    // The section's own hub page (an index.mdx at the section root) is the
    // section header link, not one of its items.
    const inSection = flat.filter(
      (i) => sectionOf(i.url) === key && i.url !== `/research/${key}`,
    );
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
    experiment: { en: "experiment", fr: "expérience" },
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
  experiment: "bg-emerald-400",
  tool: "bg-amber-400",
  reference: "bg-stone-400",
  concept: "bg-sky-400",
  basin: "bg-sky-400",
  paper: "bg-stone-400",
  page: "bg-stone-400",
};
