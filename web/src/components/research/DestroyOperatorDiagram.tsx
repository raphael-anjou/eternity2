import { useT } from "@/i18n";
import { useIsClient } from "@/lib/utils";

// The four destroy operators the study compares, each drawn as the set of cells
// it lifts on the *same* illustrative board of broken seams. The point the
// picture makes: the operators differ only in shape — scattered, conflict-picked,
// a whole band, or one connected tangle plus a halo — and that shape is what the
// study measures. The broken seams (red) are identical across all four panels;
// only the chosen cells (violet) change.

const G = 8;
const CELL = 26;

// A fixed set of "broken" cells for the illustration (a diagonal tangle plus a
// couple of strays), shared by every panel so the operators are compared on the
// same board.
const BROKEN = new Set([19, 20, 27, 28, 35, 12, 44, 51]);

const ORDER = ["random", "mismatch", "band", "component"] as const;

// Each operator's selected cells on that board. Keyed by the exact ORDER union so
// SELECT[key] is total (no undefined under noUncheckedIndexedAccess).
const SELECT: Record<(typeof ORDER)[number], Set<number>> = {
  // random: a scatter, mostly missing the broken cells (geometry-blind).
  random: new Set([2, 13, 19, 33, 41, 46, 55, 60]),
  // mismatched: exactly the broken cells, up to k.
  mismatch: new Set([12, 19, 20, 27, 28, 35, 44, 51]),
  // worst band: every cell in the two rows carrying the most breaks (rows 2–3).
  band: new Set(Array.from({ length: 2 * G }, (_, i) => 16 + i)),
  // component + halo: the connected diagonal tangle (19,20,27,28,35) plus a
  // one-cell halo ring around it.
  component: new Set([19, 20, 27, 28, 35, 11, 12, 13, 18, 21, 26, 29, 34, 36, 43]),
};

const T = {
  en: {
    labels: {
      random: "Random cells",
      mismatch: "Mismatched cells",
      band: "Worst band",
      component: "Component + halo",
    },
    caption:
      "The four destroy operators on one board of broken seams (red). Each lifts a differently-shaped set of cells (violet). Random ignores where the board is broken; mismatched targets exactly the broken cells; worst-band takes the whole row band carrying the most breaks; component-plus-halo takes one connected tangle and a ring around it. On a strong starting board these shapes behave very differently, and the study measures how.",
    busy: "Loading…",
  },
  fr: {
    labels: {
      random: "Cases aléatoires",
      mismatch: "Cases en conflit",
      band: "Pire bande",
      component: "Composante + halo",
    },
    caption:
      "Les quatre opérateurs de destruction sur un même plateau de jointures cassées (rouge). Chacun retire un ensemble de cases de forme différente (violet). Aléatoire ignore où le plateau est cassé ; en-conflit vise exactement les cases cassées ; pire-bande prend toute la bande de lignes portant le plus de cassures ; composante-plus-halo prend un amas connecté et un anneau autour. Sur un bon plateau de départ, ces formes se comportent très différemment, et l'étude mesure comment.",
    busy: "Chargement…",
  },
  es: {
    labels: {
      random: "Celdas aleatorias",
      mismatch: "Celdas en conflicto",
      band: "Peor banda",
      component: "Componente + halo",
    },
    caption:
      "Los cuatro operadores de destrucción sobre un mismo tablero de juntas rotas (rojo). Cada uno retira un conjunto de celdas con una forma distinta (violeta). El aleatorio ignora dónde está roto el tablero; el de conflicto apunta exactamente a las celdas rotas; el de peor banda toma toda la banda de filas que concentra más roturas; el de componente más halo toma un amasijo conexo y un anillo a su alrededor. Sobre un buen tablero de partida, estas formas se comportan de maneras muy distintas, y el estudio mide cómo.",
    busy: "Cargando…",
  },
};

export function DestroyOperatorDiagram() {
  const t = useT(T);
  const isClient = useIsClient();

  if (!isClient) {
    return (
      <div className="flex h-56 items-center justify-center rounded-lg border text-sm text-muted-foreground">
        {t.busy}
      </div>
    );
  }

  const size = G * CELL;

  return (
    <div className="space-y-3">
      <div className="grid grid-cols-2 gap-4 sm:grid-cols-4">
        {ORDER.map((key) => {
          const sel = SELECT[key];
          return (
            <figure key={key} className="space-y-1.5">
              <svg
                viewBox={`0 0 ${size} ${size}`}
                className="aspect-square w-full rounded-md border bg-card"
                role="img"
                aria-label={`${t.labels[key]}: the cells this operator lifts, on a shared board of broken seams`}
              >
                {Array.from({ length: G * G }, (_, pos) => {
                  const x = (pos % G) * CELL;
                  const y = Math.floor(pos / G) * CELL;
                  const selected = sel.has(pos);
                  const broken = BROKEN.has(pos);
                  return (
                    <rect
                      key={pos}
                      x={x + 1}
                      y={y + 1}
                      width={CELL - 2}
                      height={CELL - 2}
                      rx={3}
                      fill={selected ? "color-mix(in oklab, #8b5cf6 30%, transparent)" : "transparent"}
                      stroke={broken ? "#ef4444" : "currentColor"}
                      strokeOpacity={broken ? 0.9 : 0.12}
                      strokeWidth={broken ? 2 : 1}
                    />
                  );
                })}
              </svg>
              <figcaption className="text-center text-xs font-medium text-muted-foreground">
                {t.labels[key]}
              </figcaption>
            </figure>
          );
        })}
      </div>
      <p className="text-sm leading-relaxed text-muted-foreground">{t.caption}</p>
    </div>
  );
}
