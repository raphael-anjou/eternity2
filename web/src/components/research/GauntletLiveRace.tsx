import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { useT } from "@/i18n";
import { useEngine } from "@/engine/useEngine";
import { createSolver, getGeneratedPuzzle, getPath } from "@/engine";
import type { SolverHandle } from "@/engine";
import type { Puzzle, SolverReport } from "@/lib/types";
import { BoardSvg } from "@/components/board/BoardSvg";
import { boardFromEngine } from "@/lib/bucas";
import { Button } from "@/components/ui/button";
import { useRunWhileVisible } from "@/lib/useRunWhileVisible";

// GAUNTLET, for real and slowed down. This runs the *actual* Rust/WASM solver —
// not a schematic — four times over on the same small generated puzzle, each
// lane using a different fill direction (the "path"). They are stepped together,
// slowly, so you can watch the four searches diverge: the same pieces, the same
// rules, four different orders of attack, landing on four different boards. That
// divergence is exactly GAUNTLET's mechanism — instead of one fill order always
// drifting to the same region, several orders cover different regions per run.
//
// The puzzle is small (default 6×6) so each lane finishes in seconds at a
// human-watchable speed; "New puzzle" reseeds, "Race" / "Pause" / "Reset" drive
// all four lanes in lockstep.

// Four fill directions with distinct, watchable behaviour.
const LANES = [
  { kind: "row-major", color: "#a78bfa" },
  { kind: "snake", color: "#34d399" },
  { kind: "spiral-in", color: "#fbbf24" },
  { kind: "diagonal", color: "#38bdf8" },
] as const;

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
    title: "Watch it for real: four scan orders racing",
    intro:
      "The same small puzzle, solved four times at once by the real engine — each lane attacks the cells in a different scan order (four of GAUNTLET's nine). Watch them diverge. That divergence is the whole point: one order keeps landing in one region; several orders cover several.",
    race: "Race",
    pause: "Pause",
    reset: "Reset",
    newPuzzle: "New puzzle",
    loading: "Loading the engine…",
    best: "best",
    placed: "placed",
    solved: "solved",
    stuck: "exhausted",
    running: "searching",
    note: "A real depth-first search, stepped slowly in your browser — the scan order is the only difference between the four lanes (same pieces, same rules). The production GAUNTLET runs a prior-guided beam search rather than this plain DFS, but the lever is the same one shown here: change the scan order, change the region you reach. Bigger puzzles diverge far more violently; this one is kept to 6×6 so every placement is watchable.",
  },
  fr: {
    title: "En vrai : quatre ordres de parcours qui s'affrontent",
    intro:
      "Le même petit puzzle, résolu quatre fois à la fois par le vrai moteur — chaque couloir attaque les cases dans un ordre de parcours différent (quatre des neuf de GAUNTLET). Regardez-les diverger. Cette divergence est tout l'intérêt : un ordre retombe toujours dans une région ; plusieurs ordres en couvrent plusieurs.",
    race: "Lancer",
    pause: "Pause",
    reset: "Réinitialiser",
    newPuzzle: "Nouveau tirage",
    loading: "Chargement du moteur…",
    best: "meilleur",
    placed: "posées",
    solved: "résolu",
    stuck: "épuisé",
    running: "recherche",
    note: "Une vraie recherche en profondeur, pas à pas dans votre navigateur — l'ordre de parcours est la seule différence entre les quatre couloirs (mêmes pièces, mêmes règles). Le GAUNTLET de production utilise une recherche en faisceau guidée par un prior plutôt que ce DFS simple, mais le levier est le même qu'ici : changez l'ordre de parcours, changez la région atteinte. Les grands puzzles divergent bien plus violemment ; celui-ci reste en 6×6 pour que chaque pose soit suivable.",
  },
};

export function GauntletLiveRace() {
  const t = useT(T);
  const engineReady = useEngine();

  const [seed, setSeed] = useState(7);
  const [running, setRunning] = useState(false);
  const [puzzle, setPuzzle] = useState<Puzzle | null>(null);
  const [lanes, setLanes] = useState<Lane[]>([]);

  const lanesRef = useRef<Lane[]>([]);
  const rafRef = useRef(0);
  const lastTickRef = useRef(0);
  const debtRef = useRef(0);
  const { ref: rootRef, visible } = useRunWhileVisible();

  const build = useCallback(() => {
    if (!engineReady) return;
    lanesRef.current.forEach((l) => l.solver.free());
    const pz = getGeneratedPuzzle(SIZE, COLORS, seed);
    const next: Lane[] = LANES.map((lane) => {
      const path = getPath(lane.kind, pz.width, pz.height, seed);
      const solver = createSolver(pz, path, { useHints: false });
      return {
        kind: lane.kind,
        color: lane.color,
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
  }, [engineReady, seed]);

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

      <div className="grid grid-cols-2 gap-3 sm:grid-cols-4">
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
