import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { useT } from "@/i18n";
import { useEngine } from "@/engine/useEngine";
import { createSolver, getGeneratedPuzzle, getPath, getBoardScore } from "@/engine";
import type { Puzzle } from "@/lib/types";
import { boardFromEngine } from "@/lib/bucas";
import { sigmaPermutation, decomposeCycles, applyCycles } from "@/lib/sigma-cycles";
import { BoardSvg } from "@/components/board/BoardSvg";
import { Button } from "@/components/ui/button";
import { Slider, singleSliderValue } from "@/components/ui/slider";
import { Label } from "@/components/ui/label";
import { driveSolver, yieldToBrowser } from "@/lib/useRunWhileVisible";

// Basin-hopping, on a real small puzzle. We generate a puzzle, find TWO distinct
// real solutions with the engine, then compute the actual permutation σ that
// carries one into the other and split it into its real cycles. You pick a cycle
// and slide it from "not applied" to "fully applied"; every intermediate board is
// scored by the real engine. The finding the research reports shows up live: a
// fully-applied cycle lands on the other perfect board, but every partial step
// scores worse — there is no improving way to walk from one solution to the other.

interface Solved {
  puzzle: Puzzle;
  boardA: Int32Array;
  boardB: Int32Array;
  cycles: number[][];
  maxScore: number;
}

// Find two distinct full solutions of a small puzzle using the real engine.
// Each solve runs in time-boxed bursts (~8ms of synchronous work) with yields to
// the browser in between, so the search never blocks clicks or navigation.
async function findTwoSolutions(
  size: number,
  colors: number,
  seed: number,
  cancelled: () => boolean,
): Promise<Solved | null> {
  const puzzle = getGeneratedPuzzle(size, colors, seed);
  const found = new Map<string, Int32Array>();
  const kinds = ["row-major", "snake", "spiral-in", "diagonal", "column-major", "spiral-out"];
  const STEP_CAP = 80_000_000; // same per-solve cap as the old 400 × 200k loop
  for (const kind of kinds) {
    for (let s = 0; s < 6 && found.size < 6; s++) {
      if (cancelled()) return null;
      const path = getPath(kind, puzzle.width, puzzle.height, s);
      const solver = createSolver(puzzle, path, { useHints: false, shufflePieces: true, seed: s + 1 });
      try {
        const r = await driveSolver(solver, {
          batch: 5_000,
          cancelled,
          budgetCap: STEP_CAP,
        });
        if (r.status === "solved") {
          const board = solver.board();
          found.set(Array.from(board).join(","), Int32Array.from(board));
        }
      } finally {
        solver.free();
      }
    }
    if (found.size >= 2) break;
  }
  const sols = [...found.values()];
  if (sols.length < 2) return null;
  // Pick the pair whose σ has the most interesting (largest) single cycle.
  let best: Solved | null = null;
  for (let i = 0; i < sols.length; i++) {
    for (let j = i + 1; j < sols.length; j++) {
      const a = sols[i];
      const b = sols[j];
      if (!a || !b) continue;
      const cycles = decomposeCycles(sigmaPermutation(a, b));
      const big = cycles[0]?.length ?? 0;
      if (big === 0) continue;
      if (!best || big > (best.cycles[0]?.length ?? 0)) {
        best = { puzzle, boardA: a, boardB: b, cycles, maxScore: getBoardScore(puzzle, a) };
      }
    }
  }
  return best;
}

const T = {
  en: {
    title: "Hop between two real solutions — and watch the score collapse",
    intro:
      "This puzzle has two genuine perfect solutions, both found by the engine. The pieces that differ form loops (cycles). Apply one loop on its own and the score drops — it breaks where it meets the rest of the board. Only applying every loop together (the full hop) reaches the other perfect solution. There is no improving path between them; that is the wall.",
    newPuzzle: "New puzzle",
    loading: "Finding two solutions with the engine…",
    cycleLabel: "Loop to apply",
    allLoops: "All loops (the full hop)",
    cycleN: (i: number, len: number) => `Loop ${i + 1} (${len} cells)`,
    applied: (p: number) => `Applied: ${p}% of the loop`,
    score: "Live score (real engine)",
    perfect: "perfect",
    ofMax: (s: number, m: number) => `${s} / ${m}`,
    boardA: "Solution A",
    working: "Working board",
    drop: "score vs. a perfect board",
    note: "Two solutions, the permutation between them, its cycles, and every score here are computed live by the WebAssembly engine on a freshly generated puzzle — nothing precomputed. On the full 16×16 the analogous cycle spans up to 154 cells and the same all-or-nothing property holds; here it is small enough to watch.",
    verdictPartial: "One loop, applied alone — it breaks the edges where it meets the rest of the board, so the score is worse than either solution. No single loop is a step toward the other board.",
    verdictPartialAll: "Every loop, but only partway — the half-moved pieces don't line up yet, so the score is down. Push to 100%.",
    verdictFull: "Every loop, fully applied — the board is now the other perfect solution. The hop only works all at once.",
    verdictNone: "Slide to start moving pieces along the selected loop(s).",
  },
  fr: {
    title: "Sauter entre deux vraies solutions — et voir le score s'effondrer",
    intro:
      "Ce puzzle a deux vraies solutions parfaites, toutes deux trouvées par le moteur. Les pièces qui diffèrent forment des boucles (cycles). Appliquez une boucle seule et le score chute — elle casse là où elle rencontre le reste du plateau. Seule l'application de toutes les boucles ensemble (le saut complet) atteint l'autre solution parfaite. Il n'y a aucun chemin améliorant entre elles ; c'est le mur.",
    newPuzzle: "Nouveau tirage",
    loading: "Recherche de deux solutions avec le moteur…",
    cycleLabel: "Boucle à appliquer",
    allLoops: "Toutes les boucles (le saut complet)",
    cycleN: (i: number, len: number) => `Boucle ${i + 1} (${len} cellules)`,
    applied: (p: number) => `Appliqué : ${p}% de la boucle`,
    score: "Score en direct (vrai moteur)",
    perfect: "parfait",
    ofMax: (s: number, m: number) => `${s} / ${m}`,
    boardA: "Solution A",
    working: "Plateau de travail",
    drop: "score vs un plateau parfait",
    note: "Les deux solutions, la permutation entre elles, ses cycles et chaque score ici sont calculés en direct par le moteur WebAssembly sur un puzzle fraîchement généré — rien de précalculé. Sur le 16×16 complet, le cycle analogue couvre jusqu'à 154 cellules et la même propriété tout-ou-rien tient ; ici il est assez petit pour être observé.",
    verdictPartial: "Une seule boucle, appliquée seule — elle casse les bords là où elle rencontre le reste du plateau, donc le score est pire que l'une ou l'autre solution. Aucune boucle seule n'est un pas vers l'autre plateau.",
    verdictPartialAll: "Toutes les boucles, mais à mi-chemin — les pièces à demi déplacées ne s'alignent pas encore, le score baisse. Poussez à 100 %.",
    verdictFull: "Toutes les boucles, entièrement appliquées — le plateau est maintenant l'autre solution parfaite. Le saut ne marche que d'un seul coup.",
    verdictNone: "Faites glisser pour déplacer les pièces le long de la/des boucle(s) sélectionnée(s).",
  },
  es: {
    title: "Salta entre dos soluciones reales — y observa cómo se hunde la puntuación",
    intro:
      "Este puzzle tiene dos soluciones perfectas genuinas, ambas halladas por el motor. Las piezas que difieren forman bucles (ciclos). Aplica un bucle por sí solo y la puntuación cae — se rompe allí donde se encuentra con el resto del tablero. Solo al aplicar todos los bucles a la vez (el salto completo) se alcanza la otra solución perfecta. No hay ningún camino de mejora entre ellas; ese es el muro.",
    newPuzzle: "Nuevo puzzle",
    loading: "Buscando dos soluciones con el motor…",
    cycleLabel: "Bucle a aplicar",
    allLoops: "Todos los bucles (el salto completo)",
    cycleN: (i: number, len: number) => `Bucle ${i + 1} (${len} celdas)`,
    applied: (p: number) => `Aplicado: ${p}% del bucle`,
    score: "Puntuación en vivo (motor real)",
    perfect: "perfecto",
    ofMax: (s: number, m: number) => `${s} / ${m}`,
    boardA: "Solución A",
    working: "Tablero de trabajo",
    drop: "puntuación frente a un tablero perfecto",
    note: "Las dos soluciones, la permutación entre ellas, sus ciclos y cada puntuación aquí se calculan en vivo mediante el motor WebAssembly sobre un puzzle recién generado — nada precalculado. En el 16×16 completo el ciclo análogo abarca hasta 154 celdas y se cumple la misma propiedad de todo o nada; aquí es lo bastante pequeño para poder observarlo.",
    verdictPartial: "Un solo bucle, aplicado por sí solo — rompe las aristas allí donde se encuentra con el resto del tablero, así que la puntuación es peor que la de cualquiera de las dos soluciones. Ningún bucle por separado es un paso hacia el otro tablero.",
    verdictPartialAll: "Todos los bucles, pero solo a medias — las piezas movidas a medio camino aún no encajan, así que la puntuación baja. Llévalo al 100 %.",
    verdictFull: "Todos los bucles, aplicados por completo — el tablero es ahora la otra solución perfecta. El salto solo funciona de una sola vez.",
    verdictNone: "Desliza para empezar a mover las piezas a lo largo del bucle o bucles seleccionados.",
  },
};

function MiniBoard({
  puzzle,
  board,
  cycleCells,
}: {
  puzzle: Puzzle;
  board: Int32Array;
  cycleCells?: Set<number>;
}) {
  const cells = boardFromEngine(puzzle, board).cells;
  return (
    <BoardSvg
      width={puzzle.width}
      height={puzzle.height}
      cells={cells}
      highlight={cycleCells ? [...cycleCells] : undefined}
    />
  );
}

export function SigmaCycleLab() {
  const t = useT(T);
  const engineReady = useEngine();
  const [seed, setSeed] = useState(3);
  const [data, setData] = useState<Solved | null>(null);
  const [cycleIdx, setCycleIdx] = useState(0);
  const [pct, setPct] = useState(0);
  const [allCycles, setAllCycles] = useState(false);
  const seedRef = useRef(seed);

  const build = useCallback(
    async (token: { cancelled: boolean }) => {
      if (!engineReady) return;
      const cancelled = () => token.cancelled;
      // Let the effect return before doing any solver work.
      await yieldToBrowser();
      if (token.cancelled) return;
      // Try a few seeds until we get a puzzle with two solutions and a cycle.
      for (let attempt = 0; attempt < 8; attempt++) {
        const s = (seedRef.current + attempt * 101) % 100000;
        const found = await findTwoSolutions(6, 5, s, cancelled);
        if (token.cancelled) return;
        if (found) {
          setData(found);
          setCycleIdx(0);
          setPct(0);
          return;
        }
      }
      setData(null);
    },
    [engineReady],
  );

  useEffect(() => {
    seedRef.current = seed;
    // Cancellation token: flipped on unmount or seed change so no in-flight
    // chunked solve survives navigation.
    const token = { cancelled: false };
    void build(token);
    return () => {
      token.cancelled = true;
    };
  }, [build, seed]);

  const selectedCycles = useMemo(() => {
    if (!data) return new Set<number>();
    if (allCycles) return new Set(data.cycles.map((_, i) => i));
    return new Set([cycleIdx]);
  }, [data, allCycles, cycleIdx]);

  const cycleCells = useMemo(() => {
    if (!data) return new Set<number>();
    const s = new Set<number>();
    selectedCycles.forEach((ci) => data.cycles[ci]?.forEach((c) => s.add(c)));
    return s;
  }, [data, selectedCycles]);

  const working = useMemo(() => {
    if (!data) return null;
    return applyCycles(data.boardA, data.boardB, data.cycles, selectedCycles, pct / 100);
  }, [data, selectedCycles, pct]);

  const liveScore = useMemo(() => {
    if (!data || !working) return 0;
    return getBoardScore(data.puzzle, working);
  }, [data, working]);

  if (!engineReady || !data || !working) {
    return (
      <div className="flex h-72 items-center justify-center rounded-lg border text-sm text-muted-foreground">
        {t.loading}
      </div>
    );
  }

  const drop = data.maxScore - liveScore;
  const isPerfect = liveScore === data.maxScore;
  const verdict =
    pct === 0
      ? t.verdictNone
      : isPerfect
        ? t.verdictFull
        : allCycles
          ? t.verdictPartialAll
          : t.verdictPartial;
  const verdictGood = isPerfect && pct > 0;

  return (
    <div className="space-y-4 rounded-lg border bg-card p-4">
      <div className="space-y-1">
        <h3 className="text-sm font-semibold">{t.title}</h3>
        <p className="text-xs leading-relaxed text-muted-foreground">{t.intro}</p>
      </div>

      <div className="flex flex-wrap items-end gap-4">
        <div className="space-y-1.5">
          <Label>{t.cycleLabel}</Label>
          <div className="flex flex-wrap gap-1.5">
            {data.cycles.slice(0, 6).map((c, i) => (
              <button
                key={i}
                onClick={() => {
                  setAllCycles(false);
                  setCycleIdx(i);
                  setPct(0);
                }}
                className={
                  "rounded-md border px-2 py-1 text-xs transition-colors " +
                  (!allCycles && i === cycleIdx
                    ? "border-foreground bg-foreground text-background"
                    : "hover:bg-muted")
                }
              >
                {t.cycleN(i, c.length)}
              </button>
            ))}
            <button
              onClick={() => {
                setAllCycles(true);
                setPct(0);
              }}
              className={
                "rounded-md border px-2 py-1 text-xs transition-colors " +
                (allCycles
                  ? "border-emerald-500 bg-emerald-500 text-background"
                  : "border-emerald-400/60 hover:bg-muted")
              }
            >
              {t.allLoops}
            </button>
          </div>
        </div>
        <Button size="sm" variant="outline" onClick={() => setSeed((s) => (s * 7 + 13) % 100000)}>
          {t.newPuzzle}
        </Button>
      </div>

      <div className="space-y-1.5">
        <Label>{t.applied(pct)}</Label>
        <Slider min={0} max={100} step={1} value={pct} onValueChange={(v) => setPct(singleSliderValue(v))} />
      </div>

      <div className="grid grid-cols-2 gap-4 sm:grid-cols-[1fr_1fr_auto]">
        <div className="space-y-1">
          <p className="text-xs font-medium text-muted-foreground">{t.boardA}</p>
          <div className="max-w-48">
            <MiniBoard puzzle={data.puzzle} board={data.boardA} cycleCells={cycleCells} />
          </div>
        </div>
        <div className="space-y-1">
          <p className="text-xs font-medium text-muted-foreground">{t.working}</p>
          <div className="max-w-48">
            <MiniBoard puzzle={data.puzzle} board={working} cycleCells={cycleCells} />
          </div>
        </div>
        <div className="space-y-2">
          <p className="text-xs font-medium text-muted-foreground">{t.score}</p>
          <div className="rounded-lg border p-3 text-center">
            <div
              className={
                "text-2xl font-bold tabular-nums " +
                (liveScore === data.maxScore ? "text-emerald-500" : "text-amber-500")
              }
            >
              {t.ofMax(liveScore, data.maxScore)}
            </div>
            <div className="mt-1 text-[11px] text-muted-foreground">
              {liveScore === data.maxScore ? t.perfect : `−${drop} ${t.drop}`}
            </div>
          </div>
        </div>
      </div>

      <p
        className={
          "rounded-md border px-3 py-2 text-sm " +
          (verdictGood
            ? "border-emerald-300 bg-emerald-500/10"
            : pct > 0
              ? "border-amber-300 bg-amber-500/10"
              : "")
        }
      >
        {verdict}
      </p>

      <p className="text-xs text-muted-foreground">{t.note}</p>
    </div>
  );
}
