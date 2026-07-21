import { useMemo, useState } from "react";

import { useT } from "@/i18n";
import { Swatch } from "@/components/research/FamilyLegend";
import data from "@/data/hint-study.json";

// The study's flagship chart: the frontier LAW made visible. The x-axis is the
// peak open frontier of each fill order — computed from geometry alone, no solver
// — and the y-axis is the measured median score. The two are almost perfectly
// anti-correlated (mean frontier vs median score, r ≈ −0.96): score collapses as
// the frontier grows. Putting the
// computed CAUSE against the measured EFFECT turns "row-major wins" into a visible
// mechanism. Bubble area encodes the instance-to-instance spread, so the chart is
// honest about variance rather than hiding it.
//
// Built as plain SVG for full control over the regression line, the two regime
// bands, and the direct labels — and so it renders identically server and client.

const KIND: Record<string, "compact" | "fragment" | "seeking"> = {
  rowmajor: "compact",
  "rowmajor-bottomup": "compact",
  "verhaard-comb": "compact",
  "border-first": "fragment",
  "spiral-in": "fragment",
  "spiral-out": "fragment",
  "clue-rows-first": "fragment",
  "connect-hints-first": "seeking",
};

const COLOR = {
  compact: "#3b82f6",
  fragment: "#f59e0b",
  seeking: "#ef4444",
} as const;

const LABEL: Record<string, string> = {
  rowmajor: "Row-major",
  "rowmajor-bottomup": "Row-major ↑",
  "verhaard-comb": "Verhaard comb",
  "border-first": "Border first",
  "spiral-in": "Spiral in",
  "spiral-out": "Spiral out",
  "clue-rows-first": "Clue rows first",
  "connect-hints-first": "Connect hints first",
};

const T = {
  en: {
    x: "mean open frontier  (computed, cells)",
    y: "median score reached  (measured, / 480)",
    compact: "compact frontier",
    fragment: "fragmenting order",
    seeking: "hint-seeking",
    caption:
      "One dot per fill order — eight in all. The horizontal axis is pure geometry: the average open frontier that order holds across the whole fill, computed with no solver involved. The vertical axis is the measured median score. The two track each other closely (r ≈ −0.96 across these eight orders), which is what we should expect if branching cost is what a fill order really controls — though with only eight points this is a strong descriptive relationship, not a fitted law, and among the fragmenting orders on the right the frontier no longer settles the ranking. Bubble size is the spread across seeds.",
    r: "r ≈ −0.96",
    lowRegime: "compact sweep",
    highRegime: "fragmented fill",
    busy: "Drawing…",
  },
};

type Pt = { path: string; x: number; y: number; spread: number; kind: keyof typeof COLOR };

// Resolve label collisions: cluster points whose screen positions are close, then
// fan their labels out vertically above the cluster with a leader line to each dot.
// Isolated points keep a plain label just above the marker (no leader).
function labelLayout(
  pts: Pt[],
  px: (x: number) => number,
  py: (y: number) => number,
  rOf: (s: number) => number,
) {
  const scr = pts.map((p) => ({ p, sx: px(p.x), sy: py(p.y), r: rOf(p.spread) }));
  const used = new Array(scr.length).fill(false);
  const out: Array<{ p: Pt; lx: number; ly: number; leader: boolean }> = [];
  for (let i = 0; i < scr.length; i++) {
    if (used[i]) continue;
    // group everything within 40px of point i
    const group = [i];
    used[i] = true;
    for (let j = i + 1; j < scr.length; j++) {
      if (used[j]) continue;
      const a = scr[i]!;
      const b = scr[j]!;
      if (Math.hypot(a.sx - b.sx, a.sy - b.sy) < 42) { group.push(j); used[j] = true; }
    }
    if (group.length === 1) {
      const a = scr[i]!;
      out.push({ p: a.p, lx: a.sx, ly: a.sy - a.r - 5, leader: false });
    } else {
      // stack labels above the topmost point of the cluster
      const members = group.map((k) => scr[k]!).sort((u, v) => u.sy - v.sy);
      const topY = Math.min(...members.map((m) => m.sy - m.r)) - 8;
      const cx = members.reduce((s, m) => s + m.sx, 0) / members.length;
      members.forEach((m, k) => {
        out.push({ p: m.p, lx: cx, ly: topY - k * 12, leader: true });
      });
    }
  }
  return out;
}

export function FrontierPhaseChart() {
  const t = useT(T);
  const [hover, setHover] = useState<string | null>(null);

  const pts: Pt[] = useMemo(
    () =>
      (data.path_axis as Array<{
        path: string;
        mean_frontier: number | null;
        score: { median: number; min: number; max: number };
      }>)
        .filter((r) => r.mean_frontier != null)
        .map((r) => ({
          path: r.path,
          x: r.mean_frontier as number,
          y: r.score.median,
          spread: r.score.max - r.score.min,
          kind: KIND[r.path] ?? "fragment",
        })),
    [],
  );

  // plot geometry
  const W = 640;
  const H = 380;
  const M = { l: 56, r: 24, t: 20, b: 44 };
  const xMax = 50;
  const xSplit = 27; // mean-frontier boundary between compact and fragmented fills
  const yMax = 480;
  const px = (x: number) => M.l + (x / xMax) * (W - M.l - M.r);
  const py = (y: number) => H - M.b - (y / yMax) * (H - M.t - M.b);
  const rOf = (spread: number) => 5 + Math.min(14, (spread / 350) * 14);

  // least-squares line through the points, for the trend.
  const { m, b } = useMemo(() => {
    const n = pts.length;
    const sx = pts.reduce((s, p) => s + p.x, 0);
    const sy = pts.reduce((s, p) => s + p.y, 0);
    const sxx = pts.reduce((s, p) => s + p.x * p.x, 0);
    const sxy = pts.reduce((s, p) => s + p.x * p.y, 0);
    const mm = (n * sxy - sx * sy) / (n * sxx - sx * sx);
    return { m: mm, b: (sy - mm * sx) / n };
  }, [pts]);

  return (
    <figure className="space-y-2">
      <svg
        viewBox={`0 0 ${W} ${H}`}
        className="w-full rounded-lg border bg-card"
        role="img"
        aria-label="Scatter of the mean open frontier against median score for eight fill orders; the two fall on a downward line, showing score collapses as the frontier grows."
      >
        {/* regime bands */}
        <rect x={M.l} y={M.t} width={px(xSplit) - M.l} height={H - M.t - M.b} className="fill-sky-500/5" />
        <rect x={px(xSplit)} y={M.t} width={W - M.r - px(xSplit)} height={H - M.t - M.b} className="fill-amber-500/5" />
        <text x={(M.l + px(xSplit)) / 2} y={M.t + 14} textAnchor="middle" className="fill-muted-foreground text-[10px]">
          {t.lowRegime}
        </text>
        <text x={(px(xSplit) + W - M.r) / 2} y={M.t + 14} textAnchor="middle" className="fill-muted-foreground text-[10px]">
          {t.highRegime}
        </text>

        {/* axes */}
        <line x1={M.l} y1={H - M.b} x2={W - M.r} y2={H - M.b} className="stroke-border" />
        <line x1={M.l} y1={M.t} x2={M.l} y2={H - M.b} className="stroke-border" />
        {[0, 120, 240, 360, 480].map((y) => (
          <g key={y}>
            <line x1={M.l - 3} y1={py(y)} x2={M.l} y2={py(y)} className="stroke-border" />
            <text x={M.l - 6} y={py(y) + 3} textAnchor="end" className="fill-muted-foreground text-[10px]">{y}</text>
          </g>
        ))}
        {[0, 10, 20, 30, 40, 50].map((x) => (
          <g key={x}>
            <line x1={px(x)} y1={H - M.b} x2={px(x)} y2={H - M.b + 3} className="stroke-border" />
            <text x={px(x)} y={H - M.b + 15} textAnchor="middle" className="fill-muted-foreground text-[10px]">{x}</text>
          </g>
        ))}

        {/* trend line */}
        <line
          x1={px(14)} y1={py(m * 14 + b)}
          x2={px(45)} y2={py(m * 45 + b)}
          className="stroke-muted-foreground"
          strokeDasharray="4 4"
          strokeWidth={1.5}
        />
        <text x={px(44)} y={py(m * 44 + b) - 8} textAnchor="end" className="fill-muted-foreground text-[11px] font-semibold">
          {t.r}
        </text>

        {/* points, drawn first so labels sit on top */}
        {pts.map((p) => (
          <circle
            key={`c-${p.path}`}
            cx={px(p.x)} cy={py(p.y)} r={rOf(p.spread)}
            fill={COLOR[p.kind]}
            fillOpacity={hover === null || hover === p.path ? 0.7 : 0.25}
            className="stroke-card"
            strokeWidth={2}
            onMouseEnter={() => setHover(p.path)}
            onMouseLeave={() => setHover(null)}
            style={{ cursor: "pointer" }}
          />
        ))}
        {/* labels with a small anti-collision ladder: within a cluster, stack the
            labels at increasing vertical offsets and draw a thin leader to the dot,
            so tightly-packed points stay readable. */}
        {labelLayout(pts, px, py, rOf).map(({ p, lx, ly, leader }) => (
          <g key={`l-${p.path}`} style={{ pointerEvents: "none" }}>
            {leader && (
              <line x1={px(p.x)} y1={py(p.y)} x2={lx} y2={ly + 3} className="stroke-border" strokeWidth={1} />
            )}
            <text
              x={lx} y={ly}
              textAnchor="middle"
              className={`text-[10px] ${hover === p.path ? "fill-foreground font-semibold" : "fill-muted-foreground"}`}
            >
              {LABEL[p.path]}
            </text>
          </g>
        ))}

        {/* axis labels */}
        <text x={(M.l + W - M.r) / 2} y={H - 6} textAnchor="middle" className="fill-muted-foreground text-[11px]">{t.x}</text>
        <text x={-((M.t + H - M.b) / 2)} y={14} textAnchor="middle" transform="rotate(-90)" className="fill-muted-foreground text-[11px]">{t.y}</text>
      </svg>
      <div className="flex flex-wrap justify-center gap-x-4 gap-y-1.5 text-xs">
        <Swatch color={COLOR.compact} label={t.compact} />
        <Swatch color={COLOR.fragment} label={t.fragment} />
        <Swatch color={COLOR.seeking} label={t.seeking} />
      </div>
      <figcaption className="mx-auto max-w-2xl text-center text-xs text-muted-foreground">{t.caption}</figcaption>
    </figure>
  );
}
