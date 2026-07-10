import { useEffect, useState } from "react";
import { useT } from "@/i18n";
import { useIsClient } from "@/lib/utils";
import { useRunWhileVisible } from "@/lib/useRunWhileVisible";

// Piece theft, schematically. A solver fills top-left to bottom-right. Early on
// it uses the one piece that could ever serve a particular (north, west) demand,
// elsewhere. Rows later, the cell that actually needed that piece comes up and
// has nothing to place: it's dead, though the board is far from full. Four
// phases loop: place the scarce piece (wrong spot), keep filling, reach the
// hungry cell, it dies.

const N = 12;
const CELL = 22;
const STOLEN = { r: 2, c: 8 }; // where the scarce piece gets spent
const DEAD = { r: 7, c: 3 }; // the cell that needed it, later in scan order

function fillOrderIndex(r: number, c: number) {
  return r * N + c; // row-major scan order
}

const T = {
  en: {
    phases: [
      "The solver places a scarce piece here, early. It's the only piece that can serve one upcoming demand.",
      "It keeps filling, top-left to bottom-right. Everything looks fine.",
      "Rows later it reaches the cell that needed that exact piece…",
      "…and there's nothing to place. Dead, with most of the box still full.",
    ],
    scarce: "the scarce piece (used here)",
    dead: "the cell that needed it (now dead)",
    filled: "filled",
    busy: "Loading…",
  },
  fr: {
    phases: [
      "Le solveur place ici, tôt, une pièce rare. C'est la seule pièce capable de servir une demande à venir.",
      "Il continue de remplir, de haut-gauche vers bas-droite. Tout semble bien.",
      "Des rangées plus loin, il atteint la cellule qui avait besoin de cette pièce exacte…",
      "…et il n'y a rien à poser. Morte, alors que la boîte est encore presque pleine.",
    ],
    scarce: "la pièce rare (utilisée ici)",
    dead: "la cellule qui en avait besoin (morte)",
    filled: "rempli",
    busy: "Chargement…",
  },
};

export function PieceTheftDiagram() {
  const t = useT(T);
  const isClient = useIsClient();
  const [phase, setPhase] = useState(0);
  const { ref: rootRef, visible } = useRunWhileVisible();

  useEffect(() => {
    if (!visible) return;
    const id = setInterval(() => setPhase((p) => (p + 1) % 4), 1500);
    return () => clearInterval(id);
  }, [visible]);

  if (!isClient) {
    return (
      <div className="flex h-72 items-center justify-center rounded-lg border text-sm text-muted-foreground">
        {t.busy}
      </div>
    );
  }

  // How far the scan has progressed by phase.
  const progressByPhase = [
    fillOrderIndex(STOLEN.r, STOLEN.c),
    fillOrderIndex(5, N - 1),
    fillOrderIndex(DEAD.r, DEAD.c) - 1,
    fillOrderIndex(DEAD.r, DEAD.c),
  ];
  const progress = progressByPhase[phase] ?? 0;

  const cellClass = (r: number, c: number): string => {
    const idx = fillOrderIndex(r, c);
    if (r === STOLEN.r && c === STOLEN.c) return "fill-violet-500";
    if (r === DEAD.r && c === DEAD.c) {
      if (phase >= 2) return "fill-red-500";
      return "fill-muted";
    }
    return idx < progress ? "fill-sky-400/70" : "fill-muted";
  };

  return (
    <div ref={rootRef} className="space-y-3">
      <div className="mx-auto max-w-xs">
        <svg viewBox={`0 0 ${N * CELL} ${N * CELL}`} className="w-full rounded-lg border bg-card" role="img" aria-label="A scan-order grid showing a scarce piece spent early at one cell leaving a later cell dead, with an arrow tracing the theft back">
          {Array.from({ length: N }, (_, r) =>
            Array.from({ length: N }, (_, c) => (
              <rect
                key={`${r}-${c}`}
                x={c * CELL + 1}
                y={r * CELL + 1}
                width={CELL - 2}
                height={CELL - 2}
                rx={2}
                className={cellClass(r, c)}
                style={{ transition: "fill 250ms" }}
              />
            )),
          )}
          {/* arrow from the dead cell back to the stolen piece, in late phases */}
          {phase >= 2 && (
            <line
              x1={DEAD.c * CELL + CELL / 2}
              y1={DEAD.r * CELL + CELL / 2}
              x2={STOLEN.c * CELL + CELL / 2}
              y2={STOLEN.r * CELL + CELL / 2}
              stroke="currentColor"
              className="text-red-500"
              strokeWidth={1.5}
              strokeDasharray="4 3"
            />
          )}
        </svg>
      </div>
      <div className="flex flex-wrap justify-center gap-x-4 gap-y-1 text-xs text-muted-foreground">
        <span className="flex items-center gap-1">
          <span className="inline-block h-2.5 w-2.5 rounded-sm bg-violet-500" />
          {t.scarce}
        </span>
        <span className="flex items-center gap-1">
          <span className="inline-block h-2.5 w-2.5 rounded-sm bg-red-500" />
          {t.dead}
        </span>
      </div>
      <p
        className={
          "mx-auto max-w-md text-center text-sm font-medium " +
          (phase === 3 ? "text-red-600 dark:text-red-400" : "text-muted-foreground")
        }
      >
        {t.phases[phase]}
      </p>
    </div>
  );
}
