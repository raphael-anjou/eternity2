import { useMemo, useState } from "react";
import { useT } from "@/i18n";
import { useEngine } from "@/engine/useEngine";
import { getGeneratedPuzzleFramed, getPath, getPathKinds } from "@/engine";
import {
  estimateComplexity,
  COMPLEX_THEORY_SOURCE_URL,
  type ComplexityEstimate,
} from "@/engine-ts/complexity";
import { cn } from "@/lib/utils";

// Complex theory, made interactive. Pick a scan order; the funnel — the expected
// node count at each depth — redraws live from the real piece set, and the
// plateau peak (the widest point the search must cross) is called out. The whole
// lesson is that the order decides the peak height before you run a single node:
// compact frontiers (row-major, snake) keep it low; a scattered order (random)
// blows it up. Uses the same first-moment estimator the playground does,
// calibrated to McGavin's published 16×16 curve.

const SIZE = 10;
const COLORS = 6;
const ORDERS = ["row-major", "snake", "spiral-in", "diagonal", "random"];

const T = {
  en: {
    title: "Pick a scan order — watch the funnel",
    intro:
      "Complex theory scores a fill order before you run it. Choose one below: the curve is the expected number of ways to extend a partial board at each depth — the width of the search tree. The plateau peak is the widest point a backtracker must cross. Lower is better, and the order alone decides it.",
    peak: "plateau peak",
    atDepth: (d: number) => `widest at depth ${d}`,
    best: "lowest peak",
    yAxis: "expected branches (log)",
    xAxis: "depth (cells placed)",
    note: "An exact port of McGavin's reference implementation of Owen's theory — it reproduces his published 16×16 curve. A compact frontier (row-major, snake) keeps the peak low; a scattered order (random) explodes it, because a spread-out frontier constrains almost nothing. This is why every good solver fills the board in a tight, advancing band.",
    sourcePrefix: "Method:",
    sourceName: "McGavin's reference implementation",
    loading: "Loading the engine…",
  },
  fr: {
    title: "Choisissez un ordre de balayage — observez l'entonnoir",
    intro:
      "La théorie complexe évalue un ordre de remplissage avant de le lancer. Choisissez-en un : la courbe est le nombre attendu de façons d'étendre un plateau partiel à chaque profondeur — la largeur de l'arbre de recherche. Le pic de plateau est le point le plus large qu'un backtracker doit traverser. Plus c'est bas, mieux c'est, et l'ordre seul le décide.",
    peak: "pic de plateau",
    atDepth: (d: number) => `au plus large à la profondeur ${d}`,
    best: "pic le plus bas",
    yAxis: "branches attendues (log)",
    xAxis: "profondeur (cases posées)",
    note: "Un portage exact de l'implémentation de référence de McGavin de la théorie d'Owen — il reproduit sa courbe 16×16 publiée. Une frontière compacte (lignes, serpent) garde le pic bas ; un ordre dispersé (aléatoire) l'explose, car une frontière étalée ne contraint presque rien. C'est pourquoi tout bon solveur remplit le plateau en une bande serrée qui avance.",
    sourcePrefix: "Méthode :",
    sourceName: "implémentation de référence de McGavin",
    loading: "Chargement du moteur…",
  },
  es: {
    title: "Elige un orden de recorrido — observa el embudo",
    intro:
      "La teoría compleja evalúa un orden de relleno antes de ejecutarlo. Elige uno: la curva es el número esperado de formas de extender un tablero parcial a cada profundidad — la anchura del árbol de búsqueda. El pico de meseta es el punto más ancho que un backtracker debe atravesar. Cuanto más bajo, mejor, y el orden por sí solo lo decide.",
    peak: "pico de meseta",
    atDepth: (d: number) => `más ancho en la profundidad ${d}`,
    best: "pico más bajo",
    yAxis: "ramas esperadas (log)",
    xAxis: "profundidad (celdas colocadas)",
    note: "Un port exacto de la implementación de referencia de McGavin de la teoría de Owen — reproduce su curva 16×16 publicada. Una frontera compacta (por filas, serpiente) mantiene el pico bajo; un orden disperso (aleatorio) lo dispara, porque una frontera extendida casi no impone restricciones. Por eso todo buen solucionador rellena el tablero en una banda estrecha que avanza.",
    sourcePrefix: "Método:",
    sourceName: "implementación de referencia de McGavin",
    loading: "Cargando el motor…",
  },
};

const ORDER_COLOR: Record<string, string> = {
  "row-major": "hsl(210 70% 50%)",
  snake: "hsl(160 60% 45%)",
  "spiral-in": "hsl(30 80% 50%)",
  diagonal: "hsl(280 60% 55%)",
  random: "hsl(0 70% 55%)",
};

function fmtLog(log10: number): string {
  if (!isFinite(log10)) return "—";
  if (log10 < 4) return Math.round(10 ** log10).toLocaleString();
  const exp = Math.floor(log10);
  const mant = 10 ** (log10 - exp);
  return `${mant.toFixed(1)}×10^${exp}`;
}

interface Curve {
  kind: string;
  est: ComplexityEstimate;
}

export function ComplexFunnelLab() {
  const t = useT(T);
  const engineReady = useEngine();
  const [pick, setPick] = useState("row-major");

  const curves = useMemo<Curve[]>(() => {
    if (!engineReady) return [];
    const puzzle = getGeneratedPuzzleFramed(SIZE, COLORS, 1, true);
    const kinds = new Set(getPathKinds());
    return ORDERS.filter((k) => kinds.has(k)).map((kind) => {
      const path = Array.from(getPath(kind, SIZE, SIZE, 1));
      return { kind, est: estimateComplexity(puzzle, path) };
    });
  }, [engineReady]);

  const bestKind = useMemo(() => {
    if (curves.length === 0) return null;
    return curves.reduce((a, b) => (a.est.peakLog10 <= b.est.peakLog10 ? a : b)).kind;
  }, [curves]);

  if (!engineReady || curves.length === 0) {
    return (
      <div className="flex h-64 items-center justify-center rounded-lg border text-sm text-muted-foreground">
        {t.loading}
      </div>
    );
  }

  const active = curves.find((c) => c.kind === pick) ?? curves[0];
  if (!active) return null;

  return (
    <div className="space-y-4 rounded-lg border bg-card p-4">
      <div className="space-y-1">
        <h3 className="text-sm font-semibold">{t.title}</h3>
        <p className="text-xs leading-relaxed text-muted-foreground">{t.intro}</p>
      </div>

      <div className="flex flex-wrap justify-center gap-1.5">
        {curves.map((c) => (
          <button
            key={c.kind}
            onClick={() => setPick(c.kind)}
            className={cn(
              "flex items-center gap-1.5 rounded border px-2.5 py-1 text-xs font-medium transition-colors",
              c.kind === pick
                ? "border-primary bg-primary text-primary-foreground"
                : "text-muted-foreground hover:bg-muted",
            )}
          >
            <span
              className="inline-block h-2 w-2 rounded-sm"
              style={{ background: ORDER_COLOR[c.kind] ?? "hsl(0 0% 60%)" }}
            />
            {c.kind}
            {c.kind === bestKind && <span className="text-[9px]">★</span>}
          </button>
        ))}
      </div>

      <FunnelChart curves={curves} active={active.kind} yAxis={t.yAxis} xAxis={t.xAxis} />
      <p className="-mt-1 text-center text-[10px] text-muted-foreground">
        {t.sourcePrefix}{" "}
        <a className="underline" href={COMPLEX_THEORY_SOURCE_URL} target="_blank" rel="noreferrer">
          {t.sourceName}
        </a>
      </p>

      <div className="mx-auto max-w-sm rounded-md border bg-muted/30 px-3 py-2 text-center">
        <div className="text-lg font-bold tabular-nums">
          {t.peak} {fmtLog(active.est.peakLog10)}
        </div>
        <div className="text-[11px] text-muted-foreground">
          {active.kind}
          {active.kind === bestKind ? ` · ${t.best}` : ""} — {t.atDepth(active.est.peakDepth)}
        </div>
      </div>

      <p className="text-xs text-muted-foreground">{t.note}</p>
    </div>
  );
}

function FunnelChart({
  curves,
  active,
  yAxis,
  xAxis,
}: {
  curves: Curve[];
  active: string;
  yAxis: string;
  xAxis: string;
}) {
  const W = 520;
  const H = 230;
  const padL = 40;
  const padR = 10;
  const padT = 12;
  const padB = 28;

  const maxDepth = Math.max(...curves.map((c) => c.est.curve.length), 1);
  const maxLog = Math.max(...curves.map((c) => c.est.peakLog10), 1);

  const x = (d: number) => padL + (d / maxDepth) * (W - padL - padR);
  const y = (logv: number) => padT + (1 - logv / maxLog) * (H - padT - padB);

  const gridLevels: number[] = [];
  const step = Math.max(1, Math.round(maxLog / 5));
  for (let lv = 0; lv <= maxLog; lv += step) gridLevels.push(lv);

  return (
    <div className="space-y-1">
      <svg viewBox={`0 0 ${W} ${H}`} className="w-full rounded-lg border bg-card" role="img" aria-label="Log-scale curves of expected search branches against depth in cells placed, one curve per scan order, showing how the funnel narrows">
        {gridLevels.map((lv) => (
          <g key={lv}>
            <line
              x1={padL}
              y1={y(lv)}
              x2={W - padR}
              y2={y(lv)}
              className="stroke-muted"
              strokeWidth={0.5}
            />
            <text x={4} y={y(lv) + 3} className="fill-muted-foreground text-[8px]">
              10^{lv}
            </text>
          </g>
        ))}
        {curves.map((c) => {
          const isActive = c.kind === active;
          const pts = c.est.curve
            .slice(0, c.est.peakDepth)
            .map((p) => `${x(p.depth)},${y(Math.log10(Math.max(p.cumulative, 1)))}`)
            .join(" ");
          return (
            <polyline
              key={c.kind}
              points={pts}
              fill="none"
              stroke={ORDER_COLOR[c.kind] ?? "hsl(0 0% 60%)"}
              strokeWidth={isActive ? 2.4 : 1.1}
              strokeOpacity={isActive ? 1 : 0.4}
            />
          );
        })}
        <text
          x={12}
          y={(H - padB) / 2}
          textAnchor="middle"
          transform={`rotate(-90 12 ${(H - padB) / 2})`}
          className="fill-muted-foreground text-[9px]"
        >
          {yAxis}
        </text>
        <text x={W / 2} y={H - 2} textAnchor="middle" className="fill-muted-foreground text-[9px]">
          {xAxis}
        </text>
      </svg>
    </div>
  );
}
