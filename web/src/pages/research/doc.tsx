/* eslint-disable react-hooks/static-components --
   `pages` holds one stable lazy() wrapper per content file, created once at
   module scope; resolving the active page during render is a map lookup, not
   a component creation. */

// Catch-all route for research wiki pages (research/* and fr/research/*).
// Resolves the URL against the content manifest, lazy-loads the page's MDX
// module (per-page chunks via import.meta.glob), and renders it in the docs
// shell. Explicit TSX routes in routes.ts outrank this splat, so legacy pages
// keep working until their MDX replacement lands — migration is page-by-page.

import { lazy, Suspense, type ComponentType } from "react";
import { useLocation } from "react-router";
import type { MDXContent } from "mdx/types";
import { langFromPath, useT } from "@/i18n";
import { absoluteUrl } from "@/site";
import { researchDoc } from "@/lib/research/manifest";
import { DocsShell } from "@/components/docs/DocsShell";
import { mdxComponents } from "@/components/docs/mdx-map";
import { LocalizedLink } from "@/components/LocalizedLink";

const modules = import.meta.glob("/content/research/**/*.mdx") as Record<
  string,
  () => Promise<{ default: MDXContent }>
>;

// One stable lazy wrapper per content file, created at module scope (lazy()
// only wraps — nothing loads until a page actually renders).
const pages: ReadonlyMap<string, ComponentType<{ components?: typeof mdxComponents }>> = new Map(
  Object.entries(modules).map(([p, load]) => [p, lazy(load)]),
);

/** Language-neutral site path for the current location ("/fr/research/x" → "/research/x"). */
function neutralPath(pathname: string): string {
  return pathname.replace(/^\/fr(?=\/|$)/, "").replace(/\/$/, "");
}

const SUFFIX = " · Eternity II";

export function meta({ location }: { location: { pathname: string } }) {
  const lang = langFromPath(location.pathname);
  const doc = researchDoc(lang, neutralPath(location.pathname));
  if (!doc) {
    return [{ title: "Not found" + SUFFIX }];
  }
  const title = doc.title + SUFFIX;
  return [
    { title },
    { name: "description", content: doc.description },
    { property: "og:title", content: title },
    { property: "og:description", content: doc.description },
    { property: "og:url", content: absoluteUrl(location.pathname) },
  ];
}

const NF = {
  en: {
    title: "Page not found",
    body: "No research page lives at this address.",
    back: "← Research overview",
  },
  fr: {
    title: "Page introuvable",
    body: "Aucune page de recherche n'existe à cette adresse.",
    back: "← Vue d'ensemble de la recherche",
  },
};

function NotFound() {
  const t = useT(NF);
  return (
    <div className="py-24 text-center">
      <h1 className="text-3xl font-bold tracking-tight">{t.title}</h1>
      <p className="mt-3 text-muted-foreground">{t.body}</p>
      <LocalizedLink
        to="/research"
        className="mt-6 inline-block text-sm font-medium underline hover:text-foreground"
      >
        {t.back}
      </LocalizedLink>
    </div>
  );
}

export default function ResearchDocPage() {
  const { pathname } = useLocation();
  const lang = langFromPath(pathname);
  const doc = researchDoc(lang, neutralPath(pathname));
  const Content = doc ? pages.get(`/content/research/${doc.file}`) : undefined;
  if (!doc || !Content) return <NotFound />;
  return (
    <DocsShell doc={doc}>
      <Suspense fallback={null}>
        <Content components={mdxComponents} />
      </Suspense>
    </DocsShell>
  );
}
