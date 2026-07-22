import { Fragment, useMemo, useState } from "react";
import { LocalizedLink } from "@/components/LocalizedLink";
import { useLang } from "@/i18n";
import { researchDocs, researchAuthors } from "@/lib/research/manifest";
import { cn } from "@/lib/utils";
import { coreHours, formatCoreHours } from "@/lib/research/hardware-cost";
import { OUTCOME_LABELS, OUTCOME_TITLES } from "@/lib/research/outcome-labels";
import type { ScoreDatum } from "./ExperimentScoreChart";
import type { RigorKind, ReproKind, OutcomeKind, ScoringConvention } from "@/lib/research/types";

// The experiment gallery as one sortable results table, in two modes:
//
//  - PROPS mode (the per-author hub): the caller passes the page's own `scores`
//    list. Score and method family come from that list; author, month, rigor,
//    reproducibility, outcome and the link are pulled from each experiment's
//    frontmatter via the manifest, so the table never drifts from the pages it
//    summarizes.
//  - MANIFEST mode (the cross-researcher hub, `manifest` prop): rows are
//    discovered from the manifest itself — every `kind: experiment` page that
//    carries a `score`, grouped by author. There is no per-page family in
//    frontmatter, so this mode drops the method column and adds a
//    scoring-convention pill beside each score.
//
// Both modes render the `outcome` frontmatter as a translated status badge
// (labels shared with the docs shell via outcome-labels.ts).
//
// When more than one researcher has experiments, the table splits into a
// section per author (ownership at a glance); with a single author it is one
// flat table.

const EXPERIMENTS_PREFIX = "/research/lab/experiments/";

/** "2026-07" → "Jul 2026". Returns "" for missing/malformed. */
function monthYear(d: string | undefined): string {
  if (!d) return "";
  const m = /^(\d{4})-(\d{2})$/.exec(d);
  if (!m) return "";
  const months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
  const mi = Number(m[2]) - 1;
  return `${months[mi] ?? m[2]} ${m[1]}`;
}

const FAMILY_LABEL: Record<string, string> = {
  scratch: "from scratch",
  corpus: "corpus-guided",
  concentrate: "prefix-first",
  anchor: "border-anchored",
  exact: "exact",
  decode: "witness replay",
};

const RIGOR_STYLE: Record<RigorKind, string> = {
  proven: "bg-emerald-500/15 text-emerald-700 dark:text-emerald-300",
  measured: "bg-sky-500/15 text-sky-700 dark:text-sky-300",
  conjectured: "bg-amber-500/15 text-amber-700 dark:text-amber-300",
};

const OUTCOME_STYLE: Record<OutcomeKind, string> = {
  plateaued: "bg-slate-500/15 text-slate-700 dark:text-slate-300",
  refuted: "bg-rose-500/15 text-rose-700 dark:text-rose-300",
  parked: "bg-amber-500/15 text-amber-700 dark:text-amber-300",
  "new-basin": "bg-emerald-500/15 text-emerald-700 dark:text-emerald-300",
  superseded: "bg-violet-500/15 text-violet-700 dark:text-violet-300",
};

const CONVENTION_LABEL: Record<ScoringConvention, string> = {
  "matched-edges": "matched edges",
  "strict-5-clue": "strict 5-clue",
};

const REPRO_LABEL: Record<ReproKind, string> = {
  exact: "exact",
  seeded: "seeded",
  stochastic: "stochastic",
  heavy: "heavy",
  prose: "—",
};

interface Row {
  key: string;
  label: string;
  score: number;
  /** Method family (PROPS mode only). */
  family?: string;
  url: string;
  rigor?: RigorKind;
  repro?: ReproKind;
  outcome?: OutcomeKind;
  scoringConvention?: ScoringConvention;
  authorSlug?: string;
  authorName: string;
  date?: string;
  /** Compute cost of the run (cores × wall-clock hours), when hardware is
   *  declared and its budget parses; the cross-run comparison. */
  coreHours?: number;
  /** Whether the run is the standardized single-core bench (vs a native run). */
  measured?: boolean;
}

type SortKey = "score" | "label" | "family" | "date" | "cost" | "outcome";

export function ExperimentResultsTable({
  data,
  manifest = false,
}: {
  /** PROPS mode: the page's own score list. Ignored when `manifest` is set. */
  data?: ScoreDatum[];
  /** MANIFEST mode: discover every scored experiment from the manifest. */
  manifest?: boolean;
}) {
  const { lang } = useLang();
  const [sortKey, setSortKey] = useState<SortKey>("score");
  const [asc, setAsc] = useState(false);

  const rows = useMemo<Row[]>(() => {
    const docs = researchDocs(lang);
    const authors = researchAuthors(lang);
    const nameOf = (slug?: string) =>
      (slug && authors.find((a) => a.slug === slug)?.name) || "Unattributed";

    if (manifest) {
      // Every named experiment page that carries a score, grouped by author.
      // The score chart stays solver-only; this table is the fuller ledger of
      // "what board did this run reach", so it lists every scored experiment.
      return docs
        .filter((d) => d.kind === "experiment" && typeof d.score === "number" && d.url.startsWith(EXPERIMENTS_PREFIX))
        .map((d) => {
          const ch = d.hardware ? coreHours(d.hardware) : null;
          return {
            key: d.url,
            label: d.title,
            score: d.score as number,
            url: d.url,
            ...(d.rigor ? { rigor: d.rigor } : {}),
            ...(d.repro ? { repro: d.repro.kind } : {}),
            ...(d.outcome ? { outcome: d.outcome } : {}),
            ...(d.scoringConvention ? { scoringConvention: d.scoringConvention } : {}),
            ...(d.author ? { authorSlug: d.author } : {}),
            authorName: nameOf(d.author),
            ...(d.date ? { date: d.date } : {}),
            ...(ch != null ? { coreHours: ch } : {}),
            ...(d.hardware?.measured !== undefined ? { measured: d.hardware.measured } : {}),
          };
        });
    }

    // PROPS mode: the page's own scores list, enriched from the manifest.
    // Experiment pages live under a per-author folder
    // (…/experiments/<author-slug>/<key>), so match by URL segment: the last
    // segment is the key. Falls back to the bare path if no page is found.
    const isExperiment = (u: string) => u.startsWith(EXPERIMENTS_PREFIX);
    return (data ?? []).map((d) => {
      const doc = docs.find((x) => isExperiment(x.url) && x.url.split("/").pop() === d.key);
      const url = doc?.url ?? `${EXPERIMENTS_PREFIX}${d.key}`;
      const ch = doc?.hardware ? coreHours(doc.hardware) : null;
      return {
        key: d.key,
        label: d.label,
        score: d.score,
        family: d.family,
        url,
        ...(doc?.rigor ? { rigor: doc.rigor } : {}),
        ...(doc?.repro ? { repro: doc.repro.kind } : {}),
        ...(doc?.outcome ? { outcome: doc.outcome } : {}),
        ...(doc?.scoringConvention ? { scoringConvention: doc.scoringConvention } : {}),
        ...(doc?.author ? { authorSlug: doc.author } : {}),
        authorName: nameOf(doc?.author),
        ...(doc?.date ? { date: doc.date } : {}),
        ...(ch != null ? { coreHours: ch } : {}),
        ...(doc?.hardware?.measured !== undefined ? { measured: doc.hardware.measured } : {}),
      };
    });
  }, [data, lang, manifest]);

  const sortRows = (list: Row[]) => {
    const s = [...list].sort((a, b) => {
      if (sortKey === "score") return b.score - a.score;
      if (sortKey === "label") return a.label.localeCompare(b.label);
      if (sortKey === "date") return (b.date ?? "").localeCompare(a.date ?? "");
      if (sortKey === "outcome") return (a.outcome ?? "").localeCompare(b.outcome ?? "");
      // Cost: cheapest first ascending; rows without a cost sink to the bottom.
      if (sortKey === "cost") return (a.coreHours ?? Infinity) - (b.coreHours ?? Infinity);
      return (FAMILY_LABEL[a.family ?? ""] ?? a.family ?? "").localeCompare(
        FAMILY_LABEL[b.family ?? ""] ?? b.family ?? "",
      );
    });
    return asc ? s.reverse() : s;
  };

  // One section per author (ownership at a glance) when there is more than one;
  // a single flat table otherwise. Authors ordered by their best score.
  const sections = useMemo(() => {
    const byAuthor = new Map<string, Row[]>();
    for (const r of rows) {
      const bucket = byAuthor.get(r.authorName) ?? [];
      bucket.push(r);
      byAuthor.set(r.authorName, bucket);
    }
    const entries = [...byAuthor.entries()].map(([name, list]) => ({
      name,
      rows: sortRows(list),
      best: Math.max(...list.map((x) => x.score)),
    }));
    entries.sort((a, b) => b.best - a.best);
    return entries;
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [rows, sortKey, asc]);

  const grouped = sections.length > 1;

  const setSort = (k: SortKey) => {
    if (k === sortKey) setAsc((v) => !v);
    else {
      setSortKey(k);
      setAsc(false);
    }
  };

  const sortableHeader = (k: SortKey, label: string, className?: string) => (
    <th className={cn("px-3 py-2 text-left font-semibold", className)}>
      <button
        onClick={() => setSort(k)}
        className="inline-flex items-center gap-1 hover:text-foreground"
        aria-label={`Sort by ${label}`}
      >
        {label}
        <span aria-hidden className={cn("text-[10px]", sortKey === k ? "opacity-100" : "opacity-30")}>
          {sortKey === k ? (asc ? "▲" : "▼") : "▼"}
        </span>
      </button>
    </th>
  );

  const bodyRows = (list: Row[]) =>
    list.map((r) => (
      <tr key={r.key} className="border-b last:border-0 hover:bg-muted/30">
        <td className="px-3 py-2 font-medium">
          <LocalizedLink to={r.url} className="underline-offset-2 hover:underline">
            {r.label}
          </LocalizedLink>
        </td>
        <td className="px-3 py-2 text-right tabular-nums font-semibold whitespace-nowrap">
          {r.score}
          <span className="text-xs font-normal text-muted-foreground">/480</span>
          {manifest && r.scoringConvention && (
            <span
              className="ml-1.5 rounded-full bg-muted px-1.5 py-0.5 align-middle text-[10px] font-normal text-muted-foreground"
              title={r.scoringConvention}
            >
              {CONVENTION_LABEL[r.scoringConvention]}
            </span>
          )}
        </td>
        {!manifest && (
          <td className="px-3 py-2 text-muted-foreground">
            {FAMILY_LABEL[r.family ?? ""] ?? r.family ?? "—"}
          </td>
        )}
        <td className="px-3 py-2">
          {r.outcome && (
            <span
              className={cn("rounded-full px-2 py-0.5 text-xs font-medium", OUTCOME_STYLE[r.outcome])}
              title={OUTCOME_TITLES[lang][r.outcome]}
            >
              {OUTCOME_LABELS[lang][r.outcome]}
            </span>
          )}
        </td>
        <td className="px-3 py-2 whitespace-nowrap text-muted-foreground tabular-nums">
          {monthYear(r.date) || "—"}
        </td>
        <td className="px-3 py-2 whitespace-nowrap text-right tabular-nums">
          {r.coreHours != null ? (
            <span
              className={cn(r.measured ? "text-emerald-700 dark:text-emerald-300" : "text-muted-foreground")}
              title={r.measured ? "standardized single-core bench" : "native run"}
            >
              {formatCoreHours(r.coreHours)}
            </span>
          ) : (
            <span className="text-muted-foreground">—</span>
          )}
        </td>
        <td className="px-3 py-2">
          {r.rigor && (
            <span className={cn("rounded-full px-2 py-0.5 text-xs font-medium", RIGOR_STYLE[r.rigor])}>
              {r.rigor}
            </span>
          )}
        </td>
        <td className="px-3 py-2 text-muted-foreground">{r.repro ? REPRO_LABEL[r.repro] : "—"}</td>
      </tr>
    ));

  // PROPS mode shows Method and Outcome side by side; MANIFEST mode has no
  // per-page family, so it carries one column less.
  const colCount = manifest ? 7 : 8;

  return (
    <div className="my-6 overflow-x-auto rounded-lg border">
      <table className="w-full min-w-[50rem] text-sm">
        <thead className="border-b bg-muted/40 text-xs uppercase tracking-wide text-muted-foreground">
          <tr>
            {sortableHeader("label", "Experiment")}
            {sortableHeader("score", "Score", "text-right")}
            {!manifest && sortableHeader("family", "Method")}
            {sortableHeader("outcome", "Outcome")}
            {sortableHeader("date", "When")}
            {sortableHeader("cost", "Core-h", "text-right")}
            <th className="px-3 py-2 text-left font-semibold">Rigor</th>
            <th className="px-3 py-2 text-left font-semibold">Reproduces</th>
          </tr>
        </thead>
        <tbody>
          {grouped
            ? sections.map((s) => (
                <Fragment key={`sec-${s.name}`}>
                  <tr className="border-b bg-muted/20">
                    <th
                      colSpan={colCount}
                      className="px-3 py-1.5 text-left text-xs font-semibold uppercase tracking-wide text-muted-foreground"
                    >
                      {s.name}
                      <span className="ml-2 font-normal normal-case">
                        best {s.best}/480 · {s.rows.length}{" "}
                        {s.rows.length === 1 ? "experiment" : "experiments"}
                      </span>
                    </th>
                  </tr>
                  {bodyRows(s.rows)}
                </Fragment>
              ))
            : bodyRows(sections[0]?.rows ?? [])}
        </tbody>
      </table>
    </div>
  );
}
