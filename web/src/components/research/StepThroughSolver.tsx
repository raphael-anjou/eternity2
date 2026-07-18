// A reusable, narrated, step-through of the *real* solver. Generalised from the
// DFS demo the explainers are built on: precompute one engine run into a list of
// snapshots, then let the reader play / pause / scrub back and forth, one
// decision at a time. The piece supply shows which pieces could legally sit on
// the current cell (green) and which are already used (dimmed). Nothing is
// hardcoded — the board, the candidates, and the narration are all derived from
// the engine's actual run.
//
// What varies between uses is supplied by props:
//   - puzzle size / colors / seed / fill path  → defines the run
//   - `narrate(snap, prev, ctx)`               → the per-step explanation
// so the same component backs "how DFS backtracks", "how the fill order changes
// what's forced", and any future step-by-step we want, all on real runs.

import { useEffect, useMemo, useState } from "react";
import { BoardSvg } from "@/components/board/BoardSvg";
import { PieceSvg } from "@/components/board/PieceSvg";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { useEngine } from "@/engine/useEngine";
import { createSolver, getGeneratedPuzzle, getPath } from "@/engine";
import { boardFromEngine } from "@/lib/bucas";
import { countCandidates } from "@/lib/piece-fit";
import type { Puzzle } from "@/lib/types";
import { useT } from "@/i18n";
import { cn } from "@/lib/utils";
import { useRunWhileVisible } from "@/lib/useRunWhileVisible";

export interface Snapshot {
  board: number[]; // cell -> pieceId*4+rot | -1
  placed: number;
  nodes: number;
  backtracks: number;
  attempts: number;
  solved: boolean;
  /** Cell the solver is about to work on (the path's next slot), or null. */
  frontier: number | null;
}

export type StepMood = "start" | "place" | "back" | "solved";

export interface NarrationContext {
  /** Pieces that legally fit the current frontier cell right now. */
  candidates: Set<number>;
  /** width / height of the board. */
  width: number;
  height: number;
  /** path order: path[k] = cell filled at depth k. */
  path: number[];
}

export interface Narration {
  text: string;
  mood: StepMood;
}

const UI = {
  en: {
    pause: "Pause",
    play: "Play",
    back: "◀ Back",
    forward: "Forward ▶",
    restart: "Restart",
    loading: "Loading engine…",
    stepCounter: (i: number, n: number) => `step ${i} / ${n}`,
    supplyTitle: "Piece supply",
    supplyHint:
      "Green = could be placed on the red cell right now. Dimmed = already on the board.",
  },
  fr: {
    pause: "Pause",
    play: "Lecture",
    back: "◀ Reculer",
    forward: "Avancer ▶",
    restart: "Recommencer",
    loading: "Chargement du moteur…",
    stepCounter: (i: number, n: number) => `étape ${i} / ${n}`,
    supplyTitle: "Réserve de pièces",
    supplyHint:
      "En vert : les pièces posables sur la case rouge à cet instant. En estompé : déjà sur le plateau.",
  },
  es: {
    pause: "Pausa",
    play: "Reproducir",
    back: "◀ Atrás",
    forward: "Avanzar ▶",
    restart: "Reiniciar",
    loading: "Cargando el motor…",
    stepCounter: (i: number, n: number) => `paso ${i} / ${n}`,
    supplyTitle: "Reserva de piezas",
    supplyHint:
      "En verde: las piezas que caben ahora mismo en la celda roja. Atenuadas: ya están en el tablero.",
  },
};

function buildHistory(puzzle: Puzzle, path: Uint16Array, useHints: boolean): Snapshot[] {
  const solver = createSolver(puzzle, path, { useHints });
  const snaps: Snapshot[] = [];
  const snap = () => {
    const r = solver.report();
    const placed = r.placed;
    snaps.push({
      board: Array.from(solver.board()),
      placed,
      nodes: r.nodes,
      backtracks: r.backtracks,
      attempts: r.attempts,
      solved: r.status === "solved",
      frontier: r.status === "solved" ? null : (path[placed] ?? null),
    });
  };
  snap();
  for (let guard = 0; guard < 4000; guard++) {
    const r = solver.step(1);
    snap();
    if (r.status !== "running") break;
  }
  solver.free();
  return snaps;
}

export function StepThroughSolver({
  title,
  size,
  colors,
  seed,
  pathKind = "row-major",
  useHints = false,
  stepMs = 900,
  endPauseSteps = 4,
  narrate,
  footer,
  boardMaxClass = "max-w-60",
}: {
  title: string;
  size: number;
  colors: number;
  seed: number;
  pathKind?: string;
  useHints?: boolean;
  stepMs?: number;
  endPauseSteps?: number;
  narrate: (snap: Snapshot, prev: Snapshot | null, ctx: NarrationContext) => Narration;
  footer?: string;
  boardMaxClass?: string;
}) {
  const engineReady = useEngine();
  const ui = useT(UI);
  const [idx, setIdx] = useState(0);
  const [playing, setPlaying] = useState(true);
  const { ref: rootRef, visible } = useRunWhileVisible();

  const puzzle = useMemo(
    () => (engineReady ? getGeneratedPuzzle(size, colors, seed) : null),
    [engineReady, size, colors, seed],
  );
  const path = useMemo(
    () => (puzzle ? getPath(pathKind, puzzle.width, puzzle.height, seed) : null),
    [puzzle, pathKind, seed],
  );
  const history = useMemo(
    () => (puzzle && path ? buildHistory(puzzle, path, useHints) : null),
    [puzzle, path, useHints],
  );

  useEffect(() => {
    if (!playing || !history || !visible) return;
    const id = setInterval(() => {
      setIdx((i) => (i >= history.length - 1 + endPauseSteps ? 0 : i + 1));
    }, stepMs);
    return () => clearInterval(id);
  }, [playing, history, stepMs, endPauseSteps, visible]);

  if (!puzzle || !history || !path) {
    return (
      <Card>
        <CardContent className="py-10 text-center text-muted-foreground">{ui.loading}</CardContent>
      </Card>
    );
  }

  const shown = Math.min(idx, history.length - 1);
  const snap = history[shown];
  if (!snap) return null;
  const prev = shown > 0 ? (history[shown - 1] ?? null) : null;
  const cells = boardFromEngine(puzzle, snap.board).cells;

  const usedPieces = new Set(snap.board.filter((v) => v >= 0).map((v) => v >> 2));
  const candidates =
    snap.frontier !== null
      ? countCandidates(
          puzzle.pieces,
          snap.frontier,
          cells,
          usedPieces,
          puzzle.width,
          puzzle.height,
        )
      : new Set<number>();

  const pathArr = Array.from(path);
  const { text, mood } = narrate(snap, prev, {
    candidates,
    width: puzzle.width,
    height: puzzle.height,
    path: pathArr,
  });

  const stepTo = (i: number) => {
    setPlaying(false);
    setIdx(Math.max(0, Math.min(history.length - 1, i)));
  };

  return (
    <Card ref={rootRef}>
      <CardHeader>
        <CardTitle className="flex flex-wrap items-center justify-between gap-2 text-base">
          <span>{title}</span>
          <span className="flex gap-1.5">
            <Button variant="outline" size="xs" onClick={() => stepTo(shown - 1)} disabled={shown === 0}>
              {ui.back}
            </Button>
            <Button variant="outline" size="xs" onClick={() => setPlaying((p) => !p)}>
              {playing ? ui.pause : ui.play}
            </Button>
            <Button
              variant="outline"
              size="xs"
              onClick={() => stepTo(shown + 1)}
              disabled={shown >= history.length - 1}
            >
              {ui.forward}
            </Button>
            <Button
              variant="outline"
              size="xs"
              onClick={() => {
                setIdx(0);
                setPlaying(true);
              }}
            >
              {ui.restart}
            </Button>
          </span>
        </CardTitle>
      </CardHeader>
      <CardContent className="space-y-3">
        <div className="flex flex-wrap items-start justify-center gap-5">
          <div className={cn("w-full", boardMaxClass)}>
            <BoardSvg
              width={puzzle.width}
              height={puzzle.height}
              cells={cells}
              highlight={snap.frontier !== null ? [snap.frontier] : undefined}
            />
            <p className="mt-1 text-center text-xs text-muted-foreground">
              {ui.stepCounter(shown, history.length - 1)}
            </p>
          </div>
          <div className="space-y-1.5">
            <p className="text-xs font-medium">{ui.supplyTitle}</p>
            <div
              className="grid w-fit gap-1.5"
              style={{ gridTemplateColumns: `repeat(${Math.min(6, size)}, max-content)` }}
            >
              {puzzle.pieces.map((piece, i) => (
                <div
                  key={i}
                  className={cn(
                    "rounded-md p-0.5 transition-all",
                    usedPieces.has(i) && "opacity-25",
                    candidates.has(i) && "scale-105 ring-3 ring-emerald-500",
                  )}
                >
                  <PieceSvg edges={piece} size={size <= 3 ? 52 : size <= 5 ? 34 : 24} />
                </div>
              ))}
            </div>
            <p className="max-w-48 text-xs text-muted-foreground">{ui.supplyHint}</p>
          </div>
        </div>
        <p
          className={cn(
            "min-h-12 rounded-md border px-3 py-2 text-sm transition-colors",
            mood === "back" && "border-red-300 bg-red-500/10",
            mood === "solved" && "border-emerald-300 bg-emerald-500/10",
          )}
        >
          {text}
        </p>
        {footer && <p className="text-xs text-muted-foreground">{footer}</p>}
      </CardContent>
    </Card>
  );
}
