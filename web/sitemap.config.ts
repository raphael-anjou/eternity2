import { researchPagePaths, researchPagePathsFr } from "./content.config";

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
// English lives at the root; French mirrors it under /fr. Research pages get a
// /fr twin only when genuinely translated (a `<slug>.fr.mdx` sidecar, or a
// route-generated hub whose content is registry-backed) — see
// researchPagePathsFr. Untranslated research pages have no /fr URL prerendered:
// they fall back to English client-side, and mirroring them would duplicate
// English content under a French URL.
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
 *  at the root (English) and mirrored under /fr. Research pages get a /fr twin
 *  only when translated (researchPagePathsFr); untranslated ones are EN-only. */
export function allRoutePaths(prefix = ""): string[] {
  const base = prefix.replace(/\/$/, "");
  const research = new Set(researchPagePaths());
  const researchFr = new Set(researchPagePathsFr());
  return allPagePaths().flatMap((p) => {
    const en = (base + "/" + p).replace(/\/$/, "") || "/";
    const fr = (base + "/fr/" + p).replace(/\/$/, "");
    // Non-research page: always mirror under /fr.
    if (!research.has(p)) return [en, fr];
    // Research page: /fr twin only when a real French rendering exists.
    return researchFr.has(p) ? [en, fr] : [en];
  });
}

/** One entry per crawlable page, carrying its English URL and — when the page
 *  is genuinely bilingual — its French twin, so the sitemap can emit
 *  <xhtml:link rel="alternate" hreflang="…"> pairs. A page with no French twin
 *  (an untranslated research page) has `fr: null` and gets no alternates, so a
 *  crawler is never pointed at a French URL that was not prerendered. */
export function routePairs(prefix = ""): { en: string; fr: string | null }[] {
  const base = prefix.replace(/\/$/, "");
  const research = new Set(researchPagePaths());
  const researchFr = new Set(researchPagePathsFr());
  return allPagePaths().map((p) => {
    const en = (base + "/" + p).replace(/\/$/, "") || "/";
    const fr = (base + "/fr/" + p).replace(/\/$/, "");
    const hasFr = !research.has(p) || researchFr.has(p);
    return { en, fr: hasFr ? fr : null };
  });
}
