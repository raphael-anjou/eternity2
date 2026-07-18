import { useEffect, useState } from "react";
import { useT } from "@/i18n";
import { useIsClient } from "@/lib/utils";
import { useRunWhileVisible } from "@/lib/useRunWhileVisible";

// CLOISTER, schematically. The border ring is fixed first (locked). Then the
// interior fills inward, row by row, with the cells touching the frame having to
// match it from the very first placement. The ring of cells just inside the
// border lights up as "must match the frame" to make that constraint visible.

const N = 14;
const CELL = 18;

const T = {
  en: {
    border: "fixed border",
    interior: "interior, filling inward",
    constraint: "must match the frame from cell one",
    caption: "The border is locked first; the interior then fills against it, with the cells next to the frame constrained from the start, not checked only at the end.",
    busy: "Loading…",
  },
  fr: {
    border: "bordure fixée",
    interior: "intérieur, se remplit vers le centre",
    constraint: "doit s'accorder au cadre dès la première cellule",
    caption: "La bordure est verrouillée d'abord ; l'intérieur se remplit ensuite contre elle, les cellules voisines du cadre étant contraintes dès le départ, pas vérifiées seulement à la fin.",
    busy: "Chargement…",
  },
  es: {
    border: "borde fijado",
    interior: "interior, se rellena hacia el centro",
    constraint: "debe encajar con el marco desde la primera celda",
    caption: "El borde se bloquea primero; el interior se rellena luego contra él, con las celdas contiguas al marco restringidas desde el inicio, no comprobadas solo al final.",
    busy: "Cargando…",
  },
};

export function BorderAnchorDiagram() {
  const t = useT(T);
  const isClient = useIsClient();
  const [fill, setFill] = useState(0); // interior rows filled (0..N-2)
  const { ref: rootRef, visible } = useRunWhileVisible();

  useEffect(() => {
    if (!visible) return;
    const id = setInterval(() => setFill((f) => (f >= N - 2 ? 0 : f + 1)), 600);
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
    const onBorder = r === 0 || r === N - 1 || c === 0 || c === N - 1;
    if (onBorder) return "fill-stone-500"; // fixed border
    const innerRing = r === 1 || r === N - 2 || c === 1 || c === N - 2;
    const interiorRow = r - 1; // 0-based interior row
    const filled = interiorRow < fill;
    if (!filled) return "fill-muted";
    return innerRing ? "fill-amber-400" : "fill-sky-400/80";
  };

  return (
    <div ref={rootRef} className="space-y-3">
      <div className="mx-auto max-w-xs">
        <svg viewBox={`0 0 ${N * CELL} ${N * CELL}`} className="w-full rounded-lg border bg-card" role="img" aria-label="Grid showing the fixed border locked first, then the interior filling inward with the cells next to the frame highlighted as constrained from the start">
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
                style={{ transition: "fill 200ms" }}
              />
            )),
          )}
        </svg>
      </div>
      <div className="flex flex-wrap justify-center gap-x-4 gap-y-1 text-xs text-muted-foreground">
        <span className="flex items-center gap-1">
          <span className="inline-block h-2.5 w-2.5 rounded-sm bg-stone-500" />
          {t.border}
        </span>
        <span className="flex items-center gap-1">
          <span className="inline-block h-2.5 w-2.5 rounded-sm bg-amber-400" />
          {t.constraint}
        </span>
        <span className="flex items-center gap-1">
          <span className="inline-block h-2.5 w-2.5 rounded-sm bg-sky-400" />
          {t.interior}
        </span>
      </div>
      <p className="mx-auto max-w-md text-center text-xs text-muted-foreground">{t.caption}</p>
    </div>
  );
}
