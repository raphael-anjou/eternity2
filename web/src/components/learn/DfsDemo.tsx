// A 3x3 DFS, one decision at a time, narrated. The whole run is precomputed
// into a list of snapshots, so you can play, pause, and scrub back and forth
// step by step. The piece supply shows which pieces could legally go on the
// current square (green), and which are already used (dimmed).

import { useEffect, useMemo, useState } from "react";
import { BoardSvg } from "@/components/board/BoardSvg";
import { PieceSvg } from "@/components/board/PieceSvg";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { useEngine } from "@/engine/useEngine";
import { createSolver, getGeneratedPuzzle, getPath } from "@/engine";
import { boardFromEngine } from "@/lib/bucas";
import type { Edges } from "@/lib/bucas";
import { rotateEdges, BORDER } from "@/lib/types";
import type { Puzzle } from "@/lib/types";
import { useT } from "@/i18n";
import { cn } from "@/lib/utils";

const SIZE = 3;
const SEED = 15; // 21 placements, 12 backtracks: a story worth watching
const STEP_MS = 900;
const SOLVED_PAUSE_STEPS = 4; // linger on the solved board before looping

const T = {
  en: {
    title: "Backtracking on a 3×3, in slow motion",
    pause: "Pause",
    play: "Play",
    back: "◀ Back",
    forward: "Forward ▶",
    restart: "Restart",
    loading: "Loading engine…",
    stepCounter: (i: number, n: number) => `step ${i} / ${n}`,
    fresh: "A fresh board. First stop: the top-left square.",
    solved: (nodes: number, backtracks: number) =>
      `Solved! ${nodes} placements, ${backtracks} backtracks. Restarting…`,
    deadEnd: "Nothing fits here. Dead end: remove the previous piece and try its next option.",
    placed: (tries: number) =>
      `A piece fits (found after checking ${tries} candidate${tries === 1 ? "" : "s"}). On to the next square.`,
    supplyTitle: "Piece supply",
    supplyHint: "Green = could be placed on the red square right now. Dimmed = already on the board.",
    footer:
      "The red square is where the solver is working. Watch what happens when no piece fits: it un-places the previous piece and continues from there. That undo is the whole secret of backtracking.",
  },
  fr: {
    title: "Le retour en arrière sur un 3×3, au ralenti",
    pause: "Pause",
    play: "Lecture",
    back: "◀ Reculer",
    forward: "Avancer ▶",
    restart: "Recommencer",
    loading: "Chargement du moteur…",
    stepCounter: (i: number, n: number) => `étape ${i} / ${n}`,
    fresh: "Un plateau vierge. Premier arrêt : la case en haut à gauche.",
    solved: (nodes: number, backtracks: number) =>
      `Résolu ! ${nodes} placements, ${backtracks} retours en arrière. On recommence…`,
    deadEnd:
      "Rien ne convient ici. Impasse : on retire la pièce précédente et on essaie son option suivante.",
    placed: (tries: number) =>
      `Une pièce convient (trouvée après ${tries} test${tries > 1 ? "s" : ""} de compatibilité). Direction la case suivante.`,
    supplyTitle: "Réserve de pièces",
    supplyHint:
      "En vert : pourrait être posée sur la case rouge en ce moment. Estompée : déjà sur le plateau.",
    footer:
      "La case rouge indique où le solveur travaille. Observez ce qui se passe quand aucune pièce ne convient : il retire la pièce précédente et repart de là. Cette annulation, c'est tout le secret du retour en arrière.",
  },
};

interface Snapshot {
  board: number[]; // cell -> pieceId*4+rot | -1
  placed: number;
  nodes: number;
  backtracks: number;
  attempts: number;
  solved: boolean;
}

function buildHistory(puzzle: Puzzle): Snapshot[] {
  const solver = createSolver(puzzle, getPath("row-major", SIZE, SIZE, 0), { useHints: false });
  const snaps: Snapshot[] = [];
  const snap = () => {
    const r = solver.report();
    snaps.push({
      board: Array.from(solver.board()),
      placed: r.placed,
      nodes: r.nodes,
      backtracks: r.backtracks,
      attempts: r.attempts,
      solved: r.status === "solved",
    });
  };
  snap();
  for (let guard = 0; guard < 500; guard++) {
    const r = solver.step(1);
    snap();
    if (r.status !== "running") break;
  }
  solver.free();
  return snaps;
}

/** Could this piece (in some rotation) legally sit at `pos` given the board? */
function pieceFitsAt(
  piece: Edges,
  pos: number,
  cells: (Edges | null)[],
): boolean {
  const x = pos % SIZE;
  const y = Math.floor(pos / SIZE);
  for (let r = 0; r < 4; r++) {
    const e = rotateEdges(piece, r);
    const [top, right, bottom, left] = e;
    if ((y === 0) !== (top === BORDER)) continue;
    if ((y === SIZE - 1) !== (bottom === BORDER)) continue;
    if ((x === 0) !== (left === BORDER)) continue;
    if ((x === SIZE - 1) !== (right === BORDER)) continue;
    const up = y > 0 ? cells[pos - SIZE] : null;
    if (up && up[2] !== top) continue;
    const down = y < SIZE - 1 ? cells[pos + SIZE] : null;
    if (down && down[0] !== bottom) continue;
    const lf = x > 0 ? cells[pos - 1] : null;
    if (lf && lf[1] !== left) continue;
    const rt = x < SIZE - 1 ? cells[pos + 1] : null;
    if (rt && rt[3] !== right) continue;
    return true;
  }
  return false;
}

export function DfsDemo() {
  const engineReady = useEngine();
  const t = useT(T);
  const [idx, setIdx] = useState(0);
  const [playing, setPlaying] = useState(true);

  const puzzle = useMemo(
    () => (engineReady ? getGeneratedPuzzle(SIZE, 3, SEED) : null),
    [engineReady],
  );
  const history = useMemo(() => (puzzle ? buildHistory(puzzle) : null), [puzzle]);

  useEffect(() => {
    if (!playing || !history) return;
    const id = setInterval(() => {
      setIdx((i) => (i >= history.length - 1 + SOLVED_PAUSE_STEPS ? 0 : i + 1));
    }, STEP_MS);
    return () => clearInterval(id);
  }, [playing, history]);

  if (!puzzle || !history) {
    return (
      <Card className="max-w-xl">
        <CardContent className="py-10 text-center text-muted-foreground">{t.loading}</CardContent>
      </Card>
    );
  }

  const shown = Math.min(idx, history.length - 1);
  const snap = history[shown];
  const prev = shown > 0 ? history[shown - 1] : null;
  const cells = boardFromEngine(puzzle, snap.board).cells;
  const frontier = snap.solved ? null : snap.placed;

  const usedPieces = new Set(snap.board.filter((v) => v >= 0).map((v) => v >> 2));
  const fitting = new Set<number>();
  if (frontier !== null) {
    puzzle.pieces.forEach((piece, i) => {
      if (!usedPieces.has(i) && pieceFitsAt(piece, frontier, cells)) fitting.add(i);
    });
  }

  let narration: string;
  let mood: "start" | "place" | "back" | "solved";
  if (!prev) {
    narration = t.fresh;
    mood = "start";
  } else if (snap.solved) {
    narration = t.solved(snap.nodes, snap.backtracks);
    mood = "solved";
  } else if (snap.backtracks > prev.backtracks) {
    narration = t.deadEnd;
    mood = "back";
  } else {
    narration = t.placed(snap.attempts - prev.attempts);
    mood = "place";
  }

  const stepTo = (i: number) => {
    setPlaying(false);
    setIdx(Math.max(0, Math.min(history.length - 1, i)));
  };

  return (
    <Card className="max-w-2xl">
      <CardHeader>
        <CardTitle className="flex flex-wrap items-center justify-between gap-2 text-base">
          <span>{t.title}</span>
          <span className="flex gap-1.5">
            <Button variant="outline" size="xs" onClick={() => stepTo(shown - 1)} disabled={shown === 0}>
              {t.back}
            </Button>
            <Button variant="outline" size="xs" onClick={() => setPlaying((p) => !p)}>
              {playing ? t.pause : t.play}
            </Button>
            <Button
              variant="outline"
              size="xs"
              onClick={() => stepTo(shown + 1)}
              disabled={shown >= history.length - 1}
            >
              {t.forward}
            </Button>
            <Button
              variant="outline"
              size="xs"
              onClick={() => {
                setIdx(0);
                setPlaying(true);
              }}
            >
              {t.restart}
            </Button>
          </span>
        </CardTitle>
      </CardHeader>
      <CardContent className="space-y-3">
        <div className="flex flex-wrap items-start justify-center gap-5">
          <div className="w-full max-w-60">
            <BoardSvg
              width={SIZE}
              height={SIZE}
              cells={cells}
              highlight={frontier !== null ? [frontier] : undefined}
            />
            <p className="mt-1 text-center text-xs text-muted-foreground">
              {t.stepCounter(shown, history.length - 1)}
            </p>
          </div>
          <div className="space-y-1.5">
            <p className="text-xs font-medium">{t.supplyTitle}</p>
            <div className="grid grid-cols-3 gap-1.5">
              {puzzle.pieces.map((piece, i) => (
                <div
                  key={i}
                  className={cn(
                    "rounded-md p-0.5 transition-all",
                    usedPieces.has(i) && "opacity-25",
                    fitting.has(i) && "scale-105 ring-3 ring-emerald-500",
                  )}
                >
                  <PieceSvg edges={piece} size={52} />
                </div>
              ))}
            </div>
            <p className="max-w-48 text-xs text-muted-foreground">{t.supplyHint}</p>
          </div>
        </div>
        <p
          className={cn(
            "min-h-12 rounded-md border px-3 py-2 text-sm transition-colors",
            mood === "back" && "border-red-300 bg-red-500/10",
            mood === "solved" && "border-emerald-300 bg-emerald-500/10",
          )}
        >
          {narration}
        </p>
        <p className="text-xs text-muted-foreground">{t.footer}</p>
      </CardContent>
    </Card>
  );
}
