import { useEffect, useMemo, useState } from "react";
import { useT } from "@/i18n";
import { useIsClient, cn } from "@/lib/utils";
import { useRunWhileVisible } from "@/lib/useRunWhileVisible";
import { Button } from "@/components/ui/button";

// Fill orders, made watchable. Two linked views driven by one choice of
// preset order (row scan, spiral-in, diagonal, comb with adjustable teeth):
//
//  1. a 16×16 grid where the visit sequence paints itself as an animated
//     gradient (blue = visited first, red = visited last), swept by the
//     search frontier one cell per tick;
//
//  2. a live constraint-count curve: for every placement k, how many of the
//     new cell's four orthogonal neighbours are already on the board — the
//     first-order quantity that makes orders good or bad. A second line
//     accumulates the joins collected so far (every complete order collects
//     exactly the same 480 joins; only the *schedule* differs), with row
//     scan as a dashed reference.
//
// The highlighted band, placements 121–185, is the stretch Brendan Owen
// identified as the only one that really matters for node counts (groups.io
// msg 2714). Everything is exact and deterministic — no RNG, no solver; the
// only animation is the frontier sweep, gated on useRunWhileVisible with
// O(1) work per tick.

const N = 16;
const CELLS = N * N;
const PEAK_LO = 121; // Owen's "sizes between 121-185" band, 1-based (msg 2714)
const PEAK_HI = 185;
const TOTAL_JOINS = 2 * N * (N - 1); // 480 — invariant across all orders

const PAUSE_TICKS = 40;
const TICK_MS = 30;

const idx = (x: number, y: number) => y * N + x;

/** Bottom-up, left-to-right row scan — Verhaard's orientation (msg 6015). */
function rowScanOrder(): number[] {
  const o: number[] = [];
  for (let y = N - 1; y >= 0; y--) for (let x = 0; x < N; x++) o.push(idx(x, y));
  return o;
}

/** Border ring first, then inward ring by ring (clockwise from bottom-left). */
function spiralOrder(): number[] {
  const o: number[] = [];
  for (let l = 0; l < N / 2; l++) {
    const lo = l;
    const hi = N - 1 - l;
    for (let x = lo; x <= hi; x++) o.push(idx(x, hi));
    for (let y = hi - 1; y >= lo; y--) o.push(idx(hi, y));
    for (let x = hi - 1; x >= lo; x--) o.push(idx(x, lo));
    for (let y = lo + 1; y <= hi - 1; y++) o.push(idx(lo, y));
  }
  return o;
}

/** Anti-diagonal sweep from the bottom-left corner. */
function diagonalOrder(): number[] {
  const o: number[] = [];
  for (let d = 0; d <= 2 * (N - 1); d++)
    for (let x = 0; x < N; x++) {
      const y = N - 1 - (d - x);
      if (d - x >= 0 && y >= 0 && y < N) o.push(idx(x, y));
    }
  return o;
}

/**
 * Verhaard's comb (msg 6112): most rows searched horizontally, the remaining
 * `teeth` rows searched vertically, column by column.
 */
function combOrder(teeth: number): number[] {
  const o: number[] = [];
  for (let y = N - 1; y >= teeth; y--) for (let x = 0; x < N; x++) o.push(idx(x, y));
  for (let x = 0; x < N; x++) for (let y = teeth - 1; y >= 0; y--) o.push(idx(x, y));
  return o;
}

interface OrderStats {
  counts: number[]; // already-placed orthogonal neighbours of the k-th cell
  cum: number[]; // joins collected after k+1 placements (ends at 480)
  weak: number; // placements (after the first) with ≤ 1 placed neighbour
  avgBand: number; // mean neighbour count over placements 121–185
}

function analyze(order: number[]): OrderStats {
  const placed = new Array<boolean>(CELLS).fill(false);
  const counts: number[] = [];
  const cum: number[] = [];
  let total = 0;
  let weak = 0;
  let bandSum = 0;
  order.forEach((cell, k) => {
    const x = cell % N;
    const y = Math.floor(cell / N);
    let c = 0;
    if (x > 0 && placed[cell - 1]) c++;
    if (x < N - 1 && placed[cell + 1]) c++;
    if (y > 0 && placed[cell - N]) c++;
    if (y < N - 1 && placed[cell + N]) c++;
    placed[cell] = true;
    counts.push(c);
    total += c;
    cum.push(total);
    if (k > 0 && c <= 1) weak++;
    if (k + 1 >= PEAK_LO && k + 1 <= PEAK_HI) bandSum += c;
  });
  return { counts, cum, weak, avgBand: bandSum / (PEAK_HI - PEAK_LO + 1) };
}

const ROW_REF = analyze(rowScanOrder());

type Preset = "rows" | "spiral" | "diagonal" | "comb";

/** Visit-sequence gradient: blue (first) → red (last). */
function seqColor(t: number): string {
  return `hsl(${Math.round(205 + 150 * t)}, 68%, 52%)`;
}

const T = {
  en: {
    title: "The fill-order lab — one path, one tree",
    intro:
      "Pick an order and watch two things at once: the visit sequence sweeping the board (blue first, red last), and how many already-placed neighbours each new cell meets. Every complete order collects exactly the same 480 joins — the only choice is when each join is paid, and early is what prunes.",
    presetRows: "Row scan",
    presetSpiral: "Spiral in",
    presetDiagonal: "Diagonal",
    presetComb: "Comb",
    teeth: "teeth length",
    gridCaption: "visit sequence — colour is position in the order (blue → red)",
    chartTitle: "Already-placed neighbours of each new cell",
    axisJoins: "joins collected",
    legendCount: "neighbours at placement k (left axis, 0–4)",
    legendCum: "joins collected so far (right axis, of 480)",
    legendRef: "row scan reference (joins collected)",
    legendBand: "placements 121–185 — the band that decides the node count (msg 2714)",
    weak: (n: number) => `weakly constrained placements (≤ 1 neighbour): ${n}`,
    avgBand: (v: string) => `mean neighbours across the 121–185 band: ${v}`,
    cumAt: (j: number, ref: number) =>
      `joins matched by placement 185: ${j} of 480 (row scan: ${ref})`,
    footnote:
      "Exact computation, no randomness: the curve is a property of the order alone. It counts joins, not shapes — two orders with identical curves can still differ through the shape of their frontier, which is what the magic 10×16 square and complex theory score.",
    fmt: (x: number) => x.toFixed(2),
  },
  fr: {
    title: "Le labo des ordres de remplissage — un chemin, un arbre",
    intro:
      "Choisissez un ordre et regardez deux choses à la fois : la séquence de visite qui balaie le plateau (bleu d'abord, rouge à la fin), et le nombre de voisins déjà posés que rencontre chaque nouvelle case. Tout ordre complet collecte exactement les mêmes 480 jointures — le seul choix est le moment où chacune est payée, et c'est tôt qu'elle élague.",
    presetRows: "Balayage par lignes",
    presetSpiral: "Spirale",
    presetDiagonal: "Diagonale",
    presetComb: "Peigne",
    teeth: "longueur des dents",
    gridCaption: "séquence de visite — la couleur est la position dans l'ordre (bleu → rouge)",
    chartTitle: "Voisins déjà posés de chaque nouvelle case",
    axisJoins: "jointures collectées",
    legendCount: "voisins au placement k (axe gauche, 0–4)",
    legendCum: "jointures collectées (axe droit, sur 480)",
    legendRef: "référence balayage par lignes (jointures collectées)",
    legendBand: "placements 121–185 — la bande qui décide du nombre de nœuds (msg 2714)",
    weak: (n: number) => `placements faiblement contraints (≤ 1 voisin) : ${n}`,
    avgBand: (v: string) => `voisins moyens sur la bande 121–185 : ${v}`,
    cumAt: (j: number, ref: number) =>
      `jointures appariées au placement 185 : ${j} sur 480 (balayage par lignes : ${ref})`,
    footnote:
      "Calcul exact, aucun aléa : la courbe est une propriété de l'ordre seul. Elle compte des jointures, pas des formes — deux ordres à courbes identiques peuvent encore différer par la forme de leur front, ce que notent le carré magique 10×16 et la théorie complexe.",
    fmt: (x: number) => x.toFixed(2).replace(".", ","),
  },
};

export function FillOrderLab() {
  const t = useT(T);
  const isClient = useIsClient();
  const { ref: rootRef, visible } = useRunWhileVisible();
  const [preset, setPreset] = useState<Preset>("rows");
  const [teeth, setTeeth] = useState(4);
  const [step, setStep] = useState(0);

  useEffect(() => {
    if (!visible) return;
    const id = setInterval(() => setStep((s) => (s + 1) % (CELLS + PAUSE_TICKS)), TICK_MS);
    return () => clearInterval(id);
  }, [visible]);

  const order = useMemo(() => {
    switch (preset) {
      case "rows":
        return rowScanOrder();
      case "spiral":
        return spiralOrder();
      case "diagonal":
        return diagonalOrder();
      case "comb":
        return combOrder(teeth);
    }
  }, [preset, teeth]);

  const stats = useMemo(() => analyze(order), [order]);

  const posOf = useMemo(() => {
    const p = new Array<number>(CELLS).fill(0);
    order.forEach((cell, k) => {
      p[cell] = k;
    });
    return p;
  }, [order]);

  if (!isClient) {
    return (
      <div className="flex h-72 items-center justify-center rounded-lg border text-sm text-muted-foreground">
        …
      </div>
    );
  }

  const frontier = Math.min(step, CELLS - 1);
  const frontierCell = order[frontier] ?? 0;

  // --- grid geometry ---
  const CELL = 17;
  const SW = N * CELL;

  // --- chart geometry ---
  const CW = 470;
  const CH = 220;
  const PAD_L = 30;
  const PAD_R = 40;
  const PAD_T = 16;
  const PAD_B = 24;
  const plotH = CH - PAD_T - PAD_B;
  const xOf = (k: number) => PAD_L + (k / (CELLS - 1)) * (CW - PAD_L - PAD_R);
  const yCount = (c: number) => PAD_T + plotH * (1 - c / 4);
  const yCum = (j: number) => PAD_T + plotH * (1 - j / TOTAL_JOINS);

  const countsPts = stats.counts.map((c, k) => `${xOf(k).toFixed(1)},${yCount(c).toFixed(1)}`).join(" ");
  const cumPts = stats.cum.map((j, k) => `${xOf(k).toFixed(1)},${yCum(j).toFixed(1)}`).join(" ");
  const refPts = ROW_REF.cum.map((j, k) => `${xOf(k).toFixed(1)},${yCum(j).toFixed(1)}`).join(" ");

  const presets: { key: Preset; label: string }[] = [
    { key: "rows", label: t.presetRows },
    { key: "spiral", label: t.presetSpiral },
    { key: "diagonal", label: t.presetDiagonal },
    { key: "comb", label: t.presetComb },
  ];

  return (
    <div ref={rootRef} className="space-y-4 rounded-lg border bg-card p-4">
      <div className="space-y-1">
        <h3 className="text-sm font-semibold">{t.title}</h3>
        <p className="text-xs leading-relaxed text-muted-foreground">{t.intro}</p>
      </div>

      <div className="flex flex-wrap items-center gap-x-4 gap-y-2">
        <div className="flex flex-wrap gap-2">
          {presets.map((p) => (
            <Button
              key={p.key}
              size="sm"
              variant={preset === p.key ? "default" : "outline"}
              onClick={() => {
                setPreset(p.key);
                setStep(0); // restart the sweep on a new order
              }}
            >
              {p.label}
            </Button>
          ))}
        </div>
        {preset === "comb" && (
          <label className="flex min-w-48 items-center gap-3 text-sm">
            <span className="whitespace-nowrap text-muted-foreground">{t.teeth}</span>
            <input
              type="range"
              min={1}
              max={12}
              value={teeth}
              onChange={(e) => {
                setTeeth(Number(e.target.value));
                setStep(0); // restart the sweep on a new order
              }}
              className="w-full max-w-40"
            />
            <span className="w-6 text-right font-semibold tabular-nums">{teeth}</span>
          </label>
        )}
      </div>

      <div className="flex flex-wrap items-start justify-center gap-6">
        {/* visit-sequence grid */}
        <div className="w-full max-w-xs space-y-2">
          <svg viewBox={`-1 -1 ${SW + 2} ${SW + 2}`} className="w-full rounded-md border bg-muted/20" role="img" aria-label="A board grid coloured by each cell's position in the fill order, from blue early to red late, with the current frontier cell outlined">
            {Array.from({ length: CELLS }, (_, cell) => {
              const x = (cell % N) * CELL;
              const y = Math.floor(cell / N) * CELL;
              const k = posOf[cell] ?? 0;
              const placed = k <= frontier;
              return (
                <rect
                  key={cell}
                  x={x + 0.5}
                  y={y + 0.5}
                  width={CELL - 1}
                  height={CELL - 1}
                  rx={2}
                  fill={seqColor(k / (CELLS - 1))}
                  opacity={placed ? 0.9 : 0.13}
                />
              );
            })}
            <rect
              x={(frontierCell % N) * CELL + 0.5}
              y={Math.floor(frontierCell / N) * CELL + 0.5}
              width={CELL - 1}
              height={CELL - 1}
              rx={2}
              fill="none"
              strokeWidth={2}
              className="stroke-foreground"
            />
          </svg>
          <p className="text-center text-[10px] text-muted-foreground">{t.gridCaption}</p>
        </div>

        {/* constraint-count curve */}
        <div className="w-full min-w-64 max-w-lg flex-1">
          <svg viewBox={`0 0 ${CW} ${CH}`} className="w-full rounded-md border bg-muted/20" role="img" aria-label="A curve of how many already-placed neighbours each new cell has along the fill order, with Owen's 121 to 185 band shaded">
            {/* Owen's 121–185 band */}
            <rect
              x={xOf(PEAK_LO - 1)}
              y={PAD_T}
              width={xOf(PEAK_HI - 1) - xOf(PEAK_LO - 1)}
              height={plotH}
              className="fill-amber-500/15"
            />
            {/* y grid: neighbour counts 0..4 */}
            {[0, 1, 2, 3, 4].map((c) => (
              <g key={c}>
                <line
                  x1={PAD_L}
                  x2={CW - PAD_R}
                  y1={yCount(c)}
                  y2={yCount(c)}
                  className="stroke-border"
                  strokeWidth={1}
                  strokeDasharray="3 4"
                />
                <text
                  x={PAD_L - 5}
                  y={yCount(c) + 3}
                  textAnchor="end"
                  className="fill-muted-foreground"
                  fontSize={9}
                >
                  {c}
                </text>
              </g>
            ))}
            {/* right axis: cumulative joins */}
            {[0, 240, 480].map((j) => (
              <text
                key={j}
                x={CW - PAD_R + 5}
                y={yCum(j) + 3}
                className="fill-muted-foreground"
                fontSize={9}
              >
                {j}
              </text>
            ))}
            {/* row-scan cumulative reference */}
            <polyline
              points={refPts}
              fill="none"
              strokeWidth={1.5}
              strokeDasharray="4 4"
              className="stroke-muted-foreground/60"
            />
            {/* cumulative joins */}
            <polyline points={cumPts} fill="none" strokeWidth={2} className="stroke-amber-500" />
            {/* per-placement neighbour counts */}
            <polyline points={countsPts} fill="none" strokeWidth={1.5} className="stroke-sky-500" />
            {/* frontier cursor */}
            <line
              x1={xOf(frontier)}
              x2={xOf(frontier)}
              y1={PAD_T}
              y2={PAD_T + plotH}
              strokeWidth={1}
              className="stroke-foreground/60"
            />
            <circle
              cx={xOf(frontier)}
              cy={yCount(stats.counts[frontier] ?? 0)}
              r={3.5}
              className="fill-foreground"
            />
            <text x={PAD_L} y={10} fontSize={9} className="fill-muted-foreground">
              {t.chartTitle}
            </text>
            <text
              x={CW - PAD_R + 5}
              y={12}
              fontSize={8}
              className="fill-muted-foreground/80"
            >
              {t.axisJoins}
            </text>
            {/* x labels */}
            {[1, 64, 128, 192, 256].map((k) => (
              <text
                key={k}
                x={xOf(k - 1)}
                y={CH - 8}
                textAnchor="middle"
                fontSize={9}
                className="fill-muted-foreground"
              >
                {k}
              </text>
            ))}
          </svg>
          <ul className="mt-1.5 space-y-0.5 text-[10px] text-muted-foreground">
            <li className="flex items-center gap-2">
              <span className="inline-block h-0.5 w-3.5 shrink-0 rounded bg-sky-500" />
              {t.legendCount}
            </li>
            <li className="flex items-center gap-2">
              <span className="inline-block h-0.5 w-3.5 shrink-0 rounded bg-amber-500" />
              {t.legendCum}
            </li>
            <li className="flex items-center gap-2">
              <span
                className={cn(
                  "inline-block h-0.5 w-3.5 shrink-0 rounded bg-muted-foreground/60",
                )}
              />
              {t.legendRef}
            </li>
            <li className="flex items-center gap-2">
              <span className="inline-block h-2.5 w-2.5 shrink-0 rounded-sm bg-amber-500/25" />
              {t.legendBand}
            </li>
          </ul>
          <p className="mt-1.5 rounded-md border bg-muted/30 px-2.5 py-2 text-[11px] leading-relaxed tabular-nums">
            {t.weak(stats.weak)} · {t.avgBand(t.fmt(stats.avgBand))} ·{" "}
            {t.cumAt(stats.cum[PEAK_HI - 1] ?? 0, ROW_REF.cum[PEAK_HI - 1] ?? 0)}
          </p>
        </div>
      </div>

      <p className="text-xs text-muted-foreground">{t.footnote}</p>
    </div>
  );
}
