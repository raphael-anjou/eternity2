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
// KaTeX math styling, scoped to the research wiki. This is the single route
// module behind every /research/** page (the only pages that render math), so
// importing the stylesheet here keeps it out of the global critical CSS and
// lets Vite load it only with this route's chunk. See src/index.css.
import "katex/dist/katex.min.css";
import type { MDXContent } from "mdx/types";
import { langFromPath, useT, pick, pathForLang, neutralPath as langNeutral } from "@/i18n";
import { canonicalUrl, absoluteUrl } from "@/site";
import {
  researchDoc,
  researchTopic,
  researchAuthor,
  topicUpdated,
  authorUpdated,
  metaDescriptionFor,
} from "@/lib/research/manifest";
import { findSection } from "@/lib/research/nav";
import { RESEARCH_REDIRECTS } from "@/lib/research/redirects";
import { DocsShell } from "@/components/docs/DocsShell";
import { TopicHub, TopicsIndex } from "@/components/docs/TopicPages";
import { PersonHub } from "@/components/docs/PeoplePages";
import { GlossaryPage } from "@/components/docs/GlossaryPage";
import { ReproduceIndexPage } from "@/components/docs/ReproduceIndexPage";
import {
  ContributionIndex,
  ContributionHub,
  contributionRouteFor,
  contributionMeta,
} from "@/components/docs/ContributionPages";
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

/** Language-neutral site path for the current location ("/fr/research/x" or
 *  "/es/research/x" → "/research/x"), with any trailing slash trimmed. Wraps the
 *  shared registry-driven stripper so a new language is handled automatically. */
function neutralPath(pathname: string): string {
  return langNeutral(pathname).replace(/\/$/, "");
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

const TOPICS_TITLE = {
  en: "Research topics",
  fr: "Thèmes de recherche",
  es: "Temas de investigación",
} as const;
const TOPICS_DESC = {
  en: "The Eternity II research wiki sliced by theme: structure, search-space reduction, backtracking, speed, local search, exact methods, hardware and more.",
  fr: "Le wiki de recherche Eternity II par thème : structure, réduction de l'espace de recherche, retour arrière, vitesse, recherche locale, méthodes exactes, matériel…",
  es: "El wiki de investigación de Eternity II ordenado por tema: estructura, reducción del espacio de búsqueda, backtracking, velocidad, búsqueda local, métodos exactos, hardware y más.",
} as const;

const GLOSSARY_TITLE = { en: "Glossary", fr: "Glossaire", es: "Glosario" } as const;
const GLOSSARY_DESC = {
  en: "Every Eternity II term the research wiki uses, defined once: community jargon, the loaded computer-science words, and the board notation, each linking to the page that goes deep.",
  fr: "Chaque terme d'Eternity II qu'emploie le wiki, défini une fois : jargon de la communauté, mots d'informatique au sens précis, et notation des plateaux, chacun renvoyant à la page qui approfondit.",
  es: "Cada término de Eternity II que emplea el wiki, definido una sola vez: la jerga de la comunidad, los términos precisos de informática y la notación de los tableros, cada uno enlazando con la página que profundiza.",
} as const;

const REPRODUCE_TITLE = {
  en: "Reproduce index",
  fr: "Index de reproduction",
  es: "Índice de reproducción",
} as const;
const REPRODUCE_DESC = {
  en: "Every Eternity II research result that ships a reproduce command, in one sortable table: the command, whether it re-runs the search or only re-verifies a stored board, and the expected wall-clock and core-hours cost.",
  fr: "Chaque résultat de recherche Eternity II livré avec une commande de reproduction, en un tableau triable : la commande, si elle relance la recherche ou revérifie seulement un plateau enregistré, et le coût attendu en temps réel et cœur-heures.",
  es: "Cada resultado de investigación de Eternity II que incluye un comando de reproducción, en una tabla ordenable: el comando, si vuelve a ejecutar la búsqueda o solo reverifica un tablero guardado, y el coste esperado en tiempo real y horas-núcleo.",
} as const;

const BY_CONTRIB_TITLE = {
  en: "By contribution",
  fr: "Par contribution",
  es: "Por contribución",
} as const;
const BY_CONTRIB_DESC = {
  en: "The Eternity II research wiki sliced by what kind of result each page is: the solvers that score, the analyses and theory, the measurements and tools, and the dead ends kept as first-class negatives.",
  fr: "Le wiki de recherche Eternity II découpé selon le type de résultat de chaque page : les solveurs qui scorent, les analyses et la théorie, les mesures et les outils, et les impasses gardées comme résultats négatifs à part entière.",
  es: "El wiki de investigación de Eternity II organizado según qué tipo de resultado es cada página: los solucionadores que puntúan, los análisis y la teoría, las mediciones y las herramientas, y los callejones sin salida guardados como resultados negativos de pleno derecho.",
} as const;

export function meta({ location }: { location: { pathname: string } }) {
  const lang = langFromPath(location.pathname);
  const path = neutralPath(location.pathname);
  // A moved URL renders as a redirect (see the component below), so its tags
  // must describe the DESTINATION, not the old path — otherwise the head
  // advertises a titled, self-canonical page that immediately bounces. Recurse
  // once on the target (the table has no chains) in the visitor's language, so
  // title / og:url / canonical all match where the reader actually lands.
  const moved = RESEARCH_REDIRECTS[path];
  if (moved) return meta({ location: { pathname: pathForLang(moved, lang) } });
  // pack() takes the FINAL meta/og description verbatim — it does not trim. The
  // route-generated callers below (glossary, topics, person hubs) whose text
  // comes from long registry prose pre-trim it to a SERP-safe length with
  // metaDescriptionFor(); the MDX-doc branch passes its authored `metaDescription`
  // (or the fallback metaDescriptionFor already resolved), which must reach the
  // tag in full so the deliberately fuller descriptions are not re-truncated.
  const pack = (title: string, description: string) => [
    { title },
    { name: "description", content: description },
    { property: "og:title", content: title },
    { property: "og:description", content: description },
    { property: "og:url", content: canonicalUrl(location.pathname) },
  ];
  // A raw registry string trimmed to a SERP-safe length for the route-generated
  // hub pages (their descriptions are long author bios / section blurbs).
  const trimmed = (description: string) => metaDescriptionFor({ description });

  // A WebPage node carrying a route-generated hub's derived dateModified, so its
  // freshness signal matches the sitemap <lastmod>. Empty (no node) when the hub
  // aggregates no dated pages — a missing date beats a fabricated one.
  const webPageLd = (updated: string | undefined) =>
    updated
      ? [
          {
            "script:ld+json": {
              "@context": "https://schema.org",
              "@type": "WebPage",
              url: canonicalUrl(location.pathname),
              dateModified: updated,
              inLanguage: lang,
            },
          },
        ]
      : [];

  if (path === "/research/glossary")
    return pack(pick(GLOSSARY_TITLE, lang) + SUFFIX, trimmed(pick(GLOSSARY_DESC, lang)));

  if (path === "/research/build/reproduce")
    return pack(pick(REPRODUCE_TITLE, lang) + SUFFIX, trimmed(pick(REPRODUCE_DESC, lang)));

  const contribRoute = contributionRouteFor(path);
  if (contribRoute === "")
    return pack(pick(BY_CONTRIB_TITLE, lang) + SUFFIX, trimmed(pick(BY_CONTRIB_DESC, lang)));
  if (contribRoute !== null) {
    const m = contributionMeta(contribRoute, lang);
    return pack(m.title + SUFFIX, trimmed(m.description));
  }

  const topicSlug = topicSlugFor(path);
  if (topicSlug === "")
    return pack(pick(TOPICS_TITLE, lang) + SUFFIX, trimmed(pick(TOPICS_DESC, lang)));
  if (topicSlug !== null) {
    const topic = researchTopic(lang, topicSlug);
    if (topic)
      return [
        ...pack(topic.label + SUFFIX, trimmed(topic.description)),
        ...webPageLd(topicUpdated(topicSlug)),
      ];
  }

  const personSlug = personSlugFor(path);
  if (personSlug !== null) {
    const author = researchAuthor(lang, personSlug);
    if (author) {
      const desc =
        author.bio ??
        author.tagline ??
        pick(
          {
            en: `${author.name}'s work on Eternity II.`,
            fr: `Les travaux de ${author.name} sur Eternity II.`,
            es: `El trabajo de ${author.name} sobre Eternity II.`,
          },
          lang,
        );
      return [...pack(author.name + SUFFIX, trimmed(desc)), ...webPageLd(authorUpdated(personSlug))];
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
    inLanguage: lang,
    url: pageUrl,
    mainEntityOfPage: pageUrl,
    isPartOf: { "@id": "https://eternity2.dev/#website" },
    publisher: { "@id": "https://eternity2.dev/#org" },
    ...(doc.updated ? { dateModified: doc.updated } : {}),
    ...(doc.date ? { datePublished: doc.date } : {}),
    ...(doc.author
      ? { author: { "@type": "Person", name: researchAuthor(lang, doc.author)?.name ?? doc.author } }
      : { author: { "@id": "https://eternity2.dev/#org" } }),
  };
  // BreadcrumbList structured data mirrors the visual trail (Research →
  // section → page). Eligible for breadcrumb rich results and reinforces the
  // site hierarchy to crawlers. Built from the same findSection() chain the
  // DocsShell breadcrumb renders, so the two never disagree.
  const section = findSection(lang, doc.url);
  const crumbs: { name: string; url: string }[] = [
    {
      name: pick({ en: "Research", fr: "Recherche", es: "Investigación" }, lang),
      url: absoluteUrl("/research"),
    },
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
  es: {
    title: "Página no encontrada",
    body: "No existe ninguna página de investigación en esta dirección.",
    back: "← Visión general de la investigación",
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

  // The wiki is bilingual: English at /research/**, French at /fr/research/**.
  // The language comes from the URL. A page whose French translation
  // (<slug>.fr.mdx) has not been written yet still resolves — buildManifest("fr")
  // falls back to the English entry with translated:false, and DocsShell shows a
  // "not yet translated" notice — so a French URL never 404s.
  const lang = langFromPath(pathname);

  // Pages that moved (Build reorg into technique-family sub-hubs) redirect old
  // URLs to their new home, so bookmarks and external links don't 404. The
  // redirect map is language-neutral, so re-apply the current language prefix —
  // a French/Spanish visitor must land on the same-language new page, not English.
  const moved = RESEARCH_REDIRECTS[path];
  if (moved) return <Navigate to={pathForLang(moved, lang)} replace />;

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

  if (path === "/research/build/reproduce") return <ReproduceIndexPage />;

  const contribRoute = contributionRouteFor(path);
  if (contribRoute === "") return <ContributionIndex />;
  if (contribRoute !== null) return <ContributionHub kind={contribRoute} />;

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
