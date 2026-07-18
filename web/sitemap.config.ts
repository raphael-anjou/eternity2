import {
  researchPagePaths,
  researchPagePathsFor,
  LANG_PREFIXES,
  type Lang,
} from "./content.config";

const LANGS = LANG_PREFIXES;
const PREFIXED_LANGS = LANG_PREFIXES.filter((l) => l.prefix !== "");

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
// English lives at the root; every other registry language mirrors it under its
// prefix (French under /fr, Spanish under /es). Research pages get a translated
// twin only when genuinely translated (a `<slug>.<lang>.mdx` sidecar, or a
// route-generated hub whose content is registry-backed) — see
// researchPagePathsFor. Untranslated research pages have no translated URL
// prerendered: they fall back to English client-side, and mirroring them would
// duplicate English content under a translated URL.
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

// The set of research page paths that have a genuine rendering in each
// non-English language, computed once. A non-research page is translated in
// every language (it mirrors under every prefix); a research page is translated
// in language L only when it's in that language's set.
function translatedSets(): Record<string, Set<string>> {
  const out: Record<string, Set<string>> = {};
  for (const l of PREFIXED_LANGS) out[l.code] = new Set(researchPagePathsFor(l.code));
  return out;
}

/** Is the page at basename-relative path `p` available in language `lang`?
 *  English: always. A non-English language: always for non-research pages,
 *  and for research pages only when a real translation exists. */
function availableIn(p: string, lang: Lang, research: Set<string>, translated: Record<string, Set<string>>): boolean {
  if (lang === "en") return true;
  if (!research.has(p)) return true; // non-research page mirrors under every prefix
  return translated[lang]?.has(p) ?? false;
}

/** The full URL path for a basename-relative page in a language, with prefix. */
function localizedPath(base: string, prefix: string, p: string): string {
  const seg = [base, prefix, p].filter(Boolean).join("/");
  return ("/" + seg).replace(/\/+/g, "/").replace(/\/$/, "") || "/";
}

/** Every public URL path, normalized, with the prefix applied. Main pages exist
 *  at the root (English) and mirrored under every language prefix. Research
 *  pages get a translated twin only when translated (researchPagePathsFor);
 *  untranslated ones are EN-only. */
export function allRoutePaths(prefix = ""): string[] {
  const base = prefix.replace(/\/$/, "");
  const research = new Set(researchPagePaths());
  const translated = translatedSets();
  return allPagePaths().flatMap((p) =>
    LANGS.filter((l) => availableIn(p, l.code, research, translated)).map((l) =>
      localizedPath(base, l.prefix, p),
    ),
  );
}

/** One entry per crawlable URL: its `loc`, the `lang` it renders in, and the
 *  full set of hreflang `alternates` for the page it belongs to (every language
 *  the page is available in, plus x-default → English). A page available in one
 *  language only (an untranslated research page) gets a single entry whose
 *  alternates are just itself + x-default. Built from the same data the app's
 *  <head> hreflang uses, so the two channels never disagree. */
export function routeEntries(prefix = ""): {
  loc: string;
  lang: Lang;
  alternates: { lang: Lang; loc: string }[];
  xDefault: string;
}[] {
  const base = prefix.replace(/\/$/, "");
  const research = new Set(researchPagePaths());
  const translated = translatedSets();
  const out: { loc: string; lang: Lang; alternates: { lang: Lang; loc: string }[]; xDefault: string }[] = [];
  for (const p of allPagePaths()) {
    const langs = LANGS.filter((l) => availableIn(p, l.code, research, translated));
    const alternates = langs.map((l) => ({ lang: l.code, loc: localizedPath(base, l.prefix, p) }));
    const xDefault = localizedPath(base, "", p); // English
    for (const l of langs) {
      out.push({ loc: localizedPath(base, l.prefix, p), lang: l.code, alternates, xDefault });
    }
  }
  return out;
}
