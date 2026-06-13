import { Links, Meta, Outlet, Scripts, ScrollRestoration, useLocation } from "react-router";
import type { ReactNode } from "react";
import "./index.css";
import { LangProvider, langFromPath, pathForLang } from "@/i18n";
import { absoluteUrl } from "@/site";

// The document shell (formerly index.html). React Router injects <Meta> and
// <Links> from the route module `meta`/`links` exports, and <Scripts> boots
// the client bundle. ssr:false means this is rendered once per prerendered
// path at build time, producing real static HTML per route × language.

const GA_ID = import.meta.env.VITE_GA_ID as string | undefined;

export function meta() {
  // Site-wide defaults; individual routes override title/description.
  // NOTE: charset is set as a literal <meta charSet> in Layout's <head>, not
  // here — the descriptor form does not reliably reach the prerendered HTML.
  return [
    { name: "viewport", content: "width=device-width, initial-scale=1.0" },
    { property: "og:type", content: "website" },
    { property: "og:image", content: "https://eternity2.dev/og.png" },
    { name: "twitter:card", content: "summary_large_image" },
  ];
}

export function links() {
  return [{ rel: "icon", type: "image/svg+xml", href: "/favicon.svg" }];
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
  return (
    <html lang={lang}>
      <head>
        {/* charset MUST be the first thing in <head> (within the first 1024
            bytes) and emitted as a literal tag. React Router's <Meta> does not
            reliably render the { charSet } descriptor into prerendered HTML, so
            without this the browser guesses the encoding: it reads the UTF-8
            bytes C2 A0 (a &nbsp;, e.g. "Eternity II") as Latin-1 → "Â ",
            which then mismatches the UTF-8-decoding client and breaks
            hydration. */}
        <meta charSet="utf-8" />
        <Meta />
        <Links />
        <link rel="canonical" href={absoluteUrl(pathname)} />
        <link rel="alternate" hrefLang="en" href={absoluteUrl(enPath)} />
        <link rel="alternate" hrefLang="fr" href={absoluteUrl(frPath)} />
        <link rel="alternate" hrefLang="x-default" href={absoluteUrl(enPath)} />
        {/* Google Analytics (GA4). Injected only when a valid measurement ID
            is present at build time; inert on local dev and forks. The router
            sends page views (see layout.tsx) because automatic ones would not
            fire on client-side navigations. */}
        {GA_ID && /^G-[A-Z0-9]{4,}$/.test(GA_ID) && (
          <>
            <script async src={`https://www.googletagmanager.com/gtag/js?id=${GA_ID}`} />
            <script
              dangerouslySetInnerHTML={{
                __html: `window.dataLayer=window.dataLayer||[];function gtag(){dataLayer.push(arguments);}gtag('js',new Date());gtag('config','${GA_ID}',{anonymize_ip:true,send_page_view:false});`,
              }}
            />
          </>
        )}
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
