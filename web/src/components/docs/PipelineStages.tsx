// The pipeline-stages strip: a named run is usually not one algorithm but an
// ordered composition (beam-produce -> ALNS-lift -> refine). When a page
// declares `stages[]`, this renders them as a compact numbered strip near the
// top matter, so the composition is visible at a glance. The `published: false`
// flag on a stage surfaces a "not yet written up" marker, so a run never
// silently leans on an engine a reader cannot open a page for.

import { pick, useLang, useT, type Dict } from "@/i18n";
import { cn } from "@/lib/utils";
import type { ResearchDoc, StageEngine } from "@/lib/research/types";

const T = {
  en: {
    pipeline: "Pipeline",
    learns: "carries",
    notWritten: "not yet written up",
  },
  fr: {
    pipeline: "Pipeline",
    learns: "porte",
    notWritten: "pas encore documenté",
  },
  es: {
    pipeline: "Pipeline",
    learns: "aporta",
    notWritten: "aún sin documentar",
  },
};

/** Localized engine labels for each stage. A closed vocabulary mirrored from
 *  the schema in content.config.ts. */
const ENGINE_LABEL: Record<StageEngine, Dict<string>> = {
  "beam-producer": { en: "beam producer", fr: "producteur beam", es: "productor beam" },
  alns: { en: "ALNS", fr: "ALNS", es: "ALNS" },
  "break-dfs": { en: "break DFS", fr: "DFS de ruptures", es: "DFS de roturas" },
  "exact-tail": { en: "exact tail", fr: "queue exacte", es: "cola exacta" },
  maxsat: { en: "MaxSAT", fr: "MaxSAT", es: "MaxSAT" },
  "restart-tournament": {
    en: "restart tournament",
    fr: "tournoi de redémarrages",
    es: "torneo de reinicios",
  },
  refinement: { en: "refinement", fr: "raffinement", es: "refinamiento" },
  "corpus-mining": { en: "corpus mining", fr: "fouille de corpus", es: "minería de corpus" },
  clustering: { en: "clustering", fr: "clustering", es: "clustering" },
  "frame-input": { en: "frame input", fr: "cadre en entrée", es: "marco de entrada" },
};

/** The ordered stages of a run, rendered as a compact numbered strip. Gated on
 *  `doc.stages` presence: pages that are a single algorithm declare none, and
 *  render nothing here. */
export function PipelineStages({ doc }: { doc: ResearchDoc }) {
  const t = useT(T);
  const { lang } = useLang();
  const stages = doc.stages;
  if (!stages || stages.length === 0) return null;
  return (
    <section className="mt-6 rounded-lg border bg-muted/20 p-4">
      <div className="text-xs font-semibold uppercase tracking-wide text-muted-foreground">
        {t.pipeline}
      </div>
      <ol className="mt-2 space-y-2">
        {stages.map((s, i) => (
          <li key={i} className="flex gap-2.5 text-sm">
            <span className="mt-0.5 flex h-5 w-5 shrink-0 items-center justify-center rounded-full bg-muted text-xs font-semibold tabular-nums text-muted-foreground">
              {i + 1}
            </span>
            <div className="min-w-0">
              <div className="flex flex-wrap items-center gap-1.5">
                <span className="font-medium">{pick(ENGINE_LABEL[s.engine], lang)}</span>
                {!s.published && (
                  <span
                    className={cn(
                      "rounded-full border border-dashed border-amber-500/50 px-1.5 py-0.5",
                      "text-[10px] font-medium text-amber-700 dark:text-amber-400",
                    )}
                  >
                    {t.notWritten}
                  </span>
                )}
              </div>
              <p className="text-muted-foreground">{s.does}</p>
              {s.learns && (
                <p className="text-xs text-muted-foreground/80">
                  {t.learns}: {s.learns}
                </p>
              )}
            </div>
          </li>
        ))}
      </ol>
    </section>
  );
}
