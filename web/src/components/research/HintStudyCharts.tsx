import { useMemo } from "react";

import { useT } from "@/i18n";
import { Swatch } from "@/components/research/FamilyLegend";
import data from "@/data/hint-study.json";

// The hint study's measured results, rendered from the committed run data
// (web/src/data/hint-study.json, from research/experiments/hint-study/scripts/
// make_site_json.py) so the page never drifts from what was measured.
//
// Three views, all on the five-clue-shape hint set, over the common complete seed
// set (a paired design — every path runs on the identical boards):
//   1. DISTRIBUTION — every per-seed score as a dot, per path. The scores are
//      BIMODAL (an instance either nearly solves or stalls), so a single median
//      bar would lie; the dots show the two regimes directly.
//   2. HINTS: HELP OR HURT — the paired delta (score WITH the five hints minus
//      score with NO hints, same seed). Every path is ≤ 0: the hints never help,
//      and on the hint-seeking order they are catastrophic.
//   3. COUNT axis — the floor-corrected clustered-vs-spread comparison.

const PATH_LABEL: Record<string, string> = {
  rowmajor: "Row-major",
  "rowmajor-bottomup": "Row-major ↑",
  "spiral-in": "Spiral in",
  "spiral-out": "Spiral out",
  "border-first": "Border first",
  "verhaard-comb": "Verhaard comb",
  "clue-rows-first": "Clue rows first",
  "connect-hints-first": "Connect hints first",
};

// Colour by how the fill order keeps its frontier — identity, not rank.
const KIND: Record<string, "compact" | "fragment" | "seeking"> = {
  rowmajor: "compact",
  "rowmajor-bottomup": "compact",
  "verhaard-comb": "compact",
  "spiral-in": "fragment",
  "spiral-out": "fragment",
  "border-first": "fragment",
  "clue-rows-first": "fragment",
  "connect-hints-first": "seeking",
};
const COLOR = { compact: "#3b82f6", fragment: "#f59e0b", seeking: "#ef4444" } as const;

type PathRow = {
  path: string;
  perSeed: number[];
  median: number;
  solved: number;
  of: number;
  baseline: number | null;
  delta: number | null;
  deltaMin: number | null;
  deltaMax: number | null;
};

const T = {
  en: {
    distTitle: "The same five hints, eight fill orders — every instance",
    deltaTitle: "What the five hints are worth: nothing, or less",
    countTitle: "More hints help — but mind the free floor",
    scoreAxis: "matched edges / 480",
    deltaAxis: "score change from adding the five hints  (matched edges)",
    freeFloor: "free (pinned-seam floor)",
    earned: "earned by the search",
    solved: "solved",
    distCaption:
      "One dot per board. The scores are bimodal — a board either nearly solves or the search stalls early, with little in between — so the spread, not a median, is the honest summary. The compact orders land high on most boards but are dragged down by the ones where a pinned piece walls off the sweep; the fragmenting orders sit low throughout.",
    deltaCaption:
      "The paired change from adding the five clue-shaped hints, same board with and without them. Every bar is at or below zero: the hints never help a backtracker here. On the compact orders they cost a little; on spiral-out and the hint-seeking order they are catastrophic, turning a board the solver nearly finishes without help into one it barely starts. A correct hint is still a hard constraint the fixed fill order must satisfy on arrival — and sometimes it cannot.",
    countCaption:
      "Each bar is the median score of a hint layout, split into the seams the pins complete for free (both endpoints pinned) and the seams the search actually earned. A clustered block banks a tall free floor — up to a quarter of the whole board — so its raw score flatters it. Compare the earned parts, not the totals.",
    beam: "For contrast, the beam solver (not a backtracker) reaches a median of",
    onThese: "on the same boards — the hints and board are far from unsolvable; the chronological backtracker simply cannot use them.",
    compact: "compact frontier",
    fragment: "fragmenting order",
    seeking: "hint-seeking",
    busy: "Drawing…",
  },
};

export function HintStudyCharts() {
  const t = useT(T);

  const rows: PathRow[] = useMemo(
    () =>
      (data.path_axis as Array<{
        path: string;
        per_seed: Array<{ score: number }>;
        score: { median: number };
        solved: number;
        of: number;
        baseline_median: number | null;
        delta_median: number | null;
        delta_min: number | null;
        delta_max: number | null;
      }>).map((r) => ({
        path: r.path,
        perSeed: r.per_seed.map((p) => p.score),
        median: r.score.median,
        solved: r.solved,
        of: r.of,
        baseline: r.baseline_median,
        delta: r.delta_median,
        deltaMin: r.delta_min,
        deltaMax: r.delta_max,
      })),
    [],
  );

  const byScore = [...rows].sort((a, b) => b.median - a.median);
  const beam = data.beam as { score: { median: number }; of: number } | null;

  return (
    <div className="space-y-8">
      <Distribution rows={byScore} t={t} />
      <DeltaChart rows={byScore} t={t} />
      {beam && (
        <p className="rounded-lg border bg-muted/30 p-3 text-xs text-muted-foreground">
          {t.beam} <span className="font-semibold text-foreground">{beam.score.median}/480</span> {t.onThese}
        </p>
      )}
      <CountAxis t={t} />
    </div>
  );
}

// A strip/beeswarm-ish distribution: each path is a row, each board a dot.
function Distribution({ rows, t }: { rows: PathRow[]; t: (typeof T)["en"] }) {
  const W = 640;
  const rowH = 34;
  const H = rows.length * rowH + 40;
  const M = { l: 130, r: 20, t: 10, b: 26 };
  const x = (s: number) => M.l + (s / 480) * (W - M.l - M.r);

  return (
    <figure className="space-y-2">
      <figcaption className="text-sm font-semibold">{t.distTitle}</figcaption>
      <svg viewBox={`0 0 ${W} ${H}`} className="w-full rounded-lg border bg-card" role="img"
        aria-label="Per-board score distribution for each fill order, showing the bimodal split between solved and stalled boards.">
        {[0, 120, 240, 360, 480].map((s) => (
          <g key={s}>
            <line x1={x(s)} y1={M.t} x2={x(s)} y2={H - M.b} className="stroke-border" strokeDasharray="2 3" />
            <text x={x(s)} y={H - M.b + 16} textAnchor="middle" className="fill-muted-foreground text-[10px]">{s}</text>
          </g>
        ))}
        {rows.map((r, i) => {
          const cy = M.t + i * rowH + rowH / 2;
          const kind = KIND[r.path] ?? "fragment";
          return (
            <g key={r.path}>
              <text x={M.l - 8} y={cy + 3} textAnchor="end" className="fill-foreground text-[11px]">{PATH_LABEL[r.path]}</text>
              {/* median tick */}
              <line x1={x(r.median)} y1={cy - 9} x2={x(r.median)} y2={cy + 9} className="stroke-muted-foreground" strokeWidth={1.5} />
              {r.perSeed.map((s, j) => (
                <circle key={j} cx={x(s)} cy={cy + (j % 2 === 0 ? -3 : 3)} r={3.5}
                  fill={COLOR[kind]} fillOpacity={0.55} className="stroke-card" strokeWidth={0.5} />
              ))}
            </g>
          );
        })}
        <text x={(M.l + W - M.r) / 2} y={H - 2} textAnchor="middle" className="fill-muted-foreground text-[10px]">{t.scoreAxis}</text>
      </svg>
      <div className="flex flex-wrap gap-x-4 gap-y-1.5 text-xs">
        <Swatch color={COLOR.compact} label={t.compact} />
        <Swatch color={COLOR.fragment} label={t.fragment} />
        <Swatch color={COLOR.seeking} label={t.seeking} />
      </div>
      <p className="text-xs text-muted-foreground">{t.distCaption}</p>
    </figure>
  );
}

// The paired delta: how much the five hints change each path's score (≤ 0).
function DeltaChart({ rows, t }: { rows: PathRow[]; t: (typeof T)["en"] }) {
  const withDelta = rows.filter((r) => r.delta != null).sort((a, b) => (b.delta ?? 0) - (a.delta ?? 0));
  const minDelta = Math.min(-360, ...withDelta.map((r) => r.deltaMin ?? 0));
  const W = 640;
  const rowH = 30;
  const H = withDelta.length * rowH + 40;
  const M = { l: 130, r: 40, t: 10, b: 26 };
  const x = (d: number) => M.r + M.l + ((d - minDelta) / (0 - minDelta)) * (W - M.l - M.r - M.r);
  const zeroX = x(0);

  return (
    <figure className="space-y-2">
      <figcaption className="text-sm font-semibold">{t.deltaTitle}</figcaption>
      <svg viewBox={`0 0 ${W} ${H}`} className="w-full rounded-lg border bg-card" role="img"
        aria-label="Paired change in score from adding the five hints, per fill order; every value is at or below zero.">
        {/* zero line */}
        <line x1={zeroX} y1={M.t} x2={zeroX} y2={H - M.b} className="stroke-foreground" strokeWidth={1} />
        <text x={zeroX} y={H - M.b + 16} textAnchor="middle" className="fill-muted-foreground text-[10px]">0</text>
        {[-360, -240, -120].map((d) => (
          <text key={d} x={x(d)} y={H - M.b + 16} textAnchor="middle" className="fill-muted-foreground text-[10px]">{d}</text>
        ))}
        {withDelta.map((r, i) => {
          const cy = M.t + i * rowH + rowH / 2;
          const kind = KIND[r.path] ?? "fragment";
          const d = r.delta ?? 0;
          return (
            <g key={r.path}>
              <text x={M.l - 8} y={cy + 3} textAnchor="end" className="fill-foreground text-[11px]">{PATH_LABEL[r.path]}</text>
              <rect x={Math.min(zeroX, x(d))} y={cy - 8} width={Math.abs(zeroX - x(d))} height={16} rx={2}
                fill={COLOR[kind]} fillOpacity={0.8} />
              <text x={x(d) - 4} y={cy + 3} textAnchor="end" className="fill-foreground text-[10px] font-semibold">{d > 0 ? `+${d}` : d}</text>
            </g>
          );
        })}
        <text x={(M.l + W - M.r) / 2} y={H - 2} textAnchor="middle" className="fill-muted-foreground text-[10px]">{t.deltaAxis}</text>
      </svg>
      <p className="text-xs text-muted-foreground">{t.deltaCaption}</p>
    </figure>
  );
}

function CountAxis({ t }: { t: (typeof T)["en"] }) {
  const rows = (data.count_axis as Array<{
    layout: string; floor: number; score: { median: number }; earned: number; solved: number; of: number;
  }>).map((r) => ({
    label: r.layout.replace("ladder_", "").replace("_", " "),
    floor: r.floor,
    earned: Math.max(0, r.score.median - r.floor),
    total: r.score.median,
    solved: r.solved,
    of: r.of,
  }));
  const MAX = 480;

  return (
    <figure className="space-y-2">
      <figcaption className="text-sm font-semibold">{t.countTitle}</figcaption>
      <div className="space-y-2">
        {rows.map((r) => (
          <div key={r.label} className="flex items-center gap-2 text-xs">
            <div className="w-40 shrink-0 text-right text-muted-foreground">{r.label}</div>
            <div className="relative h-5 flex-1 overflow-hidden rounded bg-muted">
              <div className="absolute left-0 top-0 h-full bg-emerald-500/70" style={{ width: `${(r.floor / MAX) * 100}%` }} title={`${r.floor} free`} />
              <div className="absolute top-0 h-full bg-sky-500" style={{ left: `${(r.floor / MAX) * 100}%`, width: `${(r.earned / MAX) * 100}%` }} title={`${r.earned} earned`} />
            </div>
            <div className="w-24 shrink-0 tabular-nums text-muted-foreground">
              {r.total}/480{r.solved > 0 ? ` · ${r.solved}/${r.of}✓` : ""}
            </div>
          </div>
        ))}
      </div>
      <div className="flex flex-wrap gap-x-4 gap-y-1.5 text-xs">
        <Swatch color="rgb(16 185 129 / 0.7)" label={t.freeFloor} />
        <Swatch color="#0ea5e9" label={t.earned} />
      </div>
      <p className="text-xs text-muted-foreground">{t.countCaption}</p>
    </figure>
  );
}
