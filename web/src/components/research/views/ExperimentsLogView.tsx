import { useMemo, useState } from "react";
import { useT } from "@/i18n";
import data from "@/data/experiments.json";

// The complete experiments log: every attempt from the research notebook, with
// its outcome and a one-line note. Generated from the vault's concept frontmatter
// (see research/topics/experiments-log/extract.py). Searchable and filterable by
// outcome. This is the honest long tail — successes, dead ends, and half-built
// ideas alike — so others can build on it and skip the walls we already hit.

interface Experiment {
  id: string;
  title: string;
  status: string;
  summary: string;
  why?: string;
}
const experiments = data.experiments as Experiment[];

const STATUSES = ["built", "partial", "refuted", "idea", "wont-do"] as const;

const STATUS_CLASS: Record<string, string> = {
  built: "bg-emerald-100 text-emerald-800 dark:bg-emerald-950 dark:text-emerald-300",
  partial: "bg-sky-100 text-sky-800 dark:bg-sky-950 dark:text-sky-300",
  refuted: "bg-red-100 text-red-800 dark:bg-red-950 dark:text-red-300",
  idea: "bg-amber-100 text-amber-800 dark:bg-amber-950 dark:text-amber-300",
  "wont-do": "bg-stone-200 text-stone-700 dark:bg-stone-800 dark:text-stone-300",
  other: "bg-muted text-muted-foreground",
};

const T = {
  en: {
    search: "Search experiments…",
    counts: (shown: number, total: number) => `${shown} of ${total}`,
    statusLabels: {
      built: "worked / built",
      partial: "partial",
      refuted: "refuted",
      idea: "idea (not built)",
      "wont-do": "won't do",
    } as Record<string, string>,
    legend: "Filter by outcome:",
    whyLabel: "Why:",
    none: "No experiments match.",
    note: "Generated from the research notebook's concept notes. Summaries are the original lab notes, kept terse; some use internal shorthand.",
  },
  fr: {
    search: "Rechercher des expériences…",
    counts: (shown: number, total: number) => `${shown} sur ${total}`,
    statusLabels: {
      built: "a marché / construit",
      partial: "partiel",
      refuted: "réfuté",
      idea: "idée (non construite)",
      "wont-do": "abandonné",
    } as Record<string, string>,
    legend: "Filtrer par résultat :",
    whyLabel: "Pourquoi :",
    none: "Aucune expérience ne correspond.",
    note: "Généré à partir des notes de concept du carnet de recherche. Les résumés sont les notes de laboratoire d'origine, gardées concises ; certaines emploient des abréviations internes.",
  },
};

export function ExperimentsLogView() {
  const t = useT(T);
  const [query, setQuery] = useState("");
  const [active, setActive] = useState<Set<string>>(new Set());

  const filtered = useMemo(() => {
    const q = query.trim().toLowerCase();
    return experiments.filter((e) => {
      if (active.size > 0 && !active.has(e.status)) return false;
      if (q && !(`${e.title} ${e.summary} ${e.why ?? ""}`.toLowerCase().includes(q))) return false;
      return true;
    });
  }, [query, active]);

  const toggle = (s: string) =>
    setActive((cur) => {
      const next = new Set(cur);
      if (next.has(s)) next.delete(s);
      else next.add(s);
      return next;
    });

  const countByStatus = useMemo(() => {
    const m: Record<string, number> = {};
    for (const e of experiments) m[e.status] = (m[e.status] ?? 0) + 1;
    return m;
  }, []);

  return (
    <div className="space-y-6">
      <div className="space-y-3">
        <input
          type="search"
          value={query}
          onChange={(e) => setQuery(e.target.value)}
          placeholder={t.search}
          className="w-full max-w-md rounded-md border bg-background px-3 py-2 text-sm"
        />
        <div className="flex flex-wrap items-center gap-2">
          <span className="text-sm text-muted-foreground">{t.legend}</span>
          {STATUSES.map((s) => (
            <button
              key={s}
              onClick={() => toggle(s)}
              className={
                "rounded-full px-3 py-1 text-xs font-medium transition-opacity " +
                STATUS_CLASS[s] +
                (active.size > 0 && !active.has(s) ? " opacity-40" : "")
              }
            >
              {t.statusLabels[s]} ({countByStatus[s] ?? 0})
            </button>
          ))}
          <span className="ml-auto text-xs tabular-nums text-muted-foreground">
            {t.counts(filtered.length, experiments.length)}
          </span>
        </div>
      </div>

      <div className="divide-y rounded-lg border">
        {filtered.map((e) => (
          <div key={e.id} className="flex items-start gap-3 px-4 py-3">
            <span
              className={
                "mt-0.5 shrink-0 rounded-full px-2 py-0.5 text-[10px] font-medium uppercase tracking-wide " +
                (STATUS_CLASS[e.status] ?? STATUS_CLASS["other"])
              }
            >
              {t.statusLabels[e.status] ?? e.status}
            </span>
            <div className="min-w-0">
              <div className="text-sm font-medium">{e.title}</div>
              {e.summary && (
                <div className="mt-0.5 text-sm text-muted-foreground">{e.summary}</div>
              )}
              {e.why && (
                <div className="mt-1 border-l-2 border-muted pl-2 text-xs text-muted-foreground/90">
                  <span className="font-medium text-foreground/70">{t.whyLabel}</span> {e.why}
                </div>
              )}
            </div>
          </div>
        ))}
        {filtered.length === 0 && (
          <div className="px-4 py-8 text-center text-sm text-muted-foreground">{t.none}</div>
        )}
      </div>

      <p className="text-xs text-muted-foreground">{t.note}</p>
    </div>
  );
}
