import { useEffect, useState } from "react";
import { useT } from "@/i18n";
import { useIsClient } from "@/lib/utils";
import { useRunWhileVisible } from "@/lib/useRunWhileVisible";

// A schematic animation of why basin-hopping fails. A ring of cells must all
// rotate to their next position together (the "cycle"). Apply the whole cycle
// and every piece lands matched. Apply only part of it and the chain breaks at
// the gap, so the score drops. This is an illustration of the mechanism; the
// real cycles span up to 154 cells across the whole board.

const N = 10; // cells in the schematic ring
const R = 92; // ring radius
const CX = 130;
const CY = 130;

function pos(i: number) {
  const a = (i / N) * 2 * Math.PI - Math.PI / 2;
  return { x: CX + R * Math.cos(a), y: CY + R * Math.sin(a) };
}

const T = {
  en: {
    applied: "cells moved",
    full: "Whole cycle applied — every cell lands matched.",
    partial: "Only part applied — the chain breaks, the score drops.",
    sub: "Drag the slider: a partial cycle always leaves a broken seam.",
    fullCta: "Apply the whole cycle",
    busy: "Loading…",
  },
  fr: {
    applied: "cellules déplacées",
    full: "Cycle entier appliqué — chaque cellule retombe accordée.",
    partial: "Seulement une partie appliquée — la chaîne casse, le score baisse.",
    sub: "Glissez le curseur : un cycle partiel laisse toujours une couture cassée.",
    fullCta: "Appliquer le cycle entier",
    busy: "Chargement…",
  },
  es: {
    applied: "celdas movidas",
    full: "Ciclo completo aplicado: cada celda encaja al aterrizar.",
    partial: "Solo una parte aplicada: la cadena se rompe y la puntuación cae.",
    sub: "Desliza el control: un ciclo parcial siempre deja una costura rota.",
    fullCta: "Aplicar el ciclo completo",
    busy: "Cargando…",
  },
};

export function SigmaCycleDiagram() {
  const t = useT(T);
  const isClient = useIsClient();
  const [applied, setApplied] = useState(0);
  const [auto, setAuto] = useState(true);
  const { ref: rootRef, visible } = useRunWhileVisible({ respectReducedMotion: true });

  useEffect(() => {
    if (!auto || !visible) return;
    const id = setInterval(() => setApplied((a) => (a >= N ? 0 : a + 1)), 700);
    return () => clearInterval(id);
  }, [auto, visible]);

  if (!isClient) {
    return (
      <div className="flex h-80 items-center justify-center rounded-lg border text-sm text-muted-foreground">
        {t.busy}
      </div>
    );
  }

  const complete = applied >= N;

  return (
    <div ref={rootRef} className="space-y-4">
      <div className="mx-auto max-w-sm rounded-lg border p-2">
        <svg viewBox="0 0 260 260" className="w-full">
          {/* arrows from each cell to its target */}
          {Array.from({ length: N }, (_, i) => {
            const from = pos(i);
            const to = pos((i + 1) % N);
            const moved = i < applied;
            return (
              <line
                key={`a${i}`}
                x1={from.x}
                y1={from.y}
                x2={to.x}
                y2={to.y}
                stroke="currentColor"
                strokeWidth={2}
                className={moved ? "text-emerald-500" : "text-muted-foreground/40"}
                strokeDasharray={moved ? undefined : "4 4"}
              />
            );
          })}
          {/* the broken seam marker: between the last moved cell and the rest */}
          {!complete && applied > 0 && (
            <circle
              cx={pos(applied % N).x}
              cy={pos(applied % N).y}
              r={16}
              fill="none"
              stroke="currentColor"
              strokeWidth={2.5}
              className="text-red-500"
            />
          )}
          {/* cells */}
          {Array.from({ length: N }, (_, i) => {
            const p = pos(i);
            const moved = i < applied;
            return (
              <g key={`c${i}`}>
                <circle
                  cx={p.x}
                  cy={p.y}
                  r={12}
                  className={
                    complete
                      ? "fill-emerald-500"
                      : moved
                        ? "fill-emerald-400/70"
                        : "fill-muted-foreground/30"
                  }
                />
              </g>
            );
          })}
        </svg>
      </div>

      <div className="mx-auto flex max-w-sm flex-col items-center gap-2">
        <input
          type="range"
          min={0}
          max={N}
          value={applied}
          onChange={(e) => {
            setAuto(false);
            setApplied(Number(e.target.value));
          }}
          className="w-full"
          aria-label={t.applied}
        />
        <div className="flex items-center gap-3 text-sm text-muted-foreground">
          <span className="tabular-nums">
            {applied} / {N} {t.applied}
          </span>
          <button
            onClick={() => {
              setAuto(false);
              setApplied(N);
            }}
            className="rounded-md border px-2 py-0.5 font-medium transition-colors hover:bg-muted hover:text-foreground"
          >
            {t.fullCta}
          </button>
        </div>
      </div>

      <p
        className={
          "mx-auto max-w-md text-center text-sm font-medium " +
          (complete ? "text-emerald-700 dark:text-emerald-400" : "text-red-600 dark:text-red-400")
        }
      >
        {complete ? t.full : applied === 0 ? t.sub : t.partial}
      </p>
    </div>
  );
}
