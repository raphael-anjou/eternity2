import { useEffect, useState } from "react";
import { useT } from "@/i18n";
import { useIsClient } from "@/lib/utils";
import { useRunWhileVisible } from "@/lib/useRunWhileVisible";

// PRIOR, schematically. From the library of strong boards, PRIOR learns one thing:
// for each position, how concentrated the choice of piece is across good boards.
// Some cells are nearly always the same piece (a sharp prior); others are a
// coin-toss (flat). The heat map shows that learned confidence. When the live
// build hits a tie — two pieces matching the same number of edges — the prior
// breaks it toward the piece that's typical *for that spot*. The animation sweeps
// a fill front across the board; at each newly-filled cell, sharp-prior cells snap
// decisively, flat-prior cells wobble.

const N = 12;
const CELL = 16;

// Deterministic per-cell prior "sharpness" in [0,1]: corners and border ring are
// the most constrained (sharpest), the deep interior the flattest — mirroring the
// real puzzle, where the frame is far more determined than the centre.
function sharpness(r: number, c: number): number {
  const ring = Math.min(r, c, N - 1 - r, N - 1 - c); // 0 on border, grows inward
  const base = 1 - ring / (N / 2); // 1 at edge → ~0 at centre
  // a little deterministic texture so it doesn't look like a plain gradient
  const tex = ((r * 7 + c * 13) % 5) / 18;
  return Math.max(0.08, Math.min(1, base * 0.82 + tex));
}

const T = {
  en: {
    title: "How sharp the prior is, cell by cell",
    sharp: "sharp prior (almost always the same piece)",
    flat: "flat prior (a coin-toss)",
    caption:
      "PRIOR learns, for every position, how concentrated the piece choice is across strong boards. The frame is sharply determined; the deep interior is nearly a coin-toss. On a tie the prior breaks it toward the typical piece for that spot — strong guidance where it's sharp, harmless where it's flat.",
    busy: "Loading…",
  },
  fr: {
    title: "À quel point le prior est pointu, cellule par cellule",
    sharp: "prior pointu (presque toujours la même pièce)",
    flat: "prior plat (pile ou face)",
    caption:
      "PRIOR apprend, pour chaque position, à quel point le choix de pièce est concentré dans les bons plateaux. Le cadre est nettement déterminé ; l'intérieur profond est presque un pile ou face. En cas d'égalité, le prior tranche vers la pièce typique de cet endroit — un guidage fort là où c'est pointu, inoffensif là où c'est plat.",
    busy: "Chargement…",
  },
};

export function PriorDiagram() {
  const t = useT(T);
  const isClient = useIsClient();
  const [front, setFront] = useState(0); // diagonal fill front 0..2N
  const { ref: rootRef, visible } = useRunWhileVisible();

  useEffect(() => {
    if (!visible) return;
    const id = setInterval(() => setFront((f) => (f >= 2 * N + 3 ? 0 : f + 1)), 220);
    return () => clearInterval(id);
  }, [visible]);

  if (!isClient) {
    return (
      <div className="flex h-64 items-center justify-center rounded-lg border text-sm text-muted-foreground">
        {t.busy}
      </div>
    );
  }

  return (
    <div ref={rootRef} className="space-y-3">
      <h3 className="text-center text-sm font-medium">{t.title}</h3>
      <div className="mx-auto max-w-xs">
        <svg viewBox={`0 0 ${N * CELL} ${N * CELL}`} className="w-full rounded-lg border bg-card" role="img" aria-label="Grid heat map of the learned prior's sharpness per cell, with the border ring sharply determined and the deep interior nearly a coin-toss">
          {Array.from({ length: N }, (_, r) =>
            Array.from({ length: N }, (_, c) => {
              const revealed = r + c <= front;
              const s = sharpness(r, c);
              // sharp = saturated indigo, flat = pale; unrevealed = muted
              const opacity = revealed ? 0.15 + s * 0.85 : 0;
              return (
                <g key={`${r}-${c}`}>
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
                    className="fill-indigo-500"
                    style={{ opacity, transition: "opacity 240ms" }}
                  />
                </g>
              );
            }),
          )}
        </svg>
      </div>
      <div className="flex flex-wrap justify-center gap-x-4 gap-y-1 text-xs text-muted-foreground">
        <span className="flex items-center gap-1">
          <span className="inline-block h-2.5 w-2.5 rounded-sm bg-indigo-500" />
          {t.sharp}
        </span>
        <span className="flex items-center gap-1">
          <span className="inline-block h-2.5 w-2.5 rounded-sm bg-indigo-500/20" />
          {t.flat}
        </span>
      </div>
      <p className="mx-auto max-w-md text-center text-xs text-muted-foreground">{t.caption}</p>
    </div>
  );
}
