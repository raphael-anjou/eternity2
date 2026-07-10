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
// English lives at the root; French mirrors it under /fr — EXCEPT the research
// wiki, which is English-only (no /fr/research/* URLs are generated; the route
// exists solely to redirect stray French bookmarks back to the English page).
export const PAGE_PATHS = [
  "",
  "start",
  "status",
  "puzzle",
  "is-it-a-scam",
  "playground",
  "playground/watch",
  "playground/solve",
  "playground/paths",
  "playground/print",
  "algorithms",
  "viewer",
  "convert",
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

/** Every public URL path, normalized, with the prefix applied. Main pages exist
 *  at the root (English) and mirrored under /fr; research pages are English-only
 *  (no /fr twin is emitted). */
export function allRoutePaths(prefix = ""): string[] {
  const base = prefix.replace(/\/$/, "");
  const research = new Set(researchPagePaths());
  return allPagePaths().flatMap((p) => {
    const en = (base + "/" + p).replace(/\/$/, "") || "/";
    // Research is English-only: skip the /fr mirror for research paths.
    if (research.has(p)) return [en];
    const fr = (base + "/fr/" + p).replace(/\/$/, "");
    return [en, fr];
  });
}
