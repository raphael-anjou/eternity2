import { useCallback, useEffect, useRef, useState } from "react";
import { useT } from "@/i18n";
import { useIsClient } from "@/lib/utils";
import { BoardSvg } from "@/components/board/BoardSvg";
import { rotateEdges } from "@/lib/types";
import type { Edges } from "@/lib/bucas";
import { getOfficialPuzzle, initEngine } from "@/engine";
import { FRAME_BUDGET_MS, useRunWhileVisible } from "@/lib/useRunWhileVisible";

// Draw four real interior pieces at random and try to make a 2x2 that matches.
// Almost every draw fails. The component animates draw after draw, tallies how
// many were forbidden vs feasible, and shows the running rate creeping toward the
// exact 99.72%. Real pieces, real feasibility test, in the browser.

type Quad = readonly [Edges, Edges, Edges, Edges];

// Can these four pieces (TL, TR, BL, BR) make a matching 2x2 under some rotation
// of each? Returns the matched rotations if so, else null.
function solveSquare(q: Quad): [Edges, Edges, Edges, Edges] | null {
  const [tl, tr, bl, br] = q;
  for (let a = 0; a < 4; a++) {
    const A = rotateEdges(tl, a);
    for (let b = 0; b < 4; b++) {
      const B = rotateEdges(tr, b);
      if (A[1] !== B[3]) continue;
      for (let c = 0; c < 4; c++) {
        const C = rotateEdges(bl, c);
        if (A[2] !== C[0]) continue;
        for (let d = 0; d < 4; d++) {
          const D = rotateEdges(br, d);
          if (B[2] === D[0] && C[1] === D[3]) return [A, B, C, D];
        }
      }
    }
  }
  return null;
}

const T = {
  en: {
    title: "Draw four pieces, try to fit them",
    intro:
      "Each round picks four interior pieces at random and searches every rotation for a matching 2x2. Watch how rarely one works.",
    start: "Start drawing",
    stop: "Stop",
    speed: "Speed",
    feasible: "fits",
    forbidden: "forbidden",
    triesLabel: "draws",
    fitsLabel: "fits found",
    rateLabel: "forbidden rate",
    exact: "exact value: 99.72%",
    statusFits: "This one fits.",
    statusForbidden: "Forbidden — no rotation makes all four edges match.",
    busy: "Loading pieces…",
  },
  fr: {
    title: "Tirez quatre pièces, essayez de les assembler",
    intro:
      "Chaque tour tire quatre pièces intérieures au hasard et cherche, sur toutes les rotations, un 2x2 qui s'accorde. Regardez à quel point cela réussit rarement.",
    start: "Lancer les tirages",
    stop: "Arrêter",
    speed: "Vitesse",
    feasible: "s'accorde",
    forbidden: "interdit",
    triesLabel: "tirages",
    fitsLabel: "accords trouvés",
    rateLabel: "taux d'interdits",
    exact: "valeur exacte : 99,72 %",
    statusFits: "Celui-ci s'accorde.",
    statusForbidden: "Interdit — aucune rotation n'accorde les quatre bords.",
    busy: "Chargement des pièces…",
  },
};

export function ForbiddenPatchLab() {
  const t = useT(T);
  const isClient = useIsClient();
  const [interior, setInterior] = useState<Edges[] | null>(null);
  const [running, setRunning] = useState(false);
  const [tries, setTries] = useState(0);
  const [fits, setFits] = useState(0);
  const [cells, setCells] = useState<(Edges | null)[]>([null, null, null, null]);
  const [lastFeasible, setLastFeasible] = useState<boolean | null>(null);
  // Speed 1..5: from one slow draw at a time up to thousands per tick.
  const [speed, setSpeed] = useState(2);
  const seed = useRef(123456789);
  const { ref: rootRef, visible } = useRunWhileVisible();

  useEffect(() => {
    let alive = true;
    void initEngine().then(() => {
      const puzzle = getOfficialPuzzle();
      const inner = puzzle.pieces.filter((e) => !e.includes(0)) as Edges[];
      if (alive) setInterior(inner);
    });
    return () => {
      alive = false;
    };
  }, []);

  // Deterministic PRNG so the demo is repeatable; varied per draw.
  const rand = useCallback((n: number) => {
    seed.current = (seed.current * 1103515245 + 12345) & 0x7fffffff;
    return seed.current % n;
  }, []);

  const drawOne = useCallback(
    (pieces: Edges[]): { quad: Quad; solved: ReturnType<typeof solveSquare> } => {
      const picked: Edges[] = [];
      const used = new Set<number>();
      const fallback: Edges = pieces[0] ?? [0, 0, 0, 0];
      while (picked.length < 4) {
        const r = rand(pieces.length);
        if (used.has(r)) continue;
        used.add(r);
        picked.push(pieces[r] ?? fallback);
      }
      const quad: Quad = [
        picked[0] ?? fallback,
        picked[1] ?? fallback,
        picked[2] ?? fallback,
        picked[3] ?? fallback,
      ];
      return { quad, solved: solveSquare(quad) };
    },
    [rand],
  );

  // One tick draws `batch` patches; we render the last one and add up the rest.
  // Higher speeds draw many per tick so the counts climb fast. Each tick is
  // time-boxed by wall clock (only the draws actually done are tallied), and the
  // loop pauses entirely while off-screen or in a hidden tab.
  useEffect(() => {
    if (!running || !interior || !visible) return;
    const pieces = interior;
    const batch = [1, 1, 20, 200, 2000][speed - 1] ?? 1;
    const delay = speed === 1 ? 600 : 90;
    const id = setInterval(() => {
      let newFits = 0;
      let done = 0;
      let last = drawOne(pieces);
      const deadline = performance.now() + FRAME_BUDGET_MS;
      for (let n = 0; n < batch; n++) {
        const d = drawOne(pieces);
        if (d.solved) newFits++;
        last = d;
        done++;
        if (performance.now() >= deadline) break;
      }
      setTries((x) => x + done);
      setFits((x) => x + newFits);
      if (last.solved) {
        setCells([last.solved[0], last.solved[1], last.solved[2], last.solved[3]]);
        setLastFeasible(true);
      } else {
        setCells([last.quad[0], last.quad[1], last.quad[2], last.quad[3]]);
        setLastFeasible(false);
      }
    }, delay);
    return () => clearInterval(id);
  }, [running, interior, speed, drawOne, visible]);

  if (!isClient || !interior) {
    return (
      <div className="flex h-72 items-center justify-center rounded-lg border text-sm text-muted-foreground">
        {t.busy}
      </div>
    );
  }

  const rate = tries > 0 ? (100 * (tries - fits)) / tries : 0;

  return (
    <div ref={rootRef} className="space-y-4">
      <p className="mx-auto max-w-2xl text-center text-sm text-muted-foreground">{t.intro}</p>

      <div className="mx-auto max-w-xs space-y-2">
        <div
          className={
            "rounded-lg border-2 p-2 transition-colors " +
            (lastFeasible === null
              ? ""
              : lastFeasible
                ? "border-emerald-500"
                : "border-red-500")
          }
        >
          <BoardSvg width={2} height={2} cells={cells} />
        </div>
        {lastFeasible !== null && (
          <p
            className={
              "text-center text-sm font-medium " +
              (lastFeasible
                ? "text-emerald-700 dark:text-emerald-400"
                : "text-red-600 dark:text-red-400")
            }
          >
            {lastFeasible ? t.statusFits : t.statusForbidden}
          </p>
        )}
      </div>

      <div className="flex flex-wrap items-center justify-center gap-x-5 gap-y-2">
        <button
          onClick={() => setRunning((r) => !r)}
          className="rounded-md bg-primary px-4 py-1.5 text-sm font-semibold text-primary-foreground transition-opacity hover:opacity-90"
        >
          {running ? t.stop : t.start}
        </button>
        <label className="flex items-center gap-2 text-sm text-muted-foreground">
          {t.speed}
          <input
            type="range"
            min={1}
            max={5}
            value={speed}
            onChange={(e) => setSpeed(Number(e.target.value))}
            className="w-32"
          />
        </label>
      </div>

      <div className="mx-auto grid max-w-md grid-cols-3 gap-3 text-center">
        <div className="rounded-lg border p-3">
          <div className="text-2xl font-bold tabular-nums">{tries.toLocaleString()}</div>
          <div className="text-xs text-muted-foreground">{t.triesLabel}</div>
        </div>
        <div className="rounded-lg border p-3">
          <div className="text-2xl font-bold tabular-nums text-emerald-600">{fits}</div>
          <div className="text-xs text-muted-foreground">{t.fitsLabel}</div>
        </div>
        <div className="rounded-lg border p-3">
          <div className="text-2xl font-bold tabular-nums">{rate.toFixed(1)}%</div>
          <div className="text-xs text-muted-foreground">{t.rateLabel}</div>
        </div>
      </div>
      <p className="text-center text-xs text-muted-foreground">{t.exact}</p>
    </div>
  );
}
