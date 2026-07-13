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

/** The canonical public URL for a page, matching the exact form the host
 *  serves at 200. GitHub Pages serves an extensionless path only at its
 *  trailing-slash variant and 301-redirects the bare form (/research →
 *  /research/), so a canonical/sitemap/og:url without the slash points at a
 *  redirect — which suppresses indexing. Add the slash (except for the root
 *  and any path that already has a file extension, e.g. sitemap.xml). */
export function canonicalUrl(pathname: string): string {
  const withSlash =
    pathname === "/" || /\.[a-z0-9]+$/i.test(pathname) || pathname.endsWith("/")
      ? pathname
      : pathname + "/";
  return ORIGIN + BASE + withSlash;
}

/** The public source repository (edit links, "computed from" links). */
export const REPO_URL = "https://github.com/raphael-anjou/eternity2";
