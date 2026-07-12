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
import { Navigate, useLocation } from "react-router";
import type { MDXContent } from "mdx/types";
import { langFromPath, useT } from "@/i18n";
import { absoluteUrl } from "@/site";
import { researchDoc, researchTopic, researchAuthor } from "@/lib/research/manifest";
import { RESEARCH_REDIRECTS } from "@/lib/research/redirects";
import { DocsShell } from "@/components/docs/DocsShell";
import { TopicHub, TopicsIndex } from "@/components/docs/TopicPages";
import { PersonHub } from "@/components/docs/PeoplePages";
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

/** Topic hub URL → slug ("" for the topics index), or null if not a topic URL. */
function topicSlugFor(path: string): string | null {
  if (path === "/research/topics") return "";
  const m = /^\/research\/topics\/([a-z0-9-]+)$/.exec(path);
  return m?.[1] ?? null;
}

/** Researcher hub URL → author slug, or null. The index /research/people is a
 *  real content page (the community gallery), so only sub-paths match here. */
function personSlugFor(path: string): string | null {
  const m = /^\/research\/people\/([a-z0-9-]+)$/.exec(path);
  return m?.[1] ?? null;
}

const TOPICS_TITLE = { en: "Research topics", fr: "Thèmes de recherche" } as const;
const TOPICS_DESC = {
  en: "The Eternity II research wiki sliced by theme: structure, search-space reduction, backtracking, speed, local search, exact methods, hardware and more.",
  fr: "Le wiki de recherche Eternity II par thème : structure, réduction de l'espace de recherche, retour arrière, vitesse, recherche locale, méthodes exactes, matériel…",
} as const;

export function meta({ location }: { location: { pathname: string } }) {
  const lang = langFromPath(location.pathname);
  const path = neutralPath(location.pathname);
  const pack = (title: string, description: string) => [
    { title },
    { name: "description", content: description },
    { property: "og:title", content: title },
    { property: "og:description", content: description },
    { property: "og:url", content: absoluteUrl(location.pathname) },
  ];

  const topicSlug = topicSlugFor(path);
  if (topicSlug === "") return pack(TOPICS_TITLE[lang] + SUFFIX, TOPICS_DESC[lang]);
  if (topicSlug !== null) {
    const topic = researchTopic(lang, topicSlug);
    if (topic) return pack(topic.label + SUFFIX, topic.description);
  }

  const personSlug = personSlugFor(path);
  if (personSlug !== null) {
    const author = researchAuthor(lang, personSlug);
    if (author) {
      const desc =
        author.bio ??
        author.tagline ??
        (lang === "fr"
          ? `Les travaux de ${author.name} sur Eternity II.`
          : `${author.name}'s work on Eternity II.`);
      return pack(author.name + SUFFIX, desc);
    }
  }

  const doc = researchDoc(lang, path);
  if (!doc) return [{ title: "Not found" + SUFFIX }];
  return pack(doc.title + SUFFIX, doc.description);
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
  const path = neutralPath(pathname);

  // The research wiki is English-only. Any /fr/research/* URL (a stray bookmark
  // from when French pages existed) redirects to the English page. No /fr
  // research URLs are generated anymore; this route only catches old links.
  if (langFromPath(pathname) === "fr") {
    return <Navigate to={path} replace />;
  }
  const lang = "en" as const;

  // Pages that moved (Build reorg into technique-family sub-hubs) redirect old
  // URLs to their new home, so bookmarks and external links don't 404.
  const moved = RESEARCH_REDIRECTS[path];
  if (moved) return <Navigate to={moved} replace />;

  const topicSlug = topicSlugFor(path);
  if (topicSlug === "") return <TopicsIndex />;
  if (topicSlug !== null) {
    return researchTopic(lang, topicSlug) ? <TopicHub slug={topicSlug} /> : <NotFound />;
  }

  const personSlug = personSlugFor(path);
  if (personSlug !== null) {
    return researchAuthor(lang, personSlug) ? <PersonHub slug={personSlug} /> : <NotFound />;
  }

  const doc = researchDoc(lang, path);
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
