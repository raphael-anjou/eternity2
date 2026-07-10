import { useEffect, useState } from "react";
import { useT } from "@/i18n";
import { useIsClient } from "@/lib/utils";
import { useRunWhileVisible } from "@/lib/useRunWhileVisible";

// GAUNTLET, schematically. One beam search run in a single fill direction always
// drifts toward the same region of board-space. GAUNTLET runs several fill
// directions at once — here four, sweeping from different corners — so the
// parallel searches settle into *different* regions instead of all colliding on
// the same one. The four mini-boards fill from their own corner; a coloured dot
// marks where each lands, and the dots are deliberately spread apart.

const N = 8;
const CELL = 12;

// Four fill orders, each a corner the search grows out from, with the region it
// tends to land in (a point in the schematic "board-space" strip below).
const DIRS = [
  { key: "tl", grow: (r: number, c: number) => r + c, landing: 0.18, color: "fill-violet-400" },
  { key: "tr", grow: (r: number, c: number) => r + (N - 1 - c), landing: 0.42, color: "fill-emerald-400" },
  { key: "bl", grow: (r: number, c: number) => N - 1 - r + c, landing: 0.63, color: "fill-amber-400" },
  { key: "br", grow: (r: number, c: number) => N - 1 - r + (N - 1 - c), landing: 0.86, color: "fill-sky-400" },
] as const;

const DOT_COLOR: Record<string, string> = {
  tl: "fill-violet-400",
  tr: "fill-emerald-400",
  bl: "fill-amber-400",
  br: "fill-sky-400",
};

const T = {
  en: {
    title: "Nine scan orders, nine regions",
    one: "one direction → one region",
    many: "four directions → four regions",
    landings: "where each search lands in board-space",
    caption:
      "A single scan order keeps converging to one region. GAUNTLET sweeps nine scan orders, so the runs settle in different regions — here four of the nine, fanning from different corners. More of the landscape covered per sweep.",
    busy: "Loading…",
  },
  fr: {
    title: "Neuf ordres de parcours, neuf régions",
    one: "une direction → une région",
    many: "quatre directions → quatre régions",
    landings: "où chaque recherche atterrit dans l'espace des plateaux",
    caption:
      "Un seul ordre de parcours converge toujours vers une région. GAUNTLET balaye neuf ordres, si bien que les exécutions se posent dans des régions différentes — ici quatre des neuf, en éventail depuis différents coins. Davantage du paysage couvert par balayage.",
    busy: "Chargement…",
  },
};

export function GauntletDiagram() {
  const t = useT(T);
  const isClient = useIsClient();
  const [step, setStep] = useState(0); // 0..maxStep, fraction of board filled
  const { ref: rootRef, visible } = useRunWhileVisible();

  const maxStep = (N - 1) * 2; // max value of grow()
  useEffect(() => {
    if (!visible) return;
    const id = setInterval(() => setStep((s) => (s >= maxStep + 4 ? 0 : s + 1)), 280);
    return () => clearInterval(id);
  }, [maxStep, visible]);

  if (!isClient) {
    return (
      <div className="flex h-64 items-center justify-center rounded-lg border text-sm text-muted-foreground">
        {t.busy}
      </div>
    );
  }

  const filledFrac = Math.min(1, step / maxStep);

  return (
    <div ref={rootRef} className="space-y-3">
      <h3 className="text-center text-sm font-medium">{t.title}</h3>
      <div className="mx-auto grid max-w-md grid-cols-4 gap-2 sm:gap-3">
        {DIRS.map((dir) => (
          <svg
            key={dir.key}
            viewBox={`0 0 ${N * CELL} ${N * CELL}`}
            className="w-full rounded-md border bg-card"
            role="img"
            aria-label="Four mini-boards each filling from a different corner, showing GAUNTLET's parallel scan orders growing out from distinct starting points"
          >
            {Array.from({ length: N }, (_, r) =>
              Array.from({ length: N }, (_, c) => {
                const filled = dir.grow(r, c) <= step;
                return (
                  <rect
                    key={`${r}-${c}`}
                    x={c * CELL + 0.5}
                    y={r * CELL + 0.5}
                    width={CELL - 1}
                    height={CELL - 1}
                    rx={1.5}
                    className={filled ? dir.color : "fill-muted"}
                    style={{ transition: "fill 160ms" }}
                  />
                );
              }),
            )}
          </svg>
        ))}
      </div>

      {/* board-space strip: where each direction lands, kept spread apart */}
      <div className="mx-auto max-w-md space-y-1">
        <p className="text-center text-[11px] text-muted-foreground">{t.landings}</p>
        <svg viewBox="0 0 200 26" className="w-full" aria-hidden>
          <line x1={6} y1={20} x2={194} y2={20} className="stroke-border" strokeWidth={1} />
          {DIRS.map((dir) => {
            const cx = 6 + dir.landing * 188;
            const appeared = filledFrac > 0.85;
            return (
              <circle
                key={dir.key}
                cx={cx}
                cy={20}
                r={appeared ? 5 : 0}
                className={DOT_COLOR[dir.key]}
                style={{ transition: "r 300ms" }}
              />
            );
          })}
        </svg>
      </div>
      <p className="mx-auto max-w-md text-center text-xs text-muted-foreground">{t.caption}</p>
    </div>
  );
}
