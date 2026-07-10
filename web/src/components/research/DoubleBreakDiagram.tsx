import { useEffect, useState } from "react";
import { useT } from "@/i18n";
import { useIsClient } from "@/lib/utils";
import { useRunWhileVisible } from "@/lib/useRunWhileVisible";

// REPLAY's discovery, schematically. A strict-460 board is almost all perfect
// matches, with a few single-break cells and a handful of double-break cells
// (two mismatches paid at once). A search that allows at most one break per cell
// literally cannot place the double-break cells. The animation toggles between
// "what a one-break search can reach" and "what the 460 board actually needs".

const N = 12;
const CELL = 20;

// Hand-placed: a few single-break cells and 4 double-break cells (the witness pattern).
const SINGLE = new Set(["3-4", "6-9", "8-2", "9-7", "4-8"]);
const DOUBLE = new Set(["5-5", "5-6", "7-3", "2-8"]);

const T = {
  en: {
    toggleReach: "What one-break search reaches",
    toggleNeed: "What the 460 board needs",
    perfect: "perfect match",
    single: "one break",
    double: "two breaks at one cell",
    reachCaption: "A one-break-per-cell search can place single breaks, but never the double-break cells, so it stalls at 457-458.",
    needCaption: "The community 460 boards carry four or five double-break cells. Allowing the double break is what makes them reachable.",
    busy: "Loading…",
  },
  fr: {
    toggleReach: "Ce qu'atteint une recherche à un défaut",
    toggleNeed: "Ce qu'exige le plateau 460",
    perfect: "accord parfait",
    single: "un défaut",
    double: "deux défauts sur une cellule",
    reachCaption: "Une recherche à un défaut par cellule peut poser les défauts simples, mais jamais les cellules à double défaut, et cale donc à 457-458.",
    needCaption: "Les plateaux 460 de la communauté portent quatre ou cinq cellules à double défaut. Autoriser le double défaut est ce qui les rend atteignables.",
    busy: "Chargement…",
  },
};

export function DoubleBreakDiagram() {
  const t = useT(T);
  const isClient = useIsClient();
  const [showDouble, setShowDouble] = useState(true);
  const { ref: rootRef, visible } = useRunWhileVisible();

  useEffect(() => {
    if (!visible) return;
    const id = setInterval(() => setShowDouble((s) => !s), 1800);
    return () => clearInterval(id);
  }, [visible]);

  if (!isClient) {
    return (
      <div className="flex h-72 items-center justify-center rounded-lg border text-sm text-muted-foreground">
        {t.busy}
      </div>
    );
  }

  const cellClass = (r: number, c: number): string => {
    const key = `${r}-${c}`;
    if (DOUBLE.has(key)) return showDouble ? "fill-red-500" : "fill-muted";
    if (SINGLE.has(key)) return "fill-amber-400";
    return "fill-emerald-400/70";
  };

  return (
    <div ref={rootRef} className="space-y-3">
      <div className="mx-auto max-w-xs">
        <svg viewBox={`0 0 ${N * CELL} ${N * CELL}`} className="w-full rounded-lg border bg-card" role="img" aria-label="Board grid toggling between what a one-break search can reach and what the 460 board needs, marking perfect-match, single-break, and double-break cells">
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
        </svg>
      </div>
      <p className="text-center text-sm font-medium text-foreground">
        {showDouble ? t.toggleNeed : t.toggleReach}
      </p>
      <div className="flex flex-wrap justify-center gap-x-4 gap-y-1 text-xs text-muted-foreground">
        <span className="flex items-center gap-1">
          <span className="inline-block h-2.5 w-2.5 rounded-sm bg-emerald-400" />
          {t.perfect}
        </span>
        <span className="flex items-center gap-1">
          <span className="inline-block h-2.5 w-2.5 rounded-sm bg-amber-400" />
          {t.single}
        </span>
        <span className="flex items-center gap-1">
          <span className="inline-block h-2.5 w-2.5 rounded-sm bg-red-500" />
          {t.double}
        </span>
      </div>
      <p className="mx-auto max-w-md text-center text-xs text-muted-foreground">
        {showDouble ? t.needCaption : t.reachCaption}
      </p>
    </div>
  );
}
