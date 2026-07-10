import { useEffect, useState } from "react";
import { useT } from "@/i18n";
import { useIsClient } from "@/lib/utils";
import { useRunWhileVisible } from "@/lib/useRunWhileVisible";

// KEYRING, schematically. At one cell, several pieces fit. A single rule of thumb
// would always pick the same one and march into the same dead end. KEYRING scores
// every candidate with three *learned* signals at once — where the piece likes to
// sit, which pieces like to be its neighbours, which 2x2 patches show up in good
// boards — and adds them up. The winner shifts depending on the mix, which is what
// keeps the search out of its rut. Here four candidates carry three stacked signal
// bars; the combined height decides, and a small rotation of weights changes who
// wins, frame to frame.

// Per-candidate base contributions [position, neighbour, patch], deterministic.
const CANDIDATES = [
  { key: "A", base: [0.55, 0.22, 0.30] },
  { key: "B", base: [0.30, 0.50, 0.40] },
  { key: "C", base: [0.42, 0.38, 0.52] },
  { key: "D", base: [0.20, 0.60, 0.25] },
];

// Three weight mixes the animation rotates through, to show the winner changing.
const WEIGHT_FRAMES = [
  [1.0, 1.0, 1.0],
  [1.4, 0.7, 0.9],
  [0.7, 1.4, 1.1],
];

const SIGNALS = [
  { i: 0, color: "fill-violet-400", key: "pos" },
  { i: 1, color: "fill-emerald-400", key: "nbr" },
  { i: 2, color: "fill-amber-400", key: "patch" },
];

const T = {
  en: {
    title: "Three signals vote on the next piece",
    pos: "where it sits",
    nbr: "who it neighbours",
    patch: "2×2 patches",
    winner: "placed",
    caption:
      "Each candidate piece is scored by three learned signals at once and they're summed. The tallest stack wins. Because no single signal dominates, the winner changes with the mix — which is exactly what stops the search marching into the same dead end every time.",
    busy: "Loading…",
  },
  fr: {
    title: "Trois signaux votent pour la pièce suivante",
    pos: "où elle se place",
    nbr: "qui elle voisine",
    patch: "carrés 2×2",
    winner: "posée",
    caption:
      "Chaque pièce candidate est notée par trois signaux appris à la fois, puis on les additionne. La pile la plus haute gagne. Comme aucun signal ne domine, le gagnant change selon le mélange — ce qui empêche justement la recherche de filer dans la même impasse à chaque fois.",
    busy: "Chargement…",
  },
};

export function KeyringDiagram() {
  const t = useT(T);
  const isClient = useIsClient();
  const [frame, setFrame] = useState(0);
  const { ref: rootRef, visible } = useRunWhileVisible();

  useEffect(() => {
    if (!visible) return;
    const id = setInterval(() => setFrame((f) => (f + 1) % WEIGHT_FRAMES.length), 1600);
    return () => clearInterval(id);
  }, [visible]);

  if (!isClient) {
    return (
      <div className="flex h-64 items-center justify-center rounded-lg border text-sm text-muted-foreground">
        {t.busy}
      </div>
    );
  }

  const w = WEIGHT_FRAMES[frame] ?? [1, 1, 1];
  const totals = CANDIDATES.map((cand) =>
    cand.base.reduce((acc, v, i) => acc + v * (w[i] ?? 0), 0),
  );
  const winner = totals.indexOf(Math.max(...totals));

  const H = 130;
  const colW = 46;
  const gap = 22;
  const W = CANDIDATES.length * (colW + gap);
  const scale = 70; // px per unit-score

  return (
    <div ref={rootRef} className="space-y-3">
      <h3 className="text-center text-sm font-medium">{t.title}</h3>
      <div className="mx-auto" style={{ maxWidth: W }}>
        <svg viewBox={`0 0 ${W} ${H + 26}`} className="w-full" role="img" aria-label="Stacked bar chart of four candidate pieces, each scored by three learned signals summed into a total, with the tallest stack marked as the winner">
          {CANDIDATES.map((cand, ci) => {
            const x = ci * (colW + gap) + gap / 2;
            const isWin = ci === winner;
            let yCursor = H;
            const segs = SIGNALS.map((sig) => {
              const h = (cand.base[sig.i] ?? 0) * (w[sig.i] ?? 0) * scale;
              yCursor -= h;
              return { ...sig, y: yCursor, h };
            });
            return (
              <g key={cand.key}>
                {segs.map((seg) => (
                  <rect
                    key={seg.i}
                    x={x}
                    y={seg.y}
                    width={colW}
                    height={Math.max(0, seg.h - 1)}
                    rx={2}
                    className={seg.color}
                    style={{ transition: "y 400ms, height 400ms" }}
                  />
                ))}
                <line x1={x} y1={H} x2={x + colW} y2={H} className="stroke-border" strokeWidth={1} />
                <text
                  x={x + colW / 2}
                  y={H + 16}
                  textAnchor="middle"
                  className={
                    "text-[11px] " + (isWin ? "fill-foreground font-bold" : "fill-muted-foreground")
                  }
                >
                  {cand.key}
                </text>
                {isWin && (
                  <text
                    x={x + colW / 2}
                    y={yCursor - 6}
                    textAnchor="middle"
                    className="fill-foreground text-[9px] font-medium uppercase tracking-wide"
                  >
                    ★
                  </text>
                )}
              </g>
            );
          })}
        </svg>
      </div>
      <div className="flex flex-wrap justify-center gap-x-4 gap-y-1 text-xs text-muted-foreground">
        {SIGNALS.map((sig) => (
          <span key={sig.i} className="flex items-center gap-1">
            <span
              className={
                "inline-block h-2.5 w-2.5 rounded-sm " + sig.color.replace("fill-", "bg-")
              }
            />
            {t[sig.key as "pos" | "nbr" | "patch"]}
          </span>
        ))}
      </div>
      <p className="mx-auto max-w-md text-center text-xs text-muted-foreground">{t.caption}</p>
    </div>
  );
}
