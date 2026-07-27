// Public-URL config for SEO tags (canonical, hreflang, og:url). Both values are
// baked in at build time:
//
//   VITE_SITE_ORIGIN  the public origin, no trailing slash (default
//                     "https://eternity2.dev"). Set this when deploying to a
//                     different host so the SEO tags point at the real site.
//   BASE_PATH         the path prefix when served under one (e.g. /eternity2),
//                     matching the router basename / Vite base. Read here via
//                     VITE_BASE_PATH so it is available in client/render code.
//
// React Router strips the basename from useLocation().pathname, so canonical
// and alternate URLs must re-add it. absoluteUrl() does both.

// Empty string (e.g. an unset Docker build arg) falls back to the default,
// so `|| ` is intentional here rather than `??`.
const ORIGIN = (import.meta.env["VITE_SITE_ORIGIN"] || "https://eternity2.dev").replace(/\/$/, "");
const BASE = (import.meta.env["VITE_BASE_PATH"] || "").replace(/\/$/, "");

/** Absolute public URL for a basename-stripped in-app path (e.g. "/fr/algorithms"). */
export function absoluteUrl(pathname: string): string {
  return ORIGIN + BASE + pathname;
}

/** Add the canonical trailing slash to an in-app path, matching the exact form
 *  the host serves at 200. GitHub Pages serves an extensionless path only at its
 *  trailing-slash variant and 301-redirects the bare form (/research →
 *  /research/), so any URL without the slash — a canonical/sitemap/og:url, or an
 *  internal <a href> a crawler follows — points at a redirect, which wastes crawl
 *  budget and lands the source in Search Console's "Page with redirect" bucket.
 *
 *  A `?query` or `#hash` suffix is preserved: only the path portion is slashed,
 *  so "/viewer?board=…" → "/viewer/?board=…" and "/glossary#term" →
 *  "/glossary/#term". The root, an already-slashed path, and any path with a file
 *  extension (e.g. sitemap.xml, a raw-markdown .md sibling) are left untouched. */
export function canonicalPath(path: string): string {
  const [, pathname = path, suffix = ""] = /^([^?#]*)([?#].*)?$/.exec(path) ?? [];
  const slashed =
    pathname === "/" || pathname === "" || /\.[a-z0-9]+$/i.test(pathname) || pathname.endsWith("/")
      ? pathname
      : pathname + "/";
  return slashed + suffix;
}

/** The canonical public URL for a page, matching the exact form the host serves
 *  at 200 (see canonicalPath). */
export function canonicalUrl(pathname: string): string {
  return ORIGIN + BASE + canonicalPath(pathname);
}

/** The public source repository (edit links, "computed from" links). */
export const REPO_URL = "https://github.com/raphael-anjou/eternity2";
