import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { useT } from "@/i18n";
import { useEngine } from "@/engine/useEngine";
import { createSolver, getGeneratedPuzzle, getPath, getMaxColors } from "@/engine";
import { Button } from "@/components/ui/button";

// The hardness peak, measured live. For a fixed small board we solve a generated
// puzzle at each colour count, right now, with the real engine, and plot the
// actual work (nodes explored) it took. Few colours: many solutions, easy. Many
// colours: tightly constrained, also findable fast (or quickly proven hard).
// In between sits the peak — about one expected solution — where the engine
// grinds hardest. This is the SAT/CSP phase transition, drawn from real runs in
// your browser rather than precomputed numbers.

const SIZE = 6;
const SEEDS_PER_POINT = 5; // median over a few seeds per colour count

interface Point {
  colors: number;
  nodes: number; // median nodes to first solution (or to exhaustion)
  solved: number; // how many of the seeds were solved
}

function medianNodesAt(size: number, colors: number): Point {
  const runs: number[] = [];
  let solved = 0;
  for (let seed = 1; seed <= SEEDS_PER_POINT; seed++) {
    const puzzle = getGeneratedPuzzle(size, colors, seed);
    const path = getPath("row-major", puzzle.width, puzzle.height, 0);
    const solver = createSolver(puzzle, path, { useHints: false });
    let r;
    for (let g = 0; g < 20000; g++) {
      r = solver.step(50000);
      if (r.status !== "running") break;
    }
    if (r) {
      runs.push(r.nodes);
      if (r.status === "solved") solved++;
    }
    solver.free();
  }
  runs.sort((a, b) => a - b);
  const mid = runs.length ? (runs[Math.floor(runs.length / 2)] ?? 0) : 0;
  return { colors, nodes: Math.max(1, mid), solved };
}

const T = {
  en: {
    title: "Build the hardness curve, live",
    intro:
      "Press play. For each number of colours, the engine solves a fresh small puzzle here and now and records the work it took. The curve rises to a peak and falls — that peak is the phase transition, where there's about one solution to find and the search is hardest. Eternity II's designers placed the real puzzle right on it.",
    run: "Measure all colour counts",
    running: (c: number) => `Solving at ${c} colours…`,
    again: "Run again",
    yLabel: "nodes explored (median, log)",
    xLabel: "number of colours",
    peakAt: (c: number) => `Hardest at ${c} colours — the peak`,
    loading: "Loading the engine…",
    note: "Every point is a median over a few real solves on a 6×6 board, run live in your browser. The shape — easy, then a peak, then easy again — is the SAT/CSP phase transition. The full 16×16 with its ~17 interior colours sits on the analogous peak; that placement is what Ansótegui et al. identified as the empirical hardness maximum.",
    idle: "Press the button to measure.",
  },
  fr: {
    title: "Construisez la courbe de difficulté, en direct",
    intro:
      "Appuyez sur lecture. Pour chaque nombre de couleurs, le moteur résout ici et maintenant un petit puzzle frais et note le travail qu'il a fallu. La courbe monte vers un pic puis redescend — ce pic est la transition de phase, là où il y a environ une solution à trouver et où la recherche est la plus dure. Les concepteurs d'Eternity II y ont placé le vrai puzzle.",
    run: "Mesurer tous les nombres de couleurs",
    running: (c: number) => `Résolution à ${c} couleurs…`,
    again: "Relancer",
    yLabel: "nœuds explorés (médiane, log)",
    xLabel: "nombre de couleurs",
    peakAt: (c: number) => `Le plus dur à ${c} couleurs — le pic`,
    loading: "Chargement du moteur…",
    note: "Chaque point est une médiane sur quelques vraies résolutions sur un plateau 6×6, en direct dans votre navigateur. La forme — facile, puis un pic, puis facile à nouveau — est la transition de phase SAT/CSP. Le 16×16 complet avec ses ~17 couleurs intérieures est sur le pic analogue ; c'est ce placement qu'Ansótegui et al. ont identifié comme le maximum empirique de difficulté.",
    idle: "Appuyez sur le bouton pour mesurer.",
  },
};

export function PhaseTransitionLiveLab() {
  const t = useT(T);
  const engineReady = useEngine();
  const [points, setPoints] = useState<Point[]>([]);
  const [running, setRunning] = useState(false);
  const [current, setCurrent] = useState<number | null>(null);
  const cancelRef = useRef(false);

  const maxColors = useMemo(() => (engineReady ? Math.min(12, getMaxColors(SIZE)) : 8), [engineReady]);
  const colorRange = useMemo(() => {
    const arr: number[] = [];
    for (let c = 3; c <= maxColors; c++) arr.push(c);
    return arr;
  }, [maxColors]);

  const run = useCallback(async () => {
    if (!engineReady || running) return;
    setRunning(true);
    setPoints([]);
    cancelRef.current = false;
    const acc: Point[] = [];
    for (const c of colorRange) {
      if (cancelRef.current) break;
      setCurrent(c);
      // Yield to the event loop so the UI can paint between colour points.
      await new Promise((r) => setTimeout(r, 0));
      acc.push(medianNodesAt(SIZE, c));
      setPoints([...acc]);
    }
    setCurrent(null);
    setRunning(false);
  }, [engineReady, running, colorRange]);

  useEffect(() => {
    return () => {
      cancelRef.current = true;
    };
  }, []);

  if (!engineReady) {
    return (
      <div className="flex h-64 items-center justify-center rounded-lg border text-sm text-muted-foreground">
        {t.loading}
      </div>
    );
  }

  const maxNodes = Math.max(1, ...points.map((p) => p.nodes));
  const first = points[0];
  const peak = first ? points.reduce((a, b) => (b.nodes > a.nodes ? b : a), first) : null;

  // Simple SVG log-scale bar chart.
  const W = 360;
  const H = 180;
  const pad = 28;
  const barW = colorRange.length ? (W - pad * 2) / colorRange.length : 10;
  const logMax = Math.log10(maxNodes + 1);

  return (
    <div className="space-y-4 rounded-lg border bg-card p-4">
      <div className="space-y-1">
        <h3 className="text-sm font-semibold">{t.title}</h3>
        <p className="text-xs leading-relaxed text-muted-foreground">{t.intro}</p>
      </div>

      <div className="mx-auto max-w-md">
        <svg viewBox={`0 0 ${W} ${H + 24}`} className="w-full">
          <line x1={pad} y1={H} x2={W - pad} y2={H} className="stroke-border" />
          {colorRange.map((c, i) => {
            const pt = points.find((p) => p.colors === c);
            const h = pt ? (Math.log10(pt.nodes + 1) / logMax) * (H - 10) : 0;
            const isPeak = peak?.colors === c;
            const isCur = current === c;
            return (
              <g key={c}>
                <rect
                  x={pad + i * barW + 2}
                  y={H - h}
                  width={barW - 4}
                  height={h}
                  rx={2}
                  className={
                    isPeak ? "fill-rose-500" : isCur ? "fill-amber-400" : "fill-sky-400/80"
                  }
                  style={{ transition: "height 200ms, y 200ms" }}
                />
                <text
                  x={pad + i * barW + barW / 2}
                  y={H + 14}
                  textAnchor="middle"
                  className="fill-muted-foreground text-[9px]"
                >
                  {c}
                </text>
              </g>
            );
          })}
          <text x={W / 2} y={H + 24} textAnchor="middle" className="fill-muted-foreground text-[9px]">
            {t.xLabel}
          </text>
        </svg>
      </div>

      <div className="flex flex-wrap items-center justify-center gap-3">
        <Button size="sm" onClick={() => void run()} disabled={running}>
          {running && current ? t.running(current) : points.length ? t.again : t.run}
        </Button>
        {peak && !running && (
          <span className="rounded-md border border-rose-300 bg-rose-500/10 px-2 py-1 text-xs font-medium">
            {t.peakAt(peak.colors)}
          </span>
        )}
        {!points.length && !running && (
          <span className="text-xs text-muted-foreground">{t.idle}</span>
        )}
      </div>

      <p className="text-xs text-muted-foreground">{t.note}</p>
    </div>
  );
}
