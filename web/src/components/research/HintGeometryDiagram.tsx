import { useT } from "@/i18n";
import { useIsClient } from "@/lib/utils";

// Hint geometry, side by side. Left: 18 hints scattered on a lattice (the
// positions Peter McGavin used to solve Joe's 16x16 E2-like puzzle: every third
// column on odd rows). Right: the same puzzle "hinted" by filling contiguous
// rows from the top, the way a scan-row backtracker accumulates them. Same hint
// budget, wildly different leverage: the scattered version pins the hard lower
// region in advance; the contiguous block leaves it entirely unconstrained.

const N = 16;
const CELL = 15;

// Peter's verified 18-hint lattice (0-based row, col), from groups.io msg 11746.
const SCATTERED: Array<[number, number]> = [];
for (const r of [1, 3, 5]) {
  for (const c of [0, 3, 6, 9, 12, 15]) SCATTERED.push([r, c]);
}

// A contiguous block of the same size (18 cells): the first rows, left to right,
// the way top-down row-scan hints pile up.
const CONTIGUOUS: Array<[number, number]> = [];
for (let i = 0; i < 18; i++) CONTIGUOUS.push([Math.floor(i / N), i % N]);

const T = {
  en: {
    scattered: "18 scattered hints",
    contiguous: "18 contiguous hints",
    scatteredCap: "Spread across the board on a lattice. Solved Joe's puzzle in under 15 minutes on one core.",
    contiguousCap: "The same budget piled into the first rows. Leaves the hard lower region completely open.",
    caption:
      "Same number of hints, very different reach. Where the hints sit matters far more than how many there are: the scattered set constrains the whole board at once, the contiguous block only delays the moment the search meets an unconstrained region.",
    busy: "Loading…",
  },
  fr: {
    scattered: "18 indices dispersés",
    contiguous: "18 indices contigus",
    scatteredCap: "Répartis sur le plateau en réseau régulier. Résolvent le puzzle de Joe en moins de 15 minutes sur un seul cœur.",
    contiguousCap: "Le même budget entassé sur les premières rangées. Laisse toute la zone difficile du bas ouverte.",
    caption:
      "Même nombre d'indices, portée très différente. Leur emplacement compte bien plus que leur nombre : l'ensemble dispersé contraint tout le plateau d'un coup, le bloc contigu ne fait que retarder le moment où la recherche rencontre une zone sans contrainte.",
    busy: "Chargement…",
  },
  es: {
    scattered: "18 pistas dispersas",
    contiguous: "18 pistas contiguas",
    scatteredCap: "Repartidas por el tablero en una retícula regular. Resolvieron el puzzle de Joe en menos de 15 minutos con un solo núcleo.",
    contiguousCap: "El mismo presupuesto amontonado en las primeras filas. Deja completamente abierta la difícil zona inferior.",
    caption:
      "El mismo número de pistas, pero un alcance muy distinto. Dónde se sitúan importa mucho más que cuántas hay: el conjunto disperso restringe todo el tablero de golpe, mientras que el bloque contiguo solo retrasa el momento en que la búsqueda topa con una zona sin restringir.",
    busy: "Cargando…",
  },
};

function Grid({ hints, label }: { hints: Array<[number, number]>; label: string }) {
  const set = new Set(hints.map(([r, c]) => `${r}-${c}`));
  return (
    <svg
      viewBox={`0 0 ${N * CELL} ${N * CELL}`}
      className="w-full rounded-lg border bg-card"
      role="img"
      aria-label={label}
    >
      {Array.from({ length: N }, (_, r) =>
        Array.from({ length: N }, (_, c) => {
          const hinted = set.has(`${r}-${c}`);
          return (
            <rect
              key={`${r}-${c}`}
              x={c * CELL + 1}
              y={r * CELL + 1}
              width={CELL - 2}
              height={CELL - 2}
              rx={2}
              className={hinted ? "fill-sky-500" : "fill-muted"}
            />
          );
        }),
      )}
    </svg>
  );
}

export function HintGeometryDiagram() {
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
      <div className="grid gap-4 sm:grid-cols-2">
        <figure className="space-y-2">
          <Grid hints={SCATTERED} label={t.scattered + ": " + t.scatteredCap} />
          <figcaption className="text-center text-xs">
            <span className="font-semibold">{t.scattered}</span>
            <br />
            <span className="text-muted-foreground">{t.scatteredCap}</span>
          </figcaption>
        </figure>
        <figure className="space-y-2">
          <Grid hints={CONTIGUOUS} label={t.contiguous + ": " + t.contiguousCap} />
          <figcaption className="text-center text-xs">
            <span className="font-semibold">{t.contiguous}</span>
            <br />
            <span className="text-muted-foreground">{t.contiguousCap}</span>
          </figcaption>
        </figure>
      </div>
      <p className="mx-auto max-w-2xl text-center text-xs text-muted-foreground">{t.caption}</p>
    </div>
  );
}
