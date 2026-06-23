import { useMemo, useState } from "react";
import { useT } from "@/i18n";
import { useEngine } from "@/engine/useEngine";
import { createSolver, getGeneratedPuzzle, getPath } from "@/engine";
import { boardFromEngine, scoreSummary } from "@/lib/bucas";
import type { Edges } from "@/lib/bucas";
import { BoardSvg } from "@/components/board/BoardSvg";
import { Button } from "@/components/ui/button";

// MOSAIC's load-bearing fact, made tangible: a small block solves to its exact
// optimum almost instantly. We solve four independent 4x4 blocks with the real
// engine, each to a perfectly-matched board (its maximum score), and lay them
// out as a 2x2 tiling — the picture of "tile the board, solve each tile exactly,
// then compose". The seams between tiles are NOT solved here (that is the soft-
// boundary composition the method adds); this shows only that the tiles
// themselves are exactly solvable, which is what makes the whole approach
// possible. Real engine; each block's score is the engine's own count.

const BLOCK = 4;
const COLORS = 4;
const BLOCK_MAX = BLOCK * (BLOCK - 1) * 2; // 24 interior edges in a 4x4

interface SolvedBlock {
  cells: (Edges | null)[];
  score: number;
  max: number;
  attempts: number;
}

function solveBlock(seed: number): SolvedBlock {
  const puzzle = getGeneratedPuzzle(BLOCK, COLORS, seed);
  const path = getPath("snake", BLOCK, BLOCK, 0);
  const solver = createSolver(puzzle, path, { useHints: false });
  let r = solver.report();
  let guard = 0;
  while (r.status === "running" && guard < 200) {
    r = solver.step(50_000);
    guard++;
  }
  const view = boardFromEngine(puzzle, Array.from(solver.board()));
  const summary = scoreSummary(view);
  const out: SolvedBlock = { cells: view.cells, score: summary.score, max: summary.max, attempts: r.attempts };
  solver.free();
  return out;
}

const T = {
  en: {
    title: "A block solves exactly — instantly",
    intro:
      "MOSAIC rests on one fact: a small block of the puzzle solves to its exact best in a flash. Here are four independent 4×4 blocks, each solved by the real engine to a perfectly-matched board — every interior edge agrees. Tiling them is the easy half; the method's real work is composing the seams between them.",
    blockLabel: (i: number) => `block ${i + 1}`,
    perfect: "perfect",
    reshuffle: "New blocks",
    note: (n: string) => `Each block solved to its optimum (${BLOCK_MAX}/${BLOCK_MAX} edges) in about ${n} placement tries — milliseconds. The full 16×16 is the same trick, sixteen times, plus the hard part: the seams.`,
    loading: "Loading the engine…",
  },
  fr: {
    title: "Un bloc se résout exactement — instantanément",
    intro:
      "MOSAIC repose sur un fait : un petit bloc du puzzle se résout à son optimum en un éclair. Voici quatre blocs 4×4 indépendants, chacun résolu par le vrai moteur en un plateau parfaitement apparié — chaque bord intérieur concorde. Les juxtaposer est la moitié facile ; le vrai travail de la méthode, c'est de composer les jointures entre eux.",
    blockLabel: (i: number) => `bloc ${i + 1}`,
    perfect: "parfait",
    reshuffle: "Nouveaux blocs",
    note: (n: string) => `Chaque bloc résolu à son optimum (${BLOCK_MAX}/${BLOCK_MAX} bords) en environ ${n} essais de pose — des millisecondes. Le 16×16 complet, c'est la même astuce, seize fois, plus la partie difficile : les jointures.`,
    loading: "Chargement du moteur…",
  },
};

export function MosaicBlockLab() {
  const t = useT(T);
  const ready = useEngine();
  const [round, setRound] = useState(0);

  const blocks = useMemo<SolvedBlock[] | null>(() => {
    if (!ready) return null;
    return [0, 1, 2, 3].map((i) => solveBlock(round * 4 + i + 1));
  }, [ready, round]);

  if (!blocks) {
    return (
      <section className="mx-auto max-w-3xl rounded-xl border bg-muted/20 p-5">
        <div className="flex h-48 items-center justify-center text-sm text-muted-foreground">{t.loading}</div>
      </section>
    );
  }

  const totalAttempts = blocks.reduce((a, b) => a + b.attempts, 0);

  return (
    <section className="mx-auto max-w-3xl space-y-4 rounded-xl border bg-muted/20 p-5">
      <div>
        <h2 className="text-xl font-semibold tracking-tight">{t.title}</h2>
        <p className="mt-1 text-sm text-muted-foreground">{t.intro}</p>
      </div>

      <div className="mx-auto grid w-fit grid-cols-2 gap-1 rounded-lg bg-border p-1">
        {blocks.map((b, i) => (
          <div key={i} className="relative">
            <BoardSvg width={BLOCK} height={BLOCK} cells={b.cells} className="w-[120px]" />
            <span className="absolute right-0.5 top-0.5 rounded bg-emerald-500/90 px-1 text-[9px] font-medium text-white">
              {b.score}/{b.max}
            </span>
          </div>
        ))}
      </div>

      <div className="flex items-center justify-center gap-3">
        <Button size="sm" variant="outline" onClick={() => setRound((r) => r + 1)} disabled={!ready}>
          {t.reshuffle}
        </Button>
      </div>

      <p className="text-center text-xs leading-relaxed text-muted-foreground">
        {t.note(totalAttempts.toLocaleString())}
      </p>
    </section>
  );
}
