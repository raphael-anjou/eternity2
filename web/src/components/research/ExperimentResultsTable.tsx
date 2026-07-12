import { Fragment, useMemo, useState } from "react";
import { LocalizedLink } from "@/components/LocalizedLink";
import { useLang } from "@/i18n";
import { researchDocs, researchAuthors } from "@/lib/research/manifest";
import { cn } from "@/lib/utils";
import type { ScoreDatum } from "./ExperimentScoreChart";
import type { RigorKind, ReproKind } from "@/lib/research/types";

// The whole experiment gallery as one sortable results table — the scannable
// companion to the score chart. Score and method family come from the page's
// own `scores` list (which the chart also uses); author, month, rigor,
// reproducibility and the link are pulled from each experiment's frontmatter via
// the manifest, so the table never drifts from the pages it summarizes. When
// more than one researcher has experiments, the table splits into a section per
// author (ownership at a glance); with a single author it is one flat table.

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

const REPRO_LABEL: Record<ReproKind, string> = {
  exact: "exact",
  seeded: "seeded",
  stochastic: "stochastic",
  heavy: "heavy",
  prose: "—",
};

interface Row extends ScoreDatum {
  url: string;
  rigor?: RigorKind;
  repro?: ReproKind;
  authorSlug?: string;
  authorName: string;
  date?: string;
}

type SortKey = "score" | "label" | "family" | "date";

export function ExperimentResultsTable({ data }: { data: ScoreDatum[] }) {
  const { lang } = useLang();
  const [sortKey, setSortKey] = useState<SortKey>("score");
  const [asc, setAsc] = useState(false);

  const rows = useMemo<Row[]>(() => {
    const docs = researchDocs(lang);
    const authors = researchAuthors(lang);
    const nameOf = (slug?: string) =>
      (slug && authors.find((a) => a.slug === slug)?.name) || "Unattributed";
    return data.map((d) => {
      const doc = docs.find((x) => x.url === `/research/lab/experiments/${d.key}`);
      return {
        ...d,
        url: `/research/lab/experiments/${d.key}`,
        ...(doc?.rigor ? { rigor: doc.rigor } : {}),
        ...(doc?.repro ? { repro: doc.repro.kind } : {}),
        ...(doc?.author ? { authorSlug: doc.author } : {}),
        authorName: nameOf(doc?.author),
        ...(doc?.date ? { date: doc.date } : {}),
      };
    });
  }, [data, lang]);

  const sortRows = (list: Row[]) => {
    const s = [...list].sort((a, b) => {
      if (sortKey === "score") return b.score - a.score;
      if (sortKey === "label") return a.label.localeCompare(b.label);
      if (sortKey === "date") return (b.date ?? "").localeCompare(a.date ?? "");
      return (FAMILY_LABEL[a.family] ?? a.family).localeCompare(FAMILY_LABEL[b.family] ?? b.family);
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
        <td className="px-3 py-2 text-right tabular-nums font-semibold">
          {r.score}
          <span className="text-xs font-normal text-muted-foreground">/480</span>
        </td>
        <td className="px-3 py-2 text-muted-foreground">{FAMILY_LABEL[r.family] ?? r.family}</td>
        <td className="px-3 py-2 whitespace-nowrap text-muted-foreground tabular-nums">
          {monthYear(r.date) || "—"}
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

  const colCount = 6;

  return (
    <div className="my-6 overflow-x-auto rounded-lg border">
      <table className="w-full min-w-[40rem] text-sm">
        <thead className="border-b bg-muted/40 text-xs uppercase tracking-wide text-muted-foreground">
          <tr>
            {sortableHeader("label", "Experiment")}
            {sortableHeader("score", "Score", "text-right")}
            {sortableHeader("family", "Method")}
            {sortableHeader("date", "When")}
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
