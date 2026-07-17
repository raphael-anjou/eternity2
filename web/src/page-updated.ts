// Declared last-content-update date (YYYY-MM-DD) for each hand-built TSX page,
// keyed by basename-relative path — the SAME key space as sitemap.config.ts
// (PAGE_PATHS) and researchPageLastmod(). English lives at the path; the /fr
// mirror shares its English source's date (same source module, same content).
//
// This mirrors the research wiki's convention exactly: research MDX pages carry
// an `updated:` frontmatter date, hand-declared and bumped when the content
// meaningfully changes. Main pages had no such date, so they emitted neither a
// sitemap <lastmod> nor a dateModified signal. This map closes that gap.
//
// The values are seeded from each page's real last-commit date. Declared, not
// fabricated: bump a page's entry when you change its substantive content, the
// same discipline the frontmatter `updated:` field asks for. One source feeds
// both the page's dateModified JSON-LD (seo.ts) and the sitemap <lastmod>
// (vite.config.ts), EN and /fr alike, so the two can never drift apart.
//
// This module is deliberately import-free (no `@/` aliases, no app deps) so the
// Node-side build config can import it directly, exactly like content.config.ts.

export const PAGE_UPDATED: Record<string, string> = {
  "": "2026-07-15",
  start: "2026-07-15",
  status: "2026-07-15",
  puzzle: "2026-07-15",
  "is-it-a-scam": "2026-07-15",
  playground: "2026-07-15",
  "playground/watch": "2026-07-15",
  "playground/solve": "2026-07-15",
  "playground/paths": "2026-07-15",
  "playground/print": "2026-07-16",
  algorithms: "2026-07-16",
  viewer: "2026-07-15",
  convert: "2026-07-17",
};

/** Normalize a router path (basename already stripped) to the basename-relative
 *  key used by PAGE_UPDATED: drop leading/trailing slashes and the /fr mirror
 *  prefix so an English page and its French twin resolve to the same date. */
export function pageUpdatedKey(pathname: string): string {
  const p = pathname.replace(/^\//, "").replace(/\/$/, "");
  if (p === "fr") return "";
  return p.replace(/^fr\//, "");
}

/** Declared last-update date for a router path, or undefined if none. */
export function pageUpdated(pathname: string): string | undefined {
  return PAGE_UPDATED[pageUpdatedKey(pathname)];
}

/** Path → date map for the sitemap generator (English keys). The /fr fallback
 *  is applied in vite.config via pageUpdatedKey on each emitted route path. */
export function mainPageLastmod(): Record<string, string> {
  return PAGE_UPDATED;
}
