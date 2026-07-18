import { useMemo, useState } from "react";
import { useT } from "@/i18n";
import { useEngine } from "@/engine/useEngine";
import { getGeneratedSolvedPuzzle } from "@/engine";
import { boardFromEngine, conflictEdges, scoreSummary } from "@/lib/bucas";
import type { Puzzle } from "@/lib/types";
import { BoardSvg } from "@/components/board/BoardSvg";
import { Button } from "@/components/ui/button";

// The one term every research page leans on — "matched edges out of 480" — made
// concrete on a real, engine-generated board. We generate a genuinely solvable
// 4x4 puzzle, lay its pieces in the solution (every interior edge matched), then
// let the reader rotate a piece. Rotating breaks the edges around it; the live
// score and the red conflict marks show exactly what a "mismatch" is and how the
// score is just a count of matched interior edges. Nothing is faked: the score
// and conflicts are computed by the same code the viewer and solver use.

const SIZE = 4;
const COLORS = 4;
const SEED = 7;

const T = {
  en: {
    title: "First: what does \"matched edges\" mean?",
    intro:
      "Every score on this site is a count of matched interior edges. Here is a real, solvable 4×4 puzzle laid out in its solution — every touching pair of colours agrees, so the score is the maximum. Click a piece to rotate it and watch matched edges break.",
    score: "matched edges",
    rotateHint: "Click any piece to rotate it 90°.",
    reset: "Reset to the solution",
    perfect: "Perfect — every interior edge matches. This is what 480/480 would be on the full 16×16.",
    broken: (n: number) =>
      `${n} ${n === 1 ? "edge is" : "edges are"} now mismatched (marked red). The score dropped by exactly that many. On the real puzzle the best anyone has reached is 470 — ten short of perfect.`,
    loading: "Generating a solvable board…",
  },
  fr: {
    title: "D'abord : que veut dire « bords appariés » ?",
    intro:
      "Chaque score de ce site est un décompte de bords intérieurs appariés. Voici un vrai puzzle 4×4 soluble, disposé dans sa solution — chaque paire de couleurs qui se touche concorde, donc le score est maximal. Cliquez sur une pièce pour la tourner et regardez des bords appariés se rompre.",
    score: "bords appariés",
    rotateHint: "Cliquez une pièce pour la tourner de 90°.",
    reset: "Revenir à la solution",
    perfect: "Parfait — chaque bord intérieur concorde. C'est ce que serait 480/480 sur le 16×16 complet.",
    broken: (n: number) =>
      `${n} bord${n === 1 ? "" : "s"} ${n === 1 ? "est" : "sont"} désormais non apparié${n === 1 ? "" : "s"} (marqué${n === 1 ? "" : "s"} en rouge). Le score a chuté d'exactement autant. Sur le vrai puzzle, le meilleur jamais atteint est 470 — dix de moins que la perfection.`,
    loading: "Génération d'un plateau soluble…",
  },
  es: {
    title: "Primero: ¿qué significa «aristas coincidentes»?",
    intro:
      "Cada puntuación de este sitio es un recuento de aristas interiores coincidentes. Aquí tienes un puzzle 4×4 real y resoluble, dispuesto en su solución: cada par de colores en contacto concuerda, por lo que la puntuación es la máxima. Haz clic en una pieza para girarla y observa cómo se rompen las aristas coincidentes.",
    score: "aristas coincidentes",
    rotateHint: "Haz clic en cualquier pieza para girarla 90°.",
    reset: "Volver a la solución",
    perfect: "Perfecto: cada arista interior concuerda. Esto es lo que sería 480/480 en el 16×16 completo.",
    broken: (n: number) =>
      `${n} ${n === 1 ? "arista está ahora descuadrada" : "aristas están ahora descuadradas"} (marcada${n === 1 ? "" : "s"} en rojo). La puntuación bajó exactamente esa cantidad. En el puzzle real, lo mejor que nadie ha alcanzado es 470: diez por debajo de la perfección.`,
    loading: "Generando un tablero resoluble…",
  },
};

export function ScoringPrimer() {
  const t = useT(T);
  const ready = useEngine();
  // Per-cell rotation offset added to the identity solution.
  const [rots, setRots] = useState<number[]>(() => new Array<number>(SIZE * SIZE).fill(0));

  // The solved puzzle is a pure function of the engine being ready — derive it
  // rather than holding it in state and setting it from an effect.
  const puzzle = useMemo<Puzzle | null>(
    () => (ready ? getGeneratedSolvedPuzzle(SIZE, COLORS, SEED) : null),
    [ready],
  );

  const cells = useMemo(
    () => (puzzle ? rots.map((r, i) => i * 4 + (r % 4)) : []),
    [puzzle, rots],
  );

  const board = useMemo(
    () => (puzzle ? boardFromEngine(puzzle, cells) : null),
    [puzzle, cells],
  );

  const summary = board ? scoreSummary(board) : null;
  const conflicts = board ? conflictEdges(board) : [];
  const broken = summary ? summary.max - summary.score : 0;

  const rotate = (i: number) =>
    setRots((prev) => {
      const next = [...prev];
      next[i] = ((next[i] ?? 0) + 1) % 4;
      return next;
    });

  return (
    <section className="mx-auto max-w-3xl space-y-4 rounded-xl border bg-muted/20 p-5">
      <div>
        <h2 className="text-xl font-semibold tracking-tight">{t.title}</h2>
        <p className="mt-1 text-sm text-muted-foreground">{t.intro}</p>
      </div>

      {board && summary ? (
        <div className="grid gap-5 sm:grid-cols-[minmax(0,260px)_1fr] sm:items-center">
          <BoardSvg
            width={SIZE}
            height={SIZE}
            cells={board.cells}
            conflicts={conflicts}
            onCellClick={(i) => rotate(i)}
            className="max-w-[260px] cursor-pointer"
          />
          <div className="space-y-3">
            <div className="flex items-baseline gap-2">
              <span
                className={`text-3xl font-bold tabular-nums ${broken === 0 ? "text-emerald-600 dark:text-emerald-400" : "text-foreground"}`}
              >
                {summary.score}
              </span>
              <span className="text-lg text-muted-foreground">/ {summary.max}</span>
              <span className="text-sm text-muted-foreground">{t.score}</span>
            </div>
            <p className="text-sm leading-relaxed text-muted-foreground">
              {broken === 0 ? t.perfect : t.broken(broken)}
            </p>
            <div className="flex items-center gap-3">
              <Button size="sm" variant="outline" onClick={() => setRots(new Array<number>(SIZE * SIZE).fill(0))} disabled={broken === 0}>
                {t.reset}
              </Button>
              <span className="text-xs text-muted-foreground">{t.rotateHint}</span>
            </div>
          </div>
        </div>
      ) : (
        <div className="flex h-40 items-center justify-center text-sm text-muted-foreground">
          {t.loading}
        </div>
      )}
    </section>
  );
}
