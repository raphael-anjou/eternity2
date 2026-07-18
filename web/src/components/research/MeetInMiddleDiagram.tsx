import { useEffect, useState } from "react";
import { useT } from "@/i18n";
import { useIsClient } from "@/lib/utils";
import { useRunWhileVisible } from "@/lib/useRunWhileVisible";

// BANDSAW's meet-in-the-middle, schematically. A band of rows is split into a
// top half (grown downward) and a bottom half (grown upward); they meet at a
// seam row, and a join succeeds only where the seam colors line up. Four phases
// loop: grow top, grow bottom, test the seam (mismatch), test the seam (match).

const COLS = 8;
const CELL = 26;
const SEAM_COLORS = ["#38bdf8", "#f59e0b", "#34d399", "#a78bfa", "#fb7185", "#38bdf8", "#34d399", "#f59e0b"];

const T = {
  en: {
    top: "top half (grows down)",
    bottom: "bottom half (grows up)",
    phases: [
      "Enumerate every top half up to a small mismatch budget…",
      "…and every bottom half, from the pieces the top didn't use.",
      "Try to join at the seam: here the colors clash, no join.",
      "Join succeeds only where the seam colors line up exactly.",
    ],
    busy: "Loading…",
  },
  fr: {
    top: "moitié haute (croît vers le bas)",
    bottom: "moitié basse (croît vers le haut)",
    phases: [
      "Énumérer chaque moitié haute jusqu'à un petit budget de défauts…",
      "…et chaque moitié basse, à partir des pièces non utilisées par le haut.",
      "Tenter la jonction à la couture : ici les couleurs se heurtent, pas de jonction.",
      "La jonction ne réussit que là où les couleurs de couture s'alignent exactement.",
    ],
    busy: "Chargement…",
  },
  es: {
    top: "mitad superior (crece hacia abajo)",
    bottom: "mitad inferior (crece hacia arriba)",
    phases: [
      "Enumerar cada mitad superior hasta un pequeño presupuesto de desajustes…",
      "…y cada mitad inferior, con las piezas que la superior no usó.",
      "Intentar unir en la costura: aquí los colores chocan, no hay unión.",
      "La unión solo tiene éxito donde los colores de la costura coinciden exactamente.",
    ],
    busy: "Cargando…",
  },
};

export function MeetInMiddleDiagram() {
  const t = useT(T);
  const isClient = useIsClient();
  const [phase, setPhase] = useState(0);
  const { ref: rootRef, visible } = useRunWhileVisible();

  useEffect(() => {
    if (!visible) return;
    const id = setInterval(() => setPhase((p) => (p + 1) % 4), 1300);
    return () => clearInterval(id);
  }, [visible]);

  if (!isClient) {
    return (
      <div className="flex h-72 items-center justify-center rounded-lg border text-sm text-muted-foreground">
        {t.busy}
      </div>
    );
  }

  const topShown = phase >= 0;
  const bottomShown = phase >= 1;
  const testing = phase >= 2;
  const matched = phase === 3;
  // Bottom seam colors equal top's only in the matched phase.
  const bottomSeam = (i: number) =>
    matched ? SEAM_COLORS[i] : SEAM_COLORS[(i + 3) % SEAM_COLORS.length];

  const W = COLS * CELL;
  const rowY = (r: number) => r * CELL;

  return (
    <div ref={rootRef} className="space-y-3">
      <div className="mx-auto" style={{ maxWidth: W }}>
        <svg viewBox={`0 0 ${W} ${CELL * 5}`} className="w-full rounded-lg border bg-card" role="img" aria-label="Meet-in-the-middle diagram of a top half grown downward and a bottom half grown upward joining at a seam row, where seam markers show colours clashing or lining up exactly">
          {/* top half: rows 0,1 then the seam row 2 (top's bottom edge) */}
          {topShown &&
            [0, 1].map((r) =>
              Array.from({ length: COLS }, (_, c) => (
                <rect key={`t${r}${c}`} x={c * CELL + 1} y={rowY(r) + 1} width={CELL - 2} height={CELL - 2} rx={2} className="fill-sky-400/80" />
              )),
            )}
          {/* top seam edge (row 2 top strip) */}
          {topShown &&
            Array.from({ length: COLS }, (_, c) => (
              <rect key={`ts${c}`} x={c * CELL + 1} y={rowY(2) + 1} width={CELL - 2} height={CELL / 2 - 1} fill={SEAM_COLORS[c]} />
            ))}
          {/* bottom seam edge (row 2 bottom strip) */}
          {bottomShown &&
            Array.from({ length: COLS }, (_, c) => (
              <rect key={`bs${c}`} x={c * CELL + 1} y={rowY(2) + CELL / 2} width={CELL - 2} height={CELL / 2 - 1} fill={bottomSeam(c)} />
            ))}
          {/* bottom half: rows 3,4 */}
          {bottomShown &&
            [3, 4].map((r) =>
              Array.from({ length: COLS }, (_, c) => (
                <rect key={`b${r}${c}`} x={c * CELL + 1} y={rowY(r) + 1} width={CELL - 2} height={CELL - 2} rx={2} className="fill-amber-400/80" />
              )),
            )}
          {/* seam markers when testing */}
          {testing &&
            Array.from({ length: COLS }, (_, c) => {
              const ok = SEAM_COLORS[c] === bottomSeam(c);
              return (
                <circle
                  key={`m${c}`}
                  cx={c * CELL + CELL / 2}
                  cy={rowY(2) + CELL / 2}
                  r={4}
                  fill={ok ? "#16a34a" : "#dc2626"}
                />
              );
            })}
        </svg>
      </div>
      <div className="flex justify-center gap-4 text-xs text-muted-foreground">
        <span className="flex items-center gap-1">
          <span className="inline-block h-2.5 w-2.5 rounded-sm bg-sky-400" />
          {t.top}
        </span>
        <span className="flex items-center gap-1">
          <span className="inline-block h-2.5 w-2.5 rounded-sm bg-amber-400" />
          {t.bottom}
        </span>
      </div>
      <p
        className={
          "mx-auto max-w-md text-center text-sm font-medium " +
          (matched
            ? "text-emerald-700 dark:text-emerald-400"
            : testing
              ? "text-red-600 dark:text-red-400"
              : "text-muted-foreground")
        }
      >
        {t.phases[phase]}
      </p>
    </div>
  );
}
