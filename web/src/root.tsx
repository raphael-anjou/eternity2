import { Links, Meta, Outlet, Scripts, ScrollRestoration, useLocation } from "react-router";
import type { ReactNode } from "react";
import "./index.css";
import { LangProvider, langFromPath, pathForLang } from "@/i18n";
import { canonicalUrl } from "@/site";
import { hasFrenchTwin } from "@/lib/research/i18n-routes";
// The Latin subset of the Geist variable font, imported as a hashed URL so it
// can be preloaded. This is the one woff2 that basic Latin text (EN + FR body
// and the big hero headline) resolves to; preloading it means it arrives
// before first paint, so the headline renders in Geist from the start instead
// of swapping fallback→Geist afterwards — which is what caused the residual
// font-driven layout shift (CLS). The @import in index.css still defines the
// @font-face and pulls the other subsets on demand.
import geistLatinUrl from "@fontsource-variable/geist/files/geist-latin-wght-normal.woff2?url";

// The document shell (formerly index.html). React Router injects <Meta> and
// <Links> from the route module `meta`/`links` exports, and <Scripts> boots
// the client bundle. ssr:false means this is rendered once per prerendered
// path at build time, producing real static HTML per route × language.

export function meta() {
  // Site-wide defaults; individual routes override title/description.
  // NOTE: charset AND viewport are emitted as literal <meta> tags in Layout's
  // <head>, not here — the descriptor form does not reliably reach the
  // prerendered HTML (ssr:false + prerender silently drops them). A MISSING
  // viewport is why the desktop site renders correct-but-tiny on phones:
  // without it, mobile browsers assume a ~980px layout viewport and zoom the
  // whole page out to fit. See the literal <meta name="viewport"> below.
  // Site-wide social tags and structured data are NOT returned here: in React
  // Router 7 a child route's `meta` export REPLACES the root route's meta
  // wholesale (they do not merge), so anything returned here vanishes on every
  // page that exports its own meta — which is all of them. Instead these
  // invariant tags are rendered as literal elements in Layout's <head> below,
  // exactly like charSet/viewport, so they appear on every prerendered page
  // regardless of the route's own meta. Per-page meta() still adds the page
  // title/description and any Article/FAQ JSON-LD on top.
  return [];
}

// Site-wide structured data (WebSite + publisher Organization), rendered as a
// literal <script> in Layout. Per-page Article/FAQ nodes reference these by
// @id. Kept as a constant so the JSON is stringified once.
const SITE_JSONLD = {
  "@context": "https://schema.org",
  "@graph": [
    {
      "@type": "WebSite",
      "@id": "https://eternity2.dev/#website",
      url: "https://eternity2.dev/",
      name: "Eternity II · community",
      description:
        "An open, bilingual educational hub for the Eternity II edge-matching puzzle: play it, watch real solvers run in your browser, learn the algorithms, and read the community research.",
      inLanguage: ["en", "fr"],
      publisher: { "@id": "https://eternity2.dev/#org" },
    },
    {
      "@type": "Organization",
      "@id": "https://eternity2.dev/#org",
      name: "Eternity II community site",
      url: "https://eternity2.dev/",
      logo: "https://eternity2.dev/og.png",
    },
  ],
};

export function links() {
  return [
    { rel: "icon", type: "image/svg+xml", href: "/favicon.svg" },
    // Preload the Latin Geist subset so the headline paints in the final font
    // (no fallback→Geist swap → no font-driven CLS). crossOrigin is required on
    // font preloads: the browser fetches fonts in anonymous CORS mode even
    // same-origin, and the hint must match or it won't be reused.
    {
      rel: "preload",
      as: "font" as const,
      type: "font/woff2",
      href: geistLatinUrl,
      crossOrigin: "anonymous" as const,
    },
  ];
}

export function Layout({ children }: { children: ReactNode }) {
  // <html lang> must match the page language so crawlers and screen readers
  // see the right language. Derived from the URL (/fr/... → fr).
  const { pathname } = useLocation();
  const lang = langFromPath(pathname);
  // hreflang + canonical: tell search engines the EN and FR URLs are
  // translations of each other, and which is canonical for this page.
  const enPath = pathForLang(pathname, "en");
  const frPath = pathForLang(pathname, "fr");
  // Only advertise a French alternate when this page actually has one. An
  // untranslated research page has no prerendered /fr URL, so pointing a crawler
  // at it via hreflang would send it to the SPA fallback, not a real page.
  const frTwin = hasFrenchTwin(enPath);
  return (
    <html lang={lang}>
      <head>
        {/* charset MUST be the first thing in <head> (within the first 1024
            bytes) and emitted as a literal tag. React Router's <Meta> does not
            reliably render the { charSet } descriptor into prerendered HTML, so
            without this the browser guesses the encoding: it reads the UTF-8
            bytes C2 A0 (a &nbsp;, e.g. the gap in an "Eternity II" title) as Latin-1,
            which then mismatches the UTF-8-decoding client and breaks
            hydration. */}
        <meta charSet="utf-8" />
        {/* viewport MUST be a literal tag for the same reason as charSet above:
            React Router's <Meta> descriptor form is dropped from the prerendered
            HTML under ssr:false. Without this tag mobile browsers fall back to a
            ~980px layout viewport and render the desktop layout zoomed-out tiny.
            viewport-fit=cover lets content extend under iOS notch/home-bar. */}
        <meta name="viewport" content="width=device-width, initial-scale=1, viewport-fit=cover" />
        <Meta />
        <Links />
        <link rel="canonical" href={canonicalUrl(pathname)} />
        {frTwin && <link rel="alternate" hrefLang="en" href={canonicalUrl(enPath)} />}
        {frTwin && <link rel="alternate" hrefLang="fr" href={canonicalUrl(frPath)} />}
        <link rel="alternate" hrefLang="x-default" href={canonicalUrl(enPath)} />
        {/* Invariant site-wide social + structured-data tags. These live here as
            literal elements (not in a route `meta` export) because in React
            Router 7 a child route's meta REPLACES the root route's meta, so
            anything the root returns is dropped on every page. Per-page og:title
            / og:description still come from <Meta/> above; these add the shared
            image, card type and site-level JSON-LD on top of every page. */}
        <meta property="og:type" content="website" />
        <meta property="og:image" content="https://eternity2.dev/og.png" />
        <meta name="twitter:card" content="summary_large_image" />
        <meta name="twitter:image" content="https://eternity2.dev/og.png" />
        <script
          type="application/ld+json"
          dangerouslySetInnerHTML={{ __html: JSON.stringify(SITE_JSONLD) }}
        />
        {/* Google Analytics (GA4) is loaded lazily after the page is idle (see
            loadAnalyticsWhenIdle in layout.tsx), so it stays off the critical
            render path. Page views are sent by the router. */}
      </head>
      <body>
        <LangProvider>{children}</LangProvider>
        <ScrollRestoration />
        <Scripts />
      </body>
    </html>
  );
}

export default function Root() {
  return <Outlet />;
}
