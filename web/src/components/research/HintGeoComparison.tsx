import { useMemo } from "react";

import { useT } from "@/i18n";
import { Swatch } from "@/components/research/FamilyLegend";
import data from "@/data/hint-study.json";

// Confirming or qualifying the /research/why/hint-geometry page on OUR boards.
// That page contrasts 18 SCATTERED hints (rows {1,3,5} × every third column, the
// shape Peter McGavin used) against 18 CONTIGUOUS hints piled in the top rows, and
// reports scattered solves easily while contiguous barely helps. We ran both SHAPES
// on our own generated boards and solvers; this chart shows the median score of
// each, per fill path, so a reader can see whether scattered beats contiguous here.

const PATH_LABEL: Record<string, string> = {
  rowmajor: "Row-major",
  "rowmajor-bottomup": "Row-major ↑",
  "spiral-in": "Spiral in",
  "spiral-out": "Spiral out",
  "verhaard-comb": "Verhaard comb",
  "clue-rows-first": "Clue rows first",
  "connect-hints-first": "Connect hints first",
  "trace-hints": "Trace hints",
};

type PerPath = { path: string; score: { median: number; min: number; max: number }; solved: number; of: number };

const T = {
  en: {
    title: "18 scattered vs 18 contiguous, on our boards",
    scattered: "scattered 18 (list shape)",
    contiguous: "contiguous 18",
    axis: "median matched edges / 480",
    caption:
      "The two 18-hint layouts from the hint-geometry write-up, measured here per fill path. Contrary to that page, on our boards the contiguous block scores higher than the scattered lattice for seven of the eight orders, often by a wide margin. The two studies measure different things: the community result is time to a full solution, where scattered hints reach the endgame; ours is matched-edge score at a short budget, where a contiguous top block gives a row-major sweep a large correct region to build against fast.",
  },
};

export function HintGeoComparison() {
  const t = useT(T);

  const rows = useMemo(() => {
    const hg = (data as { hintgeo?: Record<string, PerPath[]> }).hintgeo;
    if (!hg) return [];
    const scattered = new Map((hg["hintgeo_scattered_18"] ?? []).map((p) => [p.path, p]));
    const contiguous = new Map((hg["hintgeo_contiguous_18"] ?? []).map((p) => [p.path, p]));
    const paths = [...scattered.keys()];
    return paths
      .map((path) => ({
        path,
        scattered: scattered.get(path)?.score.median ?? 0,
        contiguous: contiguous.get(path)?.score.median ?? 0,
      }))
      .sort((a, b) => b.scattered - a.scattered);
  }, []);

  if (rows.length === 0) return null;
  const MAX = 480;

  return (
    <figure className="space-y-2">
      <figcaption className="text-sm font-semibold">{t.title}</figcaption>
      <div className="space-y-2">
        {rows.map((r) => (
          <div key={r.path} className="flex items-center gap-2 text-xs">
            <div className="w-32 shrink-0 text-right text-muted-foreground">{PATH_LABEL[r.path] ?? r.path}</div>
            <div className="flex-1 space-y-1">
              {/* scattered bar */}
              <div className="flex items-center gap-1.5">
                <div className="relative h-3 flex-1 overflow-hidden rounded bg-muted">
                  <div className="absolute left-0 top-0 h-full bg-sky-500" style={{ width: `${(r.scattered / MAX) * 100}%` }} />
                </div>
                <div className="w-8 shrink-0 tabular-nums text-muted-foreground">{r.scattered}</div>
              </div>
              {/* contiguous bar */}
              <div className="flex items-center gap-1.5">
                <div className="relative h-3 flex-1 overflow-hidden rounded bg-muted">
                  <div className="absolute left-0 top-0 h-full bg-amber-400" style={{ width: `${(r.contiguous / MAX) * 100}%` }} />
                </div>
                <div className="w-8 shrink-0 tabular-nums text-muted-foreground">{r.contiguous}</div>
              </div>
            </div>
          </div>
        ))}
      </div>
      <div className="flex flex-wrap gap-x-4 gap-y-1.5 text-xs">
        <Swatch color="#0ea5e9" label={t.scattered} />
        <Swatch color="#fbbf24" label={t.contiguous} />
      </div>
      <p className="text-xs text-muted-foreground">{t.caption}</p>
    </figure>
  );
}
