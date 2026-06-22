import { useEffect, useState } from "react";
import { useT } from "@/i18n";
import { useIsClient } from "@/lib/utils";

// A schematic of STAGED's four-stage, frame-free build. The 16x16 grid fills in
// order: top half (stage 1-2), the oracle band, the third band, then the exact
// bottom finisher, with the border emerging last rather than being placed first.
// Illustration of the order, not a real solve.

const N = 16;
const CELL = 17;

// Stage of each row band (by row index). 0..3 fill in order; the border ring is
// drawn last as its own pass to make "the frame emerges last" visible.
function stageOfCell(r: number, c: number): number {
  const onRing = r === 0 || r === N - 1 || c === 0 || c === N - 1;
  if (onRing) return 4; // border, emerges last
  if (r <= 7) return 0; // top half
  if (r <= 10) return 1; // oracle band
  if (r <= 12) return 2; // third band
  return 3; // exact finisher
}

const STAGE_FILL = [
  "fill-sky-400",
  "fill-violet-400",
  "fill-emerald-400",
  "fill-amber-400",
  "fill-stone-500",
];
// Explicit bg classes (kept literal so Tailwind's JIT emits them).
const STAGE_BG = ["bg-sky-400", "bg-violet-400", "bg-emerald-400", "bg-amber-400", "bg-stone-500"];

const T = {
  en: {
    stages: ["Top half", "Oracle band", "Third band", "Exact finisher", "Border (emerges last)"],
    caption: "The build order, top to bottom, with the border filled last from whatever pieces remain. A schematic of the four stages, not a real solve.",
    busy: "Loading…",
  },
  fr: {
    stages: ["Moitié haute", "Bande oracle", "Troisième bande", "Finisseur exact", "Bordure (émerge en dernier)"],
    caption: "L'ordre de construction, de haut en bas, la bordure remplie en dernier à partir des pièces restantes. Un schéma des quatre étapes, pas une vraie résolution.",
    busy: "Chargement…",
  },
};

export function StageBuildDiagram() {
  const t = useT(T);
  const isClient = useIsClient();
  const [shown, setShown] = useState(0); // how many stages revealed (0..5)

  useEffect(() => {
    const id = setInterval(() => setShown((s) => (s >= 5 ? 0 : s + 1)), 850);
    return () => clearInterval(id);
  }, []);

  if (!isClient) {
    return (
      <div className="flex h-72 items-center justify-center rounded-lg border text-sm text-muted-foreground">
        {t.busy}
      </div>
    );
  }

  return (
    <div className="space-y-3">
      <div className="mx-auto max-w-xs">
        <svg viewBox={`0 0 ${N * CELL} ${N * CELL}`} className="w-full rounded-lg border bg-card">
          {Array.from({ length: N }, (_, r) =>
            Array.from({ length: N }, (_, c) => {
              const stage = stageOfCell(r, c);
              const visible = stage < shown;
              return (
                <rect
                  key={`${r}-${c}`}
                  x={c * CELL + 1}
                  y={r * CELL + 1}
                  width={CELL - 2}
                  height={CELL - 2}
                  rx={2}
                  className={visible ? STAGE_FILL[stage] : "fill-muted"}
                  style={{ transition: "fill 200ms" }}
                />
              );
            }),
          )}
        </svg>
      </div>
      <div className="mx-auto flex max-w-md flex-wrap justify-center gap-x-3 gap-y-1 text-xs">
        {t.stages.map((label, i) => (
          <span
            key={label}
            className={
              "flex items-center gap-1 " +
              (i < shown ? "text-foreground" : "text-muted-foreground/50")
            }
          >
            <span className={"inline-block h-2.5 w-2.5 rounded-sm " + STAGE_BG[i]} />
            {label}
          </span>
        ))}
      </div>
      <p className="mx-auto max-w-md text-center text-xs text-muted-foreground">{t.caption}</p>
    </div>
  );
}
