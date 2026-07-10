import { useEffect, useState } from "react";
import { useT } from "@/i18n";
import { useIsClient } from "@/lib/utils";
import { useRunWhileVisible } from "@/lib/useRunWhileVisible";

// MIDDEN's idea, shown as masks. Each shape marks the cells where a mismatch is
// allowed (everywhere else must match perfectly). The animation cycles through
// the shapes; the dispersed lattice is the one that lets a board stay perfect
// longest. Illustration of the geometry, not a solve.

const N = 16;
const CELL = 17;

type Shape = { key: string; mask: (r: number, c: number) => boolean };

const ROWS_SHAPE: Shape = { key: "rows", mask: (r) => r === 12 || r === 13 };
const SHAPES: [Shape, ...Shape[]] = [
  ROWS_SHAPE,
  { key: "cols", mask: (_r, c) => c === 7 || c === 8 },
  { key: "lattice", mask: (r, c) => r % 4 === 2 && c % 4 === 2 },
];

const T = {
  en: {
    shapes: { rows: "Two rows", cols: "Two columns", lattice: "Dispersed lattice" },
    legend: "allowed-mismatch cells",
    rest: "everything else must match perfectly",
    caption: "Each shape is where the board is allowed to break. The dispersed lattice spreads the damage thinly and lets the perfect region run longest.",
    busy: "Loading…",
  },
  fr: {
    shapes: { rows: "Deux rangées", cols: "Deux colonnes", lattice: "Réseau dispersé" },
    legend: "cellules à défaut autorisé",
    rest: "tout le reste doit s'accorder parfaitement",
    caption: "Chaque forme indique où le plateau peut casser. Le réseau dispersé répartit les dégâts finement et laisse la région parfaite courir le plus longtemps.",
    busy: "Chargement…",
  },
};

export function MaskShapeDiagram() {
  const t = useT(T);
  const isClient = useIsClient();
  const [idx, setIdx] = useState(2); // start on the lattice (the interesting one)
  const [auto, setAuto] = useState(true);
  const { ref: rootRef, visible } = useRunWhileVisible();

  useEffect(() => {
    if (!auto || !visible) return;
    const id = setInterval(() => setIdx((i) => (i + 1) % SHAPES.length), 1400);
    return () => clearInterval(id);
  }, [auto, visible]);

  if (!isClient) {
    return (
      <div className="flex h-72 items-center justify-center rounded-lg border text-sm text-muted-foreground">
        {t.busy}
      </div>
    );
  }

  const shape = SHAPES[idx] ?? ROWS_SHAPE;

  return (
    <div ref={rootRef} className="space-y-3">
      <div className="mx-auto max-w-xs">
        <svg viewBox={`0 0 ${N * CELL} ${N * CELL}`} className="w-full rounded-lg border bg-card" role="img" aria-label="16 by 16 grid showing the selected allowed-mismatch mask shape, with the rest of the cells marked as must-match-perfectly">
          {Array.from({ length: N }, (_, r) =>
            Array.from({ length: N }, (_, c) => {
              const allowed = shape.mask(r, c);
              return (
                <rect
                  key={`${r}-${c}`}
                  x={c * CELL + 1}
                  y={r * CELL + 1}
                  width={CELL - 2}
                  height={CELL - 2}
                  rx={2}
                  className={allowed ? "fill-red-500" : "fill-emerald-400/70"}
                  style={{ transition: "fill 250ms" }}
                />
              );
            }),
          )}
        </svg>
      </div>

      <div className="flex flex-wrap items-center justify-center gap-2">
        {SHAPES.map((s, i) => (
          <button
            key={s.key}
            onClick={() => {
              setAuto(false);
              setIdx(i);
            }}
            className={
              "rounded-md border px-3 py-1 text-sm font-medium transition-colors " +
              (i === idx
                ? "bg-primary text-primary-foreground"
                : "text-muted-foreground hover:bg-muted hover:text-foreground")
            }
          >
            {t.shapes[s.key as keyof typeof t.shapes]}
          </button>
        ))}
      </div>

      <div className="flex flex-wrap justify-center gap-x-4 gap-y-1 text-xs text-muted-foreground">
        <span className="flex items-center gap-1">
          <span className="inline-block h-2.5 w-2.5 rounded-sm bg-red-500" />
          {t.legend}
        </span>
        <span className="flex items-center gap-1">
          <span className="inline-block h-2.5 w-2.5 rounded-sm bg-emerald-400" />
          {t.rest}
        </span>
      </div>
      <p className="mx-auto max-w-md text-center text-xs text-muted-foreground">{t.caption}</p>
    </div>
  );
}
