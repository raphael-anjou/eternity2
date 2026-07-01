// The reading shell for research wiki pages: breadcrumbs, title block with
// kind/tier/score badges, the article itself (prose + MDX), a reproducibility
// block, prev/next, and the related rail — flanked by the sidebar tree (left)
// and the on-page TOC (right).

import type { ReactNode } from "react";
import { useLang, useT } from "@/i18n";
import { LocalizedLink } from "@/components/LocalizedLink";
import { cn } from "@/lib/utils";
import { REPO_URL } from "@/site";
import {
  allNavItems,
  findSection,
  sectionReadingOrder,
  kindLabel,
  KIND_DOT,
  type NavItem,
} from "@/lib/research/nav";
import { researchTopic, topicUrl } from "@/lib/research/manifest";
import type { ResearchDoc, ReproKind } from "@/lib/research/types";
import { DocsSidebar } from "./DocsSidebar";
import { DocsToc } from "./DocsToc";
import { ResearchSubnav } from "./ResearchSubnav";

const T = {
  en: {
    research: "Research",
    notTranslated:
      "This page hasn't been translated to French yet — showing the English version.",
    reproduce: "Reproduce this result",
    reproKind: {
      exact: "deterministic — reproduces byte-for-byte",
      seeded: "seeded — reproduces with the given seed",
      stochastic: "stochastic — won't reproduce exactly; the board is verifiable",
      heavy: "heavy compute — script and results are committed",
      prose: "prose — no computation behind this page",
    } as Record<ReproKind, string>,
    computedFrom: "Code & data on GitHub",
    source: "Sources",
    previous: "Previous",
    next: "Next",
    keepExploring: "Keep exploring",
    editOnGitHub: "Page source",
    updated: "Updated",
  },
  fr: {
    research: "Recherche",
    notTranslated:
      "Cette page n'est pas encore traduite en français — version anglaise affichée.",
    reproduce: "Reproduire ce résultat",
    reproKind: {
      exact: "déterministe — se reproduit à l'octet près",
      seeded: "avec graine — se reproduit avec la graine donnée",
      stochastic: "stochastique — ne se reproduit pas exactement ; le plateau est vérifiable",
      heavy: "calcul lourd — script et résultats sont dans le dépôt",
      prose: "prose — aucun calcul derrière cette page",
    } as Record<ReproKind, string>,
    computedFrom: "Code & données sur GitHub",
    source: "Sources",
    previous: "Précédent",
    next: "Suivant",
    keepExploring: "Continuer l'exploration",
    editOnGitHub: "Source de la page",
    updated: "Mis à jour",
  },
};

function Breadcrumbs({ doc }: { doc: ResearchDoc }) {
  const t = useT(T);
  const { lang } = useLang();
  const section = findSection(lang, doc.url);
  return (
    <nav className="flex flex-wrap items-center gap-1.5 text-sm text-muted-foreground">
      <LocalizedLink to="/research" className="hover:text-foreground">
        {t.research}
      </LocalizedLink>
      {section && (
        <>
          <span aria-hidden>/</span>
          <LocalizedLink to={section.url} className="hover:text-foreground">
            {section.label}
          </LocalizedLink>
        </>
      )}
      <span aria-hidden>/</span>
      <span className="text-foreground">{doc.title}</span>
    </nav>
  );
}

function Badges({ doc }: { doc: ResearchDoc }) {
  const t = useT(T);
  const { lang } = useLang();
  return (
    <div className="flex flex-wrap items-center gap-2 text-xs">
      <span className="inline-flex items-center gap-1.5 rounded-full border px-2 py-0.5 font-medium uppercase tracking-wide text-muted-foreground">
        <span className={cn("h-1.5 w-1.5 rounded-full", KIND_DOT[doc.kind])} aria-hidden />
        {kindLabel(doc.kind, lang)}
      </span>
      {doc.tier !== undefined && (
        <span
          className="rounded-full border px-2 py-0.5 font-medium text-amber-600 dark:text-amber-400"
          title={lang === "en" ? "flagship result" : "résultat phare"}
        >
          {"★".repeat(4 - doc.tier)}
        </span>
      )}
      {doc.score !== undefined && (
        <span className="rounded-full bg-emerald-500/15 px-2 py-0.5 font-semibold text-emerald-700 dark:text-emerald-300">
          {doc.score}/480
        </span>
      )}
      {doc.topics.map((slug) => {
        const topic = researchTopic(lang, slug);
        if (!topic) return null;
        return (
          <LocalizedLink
            key={slug}
            to={topicUrl(slug)}
            className="rounded-full border px-2 py-0.5 text-muted-foreground transition-colors hover:bg-muted hover:text-foreground"
          >
            {topic.label}
          </LocalizedLink>
        );
      })}
      {doc.updated && (
        <span className="text-muted-foreground">
          {t.updated} {doc.updated}
        </span>
      )}
    </div>
  );
}

function ReproBlock({ doc }: { doc: ResearchDoc }) {
  const t = useT(T);
  if (!doc.repro) return null;
  return (
    <div className="mt-10 rounded-lg border bg-muted/30 p-4 text-sm">
      <div className="font-semibold">{t.reproduce}</div>
      <p className="mt-1 text-muted-foreground">{t.reproKind[doc.repro.kind]}</p>
      {doc.repro.cmd && (
        <pre className="mt-2 overflow-x-auto rounded-md border bg-background p-2.5 text-xs">
          <code>{doc.repro.cmd}</code>
        </pre>
      )}
      {doc.repro.topic && (
        <a
          href={`${REPO_URL}/tree/main/research/topics/${doc.repro.topic}`}
          target="_blank"
          rel="noreferrer"
          className="mt-2 inline-block font-medium underline hover:text-foreground"
        >
          {t.computedFrom}
        </a>
      )}
    </div>
  );
}

/** For hub pages (a section root or a node with children): the children as
 *  cards, so a hub's MDX body only needs its intro prose. */
function HubCards({ doc }: { doc: ResearchDoc }) {
  const { lang } = useLang();
  const section = findSection(lang, doc.url);
  if (!section) return null;
  const items =
    doc.url === section.url
      ? section.items
      : (sectionReadingOrder(section).find((i) => i.url === doc.url)?.children ?? []);
  if (items.length === 0) return null;
  return (
    <div className="mt-8 grid gap-3 sm:grid-cols-2">
      {items.map((n) => (
        <LocalizedLink
          key={n.url}
          to={n.url}
          className="group block rounded-lg border p-4 transition-shadow hover:shadow-md"
        >
          <div className="flex items-center gap-2">
            <span className={cn("h-2 w-2 shrink-0 rounded-full", KIND_DOT[n.kind])} aria-hidden />
            <span className="text-[10px] font-medium uppercase tracking-wide text-muted-foreground">
              {kindLabel(n.kind, lang)}
            </span>
          </div>
          <div className="mt-1.5 text-sm font-semibold tracking-tight group-hover:underline">
            {n.title}
          </div>
          <p className="mt-1 line-clamp-2 text-xs leading-relaxed text-muted-foreground">
            {n.description}
          </p>
        </LocalizedLink>
      ))}
    </div>
  );
}

function PrevNext({ doc }: { doc: ResearchDoc }) {
  const t = useT(T);
  const { lang } = useLang();
  const section = findSection(lang, doc.url);
  const order = section ? sectionReadingOrder(section) : [];
  const idx = order.findIndex((i) => i.url === doc.url);
  if (idx === -1) return null;
  const prev = idx > 0 ? order[idx - 1] : undefined;
  const next = idx < order.length - 1 ? order[idx + 1] : undefined;
  const cell = (item: NavItem | undefined, label: string, align: "left" | "right") =>
    item ? (
      <LocalizedLink
        to={item.url}
        className={cn(
          "group flex-1 rounded-lg border p-3 transition-shadow hover:shadow-md",
          align === "right" && "text-right",
        )}
      >
        <div className="text-xs text-muted-foreground">{label}</div>
        <div className="mt-0.5 text-sm font-semibold tracking-tight group-hover:underline">
          {item.title}
        </div>
      </LocalizedLink>
    ) : (
      <div className="flex-1" />
    );
  return (
    <div className="mt-10 flex gap-3">
      {cell(prev, `← ${t.previous}`, "left")}
      {cell(next, `${t.next} →`, "right")}
    </div>
  );
}

function Related({ doc }: { doc: ResearchDoc }) {
  const t = useT(T);
  const { lang } = useLang();
  if (doc.related.length === 0) return null;
  const all = allNavItems(lang);
  const items = doc.related
    .map((url) => all.find((i) => i.url === url))
    .filter((i): i is NavItem => i !== undefined);
  if (items.length === 0) return null;
  return (
    <section className="mt-10 border-t pt-8">
      <h2 className="text-sm font-semibold uppercase tracking-wide text-muted-foreground">
        {t.keepExploring}
      </h2>
      <div className="mt-4 grid gap-3 sm:grid-cols-2">
        {items.map((n) => (
          <LocalizedLink
            key={n.url}
            to={n.url}
            className="group block rounded-lg border p-4 transition-shadow hover:shadow-md"
          >
            <div className="flex items-center gap-2">
              <span className={cn("h-2 w-2 shrink-0 rounded-full", KIND_DOT[n.kind])} aria-hidden />
              <span className="text-[10px] font-medium uppercase tracking-wide text-muted-foreground">
                {kindLabel(n.kind, lang)}
              </span>
            </div>
            <div className="mt-1.5 text-sm font-semibold tracking-tight group-hover:underline">
              {n.title}
            </div>
          </LocalizedLink>
        ))}
      </div>
    </section>
  );
}

export function DocsShell({ doc, children }: { doc: ResearchDoc; children: ReactNode }) {
  const t = useT(T);
  const { lang } = useLang();
  const section = findSection(lang, doc.url) ?? null;
  return (
    <div>
      <ResearchSubnav />
      <div className="lg:grid lg:grid-cols-[15rem_minmax(0,1fr)] lg:gap-10 xl:grid-cols-[15rem_minmax(0,1fr)_13rem]">
        <DocsSidebar section={section} />
        <article className="min-w-0">
        <Breadcrumbs doc={doc} />
        {lang === "fr" && !doc.translated && (
          <div className="mt-4 rounded-lg border border-amber-500/40 bg-amber-500/5 p-3 text-sm text-muted-foreground">
            {t.notTranslated}
          </div>
        )}
        <header className="mt-4 space-y-3">
          <h1 className="text-3xl font-bold tracking-tight sm:text-4xl">{doc.title}</h1>
          <p className="text-lg text-muted-foreground">{doc.description}</p>
          <Badges doc={doc} />
        </header>
        <div
          className={cn(
            "prose prose-neutral dark:prose-invert mt-8 max-w-none",
            "prose-headings:tracking-tight prose-headings:scroll-mt-24",
            "prose-a:underline-offset-2 hover:prose-a:text-foreground",
            "prose-code:before:content-none prose-code:after:content-none",
          )}
        >
          {children}
        </div>
        <HubCards doc={doc} />
        {doc.sources.length > 0 && (
          <div className="mt-8 rounded-lg border p-4 text-sm">
            <div className="font-semibold">{t.source}</div>
            <ul className="mt-1.5 space-y-1">
              {doc.sources.map((s) => (
                <li key={s.url}>
                  <a
                    href={s.url}
                    target="_blank"
                    rel="noreferrer"
                    className="text-muted-foreground underline underline-offset-2 hover:text-foreground"
                  >
                    {s.label}
                  </a>
                </li>
              ))}
            </ul>
          </div>
        )}
        <ReproBlock doc={doc} />
        <PrevNext doc={doc} />
        <Related doc={doc} />
        <p className="mt-8 text-xs text-muted-foreground">
          <a
            href={`${REPO_URL}/blob/main/web/content/research/${doc.file}`}
            target="_blank"
            rel="noreferrer"
            className="underline hover:text-foreground"
          >
            {t.editOnGitHub}
          </a>
        </p>
        </article>
        <DocsToc toc={doc.toc} />
      </div>
    </div>
  );
}
