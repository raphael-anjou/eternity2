// Auto-generated topic pages: /research/topics (the category index) and
// /research/topics/<slug> (one hub per category, listing every page tagged
// with it, grouped by section). No content file exists for these — they are
// entirely derived from the registry + manifests.

import { useLang, useT } from "@/i18n";
import { LocalizedLink } from "@/components/LocalizedLink";
import { cn } from "@/lib/utils";
import {
  findSection,
  kindLabel,
  topicMembers,
  KIND_DOT,
  type NavItem,
} from "@/lib/research/nav";
import { researchTopic, researchTopics, topicUrl } from "@/lib/research/manifest";
import { THEME_ROOTS, NON_PATH_THEMES } from "@/lib/research/theme-roots";
import type { Lang } from "@/i18n";
import { DocsSidebar } from "./DocsSidebar";
import { ResearchSubnav } from "./ResearchSubnav";

const T = {
  en: {
    research: "Research",
    topics: "Topics",
    topicsLede:
      "The research, sliced by theme instead of by section: pick the wall you care about and everything the wiki knows about it is in one place.",
    pages: (n: number) => (n === 1 ? "1 page" : `${n} pages`),
    empty:
      "Nothing published under this theme yet — the community history exists and the write-ups are on their way.",
    neighbours: "Neighbouring roads",
    allRoads: "All nine roads",
  },
  fr: {
    research: "Recherche",
    topics: "Thèmes",
    topicsLede:
      "La recherche découpée par thème plutôt que par section : choisissez le mur qui vous intéresse, tout ce que le wiki en sait se trouve au même endroit.",
    pages: (n: number) => (n === 1 ? "1 page" : `${n} pages`),
    empty:
      "Rien de publié sur ce thème pour l'instant — l'historique communautaire existe, la rédaction arrive.",
    neighbours: "Voies voisines",
    allRoads: "Les neuf voies",
  },
  es: {
    research: "Investigación",
    topics: "Temas",
    topicsLede:
      "La investigación organizada por tema en lugar de por sección: elige el muro que te interese y todo lo que la wiki sabe sobre él estará en un mismo lugar.",
    pages: (n: number) => (n === 1 ? "1 página" : `${n} páginas`),
    empty:
      "Aún no hay nada publicado sobre este tema — la historia de la comunidad existe y los artículos están en camino.",
    neighbours: "Vías vecinas",
    allRoads: "Las nueve vías",
  },
};

function Crumbs({ current }: { current: string }) {
  const t = useT(T);
  return (
    <nav className="flex flex-wrap items-center gap-1.5 text-sm text-muted-foreground">
      <LocalizedLink to="/research" className="hover:text-foreground">
        {t.research}
      </LocalizedLink>
      <span aria-hidden>/</span>
      {current === t.topics ? (
        <span className="text-foreground">{t.topics}</span>
      ) : (
        <>
          <LocalizedLink to="/research" className="hover:text-foreground">
            {t.topics}
          </LocalizedLink>
          <span aria-hidden>/</span>
          <span className="text-foreground">{current}</span>
        </>
      )}
    </nav>
  );
}

function PageCard({ item }: { item: NavItem }) {
  const { lang } = useLang();
  return (
    <LocalizedLink
      to={item.url}
      className="group block rounded-lg border p-4 transition-shadow hover:shadow-md"
    >
      <div className="flex items-center gap-2">
        <span className={cn("h-2 w-2 shrink-0 rounded-full", KIND_DOT[item.kind])} aria-hidden />
        <span className="text-[10px] font-medium uppercase tracking-wide text-muted-foreground">
          {kindLabel(item.kind, lang)}
        </span>
      </div>
      <div className="mt-1.5 text-sm font-semibold tracking-tight group-hover:underline">
        {item.title}
      </div>
    </LocalizedLink>
  );
}

function Frame({ children }: { children: React.ReactNode }) {
  return (
    <div>
      <ResearchSubnav />
      <div className="lg:grid lg:grid-cols-[15rem_minmax(0,1fr)] lg:gap-10">
        <DocsSidebar section={null} />
        <div className="min-w-0">{children}</div>
      </div>
    </div>
  );
}

export function TopicsIndex() {
  const t = useT(T);
  const { lang } = useLang();
  return (
    <Frame>
      <Crumbs current={t.topics} />
      <header className="mt-4 space-y-3">
        <h1 className="text-3xl font-bold tracking-tight sm:text-4xl">{t.topics}</h1>
        <p className="text-lg text-muted-foreground">{t.topicsLede}</p>
      </header>
      <div className="mt-8 grid gap-3 sm:grid-cols-2">
        {researchTopics(lang).map((topic) => {
          const n = topicMembers(lang, topic.slug).length;
          return (
            <LocalizedLink
              key={topic.slug}
              to={topicUrl(topic.slug)}
              className="group block rounded-lg border p-4 transition-shadow hover:shadow-md"
            >
              <div className="flex items-baseline justify-between gap-2">
                <span className="text-sm font-semibold tracking-tight group-hover:underline">
                  {topic.label}
                </span>
                <span className="text-xs tabular-nums text-muted-foreground">{t.pages(n)}</span>
              </div>
              <p className="mt-1.5 text-xs leading-relaxed text-muted-foreground">
                {topic.description}
              </p>
            </LocalizedLink>
          );
        })}
      </div>
    </Frame>
  );
}

/** A single neighbouring-road card: accent dot + label + hook. */
function RoadCard({ slug, lang, dir }: { slug: string; lang: Lang; dir: "prev" | "next" }) {
  const topic = researchTopic(lang, slug);
  const style = THEME_ROOTS[slug];
  if (!topic || !style) return null;
  return (
    <LocalizedLink
      to={topicUrl(slug)}
      className="group block rounded-lg border p-4 transition-shadow hover:shadow-md"
    >
      <div className="flex items-center gap-2 text-[10px] font-medium uppercase tracking-wide text-muted-foreground">
        <span aria-hidden>{dir === "prev" ? "←" : "→"}</span>
        <span className="h-2 w-2 shrink-0 rounded-full" style={{ backgroundColor: style.color }} aria-hidden />
      </div>
      <div className="mt-1.5 text-sm font-semibold tracking-tight group-hover:underline">
        {topic.label}
      </div>
      <p className="mt-1 text-xs leading-snug text-muted-foreground">{style.hook}</p>
    </LocalizedLink>
  );
}

// The nine solving paths form a ring; from any one, offer the road before and
// the road after (wrapping), plus a link back to all nine. This is the "where
// do I go next" the topic hubs otherwise lacked. Only shown for path themes
// (the meta "records" theme has no neighbouring roads).
function NeighbourRoads({ slug, lang }: { slug: string; lang: Lang }) {
  const t = useT(T);
  if (NON_PATH_THEMES.has(slug)) return null;
  const roads = researchTopics(lang).filter(
    (x) => !NON_PATH_THEMES.has(x.slug) && THEME_ROOTS[x.slug],
  );
  const i = roads.findIndex((r) => r.slug === slug);
  if (i === -1 || roads.length < 2) return null;
  const prev = roads[(i - 1 + roads.length) % roads.length];
  const next = roads[(i + 1) % roads.length];
  return (
    <section className="mt-12 border-t pt-8">
      <div className="flex items-baseline justify-between gap-2">
        <h2 className="text-sm font-semibold uppercase tracking-wide text-muted-foreground">
          {t.neighbours}
        </h2>
        <LocalizedLink
          to="/research"
          className="text-xs font-medium text-muted-foreground underline-offset-2 hover:text-foreground hover:underline"
        >
          {t.allRoads}
        </LocalizedLink>
      </div>
      <div className="mt-4 grid gap-3 sm:grid-cols-2">
        {prev && <RoadCard slug={prev.slug} lang={lang} dir="prev" />}
        {next && <RoadCard slug={next.slug} lang={lang} dir="next" />}
      </div>
    </section>
  );
}

export function TopicHub({ slug }: { slug: string }) {
  const t = useT(T);
  const { lang } = useLang();
  const topic = researchTopic(lang, slug);
  if (!topic) return null;
  const members = topicMembers(lang, slug);
  // Group members by their section for scannability.
  const groups = new Map<string, NavItem[]>();
  for (const m of members) {
    const label = findSection(lang, m.url)?.label ?? "";
    const list = groups.get(label) ?? [];
    list.push(m);
    groups.set(label, list);
  }
  return (
    <Frame>
      <Crumbs current={topic.label} />
      <header className="mt-4 space-y-3">
        <h1 className="text-3xl font-bold tracking-tight sm:text-4xl">{topic.label}</h1>
        <p className="text-lg text-muted-foreground">{topic.description}</p>
      </header>
      {members.length === 0 ? (
        <p className="mt-8 rounded-lg border border-dashed p-6 text-sm text-muted-foreground">
          {t.empty}
        </p>
      ) : (
        <div className="mt-8 space-y-8">
          {[...groups.entries()].map(([label, items]) => (
            <section key={label}>
              <h2 className="text-xs font-semibold uppercase tracking-wide text-muted-foreground">
                {label}
              </h2>
              <div className="mt-3 grid gap-3 sm:grid-cols-2">
                {items.map((i) => (
                  <PageCard key={i.url} item={i} />
                ))}
              </div>
            </section>
          ))}
        </div>
      )}
      <NeighbourRoads slug={slug} lang={lang} />
    </Frame>
  );
}
