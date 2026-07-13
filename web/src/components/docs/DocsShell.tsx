// The reading shell for research wiki pages: breadcrumbs, title block with
// kind/tier/score badges, the article itself (prose + MDX), a reproducibility
// block, prev/next, and the related rail — flanked by the sidebar tree (left)
// and the on-page TOC (right).

import type { ReactNode } from "react";
import { useLang, useT } from "@/i18n";
import { LocalizedLink } from "@/components/LocalizedLink";
import { HardwareCard } from "@/components/research/HardwareCard";
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
import { researchTopic, topicUrl, researchAuthor, authorUrl } from "@/lib/research/manifest";
import type { ResearchDoc, ReproKind, RigorKind } from "@/lib/research/types";
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
    viewMarkdown: "View as Markdown",
    updated: "Updated",
    by: "by",
    rigor: {
      proven: "proven",
      measured: "measured",
      conjectured: "conjectured",
    } as Record<RigorKind, string>,
    rigorTitle: {
      proven: "established by a formal or exhaustive proof / certificate",
      measured: "an empirical result measured on this project's engine",
      conjectured: "a hypothesis or literature reading, not yet established here",
    } as Record<RigorKind, string>,
    complexity: "Complexity",
    time: "Time",
    space: "Space",
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
    viewMarkdown: "Version Markdown",
    updated: "Mis à jour",
    by: "par",
    rigor: {
      proven: "prouvé",
      measured: "mesuré",
      conjectured: "conjecturé",
    } as Record<RigorKind, string>,
    rigorTitle: {
      proven: "établi par une preuve formelle ou exhaustive / un certificat",
      measured: "un résultat empirique mesuré sur le moteur de ce projet",
      conjectured: "une hypothèse ou une lecture de la littérature, pas encore établie ici",
    } as Record<RigorKind, string>,
    complexity: "Complexité",
    time: "Temps",
    space: "Espace",
  },
};

const RIGOR_STYLE: Record<RigorKind, string> = {
  proven:
    "border-emerald-500/40 bg-emerald-500/10 text-emerald-700 dark:text-emerald-300",
  measured: "border-sky-500/40 bg-sky-500/10 text-sky-700 dark:text-sky-300",
  conjectured:
    "border-amber-500/40 bg-amber-500/10 text-amber-700 dark:text-amber-300",
};

/** Credibility badge: how firmly the page's central claim is established. */
function RigorBadge({ rigor }: { rigor: RigorKind }) {
  const t = useT(T);
  return (
    <span
      className={cn("rounded-full border px-2 py-0.5 font-medium", RIGOR_STYLE[rigor])}
      title={t.rigorTitle[rigor]}
    >
      {t.rigor[rigor]}
    </span>
  );
}

/** Algorithmic-cost block: time / space bounds + an optional caveat. Math in
 *  the strings is written as KaTeX and rendered by the same pipeline as the
 *  body (the fields are plain text here; author writes `$O(e d^2)$`). */
function ComplexityBlock({ doc }: { doc: ResearchDoc }) {
  const t = useT(T);
  const c = doc.complexity;
  if (!c || (!c.time && !c.space && !c.note)) return null;
  return (
    <div className="mt-8 rounded-lg border p-4 text-sm">
      <div className="font-semibold">{t.complexity}</div>
      <dl className="mt-1.5 space-y-1">
        {c.time && (
          <div className="flex gap-2">
            <dt className="w-16 shrink-0 text-muted-foreground">{t.time}</dt>
            <dd className="font-mono text-[0.95em]">{c.time}</dd>
          </div>
        )}
        {c.space && (
          <div className="flex gap-2">
            <dt className="w-16 shrink-0 text-muted-foreground">{t.space}</dt>
            <dd className="font-mono text-[0.95em]">{c.space}</dd>
          </div>
        )}
      </dl>
      {c.note && <p className="mt-2 text-muted-foreground">{c.note}</p>}
    </div>
  );
}

/** Author credit line — links to the researcher's auto-generated hub. */
function Byline({ doc }: { doc: ResearchDoc }) {
  const t = useT(T);
  const { lang } = useLang();
  if (!doc.author) return null;
  const author = researchAuthor(lang, doc.author);
  if (!author) return null;
  return (
    <p className="text-sm text-muted-foreground">
      {t.by}{" "}
      <LocalizedLink
        to={authorUrl(author.slug)}
        className="font-medium text-foreground underline underline-offset-2 hover:text-muted-foreground"
      >
        {author.name}
      </LocalizedLink>
    </p>
  );
}

function Breadcrumbs({ doc }: { doc: ResearchDoc }) {
  const t = useT(T);
  const { lang } = useLang();
  const section = findSection(lang, doc.url);
  return (
    <nav className="flex flex-wrap items-center gap-1.5 text-sm text-muted-foreground">
      <LocalizedLink to="/research" className="hover:text-foreground">
        {t.research}
      </LocalizedLink>
      {section && section.url !== doc.url && (
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
      {doc.rigor && <RigorBadge rigor={doc.rigor} />}
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
  // Nothing actionable (no command, no code link): a compact honesty note
  // instead of a near-empty box.
  if (!doc.repro.cmd && !doc.repro.topic) {
    return (
      <p className="mt-10 border-t pt-4 text-xs text-muted-foreground">
        {t.reproduce} — {t.reproKind[doc.repro.kind]}
      </p>
    );
  }
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

/** The default hub card: kind tag on top, title, two-line clamped blurb —
 *  compact so mixed-kind hubs tile two-up. */
function HubCard({ item }: { item: NavItem }) {
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
      <p className="mt-1 line-clamp-2 text-xs leading-relaxed text-muted-foreground">
        {item.description}
      </p>
    </LocalizedLink>
  );
}

/** The author-hub card: no repeated kind tag (every child is the same kind),
 *  and the full subtitle unclamped — meant for a single column, so a
 *  researcher's experiments read like a list with room to breathe. */
function AuthorHubCard({ item }: { item: NavItem }) {
  return (
    <LocalizedLink
      to={item.url}
      className="group block rounded-lg border p-4 transition-shadow hover:shadow-md"
    >
      <div className="flex items-center gap-2 text-sm font-semibold tracking-tight group-hover:underline">
        <span className={cn("h-2 w-2 shrink-0 rounded-full", KIND_DOT[item.kind])} aria-hidden />
        {item.title}
      </div>
      <p className="mt-1 pl-4 text-xs leading-relaxed text-muted-foreground">
        {item.description}
      </p>
    </LocalizedLink>
  );
}

/** For hub pages (a section root or a node with children): the children as
 *  cards, so a hub's MDX body only needs its intro prose. An author's
 *  experiment hub uses the single-column list layout; every other hub uses the
 *  compact two-up grid. */
function HubCards({ doc }: { doc: ResearchDoc }) {
  const { lang } = useLang();
  const section = findSection(lang, doc.url);
  if (!section) return null;
  const items =
    doc.url === section.url
      ? section.items
      : (sectionReadingOrder(section).find((i) => i.url === doc.url)?.children ?? []);
  if (items.length === 0) return null;

  if (doc.author) {
    return (
      <div className="mt-8 grid grid-cols-1 gap-3">
        {items.map((n) => (
          <AuthorHubCard key={n.url} item={n} />
        ))}
      </div>
    );
  }
  return (
    <div className="mt-8 grid gap-3 sm:grid-cols-2">
      {items.map((n) => (
        <HubCard key={n.url} item={n} />
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

export function DocsShell({
  doc,
  children,
  sidebarVariant,
}: {
  doc: ResearchDoc;
  children: ReactNode;
  /** Override the left rail — "people" shows the alphabetical contributor list
   *  (used by the /research/people gallery so it matches the person hubs). */
  sidebarVariant?: "people";
}) {
  const t = useT(T);
  const { lang } = useLang();
  const section = findSection(lang, doc.url) ?? null;
  return (
    <div>
      <ResearchSubnav />
      <div className="lg:grid lg:grid-cols-[15rem_minmax(0,1fr)] lg:gap-10 xl:grid-cols-[15rem_minmax(0,1fr)_13rem]">
        <DocsSidebar section={section} variant={sidebarVariant} />
        <article className="min-w-0">
        <Breadcrumbs doc={doc} />
        <header className="mt-4 space-y-3">
          <h1 className="text-3xl font-bold tracking-tight sm:text-4xl">{doc.title}</h1>
          <p className="text-lg text-muted-foreground">{doc.description}</p>
          <Byline doc={doc} />
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
        <ComplexityBlock doc={doc} />
        {doc.hardware && <HardwareCard hardware={doc.hardware} />}
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
        <p className="mt-8 flex gap-4 text-xs text-muted-foreground">
          <a
            href={`${REPO_URL}/blob/main/web/content/research/${doc.file}`}
            target="_blank"
            rel="noreferrer"
            className="underline hover:text-foreground"
          >
            {t.editOnGitHub}
          </a>
          {/* Raw-markdown sibling, emitted at build time (404 on dev servers). */}
          <a
            href={`${(lang === "fr" ? "/fr" : "") + doc.url}.md`}
            target="_blank"
            rel="noreferrer"
            className="underline hover:text-foreground"
          >
            {t.viewMarkdown}
          </a>
        </p>
        </article>
        <DocsToc toc={doc.toc} />
      </div>
    </div>
  );
}
