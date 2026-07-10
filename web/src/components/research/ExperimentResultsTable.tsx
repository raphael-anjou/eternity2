import { useMemo, useState } from "react";
import { LocalizedLink } from "@/components/LocalizedLink";
import { useLang } from "@/i18n";
import { researchDocs } from "@/lib/research/manifest";
import { cn } from "@/lib/utils";
import type { ScoreDatum } from "./ExperimentScoreChart";
import type { RigorKind, ReproKind } from "@/lib/research/types";

// The whole experiment gallery as one sortable results table — the scannable
// companion to the score chart. Score and method family come from the page's
// own `scores` list (which the chart also uses); rigor, reproducibility and the
// link are pulled from each experiment's frontmatter via the manifest, so the
// table never drifts from the pages it summarizes.

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
}

type SortKey = "score" | "label" | "family";

export function ExperimentResultsTable({ data }: { data: ScoreDatum[] }) {
  const { lang } = useLang();
  const [sortKey, setSortKey] = useState<SortKey>("score");
  const [asc, setAsc] = useState(false);

  const rows = useMemo<Row[]>(() => {
    const docs = researchDocs(lang);
    return data.map((d) => {
      const doc = docs.find((x) => x.url === `/research/lab/experiments/${d.key}`);
      return {
        ...d,
        url: `/research/lab/experiments/${d.key}`,
        ...(doc?.rigor ? { rigor: doc.rigor } : {}),
        ...(doc?.repro ? { repro: doc.repro.kind } : {}),
      };
    });
  }, [data, lang]);

  const sorted = useMemo(() => {
    const s = [...rows].sort((a, b) => {
      if (sortKey === "score") return b.score - a.score;
      if (sortKey === "label") return a.label.localeCompare(b.label);
      return (FAMILY_LABEL[a.family] ?? a.family).localeCompare(FAMILY_LABEL[b.family] ?? b.family);
    });
    return asc ? s.reverse() : s;
  }, [rows, sortKey, asc]);

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

  return (
    <div className="my-6 overflow-x-auto rounded-lg border">
      <table className="w-full min-w-[34rem] text-sm">
        <thead className="border-b bg-muted/40 text-xs uppercase tracking-wide text-muted-foreground">
          <tr>
            {sortableHeader("label", "Experiment")}
            {sortableHeader("score", "Score", "text-right")}
            {sortableHeader("family", "Method")}
            <th className="px-3 py-2 text-left font-semibold">Rigor</th>
            <th className="px-3 py-2 text-left font-semibold">Reproduces</th>
          </tr>
        </thead>
        <tbody>
          {sorted.map((r) => (
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
              <td className="px-3 py-2">
                {r.rigor && (
                  <span className={cn("rounded-full px-2 py-0.5 text-xs font-medium", RIGOR_STYLE[r.rigor])}>
                    {r.rigor}
                  </span>
                )}
              </td>
              <td className="px-3 py-2 text-muted-foreground">{r.repro ? REPRO_LABEL[r.repro] : "—"}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
