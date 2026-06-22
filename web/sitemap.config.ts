// Canonical list of crawlable page paths (basename-relative, no origin).
// This is the single source of truth shared by the React Router prerender list
// and the build-time sitemap generator, so the two can never drift.
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
  "research/why/forbidden-patterns",
  "research/build",
  "research/lab",
  "research/reference",
  "research/papers",
  "research/records",
  "viewer",
] as const;

/** Every public URL path (en at root + fr under /fr), normalized, with the prefix applied. */
export function allRoutePaths(prefix = ""): string[] {
  const base = prefix.replace(/\/$/, "");
  return PAGE_PATHS.flatMap((p) => {
    const en = (base + "/" + p).replace(/\/$/, "") || "/";
    const fr = (base + "/fr/" + p).replace(/\/$/, "");
    return [en, fr];
  });
}
