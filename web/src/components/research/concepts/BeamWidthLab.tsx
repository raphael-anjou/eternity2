import { useEffect, useMemo, useState } from "react";
import { useT } from "@/i18n";
import { useIsClient, cn } from "@/lib/utils";
import { useRunWhileVisible } from "@/lib/useRunWhileVisible";

// A synthetic search tree descended by a beam, to make three things visible:
//
//  1. the mechanics — every survivor spawns b children, all children are
//     pooled and scored, only the top K live on (nothing is ever revisited);
//  2. diversity collapse — the dots' horizontal position encodes the path
//     prefix, so you can watch the beam huddle into one cluster while the
//     "distinct prefixes" counter falls toward 1;
//  3. greedy vs wide — K = 1 is drawn alongside as the rose trace; amber
//     "trap" nodes pay a big immediate score but poison their subtree (the
//     synthetic stand-in for piece theft), which is exactly what greedy eats.
//
// The scoring model is pure TS, hash-based and fully deterministic (no RNG at
// render); the whole simulation for K = 64 is ~3.5k node evaluations, well
// under the frame budget. The reveal animation is gated on useRunWhileVisible.

const BRANCH = 4;
const DEPTH = 14;
const K_MAX = 64;
const PREFIX_LEN = 3;
const PAUSE_TICKS = 5;

// Deterministic per-node noise (variant tuned so returns are monotone and
// diminishing in K, and collapse is visible at every width).
function h32(x: number): number {
  let v = x | 0;
  v = Math.imul(v ^ (v >>> 16), 0x45d9f3b);
  v = Math.imul(v ^ (v >>> 16), 0x45d9f3b);
  return (v ^ (v >>> 16)) >>> 0;
}
function noise(id: number, salt: number): number {
  return h32(id * 13 + salt) / 4294967296;
}
function clamp(x: number, lo: number, hi: number): number {
  return Math.max(lo, Math.min(hi, x));
}

interface SimNode {
  id: number;
  digits: number[];
  score: number;
  /** latent "inventory health": traps look great now and drain it. */
  q: number;
  trapped: boolean;
}

interface DrawnNode {
  x: number;
  px: number;
  trapped: boolean;
}

interface SimLevel {
  nodes: DrawnNode[];
  best: number;
  distinct: number;
}

interface Sim {
  levels: SimLevel[];
  final: number;
}

/** Horizontal position of a path: base-4 fraction, so shared prefixes cluster. */
function xFrac(digits: number[]): number {
  let x = 0;
  let denom = 1;
  for (const d of digits) {
    denom *= BRANCH;
    x += d / denom;
  }
  return x + 0.5 / denom;
}

function simulate(K: number): Sim {
  let beam: SimNode[] = [{ id: 0, digits: [], score: 0, q: 0.7, trapped: false }];
  const levels: SimLevel[] = [];
  for (let d = 1; d <= DEPTH; d++) {
    const children: SimNode[] = [];
    for (const s of beam) {
      for (let c = 0; c < BRANCH; c++) {
        const id = s.id * BRANCH + c + 1;
        const trapped = noise(id, 3) > 0.88;
        const gain = 0.45 * s.q + 0.55 * noise(id, 1) + (trapped ? 0.55 : 0);
        const q = clamp(
          0.85 * s.q + 0.35 * (noise(id, 2) - 0.5) - (trapped ? 0.7 : 0),
          0.02,
          1,
        );
        children.push({ id, digits: [...s.digits, c], score: s.score + gain, q, trapped });
      }
    }
    children.sort((a, b) => b.score - a.score || a.id - b.id);
    beam = children.slice(0, K);
    const prefixes = new Set(beam.map((n) => n.digits.slice(0, PREFIX_LEN).join("")));
    levels.push({
      nodes: beam.map((n) => ({
        x: xFrac(n.digits),
        px: xFrac(n.digits.slice(0, -1)),
        trapped: n.trapped,
      })),
      best: beam[0]?.score ?? 0,
      distinct: prefixes.size,
    });
  }
  return { levels, final: levels[DEPTH - 1]?.best ?? 0 };
}

const T = {
  en: {
    title: "Watch a beam descend",
    intro:
      "A synthetic tree, branching 4, depth 14, deterministic scores. Every tick each survivor spawns its 4 children; the pooled children are sorted and only the top K survive. Horizontal position encodes the path prefix — watch the survivors huddle. Amber nodes are traps: a big score now, a poisoned subtree after (the synthetic stand-in for piece theft). The rose trace is K = 1, plain greedy.",
    width: "beam width K",
    depth: "depth",
    beamBest: "beam best",
    greedy: "greedy (K = 1)",
    distinct: "distinct prefixes",
    of: "of",
    collapsed: "collapsed to a single branch",
    scoreCurve: "best score by depth",
    diversityCurve: "distinct depth-3 prefixes among survivors",
    finalNote: (k: number, beam: string, greedy: string) =>
      `Final: beam(${k}) = ${beam}, greedy = ${greedy}. Slide K — each doubling buys less.`,
    loading: "Loading…",
  },
  fr: {
    title: "Regarder un faisceau descendre",
    intro:
      "Un arbre synthétique : branchement 4, profondeur 14, scores déterministes. À chaque pas, chaque survivant engendre ses 4 enfants ; les enfants réunis sont triés et seuls les K meilleurs survivent. La position horizontale encode le préfixe du chemin — regardez les survivants se blottir. Les nœuds ambre sont des pièges : gros score immédiat, sous-arbre empoisonné ensuite (l'équivalent synthétique du vol de pièces). Le tracé rose est K = 1, le glouton pur.",
    width: "largeur du faisceau K",
    depth: "profondeur",
    beamBest: "meilleur du faisceau",
    greedy: "glouton (K = 1)",
    distinct: "préfixes distincts",
    of: "sur",
    collapsed: "effondré sur une seule branche",
    scoreCurve: "meilleur score par profondeur",
    diversityCurve: "préfixes de profondeur 3 distincts parmi les survivants",
    finalNote: (k: number, beam: string, greedy: string) =>
      `Bilan : faisceau(${k}) = ${beam}, glouton = ${greedy}. Faites glisser K — chaque doublement rapporte moins.`,
    loading: "Chargement…",
  },
};

const W = 560;
const ROW_H = 24;
const TOP = 16;
const H = TOP + DEPTH * ROW_H + 8;

function Sparkline({
  title,
  series,
  maxY,
  upTo,
}: {
  title: string;
  series: { values: number[]; className: string }[];
  maxY: number;
  upTo: number;
}) {
  const w = 260;
  const h = 64;
  const pad = 4;
  const px = (d: number) => pad + (d / (DEPTH - 1)) * (w - 2 * pad);
  const py = (v: number) => h - pad - (v / Math.max(maxY, 1)) * (h - 2 * pad);
  return (
    <div className="min-w-0 flex-1">
      <svg viewBox={`0 0 ${w} ${h}`} className="w-full rounded-md border bg-muted/20">
        {series.map((s, si) => {
          const pts = s.values
            .slice(0, upTo)
            .map((v, d) => `${px(d).toFixed(1)},${py(v).toFixed(1)}`)
            .join(" ");
          return pts.length > 0 ? (
            <polyline key={si} points={pts} fill="none" strokeWidth={2} className={s.className} />
          ) : null;
        })}
      </svg>
      <p className="mt-0.5 text-center text-[10px] text-muted-foreground">{title}</p>
    </div>
  );
}

export function BeamWidthLab() {
  const t = useT(T);
  const isClient = useIsClient();
  const [k, setK] = useState(16);
  const [step, setStep] = useState(0);
  const { ref: rootRef, visible } = useRunWhileVisible();

  const sim = useMemo(() => simulate(k), [k]);
  const greedySim = useMemo(() => simulate(1), []);

  useEffect(() => {
    if (!visible) return;
    const id = setInterval(() => setStep((s) => (s + 1) % (DEPTH + PAUSE_TICKS + 1)), 480);
    return () => clearInterval(id);
  }, [visible]);

  if (!isClient) {
    return (
      <div className="flex h-72 items-center justify-center rounded-lg border text-sm text-muted-foreground">
        {t.loading}
      </div>
    );
  }

  const depth = Math.min(step, DEPTH); // 0 = root only
  const level = depth > 0 ? sim.levels[depth - 1] : undefined;
  const greedyLevel = depth > 0 ? greedySim.levels[depth - 1] : undefined;
  const yOf = (d: number) => TOP + d * ROW_H;
  const xOf = (f: number) => 14 + f * (W - 28);

  return (
    <div ref={rootRef} className="space-y-4 rounded-lg border bg-card p-4">
      <div className="space-y-1">
        <h3 className="text-sm font-semibold">{t.title}</h3>
        <p className="text-xs leading-relaxed text-muted-foreground">{t.intro}</p>
      </div>

      <label className="flex items-center gap-3 text-sm">
        <span className="whitespace-nowrap text-muted-foreground">{t.width}</span>
        <input
          type="range"
          min={1}
          max={K_MAX}
          value={k}
          onChange={(e) => {
            setK(Number(e.target.value));
            setStep(0);
          }}
          className="w-full max-w-64"
        />
        <span className="w-8 text-right font-semibold tabular-nums">{k}</span>
      </label>

      <svg viewBox={`0 0 ${W} ${H}`} className="w-full rounded-md border bg-muted/20" role="img" aria-label="A beam-search tree descending depth by depth, keeping only the top K survivors per level with trap nodes in amber and the greedy K equals 1 path overlaid">
        {/* root */}
        <circle cx={xOf(0.5)} cy={yOf(0)} r={4} className="fill-foreground" />
        {/* beam levels revealed so far */}
        {sim.levels.slice(0, depth).map((lv, di) => (
          <g key={di}>
            {lv.nodes.map((n, ni) => (
              <g key={ni}>
                <line
                  x1={xOf(n.px)}
                  y1={yOf(di)}
                  x2={xOf(n.x)}
                  y2={yOf(di + 1)}
                  className="stroke-sky-400"
                  strokeWidth={1}
                  opacity={0.35}
                />
                <circle
                  cx={xOf(n.x)}
                  cy={yOf(di + 1)}
                  r={3.2}
                  className={n.trapped ? "fill-amber-500" : "fill-sky-500"}
                  opacity={di + 1 === depth ? 1 : 0.55}
                />
              </g>
            ))}
          </g>
        ))}
        {/* greedy trace on top */}
        {greedySim.levels.slice(0, depth).map((lv, di) => {
          const n = lv.nodes[0];
          if (!n) return null;
          return (
            <g key={`g${di}`}>
              <line
                x1={xOf(n.px)}
                y1={yOf(di)}
                x2={xOf(n.x)}
                y2={yOf(di + 1)}
                stroke="#fb7185"
                strokeWidth={2}
                opacity={0.9}
              />
              <circle cx={xOf(n.x)} cy={yOf(di + 1)} r={3.6} fill="none" stroke="#fb7185" strokeWidth={1.6} />
            </g>
          );
        })}
      </svg>

      <div className="grid grid-cols-2 gap-2 text-center sm:grid-cols-4">
        <div className="rounded-md border px-2 py-1.5">
          <div className="text-lg font-bold tabular-nums">
            {depth} / {DEPTH}
          </div>
          <div className="text-[10px] text-muted-foreground">{t.depth}</div>
        </div>
        <div className="rounded-md border px-2 py-1.5">
          <div className="text-lg font-bold tabular-nums text-sky-600 dark:text-sky-400">
            {(level?.best ?? 0).toFixed(2)}
          </div>
          <div className="text-[10px] text-muted-foreground">{t.beamBest}</div>
        </div>
        <div className="rounded-md border px-2 py-1.5">
          <div className="text-lg font-bold tabular-nums text-rose-500">
            {(greedyLevel?.best ?? 0).toFixed(2)}
          </div>
          <div className="text-[10px] text-muted-foreground">{t.greedy}</div>
        </div>
        <div
          className={cn(
            "rounded-md border px-2 py-1.5",
            level && level.distinct === 1 && depth >= PREFIX_LEN ? "border-amber-400 bg-amber-500/10" : "",
          )}
        >
          <div className="text-lg font-bold tabular-nums">
            {level?.distinct ?? 1} {t.of} {Math.min(k, BRANCH ** PREFIX_LEN)}
          </div>
          <div className="text-[10px] text-muted-foreground">
            {level && level.distinct === 1 && depth >= PREFIX_LEN ? t.collapsed : t.distinct}
          </div>
        </div>
      </div>

      <div className="flex flex-wrap gap-3">
        <Sparkline
          title={t.scoreCurve}
          maxY={Math.max(sim.final, greedySim.final)}
          upTo={depth}
          series={[
            { values: sim.levels.map((l) => l.best), className: "stroke-sky-500" },
            { values: greedySim.levels.map((l) => l.best), className: "stroke-rose-400" },
          ]}
        />
        <Sparkline
          title={t.diversityCurve}
          maxY={Math.min(k, BRANCH ** PREFIX_LEN)}
          upTo={depth}
          series={[{ values: sim.levels.map((l) => l.distinct), className: "stroke-emerald-500" }]}
        />
      </div>

      <p className="text-xs text-muted-foreground">
        {t.finalNote(k, sim.final.toFixed(2), greedySim.final.toFixed(2))}
      </p>
    </div>
  );
}
