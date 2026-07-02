import { useEffect, useMemo, useState } from "react";
import { useT } from "@/i18n";
import { useIsClient, cn } from "@/lib/utils";
import { useRunWhileVisible } from "@/lib/useRunWhileVisible";

// A runtime-distribution simulator for restart policies:
//
//  1. draw many DFS "runs" from a deterministic, seeded heavy-tailed mixture
//     (a lognormal body of lucky openings + an exponential-in-log — i.e.
//     Pareto-like — tail of trapped runs, calibrated so the spread echoes the
//     2007 Txibilis measurement: median in the millions of nodes, worst runs
//     in the tens of billions);
//  2. fill a log-x histogram run by run, so the reader watches the median
//     settle early while the mean lurches right every time a tail run lands;
//  3. a restart-cutoff slider: runs beyond the cutoff are abandoned at cost
//     `c` and retried with fresh randomness. Expected/median/P99 total work
//     to success is recomputed live (mean analytically from the empirical
//     distribution, quantiles by a seeded Monte-Carlo over restart campaigns).
//
// Everything is deterministic at a given (revealed count, cutoff): no RNG at
// render time. The reveal animation is gated on useRunWhileVisible.

const N_RUNS = 400;
const LOG_MIN = 5; // 10^5 nodes
const LOG_MAX = 11.5; // ~3e11 nodes
const BINS = 44;
const CAMPAIGNS = 3000;
const RUNS_PER_TICK = 4;

/** Deterministic PRNG (mulberry32). */
function mulberry32(seed: number): () => number {
  let a = seed | 0;
  return () => {
    a = (a + 0x6d2b79f5) | 0;
    let t = Math.imul(a ^ (a >>> 15), 1 | a);
    t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
}

function clamp(x: number, lo: number, hi: number): number {
  return Math.max(lo, Math.min(hi, x));
}

/** Node counts (log10) for all runs — seeded, computed once. */
function drawRuns(): number[] {
  const rnd = mulberry32(0xe2707);
  const runs: number[] = [];
  for (let i = 0; i < N_RUNS; i++) {
    let lg: number;
    if (rnd() < 0.72) {
      // Body: a lucky opening — lognormal around ~2M nodes. The left side is
      // steeper (0.28 vs 0.45): real solvers have a floor of unavoidable work,
      // so ultra-fast runs thin out quickly. This keeps the fastest of the 400
      // runs above the slider minimum, making the "cutoff below every run →
      // the policy never succeeds" regime reachable.
      const u = Math.max(rnd(), 1e-12);
      const v = rnd();
      const g = Math.sqrt(-2 * Math.log(u)) * Math.cos(2 * Math.PI * v);
      lg = 6.3 + (g < 0 ? 0.28 : 0.45) * g;
    } else {
      // Tail: a trapped run — exponential in log10(T), i.e. Pareto in T.
      lg = 6.6 + 1.15 * -Math.log(Math.max(rnd(), 1e-12));
    }
    runs.push(clamp(lg, LOG_MIN + 0.05, LOG_MAX - 0.05));
  }
  return runs;
}

function quantile(sortedAsc: number[], q: number): number {
  if (sortedAsc.length === 0) return NaN;
  const idx = Math.min(sortedAsc.length - 1, Math.ceil(q * sortedAsc.length) - 1);
  return sortedAsc[Math.max(0, idx)] ?? NaN;
}

interface Stats {
  mean: number;
  median: number;
  p99: number;
}

function rawStats(nodes: number[]): Stats {
  const sorted = [...nodes].sort((a, b) => a - b);
  const mean = nodes.reduce((s, v) => s + v, 0) / nodes.length;
  return { mean, median: quantile(sorted, 0.5), p99: quantile(sorted, 0.99) };
}

/** Total-work-to-success stats under restart-at-cutoff, from the empirical runs. */
function restartStats(nodes: number[], cutoff: number): Stats & { p: number } {
  const succ = nodes.filter((v) => v <= cutoff);
  const p = succ.length / nodes.length;
  if (p === 0) return { mean: Infinity, median: Infinity, p99: Infinity, p };
  // Mean analytically: geometric number of failed attempts, each costing `cutoff`.
  const meanSucc = succ.reduce((s, v) => s + v, 0) / succ.length;
  const mean = ((1 - p) / p) * cutoff + meanSucc;
  // Quantiles by seeded Monte-Carlo over restart campaigns.
  const rnd = mulberry32(0xcafe17);
  const totals: number[] = [];
  for (let i = 0; i < CAMPAIGNS; i++) {
    let total = 0;
    for (let a = 0; a < 100000; a++) {
      const t = nodes[Math.floor(rnd() * nodes.length)] ?? Infinity;
      if (t <= cutoff) {
        total += t;
        break;
      }
      total += cutoff;
    }
    totals.push(total);
  }
  totals.sort((a, b) => a - b);
  return { mean, median: quantile(totals, 0.5), p99: quantile(totals, 0.99), p };
}

const SUP: Record<string, string> = {
  "0": "⁰", "1": "¹", "2": "²", "3": "³", "4": "⁴",
  "5": "⁵", "6": "⁶", "7": "⁷", "8": "⁸", "9": "⁹",
};
function pow10Label(exp: number): string {
  return "10" + String(exp).split("").map((c) => SUP[c] ?? c).join("");
}

function fmtNodes(n: number, suffixes: [string, string, string, string]): string {
  if (!Number.isFinite(n)) return "∞";
  if (n < 1e3) return String(Math.round(n));
  const tiers: [number, string][] = [
    [1e12, suffixes[3]],
    [1e9, suffixes[2]],
    [1e6, suffixes[1]],
    [1e3, suffixes[0]],
  ];
  for (const [div, suf] of tiers) {
    if (n >= div) {
      const v = n / div;
      return `${v >= 100 ? v.toFixed(0) : v >= 10 ? v.toFixed(1) : v.toFixed(2)}${suf}`;
    }
  }
  return String(Math.round(n));
}

const T = {
  en: {
    title: "The tail, and the knife",
    intro:
      "400 runs of the same solver on the same puzzle class, drawn from a seeded heavy-tailed mixture (deterministic — replay gives the same story). The x axis is total nodes, logarithmic. Sky runs finish under the restart cutoff; amber runs would blow past it and are cut off at the dashed line, then retried with fresh randomness. Watch the median settle within seconds while the mean keeps lurching right — every landing tail run drags it.",
    cutoff: "restart cutoff",
    nodes: "nodes",
    runs: "runs",
    drawn: "drawn",
    replay: "replay",
    colOne: "one long run",
    colRestart: "restart at cutoff",
    median: "median",
    mean: "mean",
    p99: "P99",
    metric: "work to first solution",
    successShare: (pct: string, attempts: string) =>
      `${pct} of runs finish under the cutoff — about ${attempts} attempt(s) expected per success.`,
    noSuccess: "No run finishes under this cutoff: the policy never succeeds — expected time is infinite.",
    speedup: (x: string) => `Expected speedup from restarting: ×${x}.`,
    slowdown: (x: string) => `This cutoff is too greedy: restarting is ×${x} slower than just running on.`,
    meanMarker: "mean",
    medianMarker: "median",
    loading: "Loading…",
    suffixes: ["k", "M", "B", "T"] as [string, string, string, string],
  },
  fr: {
    title: "La queue, et le couteau",
    intro:
      "400 exécutions du même solveur sur la même classe de puzzle, tirées d'un mélange à queue lourde avec graine fixe (déterministe — rejouer raconte la même histoire). L'axe des x compte les nœuds, en échelle log. Les exécutions bleues finissent sous le seuil de redémarrage ; les ambre le dépasseraient : on les coupe à la ligne pointillée et on retente avec un aléa frais. Regardez la médiane se stabiliser en quelques secondes pendant que la moyenne continue de bondir vers la droite — chaque exécution de la queue la traîne.",
    cutoff: "seuil de redémarrage",
    nodes: "nœuds",
    runs: "exécutions",
    drawn: "tirées",
    replay: "rejouer",
    colOne: "une seule longue exécution",
    colRestart: "redémarrage au seuil",
    median: "médiane",
    mean: "moyenne",
    p99: "P99",
    metric: "travail jusqu'à la première solution",
    successShare: (pct: string, attempts: string) =>
      `${pct} des exécutions finissent sous le seuil — environ ${attempts} tentative(s) par succès en espérance.`,
    noSuccess: "Aucune exécution ne finit sous ce seuil : la politique ne réussit jamais — temps espéré infini.",
    speedup: (x: string) => `Accélération espérée grâce aux redémarrages : ×${x}.`,
    slowdown: (x: string) => `Seuil trop gourmand : redémarrer est ×${x} plus lent que laisser courir.`,
    meanMarker: "moyenne",
    medianMarker: "médiane",
    loading: "Chargement…",
    suffixes: ["k", "M", "Md", "Bn"] as [string, string, string, string],
  },
};

const W = 560;
const HIST_H = 150;
const AXIS_H = 30;
const PAD_X = 10;

export function RestartTailLab() {
  const t = useT(T);
  const isClient = useIsClient();
  const [revealed, setRevealed] = useState(0);
  const [cutLog, setCutLog] = useState(7); // 10^7 = 10M nodes, Txibilis's own cutoff
  const { ref: rootRef, visible } = useRunWhileVisible();

  const allRuns = useMemo(() => drawRuns(), []);
  const shown = useMemo(() => allRuns.slice(0, revealed), [allRuns, revealed]);
  const shownNodes = useMemo(() => shown.map((lg) => 10 ** lg), [shown]);
  const cutoff = 10 ** cutLog;

  useEffect(() => {
    if (!visible || revealed >= N_RUNS) return;
    const id = setInterval(
      () => setRevealed((r) => Math.min(N_RUNS, r + RUNS_PER_TICK)),
      90,
    );
    return () => clearInterval(id);
  }, [visible, revealed]);

  const raw = useMemo(() => (shownNodes.length ? rawStats(shownNodes) : null), [shownNodes]);
  const restarted = useMemo(
    () => (shownNodes.length ? restartStats(shownNodes, cutoff) : null),
    [shownNodes, cutoff],
  );

  const bins = useMemo(() => {
    const counts = new Array<number>(BINS).fill(0);
    for (const lg of shown) {
      const b = clamp(Math.floor(((lg - LOG_MIN) / (LOG_MAX - LOG_MIN)) * BINS), 0, BINS - 1);
      counts[b] = (counts[b] ?? 0) + 1;
    }
    return counts;
  }, [shown]);
  const maxBin = Math.max(1, ...bins);

  if (!isClient) {
    return (
      <div className="flex h-72 items-center justify-center rounded-lg border text-sm text-muted-foreground">
        {t.loading}
      </div>
    );
  }

  const xOfLog = (lg: number) => PAD_X + ((lg - LOG_MIN) / (LOG_MAX - LOG_MIN)) * (W - 2 * PAD_X);
  const binW = (W - 2 * PAD_X) / BINS;
  const cutX = xOfLog(cutLog);
  const speedup = raw && restarted && Number.isFinite(restarted.mean) ? raw.mean / restarted.mean : null;

  const cells: { label: string; one: number; restart: number }[] =
    raw && restarted
      ? [
          { label: t.median, one: raw.median, restart: restarted.median },
          { label: t.mean, one: raw.mean, restart: restarted.mean },
          { label: t.p99, one: raw.p99, restart: restarted.p99 },
        ]
      : [];

  return (
    <div ref={rootRef} className="space-y-4 rounded-lg border bg-card p-4">
      <div className="space-y-1">
        <div className="flex items-baseline justify-between gap-2">
          <h3 className="text-sm font-semibold">{t.title}</h3>
          <span className="text-[10px] tabular-nums text-muted-foreground">
            {revealed}/{N_RUNS} {t.runs} {t.drawn}
          </span>
        </div>
        <p className="text-xs leading-relaxed text-muted-foreground">{t.intro}</p>
      </div>

      <svg viewBox={`0 0 ${W} ${HIST_H + AXIS_H}`} className="w-full rounded-md border bg-muted/20">
        {/* bars */}
        {bins.map((c, i) => {
          if (c === 0) return null;
          const lgMid = LOG_MIN + ((i + 0.5) / BINS) * (LOG_MAX - LOG_MIN);
          const h = (c / maxBin) * (HIST_H - 14);
          return (
            <rect
              key={i}
              x={PAD_X + i * binW + 0.5}
              y={HIST_H - h}
              width={binW - 1}
              height={h}
              className={lgMid <= cutLog ? "fill-sky-500" : "fill-amber-500"}
              opacity={0.85}
            />
          );
        })}
        {/* axis ticks at integer exponents */}
        {[5, 6, 7, 8, 9, 10, 11].map((e) => (
          <g key={e}>
            <line x1={xOfLog(e)} y1={HIST_H} x2={xOfLog(e)} y2={HIST_H + 4} className="stroke-muted-foreground" strokeWidth={1} />
            <text x={xOfLog(e)} y={HIST_H + 15} textAnchor="middle" className="fill-muted-foreground" fontSize={9}>
              {pow10Label(e)}
            </text>
          </g>
        ))}
        <text x={W - PAD_X} y={HIST_H + 27} textAnchor="end" className="fill-muted-foreground" fontSize={9}>
          {t.nodes}
        </text>
        {/* median / mean markers */}
        {raw && (
          <g>
            <path
              d={`M ${xOfLog(Math.log10(raw.median))} ${HIST_H - 1} l -4.5 8 l 9 0 z`}
              className="fill-emerald-500"
            />
            <text x={xOfLog(Math.log10(raw.median))} y={HIST_H + 27} textAnchor="middle" className="fill-emerald-600 dark:fill-emerald-400" fontSize={9}>
              {t.medianMarker}
            </text>
            <path
              d={`M ${xOfLog(Math.log10(raw.mean))} ${HIST_H - 1} l -4.5 8 l 9 0 z`}
              className="fill-rose-500"
            />
            <text x={xOfLog(Math.log10(raw.mean))} y={HIST_H + 27} textAnchor="middle" className="fill-rose-500" fontSize={9}>
              {t.meanMarker}
            </text>
          </g>
        )}
        {/* cutoff line */}
        <line x1={cutX} y1={4} x2={cutX} y2={HIST_H} className="stroke-rose-500" strokeWidth={1.5} strokeDasharray="4 3" />
        <text x={cutX} y={12} textAnchor={cutX > W - 90 ? "end" : "start"} dx={cutX > W - 90 ? -4 : 4} className="fill-rose-500" fontSize={9}>
          {t.cutoff}
        </text>
      </svg>

      <div className="flex flex-wrap items-center gap-3">
        <label className="flex min-w-64 flex-1 items-center gap-3 text-sm">
          <span className="whitespace-nowrap text-muted-foreground">{t.cutoff}</span>
          <input
            type="range"
            min={5.3}
            max={11.4}
            step={0.05}
            value={cutLog}
            onChange={(e) => setCutLog(Number(e.target.value))}
            className="w-full max-w-72"
          />
          <span className="w-14 text-right font-semibold tabular-nums">
            {fmtNodes(cutoff, t.suffixes)}
          </span>
        </label>
        <button
          type="button"
          onClick={() => setRevealed(0)}
          className="rounded-md border px-2.5 py-1 text-xs text-muted-foreground hover:bg-muted"
        >
          {t.replay}
        </button>
      </div>

      <div className="overflow-x-auto">
        <table className="w-full text-center text-sm">
          <thead>
            <tr className="text-[10px] uppercase tracking-wide text-muted-foreground">
              <th className="py-1 text-left font-normal">{t.metric}</th>
              <th className="py-1 font-normal">{t.median}</th>
              <th className="py-1 font-normal">{t.mean}</th>
              <th className="py-1 font-normal">{t.p99}</th>
            </tr>
          </thead>
          <tbody>
            <tr className="border-t">
              <td className="py-1.5 text-left text-muted-foreground">{t.colOne}</td>
              {cells.map((c) => (
                <td key={c.label} className="py-1.5 font-semibold tabular-nums">
                  {raw ? fmtNodes(c.one, t.suffixes) : "—"}
                </td>
              ))}
              {cells.length === 0 && <td className="py-1.5" colSpan={3}>—</td>}
            </tr>
            <tr className="border-t">
              <td className="py-1.5 text-left text-muted-foreground">{t.colRestart}</td>
              {cells.map((c) => (
                <td
                  key={c.label}
                  className={cn(
                    "py-1.5 font-semibold tabular-nums",
                    Number.isFinite(c.restart) && c.restart < c.one
                      ? "text-emerald-600 dark:text-emerald-400"
                      : "text-rose-500",
                  )}
                >
                  {restarted ? fmtNodes(c.restart, t.suffixes) : "—"}
                </td>
              ))}
              {cells.length === 0 && <td className="py-1.5" colSpan={3}>—</td>}
            </tr>
          </tbody>
        </table>
      </div>

      {restarted && (
        <p className="text-xs text-muted-foreground">
          {restarted.p === 0
            ? t.noSuccess
            : t.successShare(
                `${(restarted.p * 100).toFixed(0)}%`,
                (1 / restarted.p).toFixed(1),
              )}{" "}
          {speedup !== null &&
            (speedup >= 1 ? t.speedup(speedup.toFixed(1)) : t.slowdown((1 / speedup).toFixed(1)))}
        </p>
      )}
    </div>
  );
}
