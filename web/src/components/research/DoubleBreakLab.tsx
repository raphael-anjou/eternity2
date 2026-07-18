import { useMemo, useState } from "react";
import { useT } from "@/i18n";
import { BoardSvg } from "@/components/board/BoardSvg";
import { decodeBucas, conflictEdges } from "@/lib/bucas";
import { RECORD_BOARDS } from "@/data/record-boards";

// REPLAY's central finding, on real boards. We take an actual record board, find
// its mismatched edges with the same scoring rule the engine uses, and count how
// many mismatches each broken cell carries. The cells carrying TWO (a "double
// break") are the ones a search that allows at most one break per cell can never
// place — which is why ordinary solvers saturate just below these boards. Nothing
// is hand-placed: the double-break cells are computed live from the board data.

// Build a per-cell mismatch count from the real conflict-edge list.
function doubleBreakCells(viewerParams: string): {
  width: number;
  height: number;
  cells: ReturnType<typeof decodeBucas>["cells"];
  broken: number[];
  doubles: number[];
} {
  const decoded = decodeBucas(viewerParams);
  const confl = conflictEdges(decoded);
  const perCell = new Map<number, Set<number>>();
  for (const [cell, side] of confl) {
    if (!perCell.has(cell)) perCell.set(cell, new Set());
    perCell.get(cell)?.add(side);
  }
  const broken: number[] = [];
  const doubles: number[] = [];
  for (const [cell, sides] of perCell) {
    broken.push(cell);
    if (sides.size >= 2) doubles.push(cell);
  }
  return { width: decoded.width, height: decoded.height, cells: decoded.cells, broken, doubles };
}

const BOARDS = RECORD_BOARDS.filter((b) => b.score >= 458);

const T = {
  en: {
    title: "Find the double-break cells on a real board",
    intro:
      "Pick a record board. We score it live, find every mismatched edge, and flag the cells that carry two mismatches at once. A solver that allows at most one break per cell — almost every solver — literally cannot place these cells, so it stalls a few points below. That is the move REPLAY had to add to walk the community's boards exactly.",
    boardLabel: "Board",
    score: (s: number) => `${s} / 480`,
    brokenLabel: "broken cells",
    doubleLabel: "double-break cells",
    legendDouble: "double-break cell (two mismatches)",
    legendBoard: "the rest of the board is perfectly matched",
    none: "This board has no double-break cells.",
    note: "The double-break cells are computed here from the board's own edges (the same matched-edge rule the engine and the Bucas leaderboard use) — not hand-placed. Open the board in the viewer to inspect every edge. On the community's strict-460 boards there are 4–5 such cells; REPLAY's prior-over-cost ordering plus a cost-2 break operator is what makes them reachable.",
    openCta: "Open this board in the viewer",
  },
  fr: {
    title: "Trouvez les cellules à double défaut sur un vrai plateau",
    intro:
      "Choisissez un plateau record. On le note en direct, on trouve chaque bord non apparié, et on signale les cellules qui portent deux défauts à la fois. Un solveur qui n'autorise qu'un défaut par cellule — presque tous — ne peut littéralement pas poser ces cellules, et plafonne donc quelques points en dessous. C'est le coup que REPLAY a dû ajouter pour rejouer exactement les plateaux de la communauté.",
    boardLabel: "Plateau",
    score: (s: number) => `${s} / 480`,
    brokenLabel: "cellules cassées",
    doubleLabel: "cellules à double défaut",
    legendDouble: "cellule à double défaut (deux défauts)",
    legendBoard: "le reste du plateau est parfaitement apparié",
    none: "Ce plateau n'a aucune cellule à double défaut.",
    note: "Les cellules à double défaut sont calculées ici à partir des bords du plateau (la même règle de bords appariés que le moteur et le classement Bucas) — pas placées à la main. Ouvrez le plateau dans le visualiseur pour inspecter chaque bord. Sur les plateaux strict-460 de la communauté, il y a 4 à 5 telles cellules ; l'ordre prior-over-cost de REPLAY plus un opérateur de défaut de coût 2 les rend atteignables.",
    openCta: "Ouvrir ce plateau dans le visualiseur",
  },
  es: {
    title: "Encuentra las celdas de doble defecto en un tablero real",
    intro:
      "Elige un tablero récord. Lo puntuamos en vivo, localizamos cada arista no coincidente y marcamos las celdas que arrastran dos defectos a la vez. Un solucionador que solo admite un defecto por celda —casi todos— es literalmente incapaz de colocar estas celdas, así que se estanca unos puntos por debajo. Ese es el movimiento que REPLAY tuvo que añadir para reproducir con exactitud los tableros de la comunidad.",
    boardLabel: "Tablero",
    score: (s: number) => `${s} / 480`,
    brokenLabel: "celdas rotas",
    doubleLabel: "celdas de doble defecto",
    legendDouble: "celda de doble defecto (dos defectos)",
    legendBoard: "el resto del tablero encaja a la perfección",
    none: "Este tablero no tiene celdas de doble defecto.",
    note: "Las celdas de doble defecto se calculan aquí a partir de las propias aristas del tablero (la misma regla de aristas coincidentes que usan el motor y la clasificación de Bucas), no se colocan a mano. Abre el tablero en el visor para inspeccionar cada arista. En los tableros strict-460 de la comunidad hay entre 4 y 5 de estas celdas; el orden prior-over-cost de REPLAY junto con un operador de defecto de coste 2 es lo que las hace alcanzables.",
    openCta: "Abrir este tablero en el visor",
  },
};

export function DoubleBreakLab() {
  const t = useT(T);
  const [idx, setIdx] = useState(0);
  const board = BOARDS[idx] ?? BOARDS[0];

  const data = useMemo(() => (board ? doubleBreakCells(board.viewerParams) : null), [board]);

  if (!board || !data) return null;

  return (
    <div className="space-y-4 rounded-lg border bg-card p-4">
      <div className="space-y-1">
        <h3 className="text-sm font-semibold">{t.title}</h3>
        <p className="text-xs leading-relaxed text-muted-foreground">{t.intro}</p>
      </div>

      <div className="flex flex-wrap gap-1.5">
        {BOARDS.map((b, i) => (
          <button
            key={b.id}
            onClick={() => setIdx(i)}
            className={
              "rounded-md border px-2 py-1 text-xs transition-colors " +
              (i === idx ? "border-foreground bg-foreground text-background" : "hover:bg-muted")
            }
          >
            {b.score}
          </button>
        ))}
      </div>

      <div className="flex flex-wrap items-start justify-center gap-6">
        <div className="w-full max-w-72">
          <BoardSvg
            width={data.width}
            height={data.height}
            cells={data.cells}
            highlight={data.doubles}
          />
        </div>
        <div className="space-y-2 text-sm">
          <div className="rounded-lg border p-3 text-center">
            <div className="text-2xl font-bold tabular-nums">{t.score(board.score)}</div>
            <div className="mt-1 space-y-0.5 text-[11px] text-muted-foreground">
              <div>
                <span className="font-medium text-foreground tabular-nums">{data.broken.length}</span>{" "}
                {t.brokenLabel}
              </div>
              <div>
                <span className="font-bold text-rose-500 tabular-nums">{data.doubles.length}</span>{" "}
                {t.doubleLabel}
              </div>
            </div>
          </div>
          {data.doubles.length === 0 && (
            <p className="text-xs text-muted-foreground">{t.none}</p>
          )}
        </div>
      </div>

      <div className="flex flex-wrap justify-center gap-x-4 gap-y-1 text-xs text-muted-foreground">
        <span className="flex items-center gap-1">
          <span className="inline-block h-2.5 w-2.5 rounded-sm bg-rose-500" />
          {t.legendDouble}
        </span>
        <span className="flex items-center gap-1">
          <span className="inline-block h-2.5 w-2.5 rounded-sm bg-muted" />
          {t.legendBoard}
        </span>
      </div>

      <p className="text-xs text-muted-foreground">{t.note}</p>
    </div>
  );
}
