import { useEffect, useMemo, useState } from "react";

import { useT } from "@/i18n";
import { clueShape } from "@/lib/hint-layouts";
import { useIsClient } from "@/lib/utils";

// Live fill animation for the Hint Study. A board is seeded with a handful of
// hints (fixed cells), then a chosen FILL PATH drops the remaining cells in
// order, one by one, so the reader *sees* how the order reads — or ignores —
// the hints. The point the animation makes visually: a compact sweep keeps a
// single tight frontier, while a hint-seeking "connect the hints first" order
// opens many frontiers at once and thrashes. No solver runs here; this is the
// cell-visitation ORDER made visible, which is the study's whole subject.

const N = 16;
const CELL = 20;

type Path = "rowmajor" | "spiral-in" | "border-first" | "connect-hints-first";

// The real E2 five-clue shape (official cells at 16×16), from the shared lib so
// the animation, the diagrams, and the measured board all use the same geometry.
const HINTS: number[] = clueShape(N);

const rc = (cell: number): [number, number] => [Math.floor(cell / N), cell % N];
const idx = (r: number, c: number) => r * N + c;

function rowMajor(): number[] {
  return Array.from({ length: N * N }, (_, i) => i);
}

function spiralIn(): number[] {
  let [top, bottom, left, right] = [0, N - 1, 0, N - 1];
  const out: number[] = [];
  while (top <= bottom && left <= right) {
    for (let c = left; c <= right; c++) out.push(idx(top, c));
    for (let r = top + 1; r <= bottom; r++) out.push(idx(r, right));
    if (top < bottom) for (let c = right - 1; c >= left; c--) out.push(idx(bottom, c));
    if (left < right) for (let r = bottom - 1; r > top; r--) out.push(idx(r, left));
    top++; bottom--; left++; right--;
  }
  return out;
}

function borderFirst(): number[] {
  const border: number[] = [];
  const interior: number[] = [];
  for (let r = 0; r < N; r++)
    for (let c = 0; c < N; c++) {
      const isB = r === 0 || r === N - 1 || c === 0 || c === N - 1;
      (isB ? border : interior).push(idx(r, c));
    }
  return [...border, ...interior];
}

// Multi-source BFS out of the hint cells: the seductive-but-wrong order.
function connectHintsFirst(): number[] {
  const seen = new Array<boolean>(N * N).fill(false);
  const order: number[] = [];
  const queue: number[] = [];
  for (const h of HINTS) { seen[h] = true; order.push(h); queue.push(h); }
  let head = 0;
  while (head < queue.length) {
    const cell = queue[head++];
    if (cell === undefined) break;
    const [r, c] = rc(cell);
    const nbrs = [
      r > 0 ? idx(r - 1, c) : -1,
      c > 0 ? idx(r, c - 1) : -1,
      c < N - 1 ? idx(r, c + 1) : -1,
      r < N - 1 ? idx(r + 1, c) : -1,
    ];
    for (const nb of nbrs)
      if (nb >= 0 && !seen[nb]) { seen[nb] = true; order.push(nb); queue.push(nb); }
  }
  return order;
}

function buildPath(p: Path): number[] {
  const raw =
    p === "rowmajor" ? rowMajor()
    : p === "spiral-in" ? spiralIn()
    : p === "border-first" ? borderFirst()
    : connectHintsFirst();
  const hintSet = new Set(HINTS);
  // Hints are pre-placed; the fill visits only the non-hint cells, in path order.
  return raw.filter((cell) => !hintSet.has(cell));
}

// The open frontier at a given fill step: filled cells adjacent to an empty one.
// Its size is what governs a backtracker's branching, so we surface it live.
function frontierSize(filled: Set<number>): number {
  let count = 0;
  for (const cell of filled) {
    const [r, c] = rc(cell);
    const nbrs = [
      r > 0 ? idx(r - 1, c) : -1,
      c > 0 ? idx(r, c - 1) : -1,
      c < N - 1 ? idx(r, c + 1) : -1,
      r < N - 1 ? idx(r + 1, c) : -1,
    ];
    if (nbrs.some((nb) => nb >= 0 && !filled.has(nb))) count++;
  }
  return count;
}

const T = {
  en: {
    rowmajor: "Row-major sweep",
    "spiral-in": "Spiral inward",
    "border-first": "Border first",
    "connect-hints-first": "Connect the hints first",
    play: "Play",
    pause: "Pause",
    reset: "Reset",
    step: "Step",
    filled: "Filled",
    frontier: "Open frontier",
    frontierHelp:
      "The open frontier is the count of filled cells still touching an empty one — the boundary the search must keep consistent. A backtracker's work grows explosively with it.",
    hint: "hint",
    caption:
      "The same five hints, four fill orders. A compact sweep keeps the open frontier to a single thin line; the hint-seeking order opens a frontier around every hint at once and never closes them. Watch the frontier counter: it is the whole story.",
    busy: "Loading…",
    peak: "peak",
  },
  fr: {
    rowmajor: "Balayage par lignes",
    "spiral-in": "Spirale vers l'intérieur",
    "border-first": "Le bord d'abord",
    "connect-hints-first": "Relier les indices d'abord",
    play: "Lancer",
    pause: "Pause",
    reset: "Réinitialiser",
    step: "Pas à pas",
    filled: "Remplies",
    frontier: "Front ouvert",
    frontierHelp:
      "Le front ouvert compte les cases remplies encore en contact avec une case vide — la frontière que la recherche doit garder cohérente. Le travail d'un backtracker croît de façon explosive avec lui.",
    hint: "indice",
    caption:
      "Les cinq mêmes indices, quatre ordres de remplissage. Un balayage compact réduit le front ouvert à une seule ligne fine ; l'ordre qui cherche les indices ouvre un front autour de chaque indice à la fois et ne les referme jamais. Regardez le compteur de front : tout est là.",
    busy: "Chargement…",
    peak: "pic",
  },
  es: {
    rowmajor: "Barrido por filas",
    "spiral-in": "Espiral hacia dentro",
    "border-first": "El borde primero",
    "connect-hints-first": "Conectar las pistas primero",
    play: "Reproducir",
    pause: "Pausa",
    reset: "Reiniciar",
    step: "Paso",
    filled: "Rellenadas",
    frontier: "Frontera abierta",
    frontierHelp:
      "La frontera abierta cuenta las celdas rellenadas que aún tocan una vacía — el límite que la búsqueda debe mantener coherente. El trabajo de un backtracker crece de forma explosiva con ella.",
    hint: "pista",
    caption:
      "Las mismas cinco pistas, cuatro órdenes de relleno. Un barrido compacto mantiene la frontera abierta en una sola línea fina; el orden que busca las pistas abre una frontera alrededor de cada pista a la vez y nunca las cierra. Observa el contador de frontera: ahí está toda la historia.",
    busy: "Cargando…",
    peak: "pico",
  },
};

const PATHS: Path[] = ["rowmajor", "spiral-in", "border-first", "connect-hints-first"];

export function HintPathFill() {
  const t = useT(T);
  const isClient = useIsClient();
  const [path, setPath] = useState<Path>("rowmajor");
  const [step, setStep] = useState(0);
  const [playing, setPlaying] = useState(false);

  const sequence = useMemo(() => buildPath(path), [path]);
  const hintSet = useMemo(() => new Set(HINTS), []);
  const total = sequence.length;

  // Cells filled so far = hints + first `step` path cells.
  const filled = useMemo(() => {
    const s = new Set<number>(HINTS);
    for (let i = 0; i < step; i++) {
      const cell = sequence[i];
      if (cell !== undefined) s.add(cell);
    }
    return s;
  }, [sequence, step]);

  const frontier = useMemo(() => frontierSize(filled), [filled]);

  // Peak frontier so far for the current path — derived, not stored: the fill is
  // monotonic in `step`, so the peak up to now is the max frontier over the cells
  // placed so far. Computing it as a memo avoids a setState-inside-effect.
  const peak = useMemo(() => {
    const s = new Set<number>(HINTS);
    let mx = frontierSize(s);
    for (let i = 0; i < step; i++) {
      const cell = sequence[i];
      if (cell !== undefined) s.add(cell);
      const f = frontierSize(s);
      if (f > mx) mx = f;
    }
    return mx;
  }, [sequence, step]);

  // Switching path resets the animation — done in the tab handler (`selectPath`)
  // rather than a path-watching effect, to avoid a setState-inside-effect.
  const selectPath = (p: Path) => {
    setPath(p);
    setStep(0);
    setPlaying(false);
  };

  // The play loop, self-contained in one effect: a local rAF handle, no shared
  // ref mutation and no self-referencing callback, so it satisfies the hooks
  // lints. Advances one step roughly every 26 ms while playing.
  useEffect(() => {
    if (!playing) return;
    let handle = 0;
    let previous = 0;
    const loop = (now: number) => {
      if (now - previous > 26) {
        previous = now;
        setStep((s) => {
          if (s >= total) {
            setPlaying(false);
            return s;
          }
          return s + 1;
        });
      }
      handle = requestAnimationFrame(loop);
    };
    handle = requestAnimationFrame(loop);
    return () => cancelAnimationFrame(handle);
  }, [playing, total]);

  if (!isClient) {
    return (
      <div className="flex h-96 items-center justify-center rounded-lg border text-sm text-muted-foreground">
        {t.busy}
      </div>
    );
  }

  // Which cells are on the open frontier right now (for a subtle highlight).
  const frontierCells = new Set<number>();
  for (const cell of filled) {
    const [r, c] = rc(cell);
    const nbrs = [
      r > 0 ? idx(r - 1, c) : -1,
      c > 0 ? idx(r, c - 1) : -1,
      c < N - 1 ? idx(r, c + 1) : -1,
      r < N - 1 ? idx(r + 1, c) : -1,
    ];
    if (nbrs.some((nb) => nb >= 0 && !filled.has(nb))) frontierCells.add(cell);
  }

  return (
    <div className="space-y-3">
      <div className="flex flex-wrap gap-1.5" role="tablist" aria-label="fill order">
        {PATHS.map((p) => (
          <button
            key={p}
            role="tab"
            aria-selected={path === p}
            onClick={() => selectPath(p)}
            className={`rounded-md px-2.5 py-1 text-xs font-medium transition-colors ${
              path === p
                ? "bg-primary text-primary-foreground"
                : "bg-muted text-muted-foreground hover:bg-muted-foreground/10"
            }`}
          >
            {t[p]}
          </button>
        ))}
      </div>

      <div className="grid gap-4 sm:grid-cols-[auto_1fr] sm:items-start">
        <svg
          viewBox={`0 0 ${N * CELL} ${N * CELL}`}
          className="w-full max-w-sm rounded-lg border bg-card"
          role="img"
          aria-label={`${t[path]}: ${step} of ${total} cells filled, open frontier ${frontier}`}
        >
          {Array.from({ length: N * N }, (_, cell) => {
            const [r, c] = rc(cell);
            const isHint = hintSet.has(cell);
            const isFilled = filled.has(cell);
            const isFrontier = frontierCells.has(cell);
            let cls = "fill-muted";
            if (isHint) cls = "fill-amber-400";
            else if (isFrontier) cls = "fill-sky-400";
            else if (isFilled) cls = "fill-sky-500/30";
            return (
              <rect
                key={cell}
                x={c * CELL + 1}
                y={r * CELL + 1}
                width={CELL - 2}
                height={CELL - 2}
                rx={2.5}
                className={cls}
              />
            );
          })}
        </svg>

        <div className="space-y-3">
          <div className="flex flex-wrap gap-2">
            <button
              onClick={() => setPlaying((p) => !p)}
              className="rounded-md bg-primary px-3 py-1.5 text-xs font-semibold text-primary-foreground"
            >
              {playing ? t.pause : t.play}
            </button>
            <button
              onClick={() => { setPlaying(false); setStep((s) => Math.min(total, s + 1)); }}
              className="rounded-md bg-muted px-3 py-1.5 text-xs font-medium"
            >
              {t.step}
            </button>
            <button
              onClick={() => { setPlaying(false); setStep(0); }}
              className="rounded-md bg-muted px-3 py-1.5 text-xs font-medium"
            >
              {t.reset}
            </button>
          </div>

          <div className="grid grid-cols-2 gap-2">
            <div className="rounded-lg border p-3">
              <div className="text-2xl font-bold tabular-nums">{step + HINTS.length}<span className="text-sm text-muted-foreground">/{N * N}</span></div>
              <div className="text-xs text-muted-foreground">{t.filled}</div>
            </div>
            <div className="rounded-lg border p-3">
              <div className="text-2xl font-bold tabular-nums text-sky-500">
                {frontier}
                <span className="ml-1 text-xs font-normal text-muted-foreground">
                  ({t.peak} {peak})
                </span>
              </div>
              <div className="text-xs text-muted-foreground">{t.frontier}</div>
            </div>
          </div>

          {/* Frontier bar: a live sense of the branching pressure. */}
          <div className="h-2 w-full overflow-hidden rounded-full bg-muted">
            <div
              className="h-full rounded-full bg-sky-500 transition-[width] duration-100"
              style={{ width: `${Math.min(100, (frontier / 60) * 100)}%` }}
            />
          </div>

          <p className="text-xs text-muted-foreground">{t.frontierHelp}</p>
        </div>
      </div>

      <p className="mx-auto max-w-2xl text-center text-xs text-muted-foreground">{t.caption}</p>
    </div>
  );
}
