import { useCallback, useEffect, useMemo, useState } from "react";
import { useT } from "@/i18n";
import { useEngine } from "@/engine/useEngine";
import { getGeneratedSolvedPuzzleFramed } from "@/engine";
import type { Puzzle } from "@/lib/types";
import { rotateEdges } from "@/lib/types";
import type { Edges } from "@/lib/bucas";
import { ns1Deficit } from "@/lib/ns1-deficit";
import { BoardSvg } from "@/components/board/BoardSvg";
import { motifFor } from "@/lib/motifs";
import { Button } from "@/components/ui/button";
import { cn } from "@/lib/utils";

// NS-1 deficit, made live. A framed solved board starts balanced (Δ = 0). Click
// a border cell to lift its piece: the border↔interior interface loses colours
// on one side and the deficit rises — you can watch exactly which colours fall
// out of balance. The "swap two border pieces" button shows the catch: a swap
// permutes the colours without changing either multiset, so Δ stays 0 — NS-1 is
// blind to it. This is the vault's "necessary-but-loose" point, made visible.

const SIZE = 8;
const COLORS = 6;

const T = {
  en: {
    title: "Watch the border balance — live",
    intro:
      "A solved board is balanced: every colour the border shows the interior is a colour the interior shows back (Δ = 0). Lift a border piece and the balance breaks — the deficit counts the interface edges now mismatched. Try the swap button too: swapping two border pieces keeps Δ at 0, because it shuffles colours without changing the multiset. That blind spot is exactly why NS-1 only rejects bad boards, never confirms good ones.",
    deficit: "deficit Δ",
    balanced: "balanced — a solution is still possible",
    broken: (d: number) => `${d} interface edge${d === 1 ? "" : "s"} out of balance`,
    swapInvisible: "swap kept Δ = 0 — NS-1 can't see a permutation",
    lift: "Click a border cell to lift its piece.",
    swap: "Swap two border pieces",
    reset: "Reset",
    newPuzzle: "New puzzle",
    aLabel: "border → interior (A)",
    bLabel: "interior → border (B)",
    loading: "Loading the engine…",
    note: "On the real 16×16 this is a genuine necessary condition: all four known 480-boards satisfy A = B exactly. As a solver check it prunes 10–28% of deep nodes — but ≥85% of a near-solution's mismatches are interior-to-interior, which this invariant never sees. Computed live from the engine's board.",
  },
  fr: {
    title: "Observez l'équilibre du bord — en direct",
    intro:
      "Un plateau résolu est équilibré : chaque couleur que le bord montre à l'intérieur est une couleur que l'intérieur lui renvoie (Δ = 0). Retirez une pièce de bord et l'équilibre se rompt — le déficit compte les arêtes d'interface désormais déséquilibrées. Essayez aussi le bouton d'échange : permuter deux pièces de bord laisse Δ à 0, car cela mélange les couleurs sans changer le multiset. Cet angle mort explique pourquoi NS-1 ne rejette que les mauvais plateaux, sans jamais confirmer les bons.",
    deficit: "déficit Δ",
    balanced: "équilibré — une solution reste possible",
    broken: (d: number) => `${d} arête${d === 1 ? "" : "s"} d'interface déséquilibrée${d === 1 ? "" : "s"}`,
    swapInvisible: "l'échange a gardé Δ = 0 — NS-1 ne voit pas une permutation",
    lift: "Cliquez sur une case de bord pour en retirer la pièce.",
    swap: "Échanger deux pièces de bord",
    reset: "Réinitialiser",
    newPuzzle: "Nouveau tirage",
    aLabel: "bord → intérieur (A)",
    bLabel: "intérieur → bord (B)",
    loading: "Chargement du moteur…",
    note: "Sur le vrai 16×16, c'est une vraie condition nécessaire : les quatre plateaux 480 connus vérifient A = B exactement. Comme test de solveur, cela élague 10 à 28 % des nœuds profonds — mais ≥85 % des erreurs d'une quasi-solution sont intérieur-à-intérieur, que cet invariant ne voit jamais. Calculé en direct depuis le plateau du moteur.",
  },
  es: {
    title: "Observa el equilibrio del borde — en directo",
    intro:
      "Un tablero resuelto está equilibrado: cada color que el borde muestra al interior es un color que el interior le devuelve (Δ = 0). Levanta una pieza de borde y el equilibrio se rompe — el déficit cuenta las aristas de interfaz que ahora no coinciden. Prueba también el botón de intercambio: intercambiar dos piezas de borde mantiene Δ en 0, porque baraja los colores sin alterar el multiconjunto. Ese punto ciego es justamente la razón por la que NS-1 solo rechaza tableros malos, sin confirmar nunca los buenos.",
    deficit: "déficit Δ",
    balanced: "equilibrado — todavía es posible una solución",
    broken: (d: number) => `${d} arista${d === 1 ? "" : "s"} de interfaz desequilibrada${d === 1 ? "" : "s"}`,
    swapInvisible: "el intercambio mantuvo Δ = 0 — NS-1 no ve una permutación",
    lift: "Haz clic en una celda de borde para levantar su pieza.",
    swap: "Intercambiar dos piezas de borde",
    reset: "Reiniciar",
    newPuzzle: "Nuevo puzzle",
    aLabel: "borde → interior (A)",
    bLabel: "interior → borde (B)",
    loading: "Cargando el motor…",
    note: "En el 16×16 real es una verdadera condición necesaria: los cuatro tableros 480 conocidos cumplen A = B exactamente. Como comprobación del solucionador poda entre el 10 y el 28 % de los nodos profundos — pero ≥85 % de los errores de una casi-solución son de interior a interior, algo que este invariante nunca ve. Calculado en directo a partir del tablero del motor.",
  },
};

/** The board's own motif background for this colour, so the balance-table
 *  swatches match exactly what the board shows rather than an ad-hoc ramp. */
function swatch(color: number): string {
  return motifFor(color).bg;
}

interface Built {
  puzzle: Puzzle;
  /** cell -> piece id placed there (solved order = identity), or -1 if lifted. */
  placement: number[];
}

export function Ns1Lab() {
  const t = useT(T);
  const engineReady = useEngine();
  const [seed, setSeed] = useState(1);
  const [built, setBuilt] = useState<Built | null>(null);
  const [lifted, setLifted] = useState<Set<number>>(new Set());
  const [swapNote, setSwapNote] = useState(false);

  const build = useCallback(() => {
    if (!engineReady) return;
    const puzzle = getGeneratedSolvedPuzzleFramed(SIZE, COLORS, seed, true);
    // Solved order: piece i sits at cell i, rotation 0. Start every cell placed.
    const placement = puzzle.pieces.map((_, i) => i);
    setBuilt({ puzzle, placement });
    setLifted(new Set());
    setSwapNote(false);
  }, [engineReady, seed]);

  useEffect(() => {
    // Rebuild the board when the engine is ready or the seed changes. This is a
    // reset-on-dependency-change effect; the setState lives inside build().
    // eslint-disable-next-line react-hooks/set-state-in-effect
    build();
  }, [build]);

  // Resolve the current board to Edges|null per cell, honouring lifted cells and
  // the (possibly swapped) placement.
  const cells = useMemo<(Edges | null)[] | null>(() => {
    if (!built) return null;
    const out: (Edges | null)[] = [];
    for (let cell = 0; cell < built.placement.length; cell++) {
      if (lifted.has(cell)) {
        out.push(null);
        continue;
      }
      const pid = built.placement[cell] ?? -1;
      const piece = pid >= 0 ? built.puzzle.pieces[pid] : undefined;
      out.push(piece ? rotateEdges(piece, 0) : null);
    }
    return out;
  }, [built, lifted]);

  const tally = useMemo(() => (cells ? ns1Deficit(cells, SIZE) : null), [cells]);

  const isBorderCell = (cell: number): boolean => {
    const x = cell % SIZE;
    const y = Math.floor(cell / SIZE);
    return x === 0 || y === 0 || x === SIZE - 1 || y === SIZE - 1;
  };

  const toggleLift = (cell: number) => {
    if (!isBorderCell(cell)) return;
    setSwapNote(false);
    setLifted((prev) => {
      const next = new Set(prev);
      if (next.has(cell)) next.delete(cell);
      else next.add(cell);
      return next;
    });
  };

  const swapTwoBorder = () => {
    if (!built) return;
    // Pick two distinct, non-lifted border cells and swap their pieces.
    const borderCells: number[] = [];
    for (let c = 0; c < built.placement.length; c++) {
      if (isBorderCell(c) && !lifted.has(c)) borderCells.push(c);
    }
    if (borderCells.length < 2) return;
    const i = borderCells[0] ?? 0;
    const j = borderCells[Math.floor(borderCells.length / 2)] ?? borderCells[1] ?? 0;
    setBuilt((prev) => {
      if (!prev) return prev;
      const placement = prev.placement.slice();
      const tmp = placement[i] ?? 0;
      placement[i] = placement[j] ?? 0;
      placement[j] = tmp;
      return { ...prev, placement };
    });
    setSwapNote(true);
  };

  if (!engineReady || !built || !cells || !tally) {
    return (
      <div className="flex h-64 items-center justify-center rounded-lg border text-sm text-muted-foreground">
        {t.loading}
      </div>
    );
  }

  const d = tally.deficit;
  const balanced = d === 0;
  const allColours = Array.from(new Set([...tally.a.keys(), ...tally.b.keys()])).sort(
    (x, y) => x - y,
  );

  return (
    <div className="space-y-4 rounded-lg border bg-card p-4">
      <div className="space-y-1">
        <h3 className="text-sm font-semibold">{t.title}</h3>
        <p className="text-xs leading-relaxed text-muted-foreground">{t.intro}</p>
      </div>

      <div className="flex flex-wrap items-start justify-center gap-6">
        <div className="w-full max-w-64 space-y-2">
          <BoardSvg
            width={SIZE}
            height={SIZE}
            cells={cells}
            highlight={Array.from(lifted)}
            onCellClick={toggleLift}
          />
          <p className="text-center text-[11px] text-muted-foreground">{t.lift}</p>
        </div>

        <div className="min-w-56 flex-1 space-y-3">
          <div
            className={cn(
              "rounded-md border px-3 py-2 text-center",
              balanced
                ? "border-emerald-300 bg-emerald-500/10"
                : "border-rose-300 bg-rose-500/10",
            )}
          >
            <div className="text-2xl font-bold tabular-nums">
              {t.deficit} = {d}
            </div>
            <div className="text-[11px] text-muted-foreground">
              {swapNote && balanced ? t.swapInvisible : balanced ? t.balanced : t.broken(d)}
            </div>
          </div>

          {/* A vs B colour tallies, one row per colour. */}
          <div className="space-y-1.5">
            <BalanceRow label={t.aLabel} />
            {allColours.map((c) => {
              const av = tally.a.get(c) ?? 0;
              const bv = tally.b.get(c) ?? 0;
              const max = Math.max(av, bv, 1);
              const off = av !== bv;
              return (
                <div key={c} className="flex items-center gap-2">
                  <span
                    className="inline-block h-3 w-3 shrink-0 rounded-sm"
                    style={{ background: swatch(c) }}
                  />
                  <div className="flex flex-1 items-center gap-1">
                    <Bar value={av} max={max} side="a" off={off} />
                    <span className="w-6 text-right text-[10px] tabular-nums text-muted-foreground">
                      {av}
                    </span>
                    <span className="text-muted-foreground/40">·</span>
                    <span className="w-6 text-[10px] tabular-nums text-muted-foreground">{bv}</span>
                    <Bar value={bv} max={max} side="b" off={off} />
                  </div>
                </div>
              );
            })}
            <BalanceRow label={t.bLabel} flip />
          </div>
        </div>
      </div>

      <div className="flex flex-wrap justify-center gap-2">
        <Button size="sm" variant="outline" onClick={swapTwoBorder}>
          {t.swap}
        </Button>
        <Button size="sm" variant="outline" onClick={build}>
          {t.reset}
        </Button>
        <Button
          size="sm"
          variant="outline"
          onClick={() => setSeed((s) => (s * 7 + 13) % 100000)}
        >
          {t.newPuzzle}
        </Button>
      </div>

      <p className="text-xs text-muted-foreground">{t.note}</p>
    </div>
  );
}

function BalanceRow({ label, flip }: { label: string; flip?: boolean }) {
  return (
    <div
      className={cn(
        "text-[10px] font-medium uppercase tracking-wide text-muted-foreground",
        flip ? "text-right" : "",
      )}
    >
      {label}
    </div>
  );
}

function Bar({
  value,
  max,
  side,
  off,
}: {
  value: number;
  max: number;
  side: "a" | "b";
  off: boolean;
}) {
  const pct = (value / max) * 100;
  return (
    <div className={cn("h-2.5 flex-1 overflow-hidden rounded-sm bg-muted", side === "a" && "flex justify-end")}>
      <div
        className={cn("h-full rounded-sm", off ? "bg-rose-500" : "bg-emerald-500")}
        style={{ width: `${pct}%` }}
      />
    </div>
  );
}
