import { useMemo } from "react";

import { useT } from "@/i18n";
import { Swatch } from "@/components/research/FamilyLegend";
import data from "@/data/hint-study-solve.json";

// The small-board solve-speed view. On a fully-solvable 8×8 board (with the
// E2-faithful colour ratio), we can measure the metric the 16×16-at-a-budget study
// cannot: NODES-TO-SOLUTION. This chart plots it, on a log scale, for a spread
// lattice vs a matched contiguous block across increasing hint counts. The story:
// below a threshold scattered hints barely solve, but once dense enough they beat
// the contiguous block by orders of magnitude — confirming the community
// hint-geometry claim in its proper regime.

type Stat = { of: number; solved: number; median_nodes: number | null };
type Rung = { count: number; spread: Stat | null; contiguous: Stat | null };

const T = {
  en: {
    title: "Nodes to fully solve an 8×8, scattered vs contiguous",
    yAxis: "median nodes to solution (log scale)",
    spread: "scattered lattice",
    contiguous: "contiguous block",
    baseline: "no hints",
    hints: "hints",
    unsolved: "rarely solved",
    caption:
      "Each pair of dots is one matched hint count: a spread lattice and a contiguous block of the same size, on the same boards, over the seeds that solved. At four hints the scattered lattice barely solves. Around sixteen the picture inverts: the scattered lattice solves in a few thousand nodes while the matched contiguous block still needs nearly two million, a five-hundred-fold gap. Past that both become easy as the board fills up. This is the community hint-geometry result reproduced on our own boards, and it is the reason the 16×16 score comparison looked opposite: at 16×16 in eight seconds nobody reaches the endgame, so scattered hints never get to do the pruning that wins here.",
  },
};

export function SolveSpeedChart() {
  const t = useT(T);

  const { rungs, baseline } = useMemo(() => {
    const d = data as { ladder?: Rung[]; baseline?: Stat };
    return { rungs: d.ladder ?? [], baseline: d.baseline ?? null };
  }, []);

  if (rungs.length === 0) return null;

  // log-scale plot
  const W = 640;
  const H = 360;
  const M = { l: 58, r: 20, t: 16, b: 40 };
  const counts = rungs.map((r) => r.count);
  const minC = Math.min(...counts);
  const maxC = Math.max(...counts);
  const allNodes = rungs
    .flatMap((r) => [r.spread?.median_nodes, r.contiguous?.median_nodes])
    .filter((n): n is number => n != null && n > 0);
  const loLog = Math.floor(Math.log10(Math.min(...allNodes)));
  const hiLog = Math.ceil(Math.log10(Math.max(...allNodes)));
  const px = (c: number) => M.l + ((c - minC) / (maxC - minC || 1)) * (W - M.l - M.r);
  const py = (n: number) => {
    const lg = Math.log10(Math.max(1, n));
    return H - M.b - ((lg - loLog) / (hiLog - loLog || 1)) * (H - M.t - M.b);
  };

  const line = (get: (r: Rung) => Stat | null, color: string) => {
    const pts = rungs
      .map((r) => ({ c: r.count, nodes: get(r)?.median_nodes ?? null, solved: get(r)?.solved ?? 0 }))
      .filter((p): p is { c: number; nodes: number; solved: number } => p.nodes != null && p.solved > 0);
    const path = pts.map((p, i) => `${i === 0 ? "M" : "L"}${px(p.c)},${py(p.nodes)}`).join(" ");
    return { pts, path, color };
  };

  const spread = line((r) => r.spread, "#0ea5e9");
  const contiguous = line((r) => r.contiguous, "#f59e0b");

  const yTicks: number[] = [];
  for (let e = loLog; e <= hiLog; e++) yTicks.push(10 ** e);

  return (
    <figure className="space-y-2">
      <figcaption className="text-sm font-semibold">{t.title}</figcaption>
      <svg viewBox={`0 0 ${W} ${H}`} className="w-full rounded-lg border bg-card" role="img"
        aria-label="Median nodes to solve an 8x8 board on a log scale, scattered lattice versus contiguous block, across hint counts. Scattered starts higher but drops far below contiguous once there are enough hints.">
        {/* y grid + ticks */}
        {yTicks.map((n) => (
          <g key={n}>
            <line x1={M.l} y1={py(n)} x2={W - M.r} y2={py(n)} className="stroke-border" strokeDasharray="2 3" />
            <text x={M.l - 6} y={py(n) + 3} textAnchor="end" className="fill-muted-foreground text-[9px]">
              {n >= 1e6 ? `${n / 1e6}M` : n >= 1e3 ? `${n / 1e3}k` : n}
            </text>
          </g>
        ))}
        {/* x ticks */}
        {rungs.map((r) => (
          <text key={r.count} x={px(r.count)} y={H - M.b + 15} textAnchor="middle" className="fill-muted-foreground text-[10px]">
            {r.count}
          </text>
        ))}
        <text x={(M.l + W - M.r) / 2} y={H - 4} textAnchor="middle" className="fill-muted-foreground text-[10px]">{t.hints}</text>

        {/* baseline reference */}
        {baseline?.median_nodes != null && (
          <>
            <line x1={M.l} y1={py(baseline.median_nodes)} x2={W - M.r} y2={py(baseline.median_nodes)} className="stroke-muted-foreground" strokeDasharray="5 4" strokeWidth={1} />
            <text x={W - M.r} y={py(baseline.median_nodes) - 4} textAnchor="end" className="fill-muted-foreground text-[9px]">{t.baseline}</text>
          </>
        )}

        {/* lines */}
        {[spread, contiguous].map((ln, i) => (
          <g key={i}>
            <path d={ln.path} fill="none" stroke={ln.color} strokeWidth={2} />
            {ln.pts.map((p) => (
              <circle key={p.c} cx={px(p.c)} cy={py(p.nodes)} r={4} fill={ln.color} className="stroke-card" strokeWidth={1.5} />
            ))}
          </g>
        ))}
      </svg>
      <div className="flex flex-wrap gap-x-4 gap-y-1.5 text-xs">
        <Swatch color="#0ea5e9" label={t.spread} />
        <Swatch color="#f59e0b" label={t.contiguous} />
      </div>
      <p className="text-xs text-muted-foreground">{t.caption}</p>
    </figure>
  );
}
