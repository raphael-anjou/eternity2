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
  backlinkItems,
  findSection,
  sectionReadingOrder,
  kindLabel,
  KIND_DOT,
  type NavItem,
} from "@/lib/research/nav";
import { researchTopic, topicUrl, researchAuthor, authorUrl } from "@/lib/research/manifest";
import { OUTCOME_LABELS, OUTCOME_TITLES } from "@/lib/research/outcome-labels";
import type {
  ResearchDoc,
  ReproKind,
  RigorKind,
  OutcomeKind,
  ContributionKind,
  ScoringConvention,
} from "@/lib/research/types";
import { PipelineStages } from "./PipelineStages";
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
    } satisfies Record<ReproKind, string>,
    computedFrom: "Code & data on GitHub",
    source: "Sources",
    previous: "Previous",
    next: "Next",
    keepExploring: "Keep exploring",
    referencedBy: "Referenced by",
    editOnGitHub: "Page source",
    viewMarkdown: "View as Markdown",
    updated: "Updated",
    by: "by",
    rigor: {
      proven: "proven",
      measured: "measured",
      conjectured: "conjectured",
    } satisfies Record<RigorKind, string>,
    rigorTitle: {
      proven: "established by a formal or exhaustive proof / certificate",
      measured: "an empirical result measured on this project's engine",
      conjectured: "a hypothesis or literature reading, not yet established here",
    } satisfies Record<RigorKind, string>,
    reportBadge: "technical report",
    reportTitle:
      "a technical report: published, but not yet independently reviewed. Its claims are stated in good faith and may still be revised.",
    tier: {
      1: "Flagship",
      2: "Finding",
      3: "Supporting",
    } as Record<number, string>,
    tierTitle: "editorial prominence within this section",
    contribution: {
      solver: "solver",
      analysis: "analysis",
      reconstruction: "reconstruction",
      theory: "theory",
      method: "method",
      measurement: "measurement",
      negative: "dead end",
      tool: "tool",
      exposition: "explainer",
    } satisfies Record<ContributionKind, string>,
    contributionTitle: {
      solver: "produces a competitive board by searching (a real search score)",
      analysis: "proves or computes a property of an existing board or the instance",
      reconstruction: "decodes or rebuilds community work to extract an insight",
      theory: "a mathematical property, law, or impossibility proof",
      method: "a technique described for reuse, not a scored run",
      measurement: "a benchmark or empirical observation about solvers or instances",
      negative: "a rigorously-run dead end, reported first-class rather than as a footnote",
      tool: "a software artifact",
      exposition: "an explainer for an audience",
    } satisfies Record<ContributionKind, string>,
    // Shared with the experiment results table via outcome-labels.ts, so the
    // badge wording never drifts between the page header and the hub tables.
    outcome: OUTCOME_LABELS.en,
    outcomeTitle: OUTCOME_TITLES.en,
    scoring: {
      "matched-edges": "matched edges",
      "strict-5-clue": "strict 5-clue",
    } satisfies Record<ScoringConvention, string>,
    scoringTitle: {
      "matched-edges": "every matching internal edge counts",
      "strict-5-clue": "the five-clue-fixed track",
    } satisfies Record<ScoringConvention, string>,
    reproStrip: "Reproduce",
    reproVerify: "Verify the stored board",
    reproVerifyCaveat: "This verifies the board, not the search that produced it.",
    reproSearchLabel: "reruns the search",
    reproArtifactLabel: "re-verifies a stored board",
    reproSee: "See below",
    budget: "Budget",
    flagship: "flagship result",
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
    } satisfies Record<ReproKind, string>,
    computedFrom: "Code & données sur GitHub",
    source: "Sources",
    previous: "Précédent",
    next: "Suivant",
    keepExploring: "Continuer l'exploration",
    referencedBy: "Cité par",
    editOnGitHub: "Source de la page",
    viewMarkdown: "Version Markdown",
    updated: "Mis à jour",
    by: "par",
    rigor: {
      proven: "prouvé",
      measured: "mesuré",
      conjectured: "conjecturé",
    } satisfies Record<RigorKind, string>,
    rigorTitle: {
      proven: "établi par une preuve formelle ou exhaustive / un certificat",
      measured: "un résultat empirique mesuré sur le moteur de ce projet",
      conjectured: "une hypothèse ou une lecture de la littérature, pas encore établie ici",
    } satisfies Record<RigorKind, string>,
    reportBadge: "rapport technique",
    reportTitle:
      "un rapport technique : publié, mais pas encore relu de façon indépendante. Ses affirmations sont faites de bonne foi et peuvent encore être révisées.",
    tier: {
      1: "Phare",
      2: "Résultat",
      3: "Complément",
    } as Record<number, string>,
    tierTitle: "importance éditoriale au sein de cette section",
    contribution: {
      solver: "solveur",
      analysis: "analyse",
      reconstruction: "reconstruction",
      theory: "théorie",
      method: "méthode",
      measurement: "mesure",
      negative: "impasse",
      tool: "outil",
      exposition: "explication",
    } satisfies Record<ContributionKind, string>,
    contributionTitle: {
      solver: "produit un plateau compétitif par recherche (un vrai score de recherche)",
      analysis: "prouve ou calcule une propriété d'un plateau existant ou de l'instance",
      reconstruction: "décode ou reconstruit un travail communautaire pour en tirer un enseignement",
      theory: "une propriété mathématique, une loi ou une preuve d'impossibilité",
      method: "une technique décrite pour être réutilisée, pas un run noté",
      measurement: "un banc d'essai ou une observation empirique sur les solveurs ou les instances",
      negative: "une impasse étudiée avec rigueur, rapportée de plein droit plutôt qu'en note",
      tool: "un artefact logiciel",
      exposition: "une explication pour un public",
    } satisfies Record<ContributionKind, string>,
    outcome: OUTCOME_LABELS.fr,
    outcomeTitle: OUTCOME_TITLES.fr,
    scoring: {
      "matched-edges": "arêtes appariées",
      "strict-5-clue": "strict 5 indices",
    } satisfies Record<ScoringConvention, string>,
    scoringTitle: {
      "matched-edges": "chaque arête interne appariée compte",
      "strict-5-clue": "la piste à cinq indices fixés",
    } satisfies Record<ScoringConvention, string>,
    reproStrip: "Reproduire",
    reproVerify: "Vérifier le plateau enregistré",
    reproVerifyCaveat: "Ceci vérifie le plateau, pas la recherche qui l'a produit.",
    reproSearchLabel: "relance la recherche",
    reproArtifactLabel: "revérifie un plateau enregistré",
    reproSee: "Voir ci-dessous",
    budget: "Budget",
    flagship: "résultat phare",
    complexity: "Complexité",
    time: "Temps",
    space: "Espace",
  },
  es: {
    research: "Investigación",
    notTranslated:
      "Esta página aún no se ha traducido al español — se muestra la versión en inglés.",
    reproduce: "Reproducir este resultado",
    reproKind: {
      exact: "determinista — se reproduce bit a bit",
      seeded: "con semilla — se reproduce con la semilla indicada",
      stochastic: "estocástico — no se reproduce exactamente; el tablero es verificable",
      heavy: "cómputo intensivo — el script y los resultados están en el repositorio",
      prose: "prosa — no hay ningún cálculo detrás de esta página",
    } satisfies Record<ReproKind, string>,
    computedFrom: "Código y datos en GitHub",
    source: "Fuentes",
    previous: "Anterior",
    next: "Siguiente",
    keepExploring: "Seguir explorando",
    referencedBy: "Citado por",
    editOnGitHub: "Fuente de la página",
    viewMarkdown: "Ver como Markdown",
    updated: "Actualizado",
    by: "por",
    rigor: {
      proven: "demostrado",
      measured: "medido",
      conjectured: "conjeturado",
    } satisfies Record<RigorKind, string>,
    rigorTitle: {
      proven: "establecido por una demostración formal o exhaustiva / un certificado",
      measured: "un resultado empírico medido en el motor de este proyecto",
      conjectured: "una hipótesis o lectura de la literatura, aún no establecida aquí",
    } satisfies Record<RigorKind, string>,
    reportBadge: "informe técnico",
    reportTitle:
      "un informe técnico: publicado, pero aún no revisado de forma independiente. Sus afirmaciones se hacen de buena fe y todavía pueden revisarse.",
    tier: {
      1: "Destacado",
      2: "Resultado",
      3: "Complemento",
    } as Record<number, string>,
    tierTitle: "relevancia editorial dentro de esta sección",
    contribution: {
      solver: "solucionador",
      analysis: "análisis",
      reconstruction: "reconstrucción",
      theory: "teoría",
      method: "método",
      measurement: "medición",
      negative: "callejón sin salida",
      tool: "herramienta",
      exposition: "explicación",
    } satisfies Record<ContributionKind, string>,
    contributionTitle: {
      solver: "produce un tablero competitivo mediante búsqueda (una puntuación de búsqueda real)",
      analysis: "demuestra o calcula una propiedad de un tablero existente o de la instancia",
      reconstruction: "decodifica o reconstruye trabajo de la comunidad para extraer una idea",
      theory: "una propiedad matemática, una ley o una prueba de imposibilidad",
      method: "una técnica descrita para reutilizarse, no un run puntuado",
      measurement: "un banco de pruebas o una observación empírica sobre solucionadores o instancias",
      negative: "un callejón sin salida estudiado con rigor, expuesto de pleno derecho y no como nota al pie",
      tool: "un artefacto de software",
      exposition: "una explicación para un público",
    } satisfies Record<ContributionKind, string>,
    outcome: OUTCOME_LABELS.es,
    outcomeTitle: OUTCOME_TITLES.es,
    scoring: {
      "matched-edges": "aristas emparejadas",
      "strict-5-clue": "estricto 5 pistas",
    } satisfies Record<ScoringConvention, string>,
    scoringTitle: {
      "matched-edges": "cuenta cada arista interna emparejada",
      "strict-5-clue": "la pista de cinco pistas fijadas",
    } satisfies Record<ScoringConvention, string>,
    reproStrip: "Reproducir",
    reproVerify: "Verificar el tablero guardado",
    reproVerifyCaveat: "Esto verifica el tablero, no la búsqueda que lo produjo.",
    reproSearchLabel: "relanza la búsqueda",
    reproArtifactLabel: "revalida un tablero guardado",
    reproSee: "Ver más abajo",
    budget: "Presupuesto",
    flagship: "resultado destacado",
    complexity: "Complejidad",
    time: "Tiempo",
    space: "Espacio",
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

/** Status badge for a technical report: published but not yet independently
 *  reviewed. Only rendered when status === "report"; a reviewed finding (live)
 *  carries no badge (the absence is the signal). */
function ReportBadge() {
  const t = useT(T);
  return (
    <span
      className="inline-flex items-center gap-1.5 rounded-full border border-amber-500/50 bg-amber-500/10 px-2 py-0.5 font-medium uppercase tracking-wide text-amber-700 dark:text-amber-300"
      title={t.reportTitle}
    >
      <span className="h-1.5 w-1.5 rounded-full bg-amber-500" aria-hidden />
      {t.reportBadge}
    </span>
  );
}

/** Contribution chip: what kind of research result the page is (the axis the
 *  score chart reads for membership). A neutral outline for every value; the
 *  dead-end value (`negative`) gets a distinct dashed border so a refuted
 *  result reads as a first-class outcome rather than a missing badge. */
export function ContributionBadge({ contribution }: { contribution: ContributionKind }) {
  const t = useT(T);
  const negative = contribution === "negative";
  return (
    <span
      className={cn(
        "rounded-full border px-2 py-0.5 font-medium",
        negative
          ? "border-dashed border-rose-500/50 text-rose-600 dark:text-rose-400"
          : "text-muted-foreground",
      )}
      title={t.contributionTitle[contribution]}
    >
      {t.contribution[contribution]}
    </span>
  );
}

const OUTCOME_STYLE: Record<OutcomeKind, string> = {
  plateaued: "border-slate-500/40 bg-slate-500/10 text-slate-700 dark:text-slate-300",
  refuted: "border-rose-500/40 bg-rose-500/10 text-rose-700 dark:text-rose-300",
  parked: "border-stone-500/40 bg-stone-500/10 text-stone-700 dark:text-stone-300",
  "new-basin": "border-emerald-500/40 bg-emerald-500/10 text-emerald-700 dark:text-emerald-300",
  superseded: "border-amber-500/40 bg-amber-500/10 text-amber-700 dark:text-amber-300",
};

/** Research-status chip: how an experiment or finding ended (plateaued,
 *  refuted, parked, new basin, superseded). Distinct from rigor and from the
 *  publish tier. */
function OutcomeBadge({ outcome }: { outcome: OutcomeKind }) {
  const t = useT(T);
  return (
    <span
      className={cn("rounded-full border px-2 py-0.5 font-medium", OUTCOME_STYLE[outcome])}
      title={t.outcomeTitle[outcome]}
    >
      {t.outcome[outcome]}
    </span>
  );
}

/** Editorial-prominence chip: a labeled Flagship / Finding / Supporting chip
 *  (replaces the old unlabeled star count, whose 1=flagship→3-star mapping read
 *  inverted). */
function TierChip({ tier }: { tier: number }) {
  const t = useT(T);
  const label = t.tier[tier];
  if (!label) return null;
  return (
    <span
      className="rounded-full border border-amber-500/40 bg-amber-500/10 px-2 py-0.5 font-medium text-amber-700 dark:text-amber-400"
      title={t.tierTitle}
    >
      {label}
    </span>
  );
}

/** Which scoring convention the score pill is under, rendered as a small pill
 *  right after the score so a NNN/480 number is never ambiguous. */
function ScoringPill({ convention }: { convention: ScoringConvention }) {
  const t = useT(T);
  return (
    <span
      className="rounded-full border px-2 py-0.5 font-medium text-muted-foreground"
      title={t.scoringTitle[convention]}
    >
      {t.scoring[convention]}
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

/** Author credit line — links to the researcher's auto-generated hub, and lists
 *  any secondary contributors beside them (e.g. "· method by …"). */
function Byline({ doc }: { doc: ResearchDoc }) {
  const t = useT(T);
  const { lang } = useLang();
  if (!doc.author) return null;
  const author = researchAuthor(lang, doc.author);
  if (!author) return null;
  const linkClass =
    "font-medium text-foreground underline underline-offset-2 hover:text-muted-foreground";
  // Drop any contributor whose slug is missing from the registry rather than
  // rendering a dead credit; the content check already rejects unknown slugs.
  const contributors = (doc.contributors ?? []).flatMap((c) => {
    const a = researchAuthor(lang, c.slug);
    return a ? [{ role: c.role, slug: a.slug, name: a.name }] : [];
  });
  return (
    <p className="text-sm text-muted-foreground">
      {t.by}{" "}
      <LocalizedLink to={authorUrl(author.slug)} className={linkClass}>
        {author.name}
      </LocalizedLink>
      {contributors.map((c) => (
        <span key={c.slug}>
          {" · "}
          {c.role} {t.by}{" "}
          <LocalizedLink to={authorUrl(c.slug)} className={linkClass}>
            {c.name}
          </LocalizedLink>
        </span>
      ))}
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
      {doc.status === "report" && <ReportBadge />}
      {doc.tier !== undefined && <TierChip tier={doc.tier} />}
      {doc.contribution && <ContributionBadge contribution={doc.contribution} />}
      {doc.rigor && <RigorBadge rigor={doc.rigor} />}
      {doc.outcome && <OutcomeBadge outcome={doc.outcome} />}
      {doc.score !== undefined && (
        <span className="rounded-full bg-emerald-500/15 px-2 py-0.5 font-semibold text-emerald-700 dark:text-emerald-300">
          {doc.score}/480
        </span>
      )}
      {doc.scoringConvention && <ScoringPill convention={doc.scoringConvention} />}
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

/** Does a repro command rerun the SEARCH, or only re-verify a stored ARTIFACT
 *  (a committed board)? The author can pin it via repro.produces; absent, the
 *  default is `artifact` when the result was computed from the record-boards
 *  topic (there the command re-scores a stored board) and `search` otherwise.
 *  This keeps heavy-but-runnable pages (e.g. the jit-backtracker) as `search`. */
function reproProduces(doc: ResearchDoc): "search" | "artifact" {
  const r = doc.repro;
  if (!r) return "search";
  if (r.produces) return r.produces;
  return r.topic === "record-boards" ? "artifact" : "search";
}

/** Compact one-line repro summary in the header: the repro-kind label, whether
 *  the command reruns the search or verifies a board (or a pointer to the block
 *  below when there is no inline command), and the wall-clock budget if the page
 *  records one. Rendered only when there is a repro block or a score to explain.
 *  The full ReproBlock still sits at the bottom of the page. */
function ReproStrip({ doc }: { doc: ResearchDoc }) {
  const t = useT(T);
  if (!doc.repro && doc.score === undefined) return null;
  const produces = doc.repro ? reproProduces(doc) : undefined;
  const actionLabel =
    produces === "artifact" ? t.reproArtifactLabel : produces === "search" ? t.reproSearchLabel : undefined;
  const hasInline = Boolean(doc.repro?.cmd || doc.repro?.topic);
  return (
    <div className="flex flex-wrap items-center gap-x-2 gap-y-1 text-xs text-muted-foreground">
      <span className="font-medium text-foreground">{t.reproStrip}</span>
      {doc.repro && <span>{t.reproKind[doc.repro.kind]}</span>}
      {actionLabel && (
        <>
          <span aria-hidden>·</span>
          <span>{hasInline ? actionLabel : `${actionLabel} (${t.reproSee})`}</span>
        </>
      )}
      {doc.hardware?.wallClock && (
        <>
          <span aria-hidden>·</span>
          <span>
            {t.budget}: {doc.hardware.wallClock}
          </span>
        </>
      )}
    </div>
  );
}

function ReproBlock({ doc }: { doc: ResearchDoc }) {
  const t = useT(T);
  if (!doc.repro) return null;
  const isArtifact = reproProduces(doc) === "artifact";
  // Nothing actionable (no command, no code link): a compact note instead of a
  // near-empty box.
  if (!doc.repro.cmd && !doc.repro.topic) {
    return (
      <p className="mt-10 border-t pt-4 text-xs text-muted-foreground">
        {t.reproduce}: {t.reproKind[doc.repro.kind]}
      </p>
    );
  }
  // An artifact page re-scores a stored board; it does NOT rerun the search.
  // Amber warn language + a plain caveat keep that boundary explicit.
  const boxClass = isArtifact
    ? "mt-10 rounded-lg border border-amber-500/40 bg-amber-500/5 p-4 text-sm"
    : "mt-10 rounded-lg border bg-muted/30 p-4 text-sm";
  return (
    <div className={boxClass}>
      <div className="font-semibold">{isArtifact ? t.reproVerify : t.reproduce}</div>
      <p className="mt-1 text-muted-foreground">{t.reproKind[doc.repro.kind]}</p>
      {isArtifact && (
        <p className="mt-1 text-amber-700 dark:text-amber-300">{t.reproVerifyCaveat}</p>
      )}
      {doc.repro.cmd && (
        <pre className="mt-2 overflow-x-auto rounded-md border bg-background p-2.5 text-xs">
          <code>{doc.repro.cmd}</code>
        </pre>
      )}
      {doc.repro.scope && <p className="mt-2 text-muted-foreground">{doc.repro.scope}</p>}
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
        <NegativeChip item={item} />
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

/** A compact dead-end chip for hub/listing cards: shown only when the item's
 *  contribution is `negative`, so a refuted result is visible in listings
 *  rather than invisible for lacking a positive badge. */
function NegativeChip({ item }: { item: NavItem }) {
  const t = useT(T);
  if (item.contribution !== "negative") return null;
  return (
    <span
      className="rounded-full border border-dashed border-rose-500/50 px-1.5 py-0.5 text-[10px] font-medium text-rose-600 dark:text-rose-400"
      title={t.contributionTitle.negative}
    >
      {t.contribution.negative}
    </span>
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
        <NegativeChip item={item} />
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

/** "Referenced by": the other research pages that link to this one in prose,
 *  the reverse of the related rail. A discoverability aid, not curated content,
 *  so it renders compact (a wrapped list of small links, not the card grid) and
 *  only when there is at least one inbound prose link. Capped at 8; sources
 *  arrive in section reading order from the compile-time backlink map. */
function ReferencedBy({ doc }: { doc: ResearchDoc }) {
  const t = useT(T);
  const { lang } = useLang();
  const items = backlinkItems(lang, doc.url, 8);
  if (items.length === 0) return null;
  return (
    <section className="mt-10 border-t pt-6">
      <h2 className="text-sm font-semibold uppercase tracking-wide text-muted-foreground">
        {t.referencedBy}
      </h2>
      <ul className="mt-3 flex flex-wrap gap-x-4 gap-y-2 text-sm">
        {items.map((n) => (
          <li key={n.url} className="flex items-center gap-1.5">
            <span
              className={cn("h-1.5 w-1.5 shrink-0 rounded-full", KIND_DOT[n.kind])}
              aria-hidden
            />
            <LocalizedLink
              to={n.url}
              className="text-muted-foreground underline underline-offset-2 hover:text-foreground"
            >
              {n.title}
            </LocalizedLink>
          </li>
        ))}
      </ul>
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
        <DocsSidebar section={section} {...(sidebarVariant ? { variant: sidebarVariant } : {})} />
        <article className="min-w-0">
        <Breadcrumbs doc={doc} />
        <header className="mt-4 space-y-3">
          <h1 className="text-3xl font-bold tracking-tight sm:text-4xl">{doc.title}</h1>
          <p className="text-lg text-muted-foreground">{doc.description}</p>
          <Byline doc={doc} />
          <Badges doc={doc} />
          <ReproStrip doc={doc} />
        </header>
        <PipelineStages doc={doc} />
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
        <ReferencedBy doc={doc} />
        <p className="mt-8 flex gap-4 text-xs text-muted-foreground">
          <a
            href={`${REPO_URL}/blob/main/web/content/research/${doc.file}`}
            target="_blank"
            rel="noreferrer"
            className="underline hover:text-foreground"
          >
            {t.editOnGitHub}
          </a>
          {/* Raw-markdown sibling, emitted at build time (404 on dev servers).
              Siblings are English-only (the wiki's canonical markdown), so every
              language links the same neutral `/research/<slug>.md` — `doc.url` is
              already language-neutral. The root hub (`/research`) is emitted as
              `/research/index.md`, not `/research.md`, so link that explicitly. */}
          <a
            href={doc.url === "/research" ? "/research/index.md" : `${doc.url}.md`}
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
