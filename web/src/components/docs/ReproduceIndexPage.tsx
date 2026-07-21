// The reproduce index at /research/build/reproduce: one sortable table of every
// research page that ships a reproduce command (repro.cmd), with its repro-kind
// badge, whether the command re-runs the SEARCH or only re-verifies a stored
// ARTIFACT, the expected wall-clock cost, a compact core-hours figure, and a
// link to the research/topics pipeline it comes from. Route-generated (like the
// glossary and the topic hubs): no MDX file backs it — it walks the manifest.

import { useMemo, useState } from "react";
import { useLang, useT } from "@/i18n";
import { LocalizedLink } from "@/components/LocalizedLink";
import { cn } from "@/lib/utils";
import { researchDocs } from "@/lib/research/manifest";
import { wallClockHours, coreHours, formatCoreHours } from "@/lib/research/hardware-cost";
import type { Lang } from "@/i18n";
import type { ReproKind, ResearchDoc } from "@/lib/research/types";
import { DocsSidebar } from "./DocsSidebar";
import { ResearchSubnav } from "./ResearchSubnav";

// Reuse the reproduce-kind labels the results table already uses, so the two
// surfaces read the same. `prose` never appears here (those pages carry no cmd).
const REPRO_LABEL: Record<ReproKind, Record<Lang, string>> = {
  exact: { en: "exact", fr: "exact", es: "exacto" },
  seeded: { en: "seeded", fr: "graine", es: "con semilla" },
  stochastic: { en: "stochastic", fr: "stochastique", es: "estocástico" },
  heavy: { en: "heavy", fr: "lourd", es: "pesado" },
  prose: { en: "—", fr: "—", es: "—" },
};

const REPRO_STYLE: Record<ReproKind, string> = {
  exact: "bg-emerald-500/15 text-emerald-700 dark:text-emerald-300",
  seeded: "bg-sky-500/15 text-sky-700 dark:text-sky-300",
  stochastic: "bg-amber-500/15 text-amber-700 dark:text-amber-300",
  heavy: "bg-rose-500/15 text-rose-700 dark:text-rose-300",
  prose: "bg-muted text-muted-foreground",
};

const T = {
  en: {
    research: "Research",
    build: "Build a solver",
    title: "Reproduce index",
    lede: "Every result on the wiki that ships a reproduce command, in one table: what the command does, whether it re-runs the search or only re-checks a stored board, and roughly what it costs to run. Click a column head to reorder.",
    page: "Page",
    kind: "Command",
    produces: "Reproduces",
    search: "search",
    artifact: "verify only",
    cost: "Wall-clock",
    coreH: "Core-h",
    topic: "Pipeline",
    empty: "No page ships a reproduce command yet.",
    searchTip: "re-runs the search that produced the result",
    artifactTip: "re-verifies a stored board, not the search behind it",
    count: (n: number) => (n === 1 ? "1 command" : `${n} commands`),
  },
  fr: {
    research: "Recherche",
    build: "Écrire un solveur",
    title: "Index de reproduction",
    lede: "Chaque résultat du wiki livré avec une commande de reproduction, en un seul tableau : ce que fait la commande, si elle relance la recherche ou revérifie seulement un plateau enregistré, et à peu près ce qu'elle coûte à exécuter. Cliquez sur un en-tête de colonne pour réordonner.",
    page: "Page",
    kind: "Commande",
    produces: "Reproduit",
    search: "recherche",
    artifact: "vérif. seule",
    cost: "Temps réel",
    coreH: "Cœur-h",
    topic: "Pipeline",
    empty: "Aucune page ne livre encore de commande de reproduction.",
    searchTip: "relance la recherche qui a produit le résultat",
    artifactTip: "revérifie un plateau enregistré, pas la recherche derrière lui",
    count: (n: number) => (n === 1 ? "1 commande" : `${n} commandes`),
  },
  es: {
    research: "Investigación",
    build: "Crear un solucionador",
    title: "Índice de reproducción",
    lede: "Cada resultado del wiki que incluye un comando de reproducción, en una sola tabla: qué hace el comando, si vuelve a ejecutar la búsqueda o solo reverifica un tablero guardado, y aproximadamente cuánto cuesta ejecutarlo. Pulsa un encabezado de columna para reordenar.",
    page: "Página",
    kind: "Comando",
    produces: "Reproduce",
    search: "búsqueda",
    artifact: "solo verif.",
    cost: "Tiempo real",
    coreH: "Horas-núcleo",
    topic: "Pipeline",
    empty: "Ninguna página incluye todavía un comando de reproducción.",
    searchTip: "vuelve a ejecutar la búsqueda que produjo el resultado",
    artifactTip: "reverifica un tablero guardado, no la búsqueda que hay detrás",
    count: (n: number) => (n === 1 ? "1 comando" : `${n} comandos`),
  },
};

/** Whether a repro entry re-runs the search or only re-checks a stored board.
 *  Honours an explicit `produces`; otherwise the default rule: a record-board
 *  page re-verifies an artifact, everything else re-runs a search. */
function producesOf(doc: ResearchDoc): "search" | "artifact" {
  const r = doc.repro;
  if (r?.produces) return r.produces;
  return r?.topic === "record-boards" ? "artifact" : "search";
}

interface Row {
  url: string;
  title: string;
  kind: ReproKind;
  produces: "search" | "artifact";
  wallClock: string;
  /** Parsed wall-clock hours per run, for sorting; null when not legible. */
  wallHours: number | null;
  /** Total compute (cores × hours × runs), for sorting + display; null if N/A. */
  coreHours: number | null;
  topic?: string;
}

type SortKey = "page" | "kind" | "produces" | "cost" | "coreH";

function Crumbs({ current }: { current: string }) {
  const t = useT(T);
  return (
    <nav className="flex flex-wrap items-center gap-1.5 text-sm text-muted-foreground">
      <LocalizedLink to="/research" className="hover:text-foreground">
        {t.research}
      </LocalizedLink>
      <span aria-hidden>/</span>
      <LocalizedLink to="/research/build" className="hover:text-foreground">
        {t.build}
      </LocalizedLink>
      <span aria-hidden>/</span>
      <span className="text-foreground">{current}</span>
    </nav>
  );
}

export function ReproduceIndexPage() {
  const t = useT(T);
  const { lang } = useLang();
  const [sortKey, setSortKey] = useState<SortKey>("page");
  const [asc, setAsc] = useState(true);

  const rows = useMemo<Row[]>(() => {
    return researchDocs(lang)
      .filter((d) => d.repro?.cmd)
      .map((d) => {
        const wallClock = d.hardware?.wallClock ?? "";
        const ch = d.hardware ? coreHours(d.hardware) : null;
        return {
          url: d.url,
          title: d.title,
          kind: d.repro?.kind ?? "exact",
          produces: producesOf(d),
          wallClock,
          wallHours: wallClockHours(wallClock),
          coreHours: ch,
          ...(d.repro?.topic ? { topic: d.repro.topic } : {}),
        };
      });
  }, [lang]);

  const sorted = useMemo(() => {
    const s = [...rows].sort((a, b) => {
      if (sortKey === "page") return a.title.localeCompare(b.title);
      if (sortKey === "kind") return a.kind.localeCompare(b.kind);
      if (sortKey === "produces") return a.produces.localeCompare(b.produces);
      // Cost sorts on parsed hours; rows with no legible budget sink to the end.
      if (sortKey === "cost") return (a.wallHours ?? Infinity) - (b.wallHours ?? Infinity);
      return (a.coreHours ?? Infinity) - (b.coreHours ?? Infinity);
    });
    return asc ? s : s.reverse();
  }, [rows, sortKey, asc]);

  const setSort = (k: SortKey) => {
    if (k === sortKey) setAsc((v) => !v);
    else {
      setSortKey(k);
      setAsc(true);
    }
  };

  const sortableHeader = (k: SortKey, label: string, className?: string) => (
    <th className={cn("px-3 py-2 text-left font-semibold", className)}>
      <button
        onClick={() => setSort(k)}
        className="inline-flex items-center gap-1 hover:text-foreground"
        aria-label={`${label}`}
      >
        {label}
        <span aria-hidden className={cn("text-[10px]", sortKey === k ? "opacity-100" : "opacity-30")}>
          {sortKey === k ? (asc ? "▲" : "▼") : "▼"}
        </span>
      </button>
    </th>
  );

  return (
    <div>
      <ResearchSubnav />
      <div className="lg:grid lg:grid-cols-[15rem_minmax(0,1fr)] lg:gap-10">
        <DocsSidebar section={null} />
        <div className="min-w-0">
          <Crumbs current={t.title} />
          <header className="mt-4 space-y-3">
            <h1 className="text-3xl font-bold tracking-tight sm:text-4xl">{t.title}</h1>
            <p className="text-lg text-muted-foreground">{t.lede}</p>
          </header>

          {sorted.length === 0 ? (
            <p className="mt-8 rounded-lg border border-dashed p-6 text-sm text-muted-foreground">
              {t.empty}
            </p>
          ) : (
            <>
              <p className="mt-6 text-sm text-muted-foreground">{t.count(sorted.length)}</p>
              <div className="mt-3 overflow-x-auto rounded-lg border">
                <table className="w-full min-w-[46rem] text-sm">
                  <thead className="border-b bg-muted/40 text-xs uppercase tracking-wide text-muted-foreground">
                    <tr>
                      {sortableHeader("page", t.page)}
                      {sortableHeader("kind", t.kind)}
                      {sortableHeader("produces", t.produces)}
                      {sortableHeader("cost", t.cost)}
                      {sortableHeader("coreH", t.coreH, "text-right")}
                      <th className="px-3 py-2 text-left font-semibold">{t.topic}</th>
                    </tr>
                  </thead>
                  <tbody>
                    {sorted.map((r) => (
                      <tr key={r.url} className="border-b last:border-0 hover:bg-muted/30">
                        <td className="px-3 py-2 font-medium">
                          <LocalizedLink to={r.url} className="underline-offset-2 hover:underline">
                            {r.title}
                          </LocalizedLink>
                        </td>
                        <td className="px-3 py-2">
                          <span
                            className={cn(
                              "rounded-full px-2 py-0.5 text-xs font-medium",
                              REPRO_STYLE[r.kind],
                            )}
                          >
                            {REPRO_LABEL[r.kind][lang] ?? REPRO_LABEL[r.kind].en}
                          </span>
                        </td>
                        <td className="px-3 py-2">
                          <span
                            className={cn(
                              "text-xs font-medium",
                              r.produces === "search"
                                ? "text-foreground"
                                : "text-muted-foreground",
                            )}
                            title={r.produces === "search" ? t.searchTip : t.artifactTip}
                          >
                            {r.produces === "search" ? t.search : t.artifact}
                          </span>
                        </td>
                        <td className="px-3 py-2 whitespace-nowrap tabular-nums text-muted-foreground">
                          {r.wallClock || "—"}
                        </td>
                        <td className="px-3 py-2 whitespace-nowrap text-right tabular-nums text-muted-foreground">
                          {r.coreHours != null ? formatCoreHours(r.coreHours) : "—"}
                        </td>
                        <td className="px-3 py-2 text-muted-foreground">{r.topic ?? "—"}</td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            </>
          )}
        </div>
      </div>
    </div>
  );
}
