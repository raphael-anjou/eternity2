import { useEffect, useMemo, useRef, useState } from "react";
import { useT } from "@/i18n";
import { useEngine } from "@/engine/useEngine";
import { getGeneratedSolvedPuzzleFramed } from "@/engine";
import type { Puzzle } from "@/lib/types";
import { rotateEdges } from "@/lib/types";
import { boardFromEngine, conflictEdges, scoreSummary } from "@/lib/bucas";
import { BoardSvg, CELL } from "@/components/board/BoardSvg";
import { Button } from "@/components/ui/button";
import { cn } from "@/lib/utils";
import { useRunWhileVisible } from "@/lib/useRunWhileVisible";
import { Lab } from "@/components/research/Lab";

// The crossover-conflict argument (groups.io msgs 1500/2591), made visible.
//
// Two PERFECT parents of the same engine-generated framed 8×8: parent A is
// the solved board; parent B is the same solution rotated 90° — a genuinely
// different perfect board using the same 64 pieces (edge matching is
// invariant under a global turn of a square board, and generated puzzles
// have no fixed hint). One-point crossover at cut k takes cells 0..k-1 from
// A and the rest from B, exactly the operator the 2007 threads debated.
//
// (Why 90° and not 180°? Under a 180° turn the duplicated cells and the
// missing pieces' home cells coincide exactly, so a repair step can quietly
// reconstruct parent A and fake a success. The 90° turn scatters them — the
// honest, generic case.)
//
// The pathology is deterministic: away from the trivial cuts the child
// duplicates some pieces and omits just as many — it is not a board. Yet it
// still *scores* almost perfectly (each half is internally solved), which is
// precisely how a fitness function gets fooled. The repair step (replace
// second occurrences with the missing pieces, greedy best rotation — the fix
// proposed in msg 1500) restores legality and collapses the score.
//
// Everything is measured by the same bucas-compatible scoring the viewer
// uses; nothing is faked. The sweep animation is gated by useRunWhileVisible.

const SIZE = 8;
const COLORS = 10;
const SEED = 11;
const N = SIZE * SIZE;

const T = {
  en: {
    title: "Two perfect parents, one broken child",
    intro:
      "Parent A is a solved board; parent B is the same solution turned a quarter turn — a different, equally perfect arrangement of the same 64 pieces. Drag the cut: the child takes the first k cells from A and the rest from B. Rose rings mark pieces the child now holds twice; just as many pieces are missing entirely.",
    parentA: "Parent A — solved",
    parentB: "Parent B — solved (turned 90°)",
    childNaive: "Naive child",
    childRepaired: "Repaired child",
    cut: (k: number) => `crossover cut: ${k} cells from A, ${N - k} from B`,
    duplicates: (d: number) =>
      d === 0
        ? "a clone of one parent — legal, but nothing was recombined"
        : `${d} duplicated piece${d === 1 ? "" : "s"}, ${d} missing — not a legal board`,
    naiveScore: "naive child (illegal)",
    repairedScore: "repaired child (legal)",
    edges: "matched edges",
    repair: "Repair duplicates",
    naive: "Show naive child",
    sweep: "Sweep",
    pause: "Pause",
    note: "The naive child scores almost perfectly — both halves are internally solved — but it is not a permutation of the piece set, so its fitness is a lie. Repair keeps first occurrences and greedily re-places the missing pieces (best rotation per cell): legality returns, the score collapses. That collapse is the crossover-operator problem of the 2007 threads.",
    loading: "Generating a solved board…",
  },
  fr: {
    title: "Deux parents parfaits, un enfant cassé",
    intro:
      "Le parent A est un plateau résolu ; le parent B est la même solution tournée d'un quart de tour — un arrangement différent, tout aussi parfait, des 64 mêmes pièces. Déplacez la coupure : l'enfant prend les k premières cases de A et le reste de B. Les anneaux roses marquent les pièces que l'enfant possède désormais en double ; autant de pièces manquent entièrement.",
    parentA: "Parent A — résolu",
    parentB: "Parent B — résolu (tourné de 90°)",
    childNaive: "Enfant naïf",
    childRepaired: "Enfant réparé",
    cut: (k: number) => `coupure : ${k} cases de A, ${N - k} de B`,
    duplicates: (d: number) =>
      d === 0
        ? "un clone d'un des parents — légal, mais rien n'a été recombiné"
        : `${d} pièce${d === 1 ? "" : "s"} en double, ${d} manquante${d === 1 ? "" : "s"} — ce n'est pas un plateau légal`,
    naiveScore: "enfant naïf (illégal)",
    repairedScore: "enfant réparé (légal)",
    edges: "arêtes appariées",
    repair: "Réparer les doublons",
    naive: "Voir l'enfant naïf",
    sweep: "Balayer",
    pause: "Pause",
    note: "L'enfant naïf a un score presque parfait — chaque moitié est résolue en interne — mais ce n'est pas une permutation du jeu de pièces : sa fitness est un mensonge. La réparation garde les premières occurrences et replace les pièces manquantes (meilleure rotation, gloutonnement) : la légalité revient, le score s'effondre. Cet effondrement, c'est le problème de l'opérateur de croisement des fils de 2007.",
    loading: "Génération d'un plateau résolu…",
  },
  es: {
    title: "Dos padres perfectos, un hijo roto",
    intro:
      "El padre A es un tablero resuelto; el padre B es la misma solución girada un cuarto de vuelta: una disposición distinta, igual de perfecta, de las mismas 64 piezas. Arrastra el corte: el hijo toma las primeras k celdas de A y el resto de B. Los anillos rosa marcan las piezas que el hijo tiene ahora por duplicado; otras tantas faltan por completo.",
    parentA: "Padre A — resuelto",
    parentB: "Padre B — resuelto (girado 90°)",
    childNaive: "Hijo ingenuo",
    childRepaired: "Hijo reparado",
    cut: (k: number) => `corte de cruce: ${k} celdas de A, ${N - k} de B`,
    duplicates: (d: number) =>
      d === 0
        ? "un clon de uno de los padres: legal, pero no se ha recombinado nada"
        : `${d} pieza${d === 1 ? "" : "s"} duplicada${d === 1 ? "" : "s"}, ${d} ausente${d === 1 ? "" : "s"}: no es un tablero legal`,
    naiveScore: "hijo ingenuo (ilegal)",
    repairedScore: "hijo reparado (legal)",
    edges: "aristas coincidentes",
    repair: "Reparar duplicados",
    naive: "Ver el hijo ingenuo",
    sweep: "Barrer",
    pause: "Pausar",
    note: "El hijo ingenuo obtiene una puntuación casi perfecta —cada mitad está resuelta internamente—, pero no es una permutación del conjunto de piezas: su aptitud es una mentira. La reparación conserva las primeras apariciones y vuelve a colocar las piezas ausentes (mejor rotación, de forma voraz): la legalidad regresa y la puntuación se desploma. Ese desplome es el problema del operador de cruce de los hilos de 2007.",
    loading: "Generando un tablero resuelto…",
  },
};

/** Parent B: the same solution turned 90° clockwise. The piece from A cell
 *  (r, c) lands at (c, S-1-r) rotated one quarter turn — so B cell (r, c)
 *  holds the piece whose A cell is (S-1-c, r). */
function parentBCell(pos: number): number {
  const r = Math.floor(pos / SIZE);
  const c = pos % SIZE;
  return ((SIZE - 1 - c) * SIZE + r) * 4 + 1;
}

/** Engine cells (pieceId*4+rot) of the naive one-point crossover child. */
function naiveChildCells(k: number): number[] {
  const cells: number[] = [];
  for (let pos = 0; pos < N; pos++) {
    cells.push(pos < k ? pos * 4 : parentBCell(pos));
  }
  return cells;
}

/** Cells of the child that hold a piece already used earlier (second copies).
 *  The A half is duplicate-free (pieces 0..k-1) and B is a permutation, so
 *  second copies are exactly the B-half cells whose piece id is below k. */
function duplicateBCells(k: number): number[] {
  const out: number[] = [];
  for (let pos = k; pos < N; pos++) {
    if (parentBCell(pos) >> 2 < k) out.push(pos);
  }
  return out;
}

/** Piece ids absent from the naive child. */
function missingPieces(k: number): number[] {
  const used = new Set<number>();
  for (const v of naiveChildCells(k)) used.add(v >> 2);
  const out: number[] = [];
  for (let p = 0; p < N; p++) if (!used.has(p)) out.push(p);
  return out;
}

/** Greedy deterministic repair: replace second copies with the missing
 *  pieces, best rotation per cell against already-decided neighbours. */
function repairedChildCells(puzzle: Puzzle, k: number): number[] {
  const cells = naiveChildCells(k);
  const dupCells = duplicateBCells(k);
  if (dupCells.length === 0) return cells;
  const missing = missingPieces(k);
  const open = new Set(dupCells);
  for (const pos of dupCells) cells[pos] = -1;
  for (const pos of dupCells) {
    let best = { piece: -1, rot: 0, score: -1 };
    for (const piece of missing) {
      const pieceEdges = puzzle.pieces[piece];
      if (!pieceEdges) continue;
      for (let rot = 0; rot < 4; rot++) {
        const e = rotateEdges(pieceEdges, rot);
        let s = 0;
        const x = pos % SIZE;
        const y = Math.floor(pos / SIZE);
        const neighbours: [number, number, number][] = [];
        if (y > 0) neighbours.push([pos - SIZE, 0, 2]);
        if (x < SIZE - 1) neighbours.push([pos + 1, 1, 3]);
        if (y < SIZE - 1) neighbours.push([pos + SIZE, 2, 0]);
        if (x > 0) neighbours.push([pos - 1, 3, 1]);
        for (const [nPos, mySide, theirSide] of neighbours) {
          if (open.has(nPos)) continue; // not decided yet
          const v = cells[nPos] ?? -1;
          if (v < 0) continue;
          const nPiece = puzzle.pieces[v >> 2];
          if (!nPiece) continue;
          const nEdges = rotateEdges(nPiece, v & 3);
          const mine = e[mySide as 0 | 1 | 2 | 3];
          if (mine === nEdges[theirSide as 0 | 1 | 2 | 3] && mine !== 0) s++;
        }
        if (s > best.score) best = { piece, rot, score: s };
      }
    }
    cells[pos] = best.piece * 4 + best.rot;
    missing.splice(missing.indexOf(best.piece), 1);
    open.delete(pos);
  }
  return cells;
}

export function CrossoverClashLab() {
  const t = useT(T);
  const ready = useEngine();
  const { ref, visible } = useRunWhileVisible();
  const [k, setK] = useState(12);
  const [repaired, setRepaired] = useState(false);
  const [sweeping, setSweeping] = useState(true);
  const dirRef = useRef(1);

  const puzzle = useMemo<Puzzle | null>(
    () => (ready ? getGeneratedSolvedPuzzleFramed(SIZE, COLORS, SEED, true) : null),
    [ready],
  );

  // Sweep the cut while visible; stop the moment the reader takes over.
  useEffect(() => {
    if (!visible || !sweeping || !puzzle) return;
    const id = setInterval(() => {
      setK((prev) => {
        let next = prev + dirRef.current;
        if (next > N) {
          dirRef.current = -1;
          next = N - 1;
        } else if (next < 0) {
          dirRef.current = 1;
          next = 1;
        }
        return next;
      });
    }, 160);
    return () => clearInterval(id);
  }, [visible, sweeping, puzzle]);

  const parentACells = useMemo(() => Array.from({ length: N }, (_, i) => i * 4), []);
  const parentBCells = useMemo(() => Array.from({ length: N }, (_, i) => parentBCell(i)), []);
  const childCells = useMemo(
    () => (puzzle ? (repaired ? repairedChildCells(puzzle, k) : naiveChildCells(k)) : []),
    [puzzle, k, repaired],
  );

  const parentABoard = useMemo(
    () => (puzzle ? boardFromEngine(puzzle, parentACells) : null),
    [puzzle, parentACells],
  );
  const parentBBoard = useMemo(
    () => (puzzle ? boardFromEngine(puzzle, parentBCells) : null),
    [puzzle, parentBCells],
  );
  const childBoard = useMemo(
    () => (puzzle ? boardFromEngine(puzzle, childCells) : null),
    [puzzle, childCells],
  );

  // Both copies of every duplicated piece, for the rings on the naive child.
  const dupCellSet = useMemo(() => {
    const s = new Set<number>();
    for (const pos of duplicateBCells(k)) {
      s.add(pos); // second copy (B half)
      s.add(parentBCell(pos) >> 2); // first copy: piece p sits at A cell p
    }
    return s;
  }, [k]);

  const naiveSummary = useMemo(
    () => (puzzle ? scoreSummary(boardFromEngine(puzzle, naiveChildCells(k))) : null),
    [puzzle, k],
  );
  const repairedSummary = useMemo(
    () => (puzzle ? scoreSummary(boardFromEngine(puzzle, repairedChildCells(puzzle, k))) : null),
    [puzzle, k],
  );

  if (!puzzle || !parentABoard || !parentBBoard || !childBoard || !naiveSummary || !repairedSummary) {
    return (
      <div className="flex h-64 items-center justify-center rounded-lg border text-sm text-muted-foreground">
        {t.loading}
      </div>
    );
  }

  const dupCount = duplicateBCells(k).length;
  const conflicts = conflictEdges(childBoard);

  const overlay = repaired ? undefined : (
    <g style={{ pointerEvents: "none" }}>
      {[...dupCellSet].map((pos) => (
        <circle
          key={pos}
          cx={(pos % SIZE) * CELL + CELL / 2}
          cy={Math.floor(pos / SIZE) * CELL + CELL / 2}
          r={CELL * 0.34}
          fill="none"
          stroke="#fb7185"
          strokeWidth={18}
          opacity={0.9}
        />
      ))}
    </g>
  );

  const takeOver = (value: number) => {
    setSweeping(false);
    setK(value);
  };

  return (
    <Lab ref={ref} title={t.title} intro={t.intro} note={t.note}>
      <div className="flex flex-wrap items-start justify-center gap-6">
        <div className="flex w-full max-w-44 flex-col gap-3 sm:w-44">
          <div className="space-y-1">
            <BoardSvg width={SIZE} height={SIZE} cells={parentABoard.cells} />
            <p className="text-center text-[11px] text-muted-foreground">{t.parentA}</p>
          </div>
          <div className="space-y-1">
            <BoardSvg width={SIZE} height={SIZE} cells={parentBBoard.cells} />
            <p className="text-center text-[11px] text-muted-foreground">{t.parentB}</p>
          </div>
        </div>

        <div className="w-full max-w-80 space-y-2">
          <BoardSvg
            width={SIZE}
            height={SIZE}
            cells={childBoard.cells}
            conflicts={conflicts}
            overlay={overlay}
          />
          <p className="text-center text-[11px] text-muted-foreground">
            {repaired ? t.childRepaired : t.childNaive} — {t.cut(k)}
          </p>
        </div>

        <div className="min-w-56 max-w-sm flex-1 space-y-3">
          <input
            type="range"
            min={0}
            max={N}
            step={1}
            value={k}
            onChange={(e) => takeOver(Number(e.target.value))}
            className="w-full"
            aria-label={t.cut(k)}
          />

          <div
            className={cn(
              "rounded-md border px-3 py-2 text-center",
              dupCount === 0 ? "border-emerald-300 bg-emerald-500/10" : "border-rose-300 bg-rose-500/10",
            )}
          >
            <div className="text-sm font-semibold">{t.duplicates(dupCount)}</div>
          </div>

          <div className="grid grid-cols-2 gap-2 text-center">
            <div
              className={cn(
                "rounded-md border px-2 py-2",
                !repaired && "ring-2 ring-rose-300",
              )}
            >
              <div className="text-xl font-bold tabular-nums">
                {naiveSummary.score}/{naiveSummary.max}
              </div>
              <div className="text-[11px] leading-snug text-muted-foreground">{t.naiveScore}</div>
            </div>
            <div
              className={cn(
                "rounded-md border px-2 py-2",
                repaired && "ring-2 ring-emerald-300",
              )}
            >
              <div className="text-xl font-bold tabular-nums">
                {repairedSummary.score}/{repairedSummary.max}
              </div>
              <div className="text-[11px] leading-snug text-muted-foreground">{t.repairedScore}</div>
            </div>
          </div>

          <div className="flex flex-wrap justify-center gap-2">
            <Button size="sm" variant={repaired ? "default" : "outline"} onClick={() => setRepaired((r) => !r)}>
              {repaired ? t.naive : t.repair}
            </Button>
            <Button
              size="sm"
              variant="outline"
              onClick={() => setSweeping((s) => !s)}
            >
              {sweeping ? t.pause : t.sweep}
            </Button>
          </div>
        </div>
      </div>
    </Lab>
  );
}
