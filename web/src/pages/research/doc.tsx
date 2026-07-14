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
import { canonicalUrl, absoluteUrl } from "@/site";
import { researchDoc, researchTopic, researchAuthor, metaDescriptionFor } from "@/lib/research/manifest";
import { findSection } from "@/lib/research/nav";
import { RESEARCH_REDIRECTS } from "@/lib/research/redirects";
import { DocsShell } from "@/components/docs/DocsShell";
import { TopicHub, TopicsIndex } from "@/components/docs/TopicPages";
import { PersonHub } from "@/components/docs/PeoplePages";
import { GlossaryPage } from "@/components/docs/GlossaryPage";
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

const GLOSSARY_TITLE = { en: "Glossary", fr: "Glossaire" } as const;
const GLOSSARY_DESC = {
  en: "Every Eternity II term the research wiki uses, defined once: community jargon, the loaded computer-science words, and the board notation, each linking to the page that goes deep.",
  fr: "Chaque terme d'Eternity II qu'emploie le wiki, défini une fois : jargon de la communauté, mots d'informatique au sens précis, et notation des plateaux, chacun renvoyant à la page qui approfondit.",
} as const;

export function meta({ location }: { location: { pathname: string } }) {
  const lang = langFromPath(location.pathname);
  const path = neutralPath(location.pathname);
  // Meta/og description is trimmed to a SERP-safe length (~155 chars, word
  // boundary). This covers the route-generated pages too — glossary, topics and
  // the person hubs, whose descriptions come from long author bios. MDX docs
  // pass their own already-resolved short form (see the doc branch below), which
  // is short enough to pass through untouched.
  const pack = (title: string, description: string) => [
    { title },
    { name: "description", content: metaDescriptionFor({ description }) },
    { property: "og:title", content: title },
    { property: "og:description", content: metaDescriptionFor({ description }) },
    { property: "og:url", content: canonicalUrl(location.pathname) },
  ];

  if (path === "/research/glossary") return pack(GLOSSARY_TITLE[lang] + SUFFIX, GLOSSARY_DESC[lang]);

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
  // The <meta description>/og tag use the short form (explicit metaDescription,
  // else the lede truncated to ~155 chars). The full lede stays on-page via
  // DocsShell. The Article node also uses the short form for a clean snippet.
  const metaDesc = metaDescriptionFor(doc);
  // Every research MDX page ships a raw-markdown sibling at the same URL with
  // `.md` appended (…/research/<slug>.md; the overview is research/index.md).
  // Advertise it declaratively so AI agents and markdown-aware clients can find
  // the clean source without User-Agent sniffing or a redirect (which would be
  // cloaking). Only real MDX docs have a sibling — the route-generated hubs,
  // topics, people and glossary above return before reaching here.
  const mdRel = path === "/research" ? "/research/index.md" : `${path}.md`;
  // TechArticle structured data: helps this page qualify as a citable article
  // (and surfaces its freshness date) in search and AI answers. dateModified
  // comes from the frontmatter `updated:`; author is the research author when
  // the page declares one, else the site.
  const pageUrl = canonicalUrl(location.pathname);
  const ldArticle: Record<string, unknown> = {
    "@context": "https://schema.org",
    "@type": "TechArticle",
    headline: doc.title,
    description: metaDesc,
    inLanguage: "en",
    url: pageUrl,
    mainEntityOfPage: pageUrl,
    isPartOf: { "@id": "https://eternity2.dev/#website" },
    publisher: { "@id": "https://eternity2.dev/#org" },
    ...(doc.updated ? { dateModified: doc.updated } : {}),
    ...(doc.date ? { datePublished: doc.date } : {}),
    ...(doc.author
      ? { author: { "@type": "Person", name: researchAuthor("en", doc.author)?.name ?? doc.author } }
      : { author: { "@id": "https://eternity2.dev/#org" } }),
  };
  // BreadcrumbList structured data mirrors the visual trail (Research →
  // section → page). Eligible for breadcrumb rich results and reinforces the
  // site hierarchy to crawlers. Built from the same findSection() chain the
  // DocsShell breadcrumb renders, so the two never disagree.
  const section = findSection("en", doc.url);
  const crumbs: { name: string; url: string }[] = [
    { name: "Research", url: absoluteUrl("/research") },
  ];
  if (section && section.url !== doc.url) {
    crumbs.push({ name: section.label, url: absoluteUrl(section.url) });
  }
  crumbs.push({ name: doc.title, url: pageUrl });
  const ldBreadcrumb = {
    "@context": "https://schema.org",
    "@type": "BreadcrumbList",
    itemListElement: crumbs.map((c, i) => ({
      "@type": "ListItem",
      position: i + 1,
      name: c.name,
      item: c.url,
    })),
  };
  return [
    ...pack(doc.title + SUFFIX, metaDesc),
    { tagName: "link", rel: "alternate", type: "text/markdown", href: absoluteUrl(mdRel) },
    { "script:ld+json": ldBreadcrumb },
    { "script:ld+json": ldArticle },
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

  if (path === "/research/glossary") return <GlossaryPage />;

  const doc = researchDoc(lang, path);
  const Content = doc ? pages.get(`/content/research/${doc.file}`) : undefined;
  if (!doc || !Content) return <NotFound />;
  // The people gallery is a real content page, but it heads the People tab and
  // its person hubs, so it shows the alphabetical contributor rail (not the
  // section rail) to match them.
  const sidebarVariant = path === "/research/people" ? "people" : undefined;
  return (
    <DocsShell doc={doc} {...(sidebarVariant ? { sidebarVariant } : {})}>
      <Suspense fallback={null}>
        <Content components={mdxComponents} />
      </Suspense>
    </DocsShell>
  );
}
