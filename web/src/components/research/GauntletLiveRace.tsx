import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { useT } from "@/i18n";
import { useEngine } from "@/engine/useEngine";
import { createSolver, getGeneratedPuzzle, getPath, getPathKinds } from "@/engine";
import type { SolverHandle } from "@/engine";
import type { Puzzle, SolverReport } from "@/lib/types";
import { BoardSvg } from "@/components/board/BoardSvg";
import { boardFromEngine } from "@/lib/bucas";
import { Button } from "@/components/ui/button";
import { useRunWhileVisible } from "@/lib/useRunWhileVisible";

// GAUNTLET, for real and slowed down. This runs the *actual* Rust/WASM solver
// (not a schematic) once per chosen scan order on the same small generated
// puzzle, each lane using a different fill direction (the "path"). They are
// stepped together, slowly, so you can watch the searches diverge: the same
// pieces, the same rules, different orders of attack, landing on different
// boards. That divergence is exactly GAUNTLET's mechanism; instead of one fill
// order always drifting to the same region, several orders cover different
// regions per run. The reader picks which of the engine's nine real scan orders
// race (the default four are pre-selected).
//
// The puzzle is small (default 6×6) so each lane finishes in seconds at a
// human-watchable speed; "New puzzle" reseeds, "Race" / "Pause" / "Reset" drive
// every lane in lockstep.

// The default four fill directions with distinct, watchable behaviour. The reader
// can swap any of the engine's nine real scan orders in or out (see the picker
// below); this is only the initial selection.
const DEFAULT_KINDS = ["row-major", "snake", "spiral-in", "diagonal"] as const;

// A stable colour per scan order, so a lane keeps its colour as the reader adds
// or removes orders. Any order the picker offers has an entry here.
const LANE_COLORS: Record<string, string> = {
  "row-major": "#a78bfa",
  snake: "#34d399",
  "column-major": "#f472b6",
  "spiral-in": "#fbbf24",
  "spiral-out": "#f97316",
  diagonal: "#38bdf8",
  "border-first": "#c084fc",
  "double-snake": "#2dd4bf",
  random: "#94a3b8",
};
const FALLBACK_COLOR = "#94a3b8";

const SIZE = 6;
const COLORS = 5;
const STEPS_PER_SECOND = 140; // slow enough to follow each placement

interface Lane {
  kind: string;
  color: string;
  solver: SolverHandle;
  report: SolverReport;
  cells: Int32Array;
}

const T = {
  en: {
    title: "Watch it for real: scan orders racing",
    intro:
      "The same small puzzle, solved at once by the real engine; each lane attacks the cells in a different scan order. Four of GAUNTLET's nine orders are picked to start (change them below). Watch them diverge. That divergence is the whole point: one order keeps landing in one region; several orders cover several.",
    race: "Race",
    pause: "Pause",
    reset: "Reset",
    newPuzzle: "New puzzle",
    pick: "Scan orders in the race",
    loading: "Loading the engine…",
    best: "best",
    placed: "placed",
    solved: "solved",
    stuck: "exhausted",
    running: "searching",
    note: "A real depth-first search, stepped slowly in your browser; the scan order is the only difference between the lanes (same pieces, same rules). The production GAUNTLET runs a prior-guided beam search rather than this plain DFS, but the lever is the same one shown here: change the scan order, change the region you reach. Bigger puzzles diverge far more violently; this one is kept to 6×6 so every placement is watchable.",
  },
  fr: {
    title: "En vrai : des ordres de parcours qui s'affrontent",
    intro:
      "Le même petit puzzle, résolu à la fois par le vrai moteur ; chaque couloir attaque les cases dans un ordre de parcours différent. Quatre des neuf ordres de GAUNTLET sont choisis au départ (modifiez-les ci-dessous). Regardez-les diverger. Cette divergence est tout l'intérêt : un ordre retombe toujours dans une région ; plusieurs ordres en couvrent plusieurs.",
    race: "Lancer",
    pause: "Pause",
    reset: "Réinitialiser",
    newPuzzle: "Nouveau tirage",
    pick: "Ordres de parcours dans la course",
    loading: "Chargement du moteur…",
    best: "meilleur",
    placed: "posées",
    solved: "résolu",
    stuck: "épuisé",
    running: "recherche",
    note: "Une vraie recherche en profondeur, pas à pas dans votre navigateur ; l'ordre de parcours est la seule différence entre les couloirs (mêmes pièces, mêmes règles). Le GAUNTLET de production utilise une recherche en faisceau guidée par un prior plutôt que ce DFS simple, mais le levier est le même qu'ici : changez l'ordre de parcours, changez la région atteinte. Les grands puzzles divergent bien plus violemment ; celui-ci reste en 6×6 pour que chaque pose soit suivable.",
  },
  es: {
    title: "Míralo de verdad: órdenes de recorrido compitiendo",
    intro:
      "El mismo puzzle pequeño, resuelto a la vez por el motor real: cada carril ataca las celdas en un orden de recorrido distinto. Cuatro de los nueve órdenes de GAUNTLET se eligen al empezar (cámbialos abajo). Míralos divergir. Esa divergencia es justo la clave: un orden aterriza siempre en la misma región; varios órdenes cubren varias.",
    race: "Iniciar",
    pause: "Pausar",
    reset: "Reiniciar",
    newPuzzle: "Nuevo sorteo",
    pick: "Órdenes de recorrido en la carrera",
    loading: "Cargando el motor…",
    best: "mejor",
    placed: "colocadas",
    solved: "resuelto",
    stuck: "agotado",
    running: "buscando",
    note: "Una búsqueda en profundidad real, paso a paso en tu navegador: el orden de recorrido es la única diferencia entre los carriles (mismas piezas, mismas reglas). El GAUNTLET de producción usa una búsqueda en haz guiada por un prior en lugar de este DFS simple, pero la palanca es la misma que se muestra aquí: cambia el orden de recorrido, cambia la región a la que llegas. Los puzzles más grandes divergen mucho más violentamente; este se mantiene en 6×6 para que cada colocación sea observable.",
  },
};

export function GauntletLiveRace() {
  const t = useT(T);
  const engineReady = useEngine();

  const [seed, setSeed] = useState(7);
  const [running, setRunning] = useState(false);
  const [puzzle, setPuzzle] = useState<Puzzle | null>(null);
  const [lanes, setLanes] = useState<Lane[]>([]);
  // The scan orders currently racing; the reader adds or removes any of the
  // engine's nine real orders. Starts as the default four.
  const [selected, setSelected] = useState<string[]>([...DEFAULT_KINDS]);

  // The nine real scan orders the engine exposes, in engine order.
  const kinds = useMemo(() => (engineReady ? getPathKinds() : []), [engineReady]);

  const lanesRef = useRef<Lane[]>([]);
  const rafRef = useRef(0);
  const lastTickRef = useRef(0);
  const debtRef = useRef(0);
  const { ref: rootRef, visible } = useRunWhileVisible();

  const build = useCallback(() => {
    if (!engineReady) return;
    lanesRef.current.forEach((l) => l.solver.free());
    const pz = getGeneratedPuzzle(SIZE, COLORS, seed);
    const next: Lane[] = selected.map((kind) => {
      const path = getPath(kind, pz.width, pz.height, seed);
      const solver = createSolver(pz, path, { useHints: false });
      return {
        kind,
        color: LANE_COLORS[kind] ?? FALLBACK_COLOR,
        solver,
        report: solver.report(),
        cells: solver.board(),
      };
    });
    setPuzzle(pz);
    lanesRef.current = next;
    setLanes(next);
    setRunning(false);
    debtRef.current = 0;
  }, [engineReady, seed, selected]);

  useEffect(() => {
    // eslint-disable-next-line react-hooks/set-state-in-effect
    build();
    return () => {
      lanesRef.current.forEach((l) => l.solver.free());
      lanesRef.current = [];
    };
  }, [build]);

  // Race loop; pauses entirely while scrolled out of view or in a hidden tab
  // and resumes seamlessly (the dt clamp below prevents any catch-up burst).
  useEffect(() => {
    if (!running || !visible) return;
    lastTickRef.current = performance.now();

    const tick = () => {
      const now = performance.now();
      const dt = Math.min(now - lastTickRef.current, 250) / 1000;
      lastTickRef.current = now;
      debtRef.current += STEPS_PER_SECOND * dt;
      const budget = Math.floor(debtRef.current);
      if (budget >= 1) {
        debtRef.current -= budget;
        let anyRunning = false;
        const updated = lanesRef.current.map((l) => {
          if (l.report.status !== "running") return l;
          const report = l.solver.step(budget);
          if (report.status === "running") anyRunning = true;
          return { ...l, report, cells: l.solver.bestBoard() };
        });
        lanesRef.current = updated;
        setLanes(updated);
        if (!anyRunning) {
          setRunning(false);
          return;
        }
      }
      rafRef.current = requestAnimationFrame(tick);
    };
    rafRef.current = requestAnimationFrame(tick);
    return () => cancelAnimationFrame(rafRef.current);
  }, [running, visible]);

  const laneCells = useMemo(() => {
    if (!puzzle) return [];
    return lanes.map((l) => boardFromEngine(puzzle, l.cells).cells);
  }, [puzzle, lanes]);

  const total = puzzle ? puzzle.width * puzzle.height : SIZE * SIZE;

  if (!engineReady || !puzzle) {
    return (
      <div className="flex h-72 items-center justify-center rounded-lg border text-sm text-muted-foreground">
        {t.loading}
      </div>
    );
  }

  const statusLabel = (s: SolverReport["status"]) =>
    s === "solved" ? t.solved : s === "exhausted" ? t.stuck : t.running;

  return (
    <div ref={rootRef} className="space-y-4 rounded-lg border bg-card p-4">
      <div className="space-y-1">
        <h3 className="text-sm font-semibold">{t.title}</h3>
        <p className="text-xs text-muted-foreground">{t.intro}</p>
      </div>

      <div className="flex flex-wrap items-center gap-2">
        <Button size="sm" onClick={() => setRunning((r) => !r)}>
          {running ? t.pause : t.race}
        </Button>
        <Button size="sm" variant="outline" onClick={build}>
          {t.reset}
        </Button>
        <Button
          size="sm"
          variant="outline"
          onClick={() => setSeed((s) => (s * 1103515245 + 12345) % 100000)}
        >
          {t.newPuzzle}
        </Button>
      </div>

      <fieldset className="space-y-1.5">
        <legend className="text-[11px] font-medium uppercase tracking-wide text-muted-foreground">
          {t.pick}
        </legend>
        <div className="flex flex-wrap gap-1.5">
          {kinds.map((k) => {
            const on = selected.includes(k);
            // Keep at least one lane in the race: the last active order cannot
            // be turned off.
            const last = on && selected.length === 1;
            const color = LANE_COLORS[k] ?? FALLBACK_COLOR;
            return (
              <button
                key={k}
                type="button"
                aria-pressed={on}
                disabled={last}
                onClick={() =>
                  setSelected((prev) =>
                    prev.includes(k)
                      ? prev.filter((x) => x !== k)
                      : // re-add in the engine's canonical order for a stable layout
                        kinds.filter((x) => prev.includes(x) || x === k),
                  )
                }
                className={`flex items-center gap-1.5 rounded-full border px-2.5 py-1 text-xs transition-colors disabled:cursor-not-allowed disabled:opacity-60 ${
                  on
                    ? "border-transparent bg-primary/10 font-medium text-foreground"
                    : "border-border text-muted-foreground hover:text-foreground"
                }`}
              >
                <span
                  className="h-2.5 w-2.5 shrink-0 rounded-full"
                  style={{ backgroundColor: on ? color : "transparent", outline: `1px solid ${color}` }}
                />
                {k}
              </button>
            );
          })}
        </div>
      </fieldset>

      <div className="grid grid-cols-2 gap-3 sm:grid-cols-3">
        {lanes.map((l, i) => {
          const best = l.report.bestPlaced;
          const lc = laneCells[i];
          return (
            <div key={l.kind} className="space-y-1.5">
              <div className="flex items-center gap-1.5">
                <span
                  className="h-2.5 w-2.5 shrink-0 rounded-full"
                  style={{ backgroundColor: l.color }}
                />
                <span className="truncate text-xs font-medium">{l.kind}</span>
              </div>
              <div
                className="overflow-hidden rounded-md border"
                style={{ outline: `2px solid ${l.color}33` }}
              >
                {lc && <BoardSvg width={puzzle.width} height={puzzle.height} cells={lc} />}
              </div>
              <div className="flex items-baseline justify-between text-[11px] text-muted-foreground">
                <span className="tabular-nums">
                  <span className="font-bold text-foreground">{best}</span>/{total} {t.placed}
                </span>
                <span className="uppercase tracking-wide">{statusLabel(l.report.status)}</span>
              </div>
            </div>
          );
        })}
      </div>

      <p className="text-xs text-muted-foreground">{t.note}</p>
    </div>
  );
}
