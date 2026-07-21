import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { useT } from "@/i18n";
import { useEngine } from "@/engine/useEngine";
import { createSolver, getGeneratedPuzzle, getPath, getPathKinds } from "@/engine";
import type { SolverHandle } from "@/engine";
import type { Puzzle, SolverReport } from "@/lib/types";
import { BoardSvg } from "@/components/board/BoardSvg";
import { boardFromEngine } from "@/lib/bucas";
import { Button } from "@/components/ui/button";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { useRunWhileVisible } from "@/lib/useRunWhileVisible";

// The findings page states, from the measured grid, that path order is the
// largest free lever. This lets the reader test that claim directly: pick one of
// the engine's nine real scan orders and watch the same real Rust/WASM
// backtracker, on the same small generated puzzle, reach a different depth. No
// heuristic, no breaks (useHints off) so the fill order is the only thing that
// changes between runs. The scan order is the one reader-editable control; the
// engine's WASM surface exposes no other search knob at this level (there is no
// beam-width or MRV toggle to turn), so depth is read live off the search.

const SIZE = 6;
const COLORS = 5;
const STEPS_PER_SECOND = 160;

const T = {
  en: {
    title: "Change the scan order, change how deep it reaches",
    intro:
      "The same small puzzle, the same real engine (strict, no heuristic), run under a scan order you pick. Path order is the only thing that moves. Watch the depth and node count settle somewhere different for each of the nine orders.",
    order: "Scan order",
    run: "Run",
    pause: "Pause",
    reset: "Reset",
    newPuzzle: "New puzzle",
    loading: "Loading the engine…",
    depth: "deepest cell reached",
    nodes: "search nodes",
    solved: "solved",
    stuck: "exhausted",
    running: "searching",
    note: "A real depth-first search, stepped slowly in your browser; the scan order is the only difference between runs (same pieces, same rules, no look-ahead). A strict border-first or spiral fill walks straight into the hardest constraints and stalls shallow; row-major reaches far deeper. That swing, on a fixed budget, is the lever the study measures at more than 300 points on the full 16×16. This 6×6 is kept small so every placement is watchable.",
  },
  fr: {
    title: "Changez l'ordre de parcours, changez la profondeur atteinte",
    intro:
      "Le même petit puzzle, le même vrai moteur (strict, sans heuristique), lancé selon un ordre de parcours que vous choisissez. Seul l'ordre change. Regardez la profondeur et le nombre de nœuds se fixer ailleurs pour chacun des neuf ordres.",
    order: "Ordre de parcours",
    run: "Lancer",
    pause: "Pause",
    reset: "Réinitialiser",
    newPuzzle: "Nouveau tirage",
    loading: "Chargement du moteur…",
    depth: "case la plus profonde atteinte",
    nodes: "nœuds explorés",
    solved: "résolu",
    stuck: "épuisé",
    running: "recherche",
    note: "Une vraie recherche en profondeur, pas à pas dans votre navigateur ; l'ordre de parcours est la seule différence entre les lancers (mêmes pièces, mêmes règles, sans anticipation). Un remplissage strict par la bordure ou en spirale fonce dans les contraintes les plus dures et cale bas ; le parcours par lignes va bien plus loin. Cet écart, à budget fixé, est le levier que l'étude mesure à plus de 300 points sur le 16×16 complet. Ce 6×6 reste petit pour que chaque pose soit suivable.",
  },
  es: {
    title: "Cambia el orden de recorrido, cambia hasta dónde llega",
    intro:
      "El mismo puzzle pequeño, el mismo motor real (estricto, sin heurística), ejecutado según un orden de recorrido que eliges tú. Lo único que cambia es el orden. Mira cómo la profundidad y el número de nodos se fijan en un punto distinto para cada uno de los nueve órdenes.",
    order: "Orden de recorrido",
    run: "Ejecutar",
    pause: "Pausar",
    reset: "Reiniciar",
    newPuzzle: "Nuevo sorteo",
    loading: "Cargando el motor…",
    depth: "celda más profunda alcanzada",
    nodes: "nodos de búsqueda",
    solved: "resuelto",
    stuck: "agotado",
    running: "buscando",
    note: "Una búsqueda en profundidad real, paso a paso en tu navegador; el orden de recorrido es la única diferencia entre las ejecuciones (mismas piezas, mismas reglas, sin anticipación). Un relleno estricto por el borde o en espiral choca de frente con las restricciones más duras y se estanca poco profundo; el recorrido por filas llega mucho más lejos. Ese salto, con un presupuesto fijo, es la palanca que el estudio mide en más de 300 puntos sobre el 16×16 completo. Este 6×6 se mantiene pequeño para que cada colocación sea observable.",
  },
};

export function DfsScanOrderLab() {
  const t = useT(T);
  const engineReady = useEngine();

  const kinds = useMemo(() => (engineReady ? getPathKinds() : []), [engineReady]);

  const [scan, setScan] = useState("row-major");
  const [seed, setSeed] = useState(7);
  const [running, setRunning] = useState(false);
  const [puzzle, setPuzzle] = useState<Puzzle | null>(null);
  const [report, setReport] = useState<SolverReport | null>(null);
  const [cells, setCells] = useState<Int32Array | null>(null);

  const solverRef = useRef<SolverHandle | null>(null);
  const rafRef = useRef(0);
  const lastTickRef = useRef(0);
  const debtRef = useRef(0);
  const { ref: rootRef, visible } = useRunWhileVisible();

  // (Re)build the solver whenever the puzzle, the seed or the chosen scan order
  // changes. Strict search (useHints off) so the fill order is the only lever.
  const build = useCallback(() => {
    if (!engineReady) return;
    solverRef.current?.free();
    const pz = getGeneratedPuzzle(SIZE, COLORS, seed);
    const path = getPath(scan, pz.width, pz.height, seed);
    const solver = createSolver(pz, path, { useHints: false });
    solverRef.current = solver;
    setPuzzle(pz);
    setReport(solver.report());
    setCells(solver.bestBoard());
    setRunning(false);
    debtRef.current = 0;
  }, [engineReady, seed, scan]);

  useEffect(() => {
    // eslint-disable-next-line react-hooks/set-state-in-effect
    build();
    return () => {
      solverRef.current?.free();
      solverRef.current = null;
    };
  }, [build]);

  // Search loop; pauses while scrolled out of view or in a hidden tab and
  // resumes seamlessly (the dt clamp prevents a catch-up burst).
  useEffect(() => {
    if (!running || !visible) return;
    lastTickRef.current = performance.now();

    const tick = () => {
      const solver = solverRef.current;
      if (!solver) return;
      const now = performance.now();
      const dt = Math.min(now - lastTickRef.current, 250) / 1000;
      lastTickRef.current = now;
      debtRef.current += STEPS_PER_SECOND * dt;
      const budget = Math.floor(debtRef.current);
      if (budget >= 1) {
        debtRef.current -= budget;
        const rep = solver.step(budget);
        setReport(rep);
        setCells(solver.bestBoard());
        if (rep.status !== "running") {
          setRunning(false);
          return;
        }
      }
      rafRef.current = requestAnimationFrame(tick);
    };
    rafRef.current = requestAnimationFrame(tick);
    return () => cancelAnimationFrame(rafRef.current);
  }, [running, visible]);

  const boardCells = useMemo(() => {
    if (!puzzle || !cells) return null;
    return boardFromEngine(puzzle, cells).cells;
  }, [puzzle, cells]);

  const total = puzzle ? puzzle.width * puzzle.height : SIZE * SIZE;

  if (!engineReady || !puzzle || !report) {
    return (
      <div className="flex h-72 items-center justify-center rounded-lg border text-sm text-muted-foreground">
        {t.loading}
      </div>
    );
  }

  const statusLabel =
    report.status === "solved" ? t.solved : report.status === "exhausted" ? t.stuck : t.running;

  return (
    <div ref={rootRef} className="space-y-4 rounded-lg border bg-card p-4">
      <div className="space-y-1">
        <h3 className="text-sm font-semibold">{t.title}</h3>
        <p className="text-xs leading-relaxed text-muted-foreground">{t.intro}</p>
      </div>

      <div className="flex flex-wrap items-center gap-2">
        <label className="flex items-center gap-2 text-xs font-medium">
          <span className="text-muted-foreground">{t.order}</span>
          <Select value={scan} onValueChange={(v) => v && setScan(v)}>
            <SelectTrigger className="h-8 w-40">
              <SelectValue>{scan}</SelectValue>
            </SelectTrigger>
            <SelectContent>
              {kinds.map((k) => (
                <SelectItem key={k} value={k}>
                  {k}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        </label>
        <Button size="sm" onClick={() => setRunning((r) => !r)}>
          {running ? t.pause : t.run}
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

      <div className="grid gap-3 sm:grid-cols-[minmax(0,1fr)_auto] sm:items-center">
        <div className="overflow-hidden rounded-md border">
          {boardCells && (
            <BoardSvg width={puzzle.width} height={puzzle.height} cells={boardCells} />
          )}
        </div>
        <div className="flex gap-4 sm:flex-col sm:gap-2">
          <div>
            <div className="text-2xl font-bold tabular-nums">
              {report.bestPlaced}
              <span className="text-sm font-normal text-muted-foreground">/{total}</span>
            </div>
            <div className="text-[11px] uppercase tracking-wide text-muted-foreground">
              {t.depth}
            </div>
          </div>
          <div>
            <div className="text-2xl font-bold tabular-nums">
              {report.nodes.toLocaleString()}
            </div>
            <div className="text-[11px] uppercase tracking-wide text-muted-foreground">
              {t.nodes}
            </div>
          </div>
          <div className="text-[11px] uppercase tracking-wide text-muted-foreground sm:mt-1">
            {statusLabel}
          </div>
        </div>
      </div>

      <p className="text-xs text-muted-foreground">{t.note}</p>
    </div>
  );
}
