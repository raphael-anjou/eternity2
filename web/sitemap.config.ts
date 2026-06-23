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
  "research/why/walls-and-methods",
  "research/why/mismatch-geometry",
  "research/why/prune-vs-speed",
  "research/why/complex-theory",
  "research/why/phase-transition",
  "research/why/rigidity-wall",
  "research/why/sigma-cycles",
  "research/why/forbidden-patterns",
  "research/why/no-forced-moves",
  "research/why/piece-theft",
  "research/why/entropy-area-law",
  "research/why/rare-color-geography",
  "research/build",
  "research/build/run-it-yourself",
  "research/build/dead-ends",
  "research/build/solvers",
  "research/lab",
  "research/lab/inventions",
  "research/lab/findings",
  "research/lab/inventions/palimpsest",
  "research/lab/inventions/keyring",
  "research/lab/inventions/gauntlet",
  "research/lab/inventions/prior",
  "research/lab/inventions/staged",
  "research/lab/inventions/bandsaw",
  "research/lab/inventions/ladder",
  "research/lab/inventions/replay",
  "research/lab/inventions/cloister",
  "research/lab/inventions/midden",
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
