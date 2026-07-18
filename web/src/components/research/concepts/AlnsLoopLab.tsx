import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { useT } from "@/i18n";
import { useEngine } from "@/engine/useEngine";
import { getGeneratedSolvedPuzzleFramed } from "@/engine";
import type { Puzzle } from "@/lib/types";
import { rotateEdges, maxScore } from "@/lib/types";
import type { Edges } from "@/lib/bucas";
import { BoardSvg } from "@/components/board/BoardSvg";
import { Button } from "@/components/ui/button";
import { cn } from "@/lib/utils";
import { useRunWhileVisible } from "@/lib/useRunWhileVisible";

// The ALNS loop, made watchable. We take a real solved 8×8 from the engine,
// damage it down to roughly 80% of its score, then run destroy-and-repair live:
// an operator lifts a set of cells (they go dark), a greedy pass refills them
// one placement per tick, the result is accepted or rejected against the score,
// and the operator weights adapt. A sparkline traces the score; the weight bars
// show the "adaptive" part learning which demolition pays. Deterministic seed:
// reload and you get the same run.

const SIZE = 8;
const COLORS = 6;
const SEED = 7;
const MAX = maxScore(SIZE, SIZE); // 112 internal seams on an 8×8
const TICK_MS = 150;
const SA_TEMPERATURE = 1.0;
const HISTORY_CAP = 120;

type Op = "patch" | "rows" | "worst";
const OPS: Op[] = ["patch", "rows", "worst"];

const T = {
  en: {
    title: "One ALNS loop, in slow motion",
    intro:
      "A real solved 8×8 (from the engine), deliberately damaged. Each iteration: a destroy operator lifts a region (dark cells), a greedy pass refills it one piece per tick, and the new board is kept or reverted against the score. Watch the operator weights: every success feeds back, so the roulette drifts toward the demolitions that pay.",
    play: "Play",
    pause: "Pause",
    reset: "Reset",
    damage: "Damage again",
    forceOp: "Run one iteration with a chosen operator:",
    ops: { patch: "Random patch", rows: "Worst row", worst: "Worst region" } as Record<Op, string>,
    score: "score",
    best: "best",
    iterations: "iterations",
    weights: "Adaptive operator weights",
    uses: (n: number) => `${n}×`,
    trace: "Score trace",
    phase: {
      idle: "paused — press Play or pick an operator",
      destroy: "destroy: lifting cells",
      repair: "repair: greedy refill, most-constrained cell first",
      decide: "accept or reject…",
    },
    accepted: (d: number) => (d > 0 ? `accepted (+${d})` : "accepted (sideways move)"),
    rejected: (d: number) => `rejected (${d}) — board reverted`,
    newBest: " — new best!",
    loading: "Loading the engine…",
    note: "Everything here is the real mechanism at toy scale: the greedy repair maximizes matched seams among the lifted pieces, acceptance is the simulated-annealing rule, and each operator's weight is an exponentially decayed average of its recent rewards. The full-size engine runs the same loop with a curated portfolio of five operators on a 16×16 board.",
  },
  fr: {
    title: "Une boucle ALNS, au ralenti",
    intro:
      "Un vrai 8×8 résolu (par le moteur), abîmé exprès. À chaque itération : un opérateur de destruction soulève une région (cases sombres), une passe gloutonne la rebouche pièce par pièce, et le nouveau plateau est gardé ou annulé selon le score. Regardez les poids des opérateurs : chaque succès nourrit la roulette, qui dérive vers les démolitions qui rapportent.",
    play: "Lecture",
    pause: "Pause",
    reset: "Réinitialiser",
    damage: "Abîmer à nouveau",
    forceOp: "Lancer une itération avec un opérateur choisi :",
    ops: { patch: "Patch aléatoire", rows: "Pire rangée", worst: "Pire région" } as Record<Op, string>,
    score: "score",
    best: "meilleur",
    iterations: "itérations",
    weights: "Poids adaptatifs des opérateurs",
    uses: (n: number) => `${n}×`,
    trace: "Trace du score",
    phase: {
      idle: "en pause — Lecture, ou choisissez un opérateur",
      destroy: "destruction : levée des cases",
      repair: "réparation : remplissage glouton, case la plus contrainte d'abord",
      decide: "accepter ou rejeter…",
    },
    accepted: (d: number) => (d > 0 ? `accepté (+${d})` : "accepté (mouvement latéral)"),
    rejected: (d: number) => `rejeté (${d}) — plateau restauré`,
    newBest: " — nouveau record !",
    loading: "Chargement du moteur…",
    note: "Tout ici est le vrai mécanisme, à échelle jouet : la réparation gloutonne maximise les coutures accordées parmi les pièces levées, l'acceptation suit la règle du recuit simulé, et le poids de chaque opérateur est une moyenne à décroissance exponentielle de ses récompenses récentes. Le moteur grandeur nature fait tourner la même boucle avec un portefeuille trié de cinq opérateurs sur un 16×16.",
  },
  es: {
    title: "Un ciclo ALNS, a cámara lenta",
    intro:
      "Un 8×8 resuelto de verdad (por el motor), dañado a propósito. En cada iteración: un operador de destrucción levanta una región (celdas oscuras), una pasada voraz la rellena pieza a pieza, y el nuevo tablero se conserva o se descarta según la puntuación. Observa los pesos de los operadores: cada acierto realimenta la ruleta, que se inclina hacia las demoliciones que rinden.",
    play: "Reproducir",
    pause: "Pausar",
    reset: "Reiniciar",
    damage: "Dañar de nuevo",
    forceOp: "Ejecutar una iteración con un operador elegido:",
    ops: { patch: "Parche aleatorio", rows: "Peor fila", worst: "Peor región" } as Record<Op, string>,
    score: "puntuación",
    best: "mejor",
    iterations: "iteraciones",
    weights: "Pesos adaptativos de los operadores",
    uses: (n: number) => `${n}×`,
    trace: "Traza de la puntuación",
    phase: {
      idle: "en pausa — pulsa Reproducir o elige un operador",
      destroy: "destrucción: levantando celdas",
      repair: "reparación: relleno voraz, primero la celda más restringida",
      decide: "aceptar o rechazar…",
    },
    accepted: (d: number) => (d > 0 ? `aceptado (+${d})` : "aceptado (movimiento lateral)"),
    rejected: (d: number) => `rechazado (${d}) — tablero restaurado`,
    newBest: " — ¡nuevo récord!",
    loading: "Cargando el motor…",
    note: "Todo lo que ves es el mecanismo real, a escala de juguete: la reparación voraz maximiza las aristas coincidentes entre las piezas levantadas, la aceptación sigue la regla del recocido simulado, y el peso de cada operador es una media con decaimiento exponencial de sus recompensas recientes. El motor a tamaño real ejecuta el mismo ciclo con una cartera seleccionada de cinco operadores sobre un tablero 16×16.",
  },
};

/** Deterministic PRNG (mulberry32). */
function mulberry32(seed: number): () => number {
  let a = seed >>> 0;
  return () => {
    a = (a + 0x6d2b79f5) >>> 0;
    let t = a;
    t = Math.imul(t ^ (t >>> 15), t | 1);
    t ^= t + Math.imul(t ^ (t >>> 7), t | 61);
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
}

/** Edges of the piece placed at `v` (piece*4+rot), or null when v < 0. */
function edgesOf(puzzle: Puzzle, v: number): Edges | null {
  if (v < 0) return null;
  const piece = puzzle.pieces[v >> 2];
  return piece ? rotateEdges(piece, v & 3) : null;
}

/** Matched internal seams (empty or mismatched seams count 0). Max = maxScore. */
function boardScore(puzzle: Puzzle, b: Int32Array): number {
  let s = 0;
  for (let y = 0; y < SIZE; y++) {
    for (let x = 0; x < SIZE; x++) {
      const e = edgesOf(puzzle, b[y * SIZE + x] ?? -1);
      if (!e) continue;
      if (x + 1 < SIZE) {
        const r = edgesOf(puzzle, b[y * SIZE + x + 1] ?? -1);
        if (r && e[1] === r[3]) s++;
      }
      if (y + 1 < SIZE) {
        const d = edgesOf(puzzle, b[(y + 1) * SIZE + x] ?? -1);
        if (d && e[2] === d[0]) s++;
      }
    }
  }
  return s;
}

/** Mismatched half-edges [pos, dir] for the conflict overlay. */
function conflictMarks(puzzle: Puzzle, b: Int32Array): [number, number][] {
  const out: [number, number][] = [];
  for (let y = 0; y < SIZE; y++) {
    for (let x = 0; x < SIZE; x++) {
      const pos = y * SIZE + x;
      const e = edgesOf(puzzle, b[pos] ?? -1);
      if (!e) continue;
      if (x + 1 < SIZE) {
        const r = edgesOf(puzzle, b[pos + 1] ?? -1);
        if (r && e[1] !== r[3]) out.push([pos, 1], [pos + 1, 3]);
      }
      if (y + 1 < SIZE) {
        const d = edgesOf(puzzle, b[pos + SIZE] ?? -1);
        if (d && e[2] !== d[0]) out.push([pos, 2], [pos + SIZE, 0]);
      }
    }
  }
  return out;
}

/** Mismatched seams incident to each cell (destroy-operator guidance). */
function cellConflicts(puzzle: Puzzle, b: Int32Array): number[] {
  const counts = new Array<number>(SIZE * SIZE).fill(0);
  for (const [pos] of conflictMarks(puzzle, b)) counts[pos] = (counts[pos] ?? 0) + 1;
  return counts;
}

const isInterior = (pos: number): boolean => {
  const x = pos % SIZE;
  const y = Math.floor(pos / SIZE);
  return x > 0 && y > 0 && x < SIZE - 1 && y < SIZE - 1;
};

/** Damage the solved board with random interior swaps until ≈80% of max. */
function damageBoard(puzzle: Puzzle, b: Int32Array, rng: () => number): void {
  const interior: number[] = [];
  for (let p = 0; p < SIZE * SIZE; p++) if (isInterior(p)) interior.push(p);
  for (let guard = 0; guard < 60 && boardScore(puzzle, b) > 0.8 * MAX; guard++) {
    const i = interior[Math.floor(rng() * interior.length)] ?? 9;
    const j = interior[Math.floor(rng() * interior.length)] ?? 18;
    if (i === j) continue;
    const vi = b[i] ?? -1;
    const vj = b[j] ?? -1;
    b[i] = ((vj >> 2) << 2) | Math.floor(rng() * 4);
    b[j] = ((vi >> 2) << 2) | Math.floor(rng() * 4);
  }
}

/** Cells the destroy operator lifts (interior only, deterministic given rng). */
function destroyCells(op: Op, puzzle: Puzzle, b: Int32Array, rng: () => number): number[] {
  if (op === "patch") {
    // Random 3×3 patch inside the interior ring.
    const x0 = 1 + Math.floor(rng() * (SIZE - 4));
    const y0 = 1 + Math.floor(rng() * (SIZE - 4));
    const cells: number[] = [];
    for (let dy = 0; dy < 3; dy++)
      for (let dx = 0; dx < 3; dx++) cells.push((y0 + dy) * SIZE + (x0 + dx));
    return cells;
  }
  const conflicts = cellConflicts(puzzle, b);
  if (op === "rows") {
    // Interior cells of the row with the most incident mismatches.
    let bestRow = 1;
    let bestCount = -1;
    for (let y = 1; y < SIZE - 1; y++) {
      let c = 0;
      for (let x = 1; x < SIZE - 1; x++) c += conflicts[y * SIZE + x] ?? 0;
      if (c > bestCount) {
        bestCount = c;
        bestRow = y;
      }
    }
    const cells: number[] = [];
    for (let x = 1; x < SIZE - 1; x++) cells.push(bestRow * SIZE + x);
    return cells;
  }
  // "worst": the ten most conflicted interior cells (stable order on ties).
  const ranked = [];
  for (let p = 0; p < SIZE * SIZE; p++) if (isInterior(p)) ranked.push(p);
  ranked.sort((a, z) => (conflicts[z] ?? 0) - (conflicts[a] ?? 0) || a - z);
  return ranked.slice(0, 10);
}

interface Machine {
  phase: "idle" | "destroy" | "repair" | "decide" | "flash";
  board: Int32Array;
  snapshot: Int32Array;
  holes: number[];
  pool: number[]; // piece ids lifted and not yet replaced
  op: Op;
  flashTtl: number;
  /** Iterations still queued (Infinity while playing). */
  queued: number;
  forced: Op | null;
  weights: Record<Op, number>;
  usage: Record<Op, number>;
  iter: number;
  best: number;
  history: number[];
  rng: () => number;
}

function freshMachine(puzzle: Puzzle): Machine {
  const rng = mulberry32(SEED);
  const board = Int32Array.from(puzzle.pieces.map((_, i) => i * 4)); // solved
  damageBoard(puzzle, board, rng);
  const score = boardScore(puzzle, board);
  return {
    phase: "idle",
    board,
    snapshot: Int32Array.from(board),
    holes: [],
    pool: [],
    op: "patch",
    flashTtl: 0,
    queued: 0,
    forced: null,
    weights: { patch: 1, rows: 1, worst: 1 },
    usage: { patch: 0, rows: 0, worst: 0 },
    iter: 0,
    best: score,
    history: [score],
    rng,
  };
}

/** Greedy refill of one hole: most-constrained cell, best-fitting placement. */
function repairOne(puzzle: Puzzle, m: Machine): void {
  if (m.holes.length === 0) return;
  // Most placed neighbours first — the classic most-constrained ordering.
  let bestHole = 0;
  let bestNeigh = -1;
  for (let h = 0; h < m.holes.length; h++) {
    const pos = m.holes[h] ?? 0;
    let n = 0;
    if ((m.board[pos - 1] ?? -1) >= 0) n++;
    if ((m.board[pos + 1] ?? -1) >= 0) n++;
    if ((m.board[pos - SIZE] ?? -1) >= 0) n++;
    if ((m.board[pos + SIZE] ?? -1) >= 0) n++;
    if (n > bestNeigh) {
      bestNeigh = n;
      bestHole = h;
    }
  }
  const pos = m.holes.splice(bestHole, 1)[0] ?? 0;
  const up = edgesOf(puzzle, m.board[pos - SIZE] ?? -1);
  const right = edgesOf(puzzle, m.board[pos + 1] ?? -1);
  const down = edgesOf(puzzle, m.board[pos + SIZE] ?? -1);
  const left = edgesOf(puzzle, m.board[pos - 1] ?? -1);
  let bestFit = -1;
  let bestVal = -1;
  let bestIdx = 0;
  for (let i = 0; i < m.pool.length; i++) {
    const piece = puzzle.pieces[m.pool[i] ?? 0];
    if (!piece) continue;
    for (let r = 0; r < 4; r++) {
      const e = rotateEdges(piece, r);
      let fit = 0;
      if (up && e[0] === up[2]) fit++;
      if (right && e[1] === right[3]) fit++;
      if (down && e[2] === down[0]) fit++;
      if (left && e[3] === left[1]) fit++;
      if (fit > bestFit) {
        bestFit = fit;
        bestVal = ((m.pool[i] ?? 0) << 2) | r;
        bestIdx = i;
      }
    }
  }
  m.board[pos] = bestVal;
  m.pool.splice(bestIdx, 1);
}

interface Snapshot {
  cells: (Edges | null)[];
  conflicts: [number, number][];
  lifted: number[];
  score: number;
  best: number;
  iter: number;
  weights: Record<Op, number>;
  usage: Record<Op, number>;
  history: number[];
  phase: Machine["phase"];
  op: Op;
  verdict: { delta: number; accepted: boolean; newBest: boolean } | null;
}

export function AlnsLoopLab() {
  const t = useT(T);
  const engineReady = useEngine();
  const { ref: rootRef, visible } = useRunWhileVisible();
  const [puzzle, setPuzzle] = useState<Puzzle | null>(null);
  const machineRef = useRef<Machine | null>(null);
  const verdictRef = useRef<Snapshot["verdict"]>(null);
  const [playing, setPlaying] = useState(true);
  const [snap, setSnap] = useState<Snapshot | null>(null);

  const publish = useCallback((pz: Puzzle) => {
    const m = machineRef.current;
    if (!m) return;
    const cells: (Edges | null)[] = [];
    for (let p = 0; p < SIZE * SIZE; p++) cells.push(edgesOf(pz, m.board[p] ?? -1));
    setSnap({
      cells,
      conflicts: conflictMarks(pz, m.board),
      lifted: [...m.holes],
      score: boardScore(pz, m.board),
      best: m.best,
      iter: m.iter,
      weights: { ...m.weights },
      usage: { ...m.usage },
      history: [...m.history],
      phase: m.phase,
      op: m.op,
      verdict: verdictRef.current,
    });
  }, []);

  const rebuild = useCallback(() => {
    if (!engineReady) return;
    const pz = getGeneratedSolvedPuzzleFramed(SIZE, COLORS, SEED, true);
    machineRef.current = freshMachine(pz);
    verdictRef.current = null;
    setPuzzle(pz);
    publish(pz);
  }, [engineReady, publish]);

  useEffect(() => {
    // Reset-on-dependency-change effect (engine readiness); the setState lives
    // inside rebuild(), same pattern as the other engine-backed labs.
    // eslint-disable-next-line react-hooks/set-state-in-effect
    rebuild();
  }, [rebuild]);

  // The animation loop: one small state-machine step per tick, gated on
  // visibility so a hidden tab or scrolled-away section costs nothing. Each
  // step touches at most 10 cells × ~40 candidates — far under the 8ms budget.
  useEffect(() => {
    if (!visible || !puzzle) return;
    const pz = puzzle;
    const id = setInterval(() => {
      const m = machineRef.current;
      if (!m) return;
      switch (m.phase) {
        case "idle": {
          if (playing) m.queued = Infinity;
          if (m.queued < 1) return;
          m.phase = "destroy";
          return;
        }
        case "destroy": {
          m.op =
            m.forced ??
            (() => {
              // Roulette-wheel pick over the adaptive weights.
              const total = OPS.reduce((s, o) => s + m.weights[o], 0);
              let u = m.rng() * total;
              for (const o of OPS) {
                u -= m.weights[o];
                if (u <= 0) return o;
              }
              return "patch";
            })();
          m.forced = null;
          m.snapshot = Int32Array.from(m.board);
          m.holes = destroyCells(m.op, pz, m.board, m.rng);
          m.pool = m.holes.map((p) => (m.board[p] ?? 0) >> 2);
          for (const p of m.holes) m.board[p] = -1;
          verdictRef.current = null;
          m.phase = "repair";
          break;
        }
        case "repair": {
          if (m.holes.length > 0) repairOne(pz, m);
          if (m.holes.length === 0 && m.pool.length === 0) m.phase = "decide";
          break;
        }
        case "decide": {
          const oldScore = boardScore(pz, m.snapshot);
          const newScore = boardScore(pz, m.board);
          const delta = newScore - oldScore;
          // Simulated-annealing acceptance at a fixed small temperature.
          const accepted = delta >= 0 || m.rng() < Math.exp(delta / SA_TEMPERATURE);
          if (!accepted) m.board = Int32Array.from(m.snapshot);
          const kept = accepted ? newScore : oldScore;
          const newBest = kept > m.best;
          if (newBest) m.best = kept;
          // Adapt: exponentially decayed reward, big for new bests.
          const reward = newBest ? 4 : delta > 0 ? 2 : accepted ? 0.8 : 0.2;
          m.weights[m.op] = 0.85 * m.weights[m.op] + 0.15 * reward;
          m.usage[m.op] += 1;
          m.iter += 1;
          m.history.push(kept);
          if (m.history.length > HISTORY_CAP) m.history.shift();
          verdictRef.current = { delta, accepted, newBest };
          if (m.queued !== Infinity) m.queued = Math.max(0, m.queued - 1);
          m.flashTtl = 3;
          m.phase = "flash";
          break;
        }
        case "flash": {
          m.flashTtl -= 1;
          if (m.flashTtl <= 0) m.phase = "idle";
          break;
        }
      }
      publish(pz);
    }, TICK_MS);
    return () => clearInterval(id);
  }, [visible, puzzle, playing, publish]);

  const runOnce = (op: Op) => {
    const m = machineRef.current;
    if (!m) return;
    setPlaying(false);
    m.queued = 1;
    m.forced = op;
    if (m.phase === "idle") m.phase = "destroy";
  };

  const redamage = () => {
    const m = machineRef.current;
    if (!m || !puzzle) return;
    damageBoard(puzzle, m.board, m.rng);
    m.phase = "idle";
    m.holes = [];
    m.pool = [];
    verdictRef.current = null;
    m.history.push(boardScore(puzzle, m.board));
    if (m.history.length > HISTORY_CAP) m.history.shift();
    publish(puzzle);
  };

  const weightTotal = useMemo(
    () => (snap ? OPS.reduce((s, o) => s + snap.weights[o], 0) : 1),
    [snap],
  );

  if (!engineReady || !puzzle || !snap) {
    return (
      <div className="flex h-72 items-center justify-center rounded-lg border text-sm text-muted-foreground">
        {t.loading}
      </div>
    );
  }

  const verdictText = snap.verdict
    ? (snap.verdict.accepted ? t.accepted(snap.verdict.delta) : t.rejected(snap.verdict.delta)) +
      (snap.verdict.newBest ? t.newBest : "")
    : t.phase[snap.phase === "flash" ? "decide" : snap.phase];

  return (
    <div ref={rootRef} className="space-y-4 rounded-lg border bg-card p-4">
      <div className="space-y-1">
        <h3 className="text-sm font-semibold">{t.title}</h3>
        <p className="text-xs leading-relaxed text-muted-foreground">{t.intro}</p>
      </div>

      <div className="flex flex-wrap items-start justify-center gap-6">
        <div className="w-full max-w-72 space-y-2">
          <BoardSvg
            width={SIZE}
            height={SIZE}
            cells={snap.cells}
            conflicts={snap.conflicts}
            highlight={snap.lifted}
          />
          <p
            className={cn(
              "rounded-md border px-2 py-1 text-center text-[11px]",
              snap.verdict
                ? snap.verdict.accepted
                  ? "border-emerald-300 bg-emerald-500/10"
                  : "border-rose-300 bg-rose-500/10"
                : "text-muted-foreground",
            )}
          >
            {verdictText}
          </p>
        </div>

        <div className="min-w-60 flex-1 space-y-3">
          <div className="flex items-baseline justify-between rounded-md border px-3 py-2">
            <div>
              <span className="text-2xl font-bold tabular-nums">{snap.score}</span>
              <span className="text-sm text-muted-foreground"> / {MAX}</span>
            </div>
            <div className="text-right text-[11px] text-muted-foreground">
              <div>
                {t.best}: <span className="font-semibold tabular-nums">{snap.best}</span>
              </div>
              <div>
                {t.iterations}: <span className="tabular-nums">{snap.iter}</span>
              </div>
            </div>
          </div>

          <div className="space-y-1">
            <p className="text-[10px] font-medium uppercase tracking-wide text-muted-foreground">
              {t.trace}
            </p>
            <Sparkline history={snap.history} max={MAX} />
          </div>

          <div className="space-y-1.5">
            <p className="text-[10px] font-medium uppercase tracking-wide text-muted-foreground">
              {t.weights}
            </p>
            {OPS.map((o) => (
              <div key={o} className="flex items-center gap-2">
                <span className="w-28 shrink-0 text-[11px]">{t.ops[o]}</span>
                <div className="h-2.5 flex-1 overflow-hidden rounded-sm bg-muted">
                  <div
                    className={cn(
                      "h-full rounded-sm transition-all duration-300",
                      snap.op === o && snap.phase !== "idle" ? "bg-amber-500" : "bg-emerald-500",
                    )}
                    style={{ width: `${(100 * snap.weights[o]) / weightTotal}%` }}
                  />
                </div>
                <span className="w-10 shrink-0 text-right text-[10px] tabular-nums text-muted-foreground">
                  {t.uses(snap.usage[o])}
                </span>
              </div>
            ))}
          </div>
        </div>
      </div>

      <div className="space-y-2">
        <div className="flex flex-wrap justify-center gap-2">
          <Button size="sm" variant="outline" onClick={() => setPlaying((p) => !p)}>
            {playing ? t.pause : t.play}
          </Button>
          <Button size="sm" variant="outline" onClick={redamage}>
            {t.damage}
          </Button>
          <Button
            size="sm"
            variant="outline"
            onClick={() => {
              setPlaying(false);
              rebuild();
            }}
          >
            {t.reset}
          </Button>
        </div>
        <p className="text-center text-[11px] text-muted-foreground">{t.forceOp}</p>
        <div className="flex flex-wrap justify-center gap-1.5">
          {OPS.map((o) => (
            <button
              key={o}
              onClick={() => runOnce(o)}
              className="rounded-md border px-2 py-1 text-xs transition-colors hover:bg-muted"
            >
              {t.ops[o]}
            </button>
          ))}
        </div>
      </div>

      <p className="text-xs text-muted-foreground">{t.note}</p>
    </div>
  );
}

function Sparkline({ history, max }: { history: number[]; max: number }) {
  const W = 220;
  const H = 48;
  const n = Math.max(history.length, 2);
  const lo = Math.min(...history, max * 0.6);
  const x = (i: number) => (i / (n - 1)) * W;
  const y = (v: number) => H - ((v - lo) / Math.max(max - lo, 1)) * (H - 4) - 2;
  const points = history.map((v, i) => `${x(i).toFixed(1)},${y(v).toFixed(1)}`).join(" ");
  // Running best line (plain loop: no closure reassignment during render).
  const bestParts: string[] = [];
  for (let i = 0, running = -1; i < history.length; i++) {
    running = Math.max(running, history[i] ?? -1);
    bestParts.push(`${x(i).toFixed(1)},${y(running).toFixed(1)}`);
  }
  const bestPts = bestParts.join(" ");
  return (
    <svg viewBox={`0 0 ${W} ${H}`} className="h-14 w-full rounded-md border bg-muted/30" role="img" aria-label="Sparkline of the ALNS board score over each destroy-and-repair iteration, with a running-best line and a dashed reference at the maximum">
      <line x1={0} y1={y(max)} x2={W} y2={y(max)} className="stroke-muted-foreground/40" strokeDasharray="3 3" strokeWidth={1} />
      <polyline points={bestPts} fill="none" className="stroke-emerald-500/70" strokeWidth={1.5} />
      <polyline points={points} fill="none" className="stroke-foreground" strokeWidth={1.2} />
    </svg>
  );
}
