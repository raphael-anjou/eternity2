import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { useT } from "@/i18n";
import { useEngine } from "@/engine/useEngine";
import { createSolver, getGeneratedPuzzle, getPath } from "@/engine";
import type { Puzzle } from "@/lib/types";
import { boardFromEngine } from "@/lib/bucas";
import { countCandidates } from "@/lib/piece-fit";
import { BoardSvg } from "@/components/board/BoardSvg";
import { PieceSvg } from "@/components/board/PieceSvg";
import { Button } from "@/components/ui/button";
import { cn } from "@/lib/utils";

// "No forced moves," made live. We run the real engine a few placements into a
// small puzzle, then point at the next empty cell and ask the real legality test
// how many distinct pieces could go there. The answer is essentially never 1 —
// the green set in the supply stays large. A solver would love a forced move (a
// cell with exactly one candidate, free to place); Eternity II almost never
// grants one. Step forward and watch the candidate count refuse to collapse.

interface Built {
  puzzle: Puzzle;
  board: number[];
  frontier: number;
  candidates: Set<number>;
}

const T = {
  en: {
    title: "Count the choices at the next cell — live",
    intro:
      "We place a few pieces with the real engine, then point at the next empty cell (red) and count, with the real legality test, how many unused pieces could legally sit there. A 'forced move' would be a count of exactly 1 — free to place, no branching. Step forward: the count almost never drops to 1.",
    frontierCount: (n: number) => `${n} pieces fit the red cell`,
    forced: "a forced move (count = 1)",
    notForced: "no forced move here — the search must branch",
    step: "Place next",
    reset: "Reset",
    newPuzzle: "New puzzle",
    supply: "Piece supply",
    supplyHint: "Green = legal on the red cell now. Dimmed = already placed.",
    loading: "Loading the engine…",
    note: "On the real 16×16 the same holds across the whole interior: every piece has between 73 and 137 legal right-hand neighbours and not one cell is ever pinned to a single option. Lots of local freedom, almost no global consistency — which is exactly what makes the search so wide. Counts here are the live engine's legality test on a freshly generated puzzle.",
  },
  fr: {
    title: "Comptez les choix à la case suivante — en direct",
    intro:
      "On pose quelques pièces avec le vrai moteur, puis on pointe la prochaine case vide (rouge) et on compte, avec le vrai test de légalité, combien de pièces inutilisées pourraient y aller. Un « coup forcé » serait un compte de 1 exactement — posable sans embranchement. Avancez : le compte ne descend presque jamais à 1.",
    frontierCount: (n: number) => `${n} pièces conviennent à la case rouge`,
    forced: "un coup forcé (compte = 1)",
    notForced: "aucun coup forcé ici — la recherche doit se ramifier",
    step: "Poser la suivante",
    reset: "Réinitialiser",
    newPuzzle: "Nouveau tirage",
    supply: "Réserve de pièces",
    supplyHint: "Vert = légal sur la case rouge. Estompé = déjà posée.",
    loading: "Chargement du moteur…",
    note: "Sur le vrai 16×16, c'est pareil sur tout l'intérieur : chaque pièce a entre 73 et 137 voisines droites légales et aucune case n'est jamais réduite à une seule option. Beaucoup de liberté locale, presque aucune cohérence globale — ce qui rend la recherche si large. Les comptes ici sont le test de légalité du moteur en direct sur un puzzle fraîchement généré.",
  },
};

export function ForcedMovesLab() {
  const t = useT(T);
  const engineReady = useEngine();
  const [seed, setSeed] = useState(5);
  const [placed, setPlaced] = useState(8); // how many cells filled
  const [built, setBuilt] = useState<Built | null>(null);
  const seedRef = useRef(seed);

  const SIZE = 6;
  const COLORS = 5;

  const build = useCallback(
    (fill: number) => {
      if (!engineReady) return;
      const puzzle = getGeneratedPuzzle(SIZE, COLORS, seedRef.current);
      const path = getPath("row-major", puzzle.width, puzzle.height, 0);
      const solver = createSolver(puzzle, path, { useHints: false });
      // step until `fill` cells are placed (or it can't anymore)
      let report = solver.report();
      for (let guard = 0; guard < 5000 && report.placed < fill; guard++) {
        report = solver.step(1);
        if (report.status !== "running") break;
      }
      const board = Array.from(solver.board());
      solver.free();
      const cells = boardFromEngine(puzzle, board).cells;
      const used = new Set(board.filter((v) => v >= 0).map((v) => v >> 2));
      const frontier = report.placed < puzzle.width * puzzle.height ? path[report.placed] ?? 0 : 0;
      const candidates = countCandidates(
        puzzle.pieces,
        frontier,
        cells,
        used,
        puzzle.width,
        puzzle.height,
      );
      setBuilt({ puzzle, board, frontier, candidates });
    },
    [engineReady],
  );

  useEffect(() => {
    seedRef.current = seed;
    build(placed);
  }, [build, seed, placed]);

  const cells = useMemo(
    () => (built ? boardFromEngine(built.puzzle, built.board).cells : null),
    [built],
  );
  const used = useMemo(
    () => (built ? new Set(built.board.filter((v) => v >= 0).map((v) => v >> 2)) : new Set<number>()),
    [built],
  );

  if (!engineReady || !built || !cells) {
    return (
      <div className="flex h-64 items-center justify-center rounded-lg border text-sm text-muted-foreground">
        {t.loading}
      </div>
    );
  }

  const count = built.candidates.size;
  const isForced = count === 1;

  return (
    <div className="space-y-4 rounded-lg border bg-card p-4">
      <div className="space-y-1">
        <h3 className="text-sm font-semibold">{t.title}</h3>
        <p className="text-xs leading-relaxed text-muted-foreground">{t.intro}</p>
      </div>

      <div className="flex flex-wrap items-start justify-center gap-6">
        <div className="w-full max-w-56 space-y-2">
          <BoardSvg
            width={built.puzzle.width}
            height={built.puzzle.height}
            cells={cells}
            highlight={[built.frontier]}
          />
          <div
            className={cn(
              "rounded-md border px-3 py-2 text-center text-sm",
              isForced ? "border-amber-300 bg-amber-500/10" : "border-emerald-300 bg-emerald-500/10",
            )}
          >
            <div className="text-lg font-bold tabular-nums">{t.frontierCount(count)}</div>
            <div className="text-[11px] text-muted-foreground">
              {isForced ? t.forced : t.notForced}
            </div>
          </div>
        </div>

        <div className="space-y-1.5">
          <p className="text-xs font-medium">{t.supply}</p>
          <div
            className="grid w-fit gap-1"
            style={{ gridTemplateColumns: "repeat(7, max-content)" }}
          >
            {built.puzzle.pieces.map((piece, i) => (
              <div
                key={i}
                className={cn(
                  "rounded p-0.5 transition-all",
                  used.has(i) && "opacity-20",
                  built.candidates.has(i) && "ring-2 ring-emerald-500",
                )}
              >
                <PieceSvg edges={piece} size={20} />
              </div>
            ))}
          </div>
          <p className="max-w-56 text-xs text-muted-foreground">{t.supplyHint}</p>
        </div>
      </div>

      <div className="flex flex-wrap justify-center gap-2">
        <Button
          size="sm"
          onClick={() => setPlaced((n) => Math.min(SIZE * SIZE - 1, n + 1))}
        >
          {t.step}
        </Button>
        <Button size="sm" variant="outline" onClick={() => setPlaced(8)}>
          {t.reset}
        </Button>
        <Button
          size="sm"
          variant="outline"
          onClick={() => {
            setPlaced(8);
            setSeed((s) => (s * 7 + 13) % 100000);
          }}
        >
          {t.newPuzzle}
        </Button>
      </div>

      <p className="text-xs text-muted-foreground">{t.note}</p>
    </div>
  );
}
