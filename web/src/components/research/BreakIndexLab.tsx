import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { useT } from "@/i18n";
import { useEngine } from "@/engine/useEngine";
import { getGeneratedPuzzle, getPath } from "@/engine";
import { createBreakSolver } from "@/engine/backends/rust";
import type { BreakSolverHandle } from "@/lib/types";
import { boardFromEngine, conflictEdges } from "@/lib/bucas";
import type { Edges } from "@/lib/bucas";
import { BoardSvg } from "@/components/board/BoardSvg";
import { Slider, singleSliderValue } from "@/components/ui/slider";
import { Label } from "@/components/ui/label";
import { Button } from "@/components/ui/button";

// The break-index, run live and watchable. One fixed 8x8 puzzle. Breaks (a
// single mismatch) are licensed only in the bottom `breakRows` rows — the
// "handful of fixed late positions" of the real design. With zero break rows
// the strict search can't finish (it stalls in the low 80s of 112); license a
// couple of bottom rows and it completes a near-perfect board. The search is
// stepped from an animation loop so you can slow it down and watch it place,
// stall, and (with breaks) push through. Real Rust/WASM engine throughout; the
// score and the red break-marks are the engine's own scorer.

const SIZE = 8;
const COLORS = 6;
const SEED = 4;
const MAX_SCORE = SIZE * (SIZE - 1) * 2; // 112

const T = {
  en: {
    title: "The break-index, live",
    intro:
      "One fixed 8×8 puzzle, the real solver running in your browser. A break — a single allowed mismatch — is permitted only in the bottom rows you license below, mirroring how the record solvers confine breaks to a few fixed positions. With none, the strict search stalls below a full board. License a row or two and it completes, near-perfect.",
    breakRows: (n: number) => `Break-licensed rows (from the bottom): ${n}`,
    speed: (n: string) => `Speed: ${n} steps/s`,
    run: "Run",
    pause: "Pause",
    reset: "Reset",
    score: "matched edges",
    breaks: "breaks used",
    placed: "pieces placed",
    statusStuckRun: "Strict (no breaks): the search keeps hunting for a perfect piece it will never find for the last rows — it can't complete the board.",
    statusStuckStall: "Strict (no breaks) stalled: it can't fill the board with every edge matched. The best it ever reached is shown.",
    statusDone: (s: number, b: number) =>
      `Completed — ${s} of ${MAX_SCORE} edges, paying ${b} break${b === 1 ? "" : "es"}, all in the licensed rows. Those few licensed mismatches are exactly what let the board finish.`,
    statusRunning: "Searching…",
    note: "Try one row first: where the breaks are licensed matters as much as how many. Scatter the same number across the top and the board won't finish — the conflicts this scan order accumulates land at the bottom.",
    loading: "Loading the engine…",
  },
  fr: {
    title: "L'index de rupture, en direct",
    intro:
      "Un puzzle 8×8 fixe, le vrai solveur tournant dans votre navigateur. Une rupture — un seul défaut autorisé — n'est permise que dans les rangées du bas que vous autorisez ci-dessous, à l'image des solveurs records qui confinent les ruptures à quelques positions fixes. Sans aucune, la recherche stricte cale sous un plateau complet. Autorisez une rangée ou deux et il termine, quasi parfait.",
    breakRows: (n: number) => `Rangées autorisées à rompre (depuis le bas) : ${n}`,
    speed: (n: string) => `Vitesse : ${n} étapes/s`,
    run: "Lancer",
    pause: "Pause",
    reset: "Réinitialiser",
    score: "bords appariés",
    breaks: "ruptures utilisées",
    placed: "pièces posées",
    statusStuckRun: "Stricte (aucune rupture) : la recherche cherche sans fin une pièce parfaite qu'elle ne trouvera jamais pour les dernières rangées — elle ne peut pas terminer.",
    statusStuckStall: "Stricte (aucune rupture) calée : impossible de remplir le plateau avec tous les bords appariés. Le meilleur atteint est affiché.",
    statusDone: (s: number, b: number) =>
      `Terminé — ${s} bords sur ${MAX_SCORE}, au prix de ${b} rupture${b === 1 ? "" : "s"}, toutes dans les rangées autorisées. Ces quelques défauts autorisés sont exactement ce qui permet de finir.`,
    statusRunning: "Recherche…",
    note: "Essayez d'abord une seule rangée : où les ruptures sont autorisées compte autant que combien. Dispersez-en autant en haut et le plateau ne finira pas — les conflits que cet ordre de balayage accumule tombent en bas.",
    loading: "Chargement du moteur…",
  },
};

export function BreakIndexLab() {
  const t = useT(T);
  const ready = useEngine();
  const [breakRows, setBreakRows] = useState(0);
  const [speedExp, setSpeedExp] = useState(3); // 10^3 steps/s
  const [running, setRunning] = useState(false);

  const [cells, setCells] = useState<(Edges | null)[] | null>(null);
  const [conflicts, setConflicts] = useState<[number, number][]>([]);
  const [score, setScore] = useState(0);
  const [breaks, setBreaks] = useState(0);
  const [placed, setPlaced] = useState(0);
  const [status, setStatus] = useState<"running" | "solved" | "exhausted">("running");

  const puzzle = useMemo(
    () => (ready ? getGeneratedPuzzle(SIZE, COLORS, SEED) : null),
    [ready],
  );

  // Break positions: every cell in the bottom `breakRows` rows.
  const breakPositions = useMemo(() => {
    const out: number[] = [];
    for (let r = SIZE - breakRows; r < SIZE; r++) {
      if (r < 0) continue;
      for (let c = 0; c < SIZE; c++) out.push(r * SIZE + c);
    }
    return new Uint16Array(out);
  }, [breakRows]);

  const solverRef = useRef<BreakSolverHandle | null>(null);
  const rafRef = useRef(0);

  const paint = useCallback(() => {
    const s = solverRef.current;
    if (!s || !puzzle) return;
    const r = s.report();
    const board = boardFromEngine(puzzle, Array.from(s.board()));
    setCells(board.cells);
    setConflicts(conflictEdges(board));
    setScore(s.score());
    setBreaks(s.breaks());
    setPlaced(r.placed);
    setStatus(r.status);
  }, [puzzle]);

  // (Re)build the solver when the puzzle or the break licence changes.
  const rebuild = useCallback(() => {
    solverRef.current?.free();
    if (!puzzle) return;
    const path = getPath("snake", SIZE, SIZE, 0);
    solverRef.current = createBreakSolver(puzzle, path, {
      useHints: false,
      breakPositions,
    });
    setRunning(false);
    paint();
  }, [puzzle, breakPositions, paint]);

  useEffect(() => {
    // rebuild() constructs the WASM break-solver (an external system) and seeds
    // the displayed board/score from it — a legitimate initialize-from-external.
    // eslint-disable-next-line react-hooks/set-state-in-effect
    rebuild();
    return () => {
      solverRef.current?.free();
      solverRef.current = null;
    };
  }, [rebuild]);

  // Animation loop: step the solver at the chosen rate, repaint each frame.
  useEffect(() => {
    if (!running) return;
    const stepsPerSec = Math.pow(10, speedExp);
    let last = 0;
    const tick = (now: number) => {
      const s = solverRef.current;
      if (s) {
        const r = s.report();
        if (r.status === "running") {
          const dt = last ? (now - last) / 1000 : 1 / 60;
          last = now;
          // Cap per-frame work so a fast rate never freezes the tab.
          const want = Math.min(Math.ceil(stepsPerSec * dt), 200_000);
          s.step(Math.max(1, want));
          paint();
        } else {
          setRunning(false);
          paint();
          return;
        }
      }
      rafRef.current = requestAnimationFrame(tick);
    };
    rafRef.current = requestAnimationFrame(tick);
    return () => cancelAnimationFrame(rafRef.current);
  }, [running, speedExp, paint]);

  const placeholder = useMemo<(Edges | null)[]>(
    () => Array<Edges | null>(SIZE * SIZE).fill(null),
    [],
  );
  const speedLabel = speedExp >= 6 ? "1M" : Math.pow(10, speedExp).toLocaleString();
  const stuck = status === "exhausted" || (status === "running" && !running && placed > 0);

  return (
    <section className="mx-auto max-w-3xl space-y-4 rounded-xl border bg-muted/20 p-5">
      <div>
        <h2 className="text-xl font-semibold tracking-tight">{t.title}</h2>
        <p className="mt-1 text-sm text-muted-foreground">{t.intro}</p>
      </div>

      {puzzle ? (
        <div className="grid gap-5 sm:grid-cols-[minmax(0,260px)_1fr] sm:items-start">
          <BoardSvg
            width={SIZE}
            height={SIZE}
            cells={cells ?? placeholder}
            conflicts={conflicts}
            highlight={Array.from(breakPositions)}
            className="max-w-[260px]"
          />
          <div className="space-y-4">
            <div className="space-y-1.5">
              <Label>{t.breakRows(breakRows)}</Label>
              <Slider
                min={0}
                max={4}
                step={1}
                value={breakRows}
                onValueChange={(v) => setBreakRows(singleSliderValue(v))}
              />
            </div>
            <div className="space-y-1.5">
              <Label>{t.speed(speedLabel)}</Label>
              <Slider
                min={0}
                max={6}
                step={1}
                value={speedExp}
                onValueChange={(v) => setSpeedExp(singleSliderValue(v))}
              />
            </div>

            <div className="flex items-center gap-2">
              <Button size="sm" onClick={() => setRunning((r) => !r)} disabled={status !== "running"}>
                {running ? t.pause : t.run}
              </Button>
              <Button size="sm" variant="outline" onClick={rebuild}>
                {t.reset}
              </Button>
            </div>

            <div className="flex flex-wrap gap-x-6 gap-y-1">
              <Stat value={score} sub={t.score} max={MAX_SCORE} />
              <Stat value={breaks} sub={t.breaks} danger={breaks > 0} />
              <Stat value={placed} sub={t.placed} max={SIZE * SIZE} />
            </div>

            <p className="text-sm leading-relaxed text-muted-foreground">
              {status === "solved"
                ? t.statusDone(score, breaks)
                : stuck && breakRows === 0
                  ? t.statusStuckStall
                  : status === "running" && running
                    ? t.statusRunning
                    : breakRows === 0
                      ? t.statusStuckRun
                      : t.statusRunning}
            </p>
          </div>
        </div>
      ) : (
        <div className="flex h-44 items-center justify-center text-sm text-muted-foreground">
          {t.loading}
        </div>
      )}

      <p className="text-xs leading-relaxed text-muted-foreground">{t.note}</p>
    </section>
  );
}

function Stat({ value, sub, max, danger }: { value: number; sub: string; max?: number; danger?: boolean }) {
  return (
    <div>
      <div className={`text-2xl font-bold tabular-nums ${danger ? "text-red-500" : ""}`}>
        {value}
        {max !== undefined && <span className="ml-1 text-base font-normal text-muted-foreground">/ {max}</span>}
      </div>
      <div className="text-xs text-muted-foreground">{sub}</div>
    </div>
  );
}
