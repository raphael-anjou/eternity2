import { researchPagePaths } from "./content.config";

// Canonical list of crawlable page paths (basename-relative, no origin).
// This is the single source of truth shared by the React Router prerender list
// and the build-time sitemap generator, so the two can never drift.
//
// Two sources feed it:
//   1. PAGE_PATHS — the hand-listed TSX pages below.
//   2. web/content/research/**/*.mdx — research wiki pages, scanned by
//      content.config.ts. Adding an MDX file IS the registration; nothing
//      else to touch.
//
// English lives at the root; French mirrors it under /fr.
export const PAGE_PATHS = [
  "",
  "puzzle",
  "playground",
  "playground/watch",
  "playground/solve",
  "playground/paths",
  "playground/print",
  "algorithms",
  "research",
  "research/why",
  "research/build",
  "research/build/run-it-yourself",
  "research/build/dead-ends",
  "research/build/solvers",
  "research/lab",
  "research/lab/experiments",
  "research/lab/experiments/log",
  "research/lab/findings",
  "research/lab/experiments/palimpsest",
  "research/lab/experiments/keyring",
  "research/lab/experiments/gauntlet",
  "research/lab/experiments/prior",
  "research/lab/experiments/staged",
  "research/lab/experiments/bandsaw",
  "research/lab/experiments/ladder",
  "research/lab/experiments/replay",
  "research/lab/experiments/cloister",
  "research/lab/experiments/midden",
  "research/lab/experiments/mosaic",
  "research/lab/experiments/lodestone",
  "research/reference",
  "research/papers",
  "research/records",
  "viewer",
] as const;

/** All crawlable paths: hand-listed TSX pages + scanned research MDX pages.
 *  A page must live in exactly one world — a leftover PAGE_PATHS entry for a
 *  migrated page would prerender through the wrong route, so overlap is fatal. */
export function allPagePaths(): string[] {
  const scanned = researchPagePaths();
  const overlap = scanned.filter((p) => (PAGE_PATHS as readonly string[]).includes(p));
  if (overlap.length > 0) {
    throw new Error(
      `sitemap: path(s) registered both in PAGE_PATHS and as research MDX content: ` +
        `${overlap.join(", ")} — remove the PAGE_PATHS entry (and the old TSX route) when migrating a page.`,
    );
  }
  return [...PAGE_PATHS, ...scanned];
}

/** Every public URL path (en at root + fr under /fr), normalized, with the prefix applied. */
export function allRoutePaths(prefix = ""): string[] {
  const base = prefix.replace(/\/$/, "");
  return allPagePaths().flatMap((p) => {
    const en = (base + "/" + p).replace(/\/$/, "") || "/";
    const fr = (base + "/fr/" + p).replace(/\/$/, "");
    return [en, fr];
  });
}
