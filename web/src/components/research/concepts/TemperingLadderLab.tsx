import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { useT } from "@/i18n";
import { Button } from "@/components/ui/button";
import { Slider, singleSliderValue } from "@/components/ui/slider";
import { Label } from "@/components/ui/label";
import { Math as InlineMath } from "@/components/research/Math";
import { cn } from "@/lib/utils";
import { useRunWhileVisible } from "@/lib/useRunWhileVisible";

// Parallel tempering on a synthetic rugged 1-D landscape. Four replicas run
// Metropolis at geometrically spaced temperatures; every few sweeps an adjacent
// pair proposes a state swap, accepted with min(1, e^{Δβ·ΔE}). A grey "ghost"
// chain runs at the coldest temperature alone — the honest control. Watch the
// cold replica reach the global minimum through the ladder while the ghost
// stays trapped in the starting basin. Deterministic seed: every reload
// replays the same run.

const R = 4;
const T_COLD = 0.06;
const SEED = 12;
const START_X = 0.14; // everyone starts in the shallow left basin
const TICK_MS = 70;
const SWEEPS_PER_TICK = 3;
const SWAP_EVERY = 10; // ticks between swap attempts
const TRAIL_CAP = 50;

const RUNG_COLORS = ["#3b82f6", "#10b981", "#f59e0b", "#ef4444"]; // cold → hot

const T = {
  en: {
    title: "Four temperatures, one ladder — versus one cold chain",
    intro:
      "A rugged energy landscape with a shallow basin on the left (where everything starts) and the global minimum on the right, behind barriers. Four replicas run Metropolis at geometrically spaced temperatures; adjacent pairs periodically propose to swap states. The grey ghost is the control: the same cold chain, alone. The cold replica escapes through the ladder — hot rungs cross the barriers, swaps hand the good states down. The ghost never leaves.",
    ratio: "Ladder spacing (geometric ratio)",
    play: "Play",
    pause: "Pause",
    reset: "Replay",
    rung: (i: number, temp: number) => `T${i + 1} = ${temp.toFixed(2)}`,
    swapRate: "swap acceptance",
    lastSwap: "Last swap attempt",
    accepted: "accepted",
    rejected: "rejected",
    none: "none yet",
    coldBest: "cold replica, best E",
    ghostBest: "single cold chain, best E",
    globalMin: "global minimum",
    found: "reached the global minimum — via the ladder",
    ghost: "single cold chain (control)",
    tooTight: "≈100% acceptance: rungs see the same landscape — the ladder adds nothing",
    tooWide: "≈0% acceptance: rungs never talk — the ladder decouples",
    healthy: "swaps frequent but not free — a working ladder",
    note: "The landscape is synthetic, the mechanism is exact: Metropolis proposals within each rung, and the true replica-exchange rule between rungs. Drag the spacing to its extremes to reproduce the failure modes the page describes: too tight and every swap is accepted (adjacent rungs are redundant), too wide and none are (the ladder falls apart). Deterministic seed — Replay reruns the identical trajectory.",
  },
  fr: {
    title: "Quatre températures, une échelle — contre une seule chaîne froide",
    intro:
      "Un paysage d'énergie accidenté : un bassin peu profond à gauche (où tout démarre) et le minimum global à droite, derrière des barrières. Quatre répliques font du Metropolis à des températures espacées géométriquement ; les paires adjacentes proposent périodiquement d'échanger leurs états. Le fantôme gris est le témoin : la même chaîne froide, seule. La réplique froide s'échappe par l'échelle — les barreaux chauds franchissent les barrières, les échanges font descendre les bons états. Le fantôme, lui, ne sort jamais.",
    ratio: "Espacement de l'échelle (raison géométrique)",
    play: "Lecture",
    pause: "Pause",
    reset: "Rejouer",
    rung: (i: number, temp: number) => `T${i + 1} = ${temp.toFixed(2)}`,
    swapRate: "acceptation des échanges",
    lastSwap: "Dernier échange proposé",
    accepted: "accepté",
    rejected: "rejeté",
    none: "aucun pour l'instant",
    coldBest: "réplique froide, meilleur E",
    ghostBest: "chaîne froide seule, meilleur E",
    globalMin: "minimum global",
    found: "minimum global atteint — grâce à l'échelle",
    ghost: "chaîne froide seule (témoin)",
    tooTight: "≈100 % d'acceptation : les barreaux voient le même paysage — l'échelle n'apporte rien",
    tooWide: "≈0 % d'acceptation : les barreaux ne se parlent plus — l'échelle se découple",
    healthy: "échanges fréquents mais pas gratuits — une échelle qui travaille",
    note: "Le paysage est synthétique, le mécanisme est exact : propositions de Metropolis dans chaque barreau, et la vraie règle d'échange de répliques entre barreaux. Poussez l'espacement aux extrêmes pour reproduire les pannes décrites par la page : trop serré, tout échange est accepté (les barreaux adjacents sont redondants) ; trop large, plus aucun ne l'est (l'échelle se disloque). Graine déterministe — Rejouer reproduit la trajectoire à l'identique.",
  },
};

/** Deterministic PRNG (mulberry32). */
function mulberry32(seed: number): () => number {
  let a = seed >>> 0;
  return () => {
    a = (a + 0x6d2b79f5) >>> 0;
    let t = a;
    t = Math.imul(t ^ (t >>> 15), t | 1);
    t ^= t + Math.imul(t ^ (t >>> 7), t | 61);
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
}

/** The rugged landscape: two deep wells + a middle one + fine ripples. */
function energy(x: number): number {
  return (
    3 -
    1.6 * Math.exp(-(((x - 0.14) / 0.06) ** 2)) -
    2.6 * Math.exp(-(((x - 0.82) / 0.05) ** 2)) -
    1.1 * Math.exp(-(((x - 0.48) / 0.05) ** 2)) +
    0.3 * Math.sin(46 * x)
  );
}

interface SwapEvent {
  pair: number; // colder rung index i (swap with i+1)
  accepted: boolean;
  dBeta: number;
  dE: number;
  p: number;
  ttl: number;
}

interface Sim {
  xs: number[]; // state at rung i (index = temperature rung, not walker id)
  ghostX: number;
  bestCold: number;
  bestGhost: number;
  coldTrail: number[];
  swapAtt: number[];
  swapAcc: number[];
  tick: number;
  event: SwapEvent | null;
  rng: () => number;
}

function freshSim(): Sim {
  return {
    xs: Array.from({ length: R }, () => START_X),
    ghostX: START_X,
    bestCold: energy(START_X),
    bestGhost: energy(START_X),
    coldTrail: [],
    swapAtt: Array.from({ length: R - 1 }, () => 0),
    swapAcc: Array.from({ length: R - 1 }, () => 0),
    tick: 0,
    event: null,
    rng: mulberry32(SEED),
  };
}

/** One Metropolis move at temperature temp; returns the new position. */
function metropolis(x: number, temp: number, rng: () => number): number {
  // Box-Muller gaussian proposal; hotter rungs take bigger steps.
  const u1 = Math.max(rng(), 1e-12);
  const u2 = rng();
  const g = Math.sqrt(-2 * Math.log(u1)) * Math.cos(2 * Math.PI * u2);
  const nx = x + g * (0.015 + 0.05 * Math.sqrt(temp));
  if (nx < 0 || nx > 1) return x;
  const dE = energy(nx) - energy(x);
  if (dE <= 0 || rng() < Math.exp(-dE / temp)) return nx;
  return x;
}

// Curve geometry.
const W = 640;
const H = 280;
const PAD_X = 16;
const PAD_TOP = 14;
const PAD_BOT = 26;
const SAMPLES = 240;

/** Immutable snapshot of the simulation, for rendering. */
type View = Omit<Sim, "rng">;

function viewOf(s: Sim): View {
  return {
    xs: [...s.xs],
    ghostX: s.ghostX,
    bestCold: s.bestCold,
    bestGhost: s.bestGhost,
    coldTrail: [...s.coldTrail],
    swapAtt: [...s.swapAtt],
    swapAcc: [...s.swapAcc],
    tick: s.tick,
    event: s.event ? { ...s.event } : null,
  };
}

export function TemperingLadderLab() {
  const t = useT(T);
  const { ref: rootRef, visible } = useRunWhileVisible();
  const [ratio, setRatio] = useState(3);
  const [playing, setPlaying] = useState(true);
  // Mutable simulation lives in a ref; rendering reads immutable snapshots.
  // freshSim is deterministic, so the initial view matches the ref's state.
  const simRef = useRef<Sim | null>(null);
  const [view, setView] = useState<View>(() => viewOf(freshSim()));

  const publish = useCallback(() => {
    const s = simRef.current;
    if (s) setView(viewOf(s));
  }, []);

  useEffect(() => {
    simRef.current ??= freshSim();
  }, []);

  const temps = useMemo(() => Array.from({ length: R }, (_, i) => T_COLD * ratio ** i), [ratio]);

  const curve = useMemo(() => {
    const pts: [number, number][] = [];
    let lo = Infinity;
    let hi = -Infinity;
    for (let i = 0; i <= SAMPLES; i++) {
      const x = i / SAMPLES;
      const e = energy(x);
      pts.push([x, e]);
      if (e < lo) lo = e;
      if (e > hi) hi = e;
    }
    const minPt = pts.reduce((a, b) => (b[1] < a[1] ? b : a));
    return { pts, lo, hi, minX: minPt[0], minE: minPt[1] };
  }, []);

  const sx = useCallback((x: number) => PAD_X + x * (W - 2 * PAD_X), []);
  const sy = useCallback(
    (e: number) =>
      PAD_TOP + ((e - curve.lo) / (curve.hi - curve.lo)) * (H - PAD_TOP - PAD_BOT),
    [curve],
  );

  // Reset swap statistics when the spacing changes: the acceptance rates are
  // the whole point of the slider, so they must reflect the current ladder.
  useEffect(() => {
    const s = simRef.current;
    if (!s) return;
    s.swapAtt = s.swapAtt.map(() => 0);
    s.swapAcc = s.swapAcc.map(() => 0);
    s.event = null;
    // Mirror the mutable sim into render state (reset-on-dependency-change).
    publish();
  }, [ratio, publish]);

  // Animation loop, gated on visibility. Each tick runs a handful of O(1)
  // Metropolis moves — microseconds of work, far inside the frame budget.
  useEffect(() => {
    if (!visible || !playing) return;
    const id = setInterval(() => {
      const s = simRef.current;
      if (!s) return;
      for (let sweep = 0; sweep < SWEEPS_PER_TICK; sweep++) {
        for (let i = 0; i < R; i++) {
          const temp = temps[i] ?? T_COLD;
          s.xs[i] = metropolis(s.xs[i] ?? START_X, temp, s.rng);
        }
        s.ghostX = metropolis(s.ghostX, T_COLD, s.rng);
      }
      s.bestCold = Math.min(s.bestCold, energy(s.xs[0] ?? START_X));
      s.bestGhost = Math.min(s.bestGhost, energy(s.ghostX));
      s.coldTrail.push(s.xs[0] ?? START_X);
      if (s.coldTrail.length > TRAIL_CAP) s.coldTrail.shift();
      s.tick += 1;
      if (s.event) {
        s.event.ttl -= 1;
        if (s.event.ttl <= 0) s.event = null;
      }
      if (s.tick % SWAP_EVERY === 0) {
        // Cycle through the adjacent pairs deterministically.
        const i = Math.floor(s.tick / SWAP_EVERY) % (R - 1);
        const j = i + 1;
        const bi = 1 / (temps[i] ?? T_COLD);
        const bj = 1 / (temps[j] ?? T_COLD);
        const Ei = energy(s.xs[i] ?? START_X);
        const Ej = energy(s.xs[j] ?? START_X);
        // Replica-exchange rule: accept with min(1, e^{Δβ·ΔE}),
        // Δβ = βi − βj (>0, i is colder), ΔE = Ei − Ej.
        const dBeta = bi - bj;
        const dE = Ei - Ej;
        const p = Math.min(1, Math.exp(dBeta * dE));
        const accepted = s.rng() < p;
        if (accepted) {
          const tmp = s.xs[i] ?? START_X;
          s.xs[i] = s.xs[j] ?? START_X;
          s.xs[j] = tmp;
        }
        s.swapAtt[i] = (s.swapAtt[i] ?? 0) + 1;
        if (accepted) s.swapAcc[i] = (s.swapAcc[i] ?? 0) + 1;
        s.event = { pair: i, accepted, dBeta, dE, p, ttl: 6 };
      }
      publish();
    }, TICK_MS);
    return () => clearInterval(id);
  }, [visible, playing, temps, publish]);

  const s = view;
  const overallAtt = s.swapAtt.reduce((a, b) => a + b, 0);
  const overallAcc = s.swapAcc.reduce((a, b) => a + b, 0);
  const overallRate = overallAtt > 0 ? overallAcc / overallAtt : 0;
  const ladderVerdict =
    overallAtt < 5 ? null : overallRate > 0.9 ? t.tooTight : overallRate < 0.05 ? t.tooWide : t.healthy;
  const coldFound = s.bestCold < curve.minE + 0.1;

  const path = curve.pts
    .map(([x, e], i) => `${i === 0 ? "M" : "L"}${sx(x).toFixed(1)},${sy(e).toFixed(1)}`)
    .join(" ");

  return (
    <div ref={rootRef} className="space-y-4 rounded-lg border bg-card p-4">
      <div className="space-y-1">
        <h3 className="text-sm font-semibold">{t.title}</h3>
        <p className="text-xs leading-relaxed text-muted-foreground">{t.intro}</p>
      </div>

      <svg viewBox={`0 0 ${W} ${H}`} className="w-full rounded-md border bg-muted/20">
        {/* landscape */}
        <path d={path} fill="none" className="stroke-muted-foreground/70" strokeWidth={1.6} />
        {/* marker for the global minimum */}
        <line
          x1={sx(curve.minX)}
          y1={sy(curve.minE) + 6}
          x2={sx(curve.minX)}
          y2={H - 8}
          className="stroke-amber-500"
          strokeDasharray="4 3"
          strokeWidth={1.2}
        />
        <text
          x={sx(curve.minX)}
          y={H - 12}
          textAnchor="middle"
          fontSize={11}
          className="fill-amber-600"
        >
          {t.globalMin}
        </text>
        {/* cold-replica trail */}
        {s.coldTrail.map((x, i) => (
          <circle
            key={i}
            cx={sx(x)}
            cy={sy(energy(x))}
            r={2}
            fill={RUNG_COLORS[0]}
            opacity={(0.35 * (i + 1)) / s.coldTrail.length}
          />
        ))}
        {/* swap attempt flash */}
        {s.event && (
          <line
            x1={sx(s.xs[s.event.pair] ?? START_X)}
            y1={sy(energy(s.xs[s.event.pair] ?? START_X))}
            x2={sx(s.xs[s.event.pair + 1] ?? START_X)}
            y2={sy(energy(s.xs[s.event.pair + 1] ?? START_X))}
            stroke={s.event.accepted ? "#10b981" : "#f43f5e"}
            strokeWidth={2}
            strokeDasharray={s.event.accepted ? undefined : "4 3"}
            opacity={s.event.ttl / 6}
          />
        )}
        {/* ghost chain */}
        <circle cx={sx(s.ghostX)} cy={sy(energy(s.ghostX))} r={5} fill="#9ca3af" opacity={0.8} />
        {/* replicas, cold drawn last so it stays on top */}
        {[...Array(R).keys()].reverse().map((i) => (
          <circle
            key={i}
            cx={sx(s.xs[i] ?? START_X)}
            cy={sy(energy(s.xs[i] ?? START_X))}
            r={i === 0 ? 7 : 5.5}
            fill={RUNG_COLORS[i]}
            stroke="#fff"
            strokeWidth={1.2}
          />
        ))}
      </svg>

      <div className="flex flex-wrap items-center justify-center gap-x-4 gap-y-1 text-[11px] text-muted-foreground">
        {temps.map((temp, i) => (
          <span key={i} className="flex items-center gap-1.5">
            <span
              className="inline-block h-2.5 w-2.5 rounded-full"
              style={{ background: RUNG_COLORS[i] }}
            />
            {t.rung(i, temp)}
            {i < R - 1 && (
              <span className="tabular-nums text-muted-foreground/70">
                ⇄ {(s.swapAtt[i] ?? 0) > 0 ? Math.round((100 * (s.swapAcc[i] ?? 0)) / (s.swapAtt[i] ?? 1)) : 0}%
              </span>
            )}
          </span>
        ))}
        <span className="flex items-center gap-1.5">
          <span className="inline-block h-2.5 w-2.5 rounded-full bg-gray-400" />
          {t.ghost}
        </span>
      </div>

      <div className="grid gap-3 sm:grid-cols-2">
        <div className="space-y-1.5 rounded-md border p-3">
          <Label>{t.ratio}</Label>
          <Slider
            min={1.2}
            max={6}
            step={0.1}
            value={ratio}
            onValueChange={(v) => setRatio(singleSliderValue(v))}
          />
          {ladderVerdict && (
            <p
              className={cn(
                "text-[11px]",
                ladderVerdict === t.healthy ? "text-emerald-600" : "text-amber-600",
              )}
            >
              {ladderVerdict} ({Math.round(100 * overallRate)}% — {t.swapRate})
            </p>
          )}
        </div>
        <div className="rounded-md border p-3 text-[11px] leading-relaxed">
          <div className="mb-1 font-medium">{t.lastSwap}</div>
          {s.event ? (
            <div className={s.event.accepted ? "text-emerald-600" : "text-rose-500"}>
              <InlineMath>{`p = \\min(1, e^{\\Delta\\beta\\,\\Delta E}) = \\min(1, e^{${(s.event.dBeta * s.event.dE).toFixed(1)}}) = ${s.event.p < 0.001 ? s.event.p.toExponential(1) : s.event.p.toFixed(2)}`}</InlineMath>{" "}
              → {s.event.accepted ? t.accepted : t.rejected}
            </div>
          ) : (
            <div className="text-muted-foreground">{t.none}</div>
          )}
          <div className="mt-2 space-y-0.5 tabular-nums text-muted-foreground">
            <div>
              {t.coldBest}: <span className={cn("font-semibold", coldFound && "text-emerald-600")}>{s.bestCold.toFixed(2)}</span>
              {coldFound && <span className="text-emerald-600"> — {t.found}</span>}
            </div>
            <div>
              {t.ghostBest}: <span className="font-semibold">{s.bestGhost.toFixed(2)}</span>
            </div>
            <div>
              {t.globalMin}: {curve.minE.toFixed(2)}
            </div>
          </div>
        </div>
      </div>

      <div className="flex flex-wrap justify-center gap-2">
        <Button size="sm" variant="outline" onClick={() => setPlaying((p) => !p)}>
          {playing ? t.pause : t.play}
        </Button>
        <Button
          size="sm"
          variant="outline"
          onClick={() => {
            simRef.current = freshSim();
            publish();
          }}
        >
          {t.reset}
        </Button>
      </div>

      <p className="text-xs text-muted-foreground">{t.note}</p>
    </div>
  );
}
