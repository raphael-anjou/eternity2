import { useEffect, useMemo, useRef, useState } from "react";

import { useT } from "@/i18n";
import {
  clueShape,
  rowMajorSeq,
  spiralInSeq,
  spiralOutSeq,
  traceHintsSeq,
} from "@/lib/hint-layouts";
import { useIsClient } from "@/lib/utils";

// Live fill animation for the Hint Study, shown as several boards side by side,
// each looping on its own so the reader can compare fill orders at a glance. Each
// board is seeded with the five official clue hints, then a chosen FILL PATH drops
// the remaining cells one by one. The newest cell is deep blue and older cells
// fade lighter, so the *wavefront* of the fill — and the open frontier it drags
// behind it — is visible. No solver runs here; this is the cell-visitation ORDER
// made visible, which is the study's whole subject.

const N = 16;
const CELL = 15;
const HINTS = clueShape(N);
const TRAIL = 14; // cells behind the wavefront that stay tinted before fading out

type PathKey = "rowmajor" | "spiral-in" | "spiral-out" | "trace-hints";

const BUILDERS: Record<PathKey, () => number[]> = {
  rowmajor: () => rowMajorSeq(N),
  "spiral-in": () => spiralInSeq(N),
  "spiral-out": () => spiralOutSeq(N),
  "trace-hints": () => traceHintsSeq(N, HINTS),
};

// Each board's fill sequence excludes the pinned hints (they start placed).
function buildSequence(key: PathKey): number[] {
  const hintSet = new Set(HINTS);
  return BUILDERS[key]().filter((c) => !hintSet.has(c));
}

const T = {
  en: {
    rowmajor: "Row-major sweep",
    "spiral-in": "Spiral inward",
    "spiral-out": "Spiral outward",
    "trace-hints": "Trace the hints",
    rowmajorNote: "one tight frontier, rolled down the board",
    "spiral-inNote": "a ring that closes in; the frontier is a whole perimeter",
    "spiral-outNote": "grows from the centre; the frontier balloons outward",
    "trace-hintsNote": "skeleton between the clues first; many open fronts at once",
    caption:
      "The same five hints (amber), four fill orders, each looping on its own. The bright cell is the one just placed; the trail behind it is the recent wavefront. Watch how much boundary each order keeps open as it runs: the row-major sweep drags a single thin line, while the hint-tracing order scatters its wavefront across the whole board.",
    busy: "Loading…",
  },
  fr: {
    rowmajor: "Balayage par lignes",
    "spiral-in": "Spirale vers l'intérieur",
    "spiral-out": "Spirale vers l'extérieur",
    "trace-hints": "Relier les indices",
    rowmajorNote: "un seul front serré, déroulé sur le plateau",
    "spiral-inNote": "un anneau qui se referme ; le front est tout un périmètre",
    "spiral-outNote": "croît depuis le centre ; le front enfle vers l'extérieur",
    "trace-hintsNote": "d'abord un squelette entre les indices ; plusieurs fronts à la fois",
    caption:
      "Les cinq mêmes indices (ambre), quatre ordres de remplissage, chacun en boucle. La case vive est celle qu'on vient de poser ; la traînée derrière est le front récent. Observez la frontière que chaque ordre laisse ouverte : le balayage par lignes tire une seule ligne fine, tandis que l'ordre qui relie les indices disperse son front sur tout le plateau.",
    busy: "Chargement…",
  },
  es: {
    rowmajor: "Barrido por filas",
    "spiral-in": "Espiral hacia dentro",
    "spiral-out": "Espiral hacia fuera",
    "trace-hints": "Trazar las pistas",
    rowmajorNote: "un frente estrecho, desplegado por el tablero",
    "spiral-inNote": "un anillo que se cierra; el frente es todo un perímetro",
    "spiral-outNote": "crece desde el centro; el frente se hincha hacia fuera",
    "trace-hintsNote": "primero un esqueleto entre las pistas; muchos frentes a la vez",
    caption:
      "Las mismas cinco pistas (ámbar), cuatro órdenes de relleno, cada uno en bucle. La celda brillante es la recién colocada; la estela es el frente reciente. Observa cuánta frontera deja abierta cada orden: el barrido por filas arrastra una sola línea fina, mientras que el orden que traza las pistas dispersa su frente por todo el tablero.",
    busy: "Cargando…",
  },
};

const ORDER: PathKey[] = ["rowmajor", "spiral-in", "spiral-out", "trace-hints"];

export function HintPathFill() {
  const t = useT(T);
  const isClient = useIsClient();

  if (!isClient) {
    return (
      <div className="flex h-72 items-center justify-center rounded-lg border text-sm text-muted-foreground">
        {t.busy}
      </div>
    );
  }

  return (
    <div className="space-y-3">
      <div className="grid grid-cols-2 gap-4 lg:grid-cols-4">
        {ORDER.map((key) => (
          <LoopingBoard key={key} pathKey={key} title={t[key]} note={t[`${key}Note`]} />
        ))}
      </div>
      <p className="mx-auto max-w-2xl text-center text-xs text-muted-foreground">{t.caption}</p>
    </div>
  );
}

// One board that fills along its path on a loop, with a fading trail.
function LoopingBoard({ pathKey, title, note }: { pathKey: PathKey; title: string; note: string }) {
  const sequence = useMemo(() => buildSequence(pathKey), [pathKey]);
  const hintSet = useMemo(() => new Set(HINTS), []);
  const [step, setStep] = useState(0);
  const raf = useRef<number>(0);
  const last = useRef<number>(0);

  useEffect(() => {
    const total = sequence.length;
    const loop = (now: number) => {
      if (now - last.current > 45) {
        last.current = now;
        // advance; when the board is full, pause a beat then restart.
        setStep((s) => (s >= total + 8 ? 0 : s + 1));
      }
      raf.current = requestAnimationFrame(loop);
    };
    raf.current = requestAnimationFrame(loop);
    return () => cancelAnimationFrame(raf.current);
  }, [sequence]);

  // For each cell, its position in the fill order (or -1 if a hint / unfilled).
  const orderOf = useMemo(() => {
    const pos = new Array<number>(N * N).fill(-1);
    sequence.forEach((cell, i) => (pos[cell] = i));
    return pos;
  }, [sequence]);

  const size = N * CELL;

  return (
    <figure className="space-y-1.5">
      <svg
        viewBox={`0 0 ${size} ${size}`}
        className="w-full rounded-lg border bg-card"
        role="img"
        aria-label={`${title}: fill order animation`}
      >
        {Array.from({ length: N * N }, (_, cell) => {
          const r = Math.floor(cell / N);
          const c = cell % N;
          const isHint = hintSet.has(cell);
          const ord = orderOf[cell] ?? -1;
          const placed = ord >= 0 && ord < step;
          // Distance behind the wavefront (0 = just placed).
          const behind = placed ? step - 1 - ord : -1;

          let fill = "fill-muted";
          let opacity = 1;
          if (isHint) {
            fill = "fill-amber-400";
          } else if (placed) {
            fill = "fill-sky-500";
            if (behind === 0) {
              opacity = 1; // brightest: just placed
            } else if (behind < TRAIL) {
              opacity = 0.55 - (behind / TRAIL) * 0.35; // fade through the trail
            } else {
              opacity = 0.16; // settled fill
            }
          }
          return (
            <rect
              key={cell}
              x={c * CELL + 0.6}
              y={r * CELL + 0.6}
              width={CELL - 1.2}
              height={CELL - 1.2}
              rx={1.6}
              className={fill}
              fillOpacity={opacity}
            />
          );
        })}
      </svg>
      <figcaption className="text-center text-[11px] leading-tight">
        <span className="font-semibold">{title}</span>
        <br />
        <span className="text-muted-foreground">{note}</span>
      </figcaption>
    </figure>
  );
}
