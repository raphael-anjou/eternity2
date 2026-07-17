import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import data from "@/data/complex-theory.json";
import { Button } from "@/components/ui/button";
import { useT } from "@/i18n";
import { useRunWhileVisible } from "@/lib/useRunWhileVisible";
import { formatScientific, superscript } from "@/lib/format";

// User-facing strings, keyed by regime where relevant. Regime style/geometry
// stays in REGIMES below; only the human-readable label + line live here.
const T = {
  en: {
    chartAria: "Expected search branches at each depth, sweeping from 1 to 256 cells placed",
    depthAxis: "depth (cells placed, of 256)",
    depthReadout: "depth",
    waysToExtend: "ways to extend",
    regime: "regime",
    play: "Play the sweep",
    replay: "Replay the sweep",
    pause: "Pause",
    scrub: "scrub",
    scrubAria: "Scrub search depth",
    regimes: {
      growth: {
        label: "Growth",
        line: "Every placement still has dozens of legal successors. The tree widens fast.",
      },
      plateau: {
        label: "Plateau — the wall",
        line: "The tree is astronomically wide and almost nothing prunes. A backtracker burns ~99% of its time crossing this band.",
      },
      collapse: {
        label: "Collapse",
        line: "The last ~60 pieces are tightly constrained: each one kills orders of magnitude of branches, funnelling down to the expected solutions.",
      },
    },
  },
  fr: {
    chartAria: "Branches de recherche attendues à chaque profondeur, balayées de 1 à 256 cases posées",
    depthAxis: "profondeur (cases posées, sur 256)",
    depthReadout: "profondeur",
    waysToExtend: "façons de prolonger",
    regime: "régime",
    play: "Lancer le balayage",
    replay: "Rejouer le balayage",
    pause: "Pause",
    scrub: "défiler",
    scrubAria: "Faire défiler la profondeur de recherche",
    regimes: {
      growth: {
        label: "Croissance",
        line: "Chaque placement a encore des dizaines de successeurs légaux. L'arbre s'élargit vite.",
      },
      plateau: {
        label: "Plateau — le mur",
        line: "L'arbre est astronomiquement large et presque rien n'élague. Un backtracker passe ~99 % de son temps à traverser cette bande.",
      },
      collapse: {
        label: "Effondrement",
        line: "Les ~60 dernières pièces sont fortement contraintes : chacune élimine des ordres de grandeur de branches, resserrant l'entonnoir jusqu'aux solutions attendues.",
      },
    },
  },
};

// The E2 funnel as an animated explainer. A "search head" sweeps depth 1 → 256
// along the expected-branches curve (Brendan Owen's complex-theory numbers). As
// it crosses each regime — exponential growth, the vast plateau where the tree
// is widest, then the collapse — the band lights up and a live readout names
// what is happening and how many ways there are to extend the partial board.
//
// This is the static ComplexFunnel made temporal: a static curve shows the
// shape, but watching the head crawl through the plateau — where the counter
// barely moves for a hundred cells — is what makes "the wall" land.
//
// The animation pauses off-screen / when collapsed (useRunWhileVisible) and
// respects prefers-reduced-motion (starts paused, no auto-play).

interface Point {
  depth: number;
  atDepth: number;
  cumulative: number;
}
const curve = data.curve as Point[];

const MAX_DEPTH = 256;
const SWEEP_MS = 7000; // full 1 → 256 sweep

// Regime bands (depth ranges) and the story each tells.
const REGIMES = [
  {
    from: 1,
    to: 50,
    key: "growth",
    fill: "fill-emerald-400/12",
    accent: "text-emerald-600 dark:text-emerald-400",
    dot: "bg-emerald-500",
  },
  {
    from: 50,
    to: 200,
    key: "plateau",
    fill: "fill-amber-400/16",
    accent: "text-amber-600 dark:text-amber-400",
    dot: "bg-amber-500",
  },
  {
    from: 200,
    to: 256,
    key: "collapse",
    fill: "fill-sky-400/12",
    accent: "text-sky-600 dark:text-sky-400",
    dot: "bg-sky-500",
  },
] as const;

const COLLAPSE_REGIME = REGIMES[2];

function regimeAt(depth: number): (typeof REGIMES)[number] {
  return REGIMES.find((r) => depth >= r.from && depth <= r.to) ?? COLLAPSE_REGIME;
}

/** Interpolate expected branch count (log-space) at a fractional depth. */
function branchesAt(depth: number): number {
  const first = curve[0];
  const last = curve[curve.length - 1];
  if (!first || !last) return 1;
  if (depth <= first.depth) return first.atDepth;
  if (depth >= last.depth) return last.atDepth;
  let lo = first;
  for (let i = 1; i < curve.length; i++) {
    const hi = curve[i];
    if (!hi) break;
    if (depth <= hi.depth) {
      const f = (depth - lo.depth) / (hi.depth - lo.depth);
      const logv = Math.log10(Math.max(1, lo.atDepth)) * (1 - f) + Math.log10(Math.max(1, hi.atDepth)) * f;
      return Math.pow(10, logv);
    }
    lo = hi;
  }
  return last.atDepth;
}

const prefersReducedMotion = () =>
  typeof window !== "undefined" &&
  window.matchMedia?.("(prefers-reduced-motion: reduce)").matches;

export function ComplexFunnelAnimated() {
  const t = useT(T);
  const { ref, visible } = useRunWhileVisible();
  const [depth, setDepth] = useState(1); // 1 … 256, may be fractional while animating
  const [playing, setPlaying] = useState(false);
  const rafRef = useRef<number | null>(null);
  const lastTsRef = useRef<number | null>(null);

  // Autoplay once when it first scrolls into view, unless reduced motion.
  const autostartedRef = useRef(false);
  useEffect(() => {
    if (visible && !autostartedRef.current && !prefersReducedMotion()) {
      autostartedRef.current = true;
      setPlaying(true);
    }
  }, [visible]);

  // The animation loop. Advances depth while playing and visible.
  useEffect(() => {
    if (!playing || !visible) {
      lastTsRef.current = null;
      if (rafRef.current !== null) cancelAnimationFrame(rafRef.current);
      return;
    }
    const tick = (ts: number) => {
      const last = lastTsRef.current;
      lastTsRef.current = ts;
      if (last !== null) {
        const dt = ts - last;
        setDepth((d) => {
          const next = d + (dt / SWEEP_MS) * (MAX_DEPTH - 1);
          if (next >= MAX_DEPTH) {
            setPlaying(false);
            return MAX_DEPTH;
          }
          return next;
        });
      }
      rafRef.current = requestAnimationFrame(tick);
    };
    rafRef.current = requestAnimationFrame(tick);
    return () => {
      if (rafRef.current !== null) cancelAnimationFrame(rafRef.current);
    };
  }, [playing, visible]);

  const toggle = useCallback(() => {
    setDepth((d) => (d >= MAX_DEPTH ? 1 : d)); // replay from start if at the end
    setPlaying((p) => !p);
  }, []);

  // Geometry.
  const W = 520;
  const H = 240;
  const padL = 40;
  const padR = 14;
  const padT = 16;
  const padB = 30;
  const logVals = useMemo(() => curve.map((p) => Math.log10(Math.max(1, p.atDepth))), []);
  const maxLog = useMemo(() => Math.max(...logVals), [logVals]);
  const x = (d: number) => padL + (d / MAX_DEPTH) * (W - padL - padR);
  const y = (logv: number) => padT + (1 - logv / maxLog) * (H - padT - padB);
  const pts = useMemo(
    () => curve.map((p, i) => `${x(p.depth)},${y(logVals[i] ?? 0)}`).join(" "),
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [logVals],
  );

  const intDepth = Math.max(1, Math.round(depth));
  const branches = branchesAt(depth);
  const headLog = Math.log10(Math.max(1, branches));
  const headX = x(depth);
  const headY = y(headLog);
  const regime = regimeAt(intDepth);
  const rt = t.regimes[regime.key];

  // The swept portion of the curve, as a highlighted overlay up to the head.
  const sweptPts = useMemo(() => {
    const parts: string[] = [];
    for (const p of curve) {
      if (p.depth > depth) break;
      parts.push(`${x(p.depth)},${y(Math.log10(Math.max(1, p.atDepth)))}`);
    }
    parts.push(`${headX},${headY}`);
    return parts.join(" ");
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [depth]);

  return (
    <div ref={ref} className="space-y-3">
      <div className="mx-auto max-w-2xl">
        <svg viewBox={`0 0 ${W} ${H}`} className="w-full rounded-lg border bg-card" role="img" aria-label={t.chartAria}>
          {/* regime bands, brightened once the head has entered them */}
          {REGIMES.map((b) => {
            const entered = intDepth >= b.from;
            return (
              <rect
                key={b.key}
                x={x(b.from)}
                y={padT}
                width={x(b.to) - x(b.from)}
                height={H - padT - padB}
                className={b.fill}
                opacity={entered ? 1 : 0.35}
              />
            );
          })}
          {/* y gridlines at powers of 10 */}
          {[0, 10, 20, 30, 40].map((lv) => (
            <g key={lv}>
              <line x1={padL} y1={y(lv)} x2={W - padR} y2={y(lv)} className="stroke-muted" strokeWidth={0.5} />
              <text x={4} y={y(lv) + 3} className="fill-muted-foreground text-[8px]">
                10{superscript(lv)}
              </text>
            </g>
          ))}
          {/* full curve (faint), then swept portion (bold) */}
          <polyline points={pts} fill="none" className="stroke-muted-foreground/40" strokeWidth={1.25} />
          <polyline points={sweptPts} fill="none" className="stroke-foreground" strokeWidth={2} />
          {/* search head */}
          <line x1={headX} y1={padT} x2={headX} y2={H - padB} className="stroke-foreground/30" strokeWidth={0.75} strokeDasharray="2 2" />
          <circle cx={headX} cy={headY} r={4} className="fill-foreground" />
          <circle cx={headX} cy={headY} r={7} className="fill-foreground/20">
            {playing && <animate attributeName="r" values="5;9;5" dur="1.2s" repeatCount="indefinite" />}
          </circle>
          {/* x ticks */}
          {[1, 64, 128, 192, 256].map((d) => (
            <text key={d} x={x(d)} y={H - padB + 14} textAnchor="middle" className="fill-muted-foreground text-[8px]">
              {d}
            </text>
          ))}
          <text x={(padL + W - padR) / 2} y={H - 2} textAnchor="middle" className="fill-muted-foreground text-[9px]">
            {t.depthAxis}
          </text>
        </svg>
      </div>

      {/* live readout */}
      <div className="mx-auto grid max-w-2xl grid-cols-3 gap-2 text-center">
        <div className="rounded-md border p-2">
          <div className="text-[10px] uppercase tracking-wide text-muted-foreground">{t.depthReadout}</div>
          <div className="text-lg font-semibold tabular-nums">{intDepth}<span className="text-xs text-muted-foreground">/256</span></div>
        </div>
        <div className="rounded-md border p-2">
          <div className="text-[10px] uppercase tracking-wide text-muted-foreground">{t.waysToExtend}</div>
          <div className="text-lg font-semibold tabular-nums">{formatScientific(branches)}</div>
        </div>
        <div className="rounded-md border p-2">
          <div className="text-[10px] uppercase tracking-wide text-muted-foreground">{t.regime}</div>
          <div className={`flex items-center justify-center gap-1.5 text-sm font-semibold ${regime.accent}`}>
            <span className={`inline-block h-2 w-2 rounded-full ${regime.dot}`} />
            {rt.label}
          </div>
        </div>
      </div>

      <p className="mx-auto max-w-2xl text-center text-xs leading-relaxed text-muted-foreground">
        <span className={`font-medium ${regime.accent}`}>{rt.label}.</span> {rt.line}
      </p>

      {/* controls */}
      <div className="flex flex-wrap items-center justify-center gap-3">
        <Button size="sm" onClick={toggle}>
          {playing ? t.pause : depth >= MAX_DEPTH ? t.replay : t.play}
        </Button>
        <label className="flex items-center gap-2 text-xs text-muted-foreground">
          {t.scrub}
          <input
            type="range"
            min={1}
            max={MAX_DEPTH}
            step={1}
            value={intDepth}
            onChange={(e) => {
              setPlaying(false);
              setDepth(Number(e.target.value));
            }}
            className="w-40 accent-foreground"
            aria-label={t.scrubAria}
          />
        </label>
      </div>
    </div>
  );
}
