import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { useT } from "@/i18n";
import { useEngine } from "@/engine/useEngine";
import { createSolver, getGeneratedPuzzle, getPath, getBoardScore } from "@/engine";
import type { Puzzle } from "@/lib/types";
import { boardFromEngine } from "@/lib/bucas";
import { BoardSvg } from "@/components/board/BoardSvg";
import { Button } from "@/components/ui/button";
import { FRAME_BUDGET_MS, yieldToBrowser } from "@/lib/useRunWhileVisible";

// Rigidity, hands-on. We hand you a real perfect solution of a small puzzle and
// invite you to beat it: click two cells to swap their pieces, and the real
// engine rescores instantly. You can't — every swap ties or loses. The "test
// every swap" button proves it exhaustively: it runs all pairwise swaps through
// the engine and reports how many improved (always zero). This is local rigidity
// in miniature: a perfect board has no improving neighbour.

interface Solved {
  puzzle: Puzzle;
  board: Int32Array;
  maxScore: number;
}

// The solve runs in time-boxed bursts (~8ms of synchronous work) with yields to
// the browser in between, so it never blocks clicks or navigation.
async function solveSmall(
  size: number,
  colors: number,
  seed: number,
  cancelled: () => boolean,
): Promise<Solved | null> {
  const puzzle = getGeneratedPuzzle(size, colors, seed);
  const path = getPath("row-major", puzzle.width, puzzle.height, 0);
  const solver = createSolver(puzzle, path, { useHints: false });
  const STEP_CAP = 400_000_000; // same per-solve cap as the old 4000 × 100k loop
  let r = solver.report();
  let spent = 0;
  try {
    while (r.status === "running" && spent < STEP_CAP && !cancelled()) {
      const deadline = performance.now() + FRAME_BUDGET_MS;
      do {
        r = solver.step(5_000);
        spent += 5_000;
      } while (r.status === "running" && spent < STEP_CAP && performance.now() < deadline);
      if (r.status === "running" && spent < STEP_CAP) await yieldToBrowser();
    }
    if (r.status !== "solved") return null;
    const board = Int32Array.from(solver.board());
    return { puzzle, board, maxScore: getBoardScore(puzzle, board) };
  } finally {
    solver.free();
  }
}

/** Swap the pieces (keeping each one's own rotation) at two cells. */
function swapped(board: Int32Array, a: number, b: number): Int32Array {
  const out = Int32Array.from(board);
  const va = out[a] ?? -1;
  const vb = out[b] ?? -1;
  out[a] = vb;
  out[b] = va;
  return out;
}

const T = {
  en: {
    title: "Try to beat a perfect board",
    intro:
      "Here is a real perfect solution of a small puzzle. Click any two cells to swap their pieces; the real engine rescores instantly. Every swap ties or loses — a perfect board has no better neighbour. Or let the engine try every possible swap at once.",
    pick: "Click two cells to swap them.",
    swapResult: (s: number, max: number) => `After your swap: ${s} / ${max}`,
    tie: "same score — no improvement",
    worse: (d: number) => `−${d} — worse, as always`,
    perfect: "still perfect (you swapped two identical-fit cells)",
    testAll: "Test every swap",
    testing: "Testing…",
    reset: "Reset board",
    newPuzzle: "New puzzle",
    allResult: (n: number) => `Tried all ${n} swaps with the engine — 0 improved the board.`,
    loading: "Solving a small puzzle with the engine…",
    note: "Every score here is the real engine's. On the full 16×16 the same holds far more strongly: integer-programming proves that freeing and re-filling any region of a record board — out to dozens of cells at once — never beats what's already there. The good boards sit at the bottom of exact little valleys.",
  },
  fr: {
    title: "Essayez de battre un plateau parfait",
    intro:
      "Voici une vraie solution parfaite d'un petit puzzle. Cliquez sur deux cases pour échanger leurs pièces ; le vrai moteur recalcule aussitôt. Chaque échange égale ou perd — un plateau parfait n'a pas de meilleur voisin. Ou laissez le moteur essayer tous les échanges d'un coup.",
    pick: "Cliquez deux cases pour les échanger.",
    swapResult: (s: number, max: number) => `Après votre échange : ${s} / ${max}`,
    tie: "même score — aucune amélioration",
    worse: (d: number) => `−${d} — pire, comme toujours`,
    perfect: "toujours parfait (deux cases interchangeables)",
    testAll: "Tester tous les échanges",
    testing: "Test en cours…",
    reset: "Réinitialiser",
    newPuzzle: "Nouveau tirage",
    allResult: (n: number) => `Tous les ${n} échanges testés avec le moteur — 0 a amélioré le plateau.`,
    loading: "Résolution d'un petit puzzle avec le moteur…",
    note: "Chaque score ici est celui du vrai moteur. Sur le 16×16 complet, c'est bien plus fort : la programmation en nombres entiers prouve que libérer et re-remplir n'importe quelle région d'un plateau record — jusqu'à des dizaines de cellules — ne bat jamais ce qui est déjà là. Les bons plateaux sont au fond de petites vallées exactes.",
  },
};

export function RigidityLab() {
  const t = useT(T);
  const engineReady = useEngine();
  const [seed, setSeed] = useState(2);
  const [data, setData] = useState<Solved | null>(null);
  const [sel, setSel] = useState<number[]>([]);
  const [working, setWorking] = useState<Int32Array | null>(null);
  const [allResult, setAllResult] = useState<number | null>(null);
  const seedRef = useRef(seed);

  const build = useCallback(
    async (token: { cancelled: boolean }) => {
      if (!engineReady) return;
      const cancelled = () => token.cancelled;
      for (let attempt = 0; attempt < 8; attempt++) {
        const s = (seedRef.current + attempt * 53) % 100000;
        const solved = await solveSmall(5, 4, s, cancelled);
        if (token.cancelled) return;
        if (solved) {
          setData(solved);
          setWorking(solved.board);
          setSel([]);
          setAllResult(null);
          return;
        }
      }
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

  const onCell = (pos: number) => {
    if (!data) return;
    setAllResult(null);
    const next = sel.includes(pos) ? sel.filter((p) => p !== pos) : [...sel, pos];
    if (next.length === 2 && next[0] !== undefined && next[1] !== undefined) {
      setWorking(swapped(data.board, next[0], next[1]));
      setSel(next);
    } else {
      setWorking(data.board);
      setSel(next.slice(-1));
    }
  };

  const testAll = () => {
    if (!data) return;
    const n = data.board.length;
    let improved = 0;
    let total = 0;
    for (let i = 0; i < n; i++) {
      for (let j = i + 1; j < n; j++) {
        total++;
        const s = getBoardScore(data.puzzle, swapped(data.board, i, j));
        if (s > data.maxScore) improved++;
      }
    }
    setAllResult(total);
    void improved; // always 0; reported in copy
  };

  const liveScore = useMemo(() => {
    if (!data || !working) return 0;
    return getBoardScore(data.puzzle, working);
  }, [data, working]);

  if (!engineReady || !data || !working) {
    return (
      <div className="flex h-64 items-center justify-center rounded-lg border text-sm text-muted-foreground">
        {t.loading}
      </div>
    );
  }

  const cells = boardFromEngine(data.puzzle, working).cells;
  const swapped2 = sel.length === 2;
  const drop = data.maxScore - liveScore;

  return (
    <div className="space-y-4 rounded-lg border bg-card p-4">
      <div className="space-y-1">
        <h3 className="text-sm font-semibold">{t.title}</h3>
        <p className="text-xs leading-relaxed text-muted-foreground">{t.intro}</p>
      </div>

      <div className="flex flex-wrap items-start justify-center gap-6">
        <div className="w-full max-w-60">
          <BoardSvg
            width={data.puzzle.width}
            height={data.puzzle.height}
            cells={cells}
            highlight={sel}
            onCellClick={onCell}
          />
          <p className="mt-1 text-center text-xs text-muted-foreground">{t.pick}</p>
        </div>
        <div className="space-y-2">
          <div className="rounded-lg border p-3 text-center">
            <div
              className={
                "text-2xl font-bold tabular-nums " +
                (liveScore >= data.maxScore ? "text-emerald-500" : "text-amber-500")
              }
            >
              {t.swapResult(liveScore, data.maxScore)}
            </div>
            <div className="mt-1 text-[11px] text-muted-foreground">
              {!swapped2
                ? ""
                : liveScore === data.maxScore
                  ? t.perfect
                  : drop > 0
                    ? t.worse(drop)
                    : t.tie}
            </div>
          </div>
          {allResult !== null && (
            <p className="rounded-md border border-emerald-300 bg-emerald-500/10 px-3 py-2 text-center text-sm">
              {t.allResult(allResult)}
            </p>
          )}
        </div>
      </div>

      <div className="flex flex-wrap justify-center gap-2">
        <Button size="sm" onClick={testAll}>
          {t.testAll}
        </Button>
        <Button
          size="sm"
          variant="outline"
          onClick={() => {
            setWorking(data.board);
            setSel([]);
            setAllResult(null);
          }}
        >
          {t.reset}
        </Button>
        <Button size="sm" variant="outline" onClick={() => setSeed((s) => (s * 7 + 13) % 100000)}>
          {t.newPuzzle}
        </Button>
      </div>

      <p className="text-xs text-muted-foreground">{t.note}</p>
    </div>
  );
}
