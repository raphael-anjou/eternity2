// The four recovered clue puzzles: each solved board rendered with the real
// Eternity II motifs (same BoardSvg the record boards use, so no flat-triangle
// stand-ins and no clipped border), plus a download of the puzzle as one SVG
// per piece — mirroring the main puzzle's affordance (/puzzle). Clues 1 & 2 come
// from Le Bail's SHORTER solver, Clues 3 & 4 from jwortmann's Eternity2Puzzles.jl;
// see research/topics/clue-puzzle-pieces for provenance and the solve.

import { useState } from "react";
import { Button } from "@/components/ui/button";
import { BoardSvg } from "@/components/board/BoardSvg";
import { downloadPiecesZip } from "@/lib/pieceSvg";
import {
  CLUE1_PIECES,
  CLUE2_PIECES,
  CLUE3_PIECES,
  CLUE4_PIECES,
} from "@/data/clue-puzzle-pieces";
import {
  CLUE1_SOLVED,
  CLUE2_SOLVED,
  CLUE3_SOLVED,
  CLUE4_SOLVED,
  type ClueSolution,
} from "@/data/clue-puzzle-solutions";
import type { Edges } from "@/lib/bucas";

interface Clue {
  n: number;
  label: string;
  dims: string;
  pieces: Edges[];
  solved: ClueSolution;
  filename: string;
}

const CLUES: Clue[] = [
  { n: 1, label: "Clue Puzzle 1", dims: "36 pieces · 6×6", pieces: CLUE1_PIECES, solved: CLUE1_SOLVED, filename: "eternity2-clue-puzzle-1.svg.zip" },
  { n: 2, label: "Clue Puzzle 2", dims: "72 pieces · 12×6", pieces: CLUE2_PIECES, solved: CLUE2_SOLVED, filename: "eternity2-clue-puzzle-2.svg.zip" },
  { n: 3, label: "Clue Puzzle 3", dims: "36 pieces · 6×6", pieces: CLUE3_PIECES, solved: CLUE3_SOLVED, filename: "eternity2-clue-puzzle-3.svg.zip" },
  { n: 4, label: "Clue Puzzle 4", dims: "72 pieces · 12×6", pieces: CLUE4_PIECES, solved: CLUE4_SOLVED, filename: "eternity2-clue-puzzle-4.svg.zip" },
];

function DownloadButton({ pieces, filename }: { pieces: Edges[]; filename: string }) {
  const [zipping, setZipping] = useState(false);
  return (
    <Button
      variant="outline"
      size="sm"
      disabled={zipping}
      onClick={() => {
        setZipping(true);
        // Defer so the label repaints before the synchronous zip blocks the thread.
        setTimeout(() => {
          try {
            downloadPiecesZip(pieces, filename);
          } finally {
            setZipping(false);
          }
        }, 0);
      }}
    >
      {zipping ? "Preparing…" : "Download pieces (.zip)"}
    </Button>
  );
}

function ClueCard({ clue }: { clue: Clue }) {
  return (
    <figure className="flex flex-col gap-2 rounded-lg border p-3">
      <BoardSvg width={clue.solved.w} height={clue.solved.h} cells={clue.solved.cells} />
      <figcaption className="flex flex-wrap items-center justify-between gap-2">
        <span className="text-sm">
          <span className="font-medium">{clue.label}</span>{" "}
          <span className="text-muted-foreground">· {clue.dims}</span>
        </span>
        <DownloadButton pieces={clue.pieces} filename={clue.filename} />
      </figcaption>
    </figure>
  );
}

/**
 * Solved clue puzzles, rendered in the real motifs, each downloadable.
 * With `n` (1–4), shows just that puzzle inline in its own section; without it,
 * shows all four in a grid.
 */
export function CluePuzzlePieces({ n }: { n?: number } = {}) {
  if (n != null) {
    const clue = CLUES.find((c) => c.n === n);
    if (!clue) return null;
    // A single 12×6 board is wide; cap the width so it sits neatly in a section.
    const maxW = clue.solved.w > clue.solved.h ? "sm:max-w-lg" : "sm:max-w-xs";
    return (
      <div className={`my-4 ${maxW}`}>
        <ClueCard clue={clue} />
      </div>
    );
  }
  return (
    <div className="my-6 grid gap-4 sm:grid-cols-2">
      {CLUES.map((c) => (
        <ClueCard key={c.n} clue={c} />
      ))}
    </div>
  );
}
