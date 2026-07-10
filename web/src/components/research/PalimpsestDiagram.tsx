import { useEffect, useState } from "react";
import { useT } from "@/i18n";
import { useIsClient } from "@/lib/utils";
import { useRunWhileVisible } from "@/lib/useRunWhileVisible";

// PALIMPSEST, schematically. Lay many strong boards on top of one another and a
// consensus appears: most cells agree (the board is "written over" again and
// again in the same place). But a handful of cells agree *too* much — the whole
// corpus made the same quiet choice there, and that shared choice is exactly what
// caps every board below the top. PALIMPSEST finds those cells and targets them.
// The animation stacks boards one by one; cells darken toward consensus, and once
// the stack is deep enough the over-agreed cells get ringed as the targets.

const N = 12;
const CELL = 16;
const STACK = 9; // how many boards we overlay

// Deterministic "agreement" per cell once fully stacked, in [0,1]. A few cells
// are pushed to ~1 (the locked choices); the rest sit in a believable mid-range.
function agreement(r: number, c: number): number {
  const h = (r * 31 + c * 17) % 100;
  const base = 0.35 + (h / 100) * 0.4; // 0.35..0.75
  return base;
}
// The handful of "quietly locked" cells PALIMPSEST flags (over-agreed).
const LOCKED = new Set(["3-4", "4-8", "7-3", "8-7", "5-6"]);

const T = {
  en: {
    title: "Stack the strong boards; find the shared choice",
    boards: "boards overlaid",
    locked: "over-agreed cell — a quiet lock PALIMPSEST targets",
    caption:
      "Overlay many strong boards and most cells reach a normal consensus. A few agree too much: the whole corpus made the same choice there, and that shared choice is what caps every board below the top. PALIMPSEST flags those cells and aims the search at breaking them.",
    busy: "Loading…",
  },
  fr: {
    title: "Empiler les bons plateaux ; trouver le choix partagé",
    boards: "plateaux superposés",
    locked: "cellule sur-accordée — un verrou discret que PALIMPSEST cible",
    caption:
      "Superposez de nombreux bons plateaux et la plupart des cellules atteignent un consensus normal. Quelques-unes s'accordent trop : tout le corpus y a fait le même choix, et ce choix partagé est ce qui plafonne chaque plateau sous le sommet. PALIMPSEST repère ces cellules et oriente la recherche vers leur déverrouillage.",
    busy: "Chargement…",
  },
};

export function PalimpsestDiagram() {
  const t = useT(T);
  const isClient = useIsClient();
  const [n, setN] = useState(0); // boards stacked so far
  const { ref: rootRef, visible } = useRunWhileVisible();

  useEffect(() => {
    if (!visible) return;
    const id = setInterval(() => setN((x) => (x >= STACK + 4 ? 0 : x + 1)), 360);
    return () => clearInterval(id);
  }, [visible]);

  if (!isClient) {
    return (
      <div className="flex h-64 items-center justify-center rounded-lg border text-sm text-muted-foreground">
        {t.busy}
      </div>
    );
  }

  const depth = Math.min(STACK, n) / STACK; // 0..1
  const showLocks = n >= STACK; // rings appear once the stack is deep enough

  return (
    <div ref={rootRef} className="space-y-3">
      <h3 className="text-center text-sm font-medium">{t.title}</h3>
      <div className="mx-auto max-w-xs">
        <svg viewBox={`0 0 ${N * CELL} ${N * CELL}`} className="w-full rounded-lg border bg-card" role="img" aria-label="Grid of strong boards stacked into a consensus heat map, with a handful of over-agreed cells ringed as the quiet locks PALIMPSEST targets">
          {Array.from({ length: N }, (_, r) =>
            Array.from({ length: N }, (_, c) => {
              const key = `${r}-${c}`;
              const locked = LOCKED.has(key);
              const target = locked ? 0.95 : agreement(r, c);
              const a = target * depth; // consensus deepens as boards stack
              return (
                <g key={key}>
                  <rect
                    x={c * CELL + 1}
                    y={r * CELL + 1}
                    width={CELL - 2}
                    height={CELL - 2}
                    rx={2}
                    className="fill-muted"
                  />
                  <rect
                    x={c * CELL + 1}
                    y={r * CELL + 1}
                    width={CELL - 2}
                    height={CELL - 2}
                    rx={2}
                    className="fill-rose-500"
                    style={{ opacity: a, transition: "opacity 300ms" }}
                  />
                  {locked && (
                    <rect
                      x={c * CELL + 1}
                      y={r * CELL + 1}
                      width={CELL - 2}
                      height={CELL - 2}
                      rx={2}
                      fill="none"
                      className="stroke-amber-300"
                      strokeWidth={showLocks ? 2.5 : 0}
                      style={{ transition: "stroke-width 300ms" }}
                    />
                  )}
                </g>
              );
            }),
          )}
        </svg>
      </div>
      <div className="flex items-center justify-center gap-3 text-xs text-muted-foreground">
        <span className="tabular-nums font-medium text-foreground">
          {Math.min(STACK, n)}/{STACK} {t.boards}
        </span>
      </div>
      <div className="flex justify-center text-xs text-muted-foreground">
        <span className="flex items-center gap-1">
          <span className="inline-block h-2.5 w-2.5 rounded-sm border-2 border-amber-300" />
          {t.locked}
        </span>
      </div>
      <p className="mx-auto max-w-md text-center text-xs text-muted-foreground">{t.caption}</p>
    </div>
  );
}
