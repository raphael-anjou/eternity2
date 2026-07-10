// Auto-generated researcher pages: /research/people/<slug> — one hub per
// author, listing every page they wrote, grouped by kind. No content file
// backs these: the profile comes from authors.json, and the list of a
// person's work is derived entirely from each doc's `author` frontmatter —
// the same mechanic as the topic hubs (TopicPages.tsx), keyed on author
// instead of topic. The community-gallery index still lives at the real
// content page /research/people (people.mdx).

import { useLang, useT } from "@/i18n";
import { LocalizedLink } from "@/components/LocalizedLink";
import { cn } from "@/lib/utils";
import { kindLabel, KIND_DOT } from "@/lib/research/nav";
import { authorDocs, researchAuthor } from "@/lib/research/manifest";
import type { ResearchDoc, ResearchKind } from "@/lib/research/types";
import { DocsSidebar } from "./DocsSidebar";
import { ResearchSubnav } from "./ResearchSubnav";

const T = {
  en: {
    research: "Research",
    people: "People",
    empty: "No pages credited to this researcher yet.",
    pages: (n: number) => (n === 1 ? "1 page" : `${n} pages`),
    backToGallery: "All contributors",
  },
  fr: {
    research: "Recherche",
    people: "Contributeurs",
    empty: "Aucune page créditée à ce chercheur pour l'instant.",
    pages: (n: number) => (n === 1 ? "1 page" : `${n} pages`),
    backToGallery: "Tous les contributeurs",
  },
};

// Order the kind groups so a researcher's headline work reads first.
const KIND_ORDER: ResearchKind[] = [
  "finding",
  "experiment",
  "basin",
  "concept",
  "tool",
  "reference",
  "paper",
  "page",
];

const KIND_GROUP: Record<ResearchKind, { en: string; fr: string }> = {
  finding: { en: "Findings", fr: "Résultats" },
  experiment: { en: "Experiments", fr: "Expériences" },
  basin: { en: "Board studies", fr: "Études de plateau" },
  concept: { en: "Concepts", fr: "Concepts" },
  tool: { en: "Tools", fr: "Outils" },
  reference: { en: "References", fr: "Références" },
  paper: { en: "Papers", fr: "Articles" },
  page: { en: "Pages", fr: "Pages" },
};

function Crumbs({ name }: { name: string }) {
  const t = useT(T);
  return (
    <nav className="flex flex-wrap items-center gap-1.5 text-sm text-muted-foreground">
      <LocalizedLink to="/research" className="hover:text-foreground">
        {t.research}
      </LocalizedLink>
      <span aria-hidden>/</span>
      <LocalizedLink to="/research/people" className="hover:text-foreground">
        {t.people}
      </LocalizedLink>
      <span aria-hidden>/</span>
      <span className="text-foreground">{name}</span>
    </nav>
  );
}

function PageCard({ doc }: { doc: ResearchDoc }) {
  const { lang } = useLang();
  return (
    <LocalizedLink
      to={doc.url}
      className="group block rounded-lg border p-4 transition-shadow hover:shadow-md"
    >
      <div className="flex items-center gap-2">
        <span className={cn("h-2 w-2 shrink-0 rounded-full", KIND_DOT[doc.kind])} aria-hidden />
        <span className="text-[10px] font-medium uppercase tracking-wide text-muted-foreground">
          {kindLabel(doc.kind, lang)}
        </span>
        {doc.score !== undefined && (
          <span className="ml-auto rounded-full bg-emerald-500/15 px-1.5 py-0.5 text-[10px] font-semibold tabular-nums text-emerald-700 dark:text-emerald-300">
            {doc.score}/480
          </span>
        )}
      </div>
      <div className="mt-1.5 text-sm font-semibold tracking-tight group-hover:underline">
        {doc.title}
      </div>
      <p className="mt-1 line-clamp-2 text-xs leading-relaxed text-muted-foreground">
        {doc.description}
      </p>
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

export function PersonHub({ slug }: { slug: string }) {
  const t = useT(T);
  const { lang } = useLang();
  const author = researchAuthor(lang, slug);
  if (!author) return null;
  const docs = authorDocs(lang, slug);

  // Group by kind, in the curated order.
  const groups = new Map<ResearchKind, ResearchDoc[]>();
  for (const d of docs) {
    const list = groups.get(d.kind) ?? [];
    list.push(d);
    groups.set(d.kind, list);
  }
  const orderedGroups = KIND_ORDER.map((k) => [k, groups.get(k) ?? []] as const).filter(
    ([, items]) => items.length > 0,
  );

  return (
    <Frame>
      <Crumbs name={author.name} />
      <header className="mt-4 space-y-3">
        <h1 className="text-3xl font-bold tracking-tight sm:text-4xl">{author.name}</h1>
        {author.tagline && <p className="text-lg text-muted-foreground">{author.tagline}</p>}
        {author.bio && <p className="max-w-[65ch] leading-relaxed text-foreground/90">{author.bio}</p>}
        <div className="flex flex-wrap items-center gap-x-4 gap-y-1.5 text-sm">
          {author.affiliation && (
            <span className="text-muted-foreground">{author.affiliation}</span>
          )}
          {author.links.map((l) => (
            <a
              key={l.url}
              href={l.url}
              target="_blank"
              rel="noreferrer"
              className="font-medium underline underline-offset-2 hover:text-foreground"
            >
              {l.label}
            </a>
          ))}
        </div>
      </header>

      {docs.length === 0 ? (
        <p className="mt-8 rounded-lg border border-dashed p-6 text-sm text-muted-foreground">
          {t.empty}
        </p>
      ) : (
        <div className="mt-10 space-y-8">
          {orderedGroups.map(([kind, items]) => (
            <section key={kind}>
              <h2 className="flex items-baseline gap-2 text-xs font-semibold uppercase tracking-wide text-muted-foreground">
                {KIND_GROUP[kind][lang]}
                <span className="tabular-nums font-normal">{t.pages(items.length)}</span>
              </h2>
              <div className="mt-3 grid gap-3 sm:grid-cols-2">
                {items.map((d) => (
                  <PageCard key={d.url} doc={d} />
                ))}
              </div>
            </section>
          ))}
        </div>
      )}
    </Frame>
  );
}
