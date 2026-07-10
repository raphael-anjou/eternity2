import { useEffect, useState } from "react";
import { useT } from "@/i18n";
import { Button } from "@/components/ui/button";
import { cn } from "@/lib/utils";
import { useRunWhileVisible } from "@/lib/useRunWhileVisible";

// AC-3, made watchable. A synthetic 4×4 edge-matching CSP with 3 colours:
// every cell starts with all 64 candidates (16 tiles × 4 rotations). The user
// places solution pieces one at a time and watches the worklist propagate:
// arcs entering the queue, one arc revised per tick, domains shrinking, the
// ripple dying out at the fixed point. A side counter compares AC-3's work
// against the naive "re-sweep every arc until stable" baseline (AC-1) on the
// exact same instance. Everything is precomputed and deterministic (fixed
// seed), so the component is a pure frame player — no work on the UI thread
// beyond rendering one frame.

const N = 4;
const CELLS = N * N;
const NCOLORS = 3;
const NCAND = CELLS * 4; // 16 tiles × 4 rotations

// Placement order: start in the middle so the ripple has room to travel.
const PLACE_ORDER = [5, 6, 10, 9, 1, 13, 4, 11, 2, 8, 14, 7, 0, 3, 12, 15];

const PALETTE = ["hsl(215 65% 55%)", "hsl(38 85% 55%)", "hsl(150 50% 42%)"];

// --- Deterministic instance -------------------------------------------------

/** Edges per candidate id (tile*4 + rotation), each [N, E, S, W]. */
const CAND_EDGES: number[][] = (() => {
  let s = 987654321 >>> 0;
  const rnd = (m: number): number => {
    s = (Math.imul(s, 1103515245) + 12345) >>> 0;
    return (s >>> 8) % m;
  };
  // Solved board: tile i sits at cell i; edges agree with N/W neighbours.
  const sol: number[][] = [];
  for (let i = 0; i < CELLS; i++) {
    const x = i % N;
    const y = Math.floor(i / N);
    const n = y > 0 ? (sol[i - N]?.[2] ?? 0) : rnd(NCOLORS);
    const w = x > 0 ? (sol[i - 1]?.[1] ?? 0) : rnd(NCOLORS);
    sol.push([n, rnd(NCOLORS), rnd(NCOLORS), w]);
  }
  const out: number[][] = [];
  for (let tile = 0; tile < CELLS; tile++) {
    const base = sol[tile] ?? [0, 0, 0, 0];
    for (let r = 0; r < 4; r++) {
      out.push([0, 1, 2, 3].map((d) => base[(d + r) % 4] ?? 0));
    }
  }
  return out;
})();

/** Neighbour j lies in direction d (0=N,1=E,2=S,3=W) from cell i. */
const ADJ: { j: number; d: number }[][] = (() => {
  const out: { j: number; d: number }[][] = [];
  for (let i = 0; i < CELLS; i++) {
    const x = i % N;
    const y = Math.floor(i / N);
    const list: { j: number; d: number }[] = [];
    if (y > 0) list.push({ j: i - N, d: 0 });
    if (x < N - 1) list.push({ j: i + 1, d: 1 });
    if (y < N - 1) list.push({ j: i + N, d: 2 });
    if (x > 0) list.push({ j: i - 1, d: 3 });
    out.push(list);
  }
  return out;
})();

function edgeColor(cand: number, d: number): number {
  return CAND_EDGES[cand]?.[d] ?? -1;
}

/** Candidate a in a cell is compatible with b in the neighbour in direction d. */
function compat(a: number, d: number, b: number): boolean {
  const ea = edgeColor(a, d);
  return ea !== -1 && ea === edgeColor(b, (d + 2) % 4);
}

function revise(
  domains: Set<number>[],
  i: number,
  j: number,
  d: number,
): { removed: number; checks: number } {
  const di = domains[i];
  const dj = domains[j];
  if (!di || !dj) return { removed: 0, checks: 0 };
  let removed = 0;
  let checks = 0;
  for (const a of [...di]) {
    let supported = false;
    for (const b of dj) {
      checks++;
      if (compat(a, d, b)) {
        supported = true;
        break;
      }
    }
    if (!supported) {
      di.delete(a);
      removed++;
    }
  }
  return { removed, checks };
}

// --- Precomputed trace --------------------------------------------------------

interface Arc {
  i: number; // cell being revised
  j: number; // cell it is revised against
  d: number; // direction from i to j
}

interface Frame {
  kind: "start" | "place" | "revise";
  cell: number;
  against: number;
  removed: number;
  requeued: number;
  sizes: number[];
  queue: Arc[];
  placedCount: number;
  fixedPoint: boolean;
  ac3Rev: number;
  ac3Chk: number;
  ac1Rev: number;
  ac1Chk: number;
}

function buildFrames(): Frame[] {
  const domains: Set<number>[] = Array.from(
    { length: CELLS },
    () => new Set(Array.from({ length: NCAND }, (_, k) => k)),
  );
  const frames: Frame[] = [];
  let ac3Rev = 0;
  let ac3Chk = 0;
  let ac1Rev = 0;
  let ac1Chk = 0;
  const sizes = (): number[] => domains.map((s) => s.size);

  frames.push({
    kind: "start", cell: -1, against: -1, removed: 0, requeued: 0,
    sizes: sizes(), queue: [], placedCount: 0, fixedPoint: true,
    ac3Rev, ac3Chk, ac1Rev, ac1Chk,
  });

  for (let p = 0; p < PLACE_ORDER.length; p++) {
    const cell = PLACE_ORDER[p] ?? 0;
    const solCand = cell * 4; // solution tile = cell index, rotation 0

    // Naive baseline on a copy of the post-placement state: sweep every arc,
    // over and over, until one full pass changes nothing (AC-1).
    {
      const copy = domains.map((s) => new Set(s));
      copy[cell] = new Set([solCand]);
      let changed = true;
      while (changed) {
        changed = false;
        for (let i = 0; i < CELLS; i++) {
          for (const { j, d } of ADJ[i] ?? []) {
            ac1Rev++;
            const r = revise(copy, i, j, d);
            ac1Chk += r.checks;
            if (r.removed > 0) changed = true;
          }
        }
      }
    }

    // The real thing: place, enqueue only the arcs aimed at the changed cell.
    const before = domains[cell]?.size ?? 1;
    domains[cell] = new Set([solCand]);
    const queue: Arc[] = [];
    const inQ = new Set<string>();
    const push = (i: number, j: number, d: number): void => {
      const key = `${i}:${j}`;
      if (inQ.has(key)) return;
      inQ.add(key);
      queue.push({ i, j, d });
    };
    for (const { j, d } of ADJ[cell] ?? []) push(j, cell, (d + 2) % 4);

    frames.push({
      kind: "place", cell, against: -1, removed: Math.max(0, before - 1),
      requeued: queue.length, sizes: sizes(), queue: [...queue],
      placedCount: p + 1, fixedPoint: false, ac3Rev, ac3Chk, ac1Rev, ac1Chk,
    });

    while (queue.length > 0) {
      const arc = queue.shift();
      if (!arc) break;
      inQ.delete(`${arc.i}:${arc.j}`);
      ac3Rev++;
      const r = revise(domains, arc.i, arc.j, arc.d);
      ac3Chk += r.checks;
      let requeued = 0;
      if (r.removed > 0) {
        for (const { j: k, d: dk } of ADJ[arc.i] ?? []) {
          if (k === arc.j) continue;
          const len = queue.length;
          push(k, arc.i, (dk + 2) % 4);
          if (queue.length > len) requeued++;
        }
      }
      frames.push({
        kind: "revise", cell: arc.i, against: arc.j, removed: r.removed,
        requeued, sizes: sizes(), queue: [...queue], placedCount: p + 1,
        fixedPoint: queue.length === 0, ac3Rev, ac3Chk, ac1Rev, ac1Chk,
      });
    }
  }
  return frames;
}

const FRAMES: Frame[] = buildFrames();

// --- Rendering ----------------------------------------------------------------

const CS = 52; // cell size
const GAP = 24;
const PAD = 8;
const W = PAD * 2 + N * CS + (N - 1) * GAP;

function cellName(i: number): string {
  return `${String.fromCharCode(65 + (i % N))}${Math.floor(i / N) + 1}`;
}

function center(i: number): { x: number; y: number } {
  return {
    x: PAD + (i % N) * (CS + GAP) + CS / 2,
    y: PAD + Math.floor(i / N) * (CS + GAP) + CS / 2,
  };
}

function ArcArrow({ arc, active }: { arc: Arc; active: boolean }) {
  const from = center(arc.j);
  const to = center(arc.i);
  const dx = Math.sign(to.x - from.x);
  const dy = Math.sign(to.y - from.y);
  const ox = dy * 6;
  const oy = -dx * 6;
  const sx = from.x + dx * (CS / 2 + 2) + ox;
  const sy = from.y + dy * (CS / 2 + 2) + oy;
  const ex = to.x - dx * (CS / 2 + 4) + ox;
  const ey = to.y - dy * (CS / 2 + 4) + oy;
  const w = active ? 2.5 : 1.5;
  return (
    <g
      className={active ? "stroke-primary" : "stroke-muted-foreground"}
      opacity={active ? 1 : 0.4}
      fill="none"
    >
      <line x1={sx} y1={sy} x2={ex} y2={ey} strokeWidth={w} />
      <polyline
        points={`${ex - dx * 6 + dy * 4},${ey - dy * 6 + dx * 4} ${ex},${ey} ${ex - dx * 6 - dy * 4},${ey - dy * 6 - dx * 4}`}
        strokeWidth={w}
      />
    </g>
  );
}

function PlacedTile({ x, y, cand }: { x: number; y: number; cand: number }) {
  const e = CAND_EDGES[cand] ?? [0, 0, 0, 0];
  const cx = x + CS / 2;
  const cy = y + CS / 2;
  const tri = [
    `${x},${y} ${x + CS},${y} ${cx},${cy}`, // N
    `${x + CS},${y} ${x + CS},${y + CS} ${cx},${cy}`, // E
    `${x + CS},${y + CS} ${x},${y + CS} ${cx},${cy}`, // S
    `${x},${y + CS} ${x},${y} ${cx},${cy}`, // W
  ];
  return (
    <g>
      {tri.map((pts, d) => (
        <polygon key={d} points={pts} fill={PALETTE[e[d] ?? 0]} />
      ))}
      <rect x={x} y={y} width={CS} height={CS} rx={6} fill="none" className="stroke-border" strokeWidth={1.5} />
    </g>
  );
}

const T = {
  en: {
    title: "Watch AC-3 propagate — one arc at a time",
    intro:
      "A 4×4 edge-matching toy with 3 colours: each cell starts with all 64 candidates (16 tiles × 4 rotations). Place a piece and watch the worklist: arcs aimed at the changed cell enter the queue, each revision deletes unsupported candidates, and every deletion re-enqueues the arcs pointing at the shrunken domain — until the queue runs dry.",
    place: "Place next piece",
    step: "Step",
    play: "Play",
    pause: "Pause",
    reset: "Reset",
    placed: "placed",
    queueLen: "arcs in queue",
    fixedPoint: "fixed point — queue empty",
    ac3: "AC-3 (worklist)",
    ac1: "naive re-sweep (AC-1)",
    revisions: "arc revisions",
    checks: "support checks",
    legend: "Number = candidates left in the cell's domain. Arrows = arcs waiting in the queue (the bold arrow is the arc being revised).",
    narStart:
      "Fresh board: every cell keeps all 64 candidates. Nothing has changed yet, so the queue is empty.",
    narPlace: (c: string, q: number) =>
      `Piece placed at ${c} — its domain collapses to 1 candidate. The ${q} arcs aimed at ${c} enter the queue: each neighbour must now re-check its supports.`,
    narHit: (a: string, b: string, k: number, m: number) =>
      `Revise ${a} against ${b}: ${k} candidate${k === 1 ? "" : "s"} in ${a} had no support left and ${k === 1 ? "was" : "were"} deleted.${m > 0 ? ` ${a} shrank, so ${m} arc${m === 1 ? "" : "s"} aimed at it re-enter the queue — the ripple spreads.` : " No new arcs to enqueue."}`,
    narMiss: (a: string, b: string) =>
      `Revise ${a} against ${b}: every candidate in ${a} still has a support in ${b}. Nothing deleted, the arc is simply dropped — this is how the ripple dies out.`,
    note: "The counters are the whole point: AC-3 only revisits arcs whose far domain actually changed, while the naive fixed-point loop re-sweeps all 48 arcs until a full pass is clean. Same deletions, wildly different work — and AC-2001 would cut the support checks further by remembering where each search stopped.",
  },
  fr: {
    title: "Regardez AC-3 se propager — un arc à la fois",
    intro:
      "Un jouet d'appariement d'arêtes 4×4 à 3 couleurs : chaque case démarre avec ses 64 candidats (16 tuiles × 4 rotations). Posez une pièce et observez la liste de travail : les arcs visant la case modifiée entrent dans la file, chaque révision supprime les candidats sans support, et chaque suppression remet en file les arcs pointant vers le domaine rétréci — jusqu'à ce que la file se vide.",
    place: "Poser la pièce suivante",
    step: "Pas",
    play: "Lecture",
    pause: "Pause",
    reset: "Réinitialiser",
    placed: "posées",
    queueLen: "arcs en file",
    fixedPoint: "point fixe — file vide",
    ac3: "AC-3 (liste de travail)",
    ac1: "balayage naïf (AC-1)",
    revisions: "révisions d'arcs",
    checks: "tests de support",
    legend: "Le nombre = candidats restants dans le domaine de la case. Les flèches = arcs en attente dans la file (la flèche épaisse est l'arc en cours de révision).",
    narStart:
      "Plateau vierge : chaque case garde ses 64 candidats. Rien n'a encore changé, la file est vide.",
    narPlace: (c: string, q: number) =>
      `Pièce posée en ${c} — son domaine s'effondre à 1 candidat. Les ${q} arcs visant ${c} entrent dans la file : chaque voisine doit revérifier ses supports.`,
    narHit: (a: string, b: string, k: number, m: number) =>
      `Révision de ${a} contre ${b} : ${k} candidat${k === 1 ? "" : "s"} de ${a} n'avai${k === 1 ? "t" : "ent"} plus de support — supprimé${k === 1 ? "" : "s"}.${m > 0 ? ` ${a} a rétréci, donc ${m} arc${m === 1 ? "" : "s"} pointant vers elle repasse${m === 1 ? "" : "nt"} en file — l'onde se propage.` : " Aucun arc à remettre en file."}`,
    narMiss: (a: string, b: string) =>
      `Révision de ${a} contre ${b} : chaque candidat de ${a} garde un support dans ${b}. Rien à supprimer, l'arc est simplement retiré — c'est ainsi que l'onde s'éteint.`,
    note: "Les compteurs sont tout l'enjeu : AC-3 ne revisite que les arcs dont le domaine d'en face a réellement changé, quand la boucle naïve rebalaye les 48 arcs jusqu'à une passe complète sans changement. Mêmes suppressions, travail radicalement différent — et AC-2001 réduirait encore les tests de support en mémorisant où chaque recherche s'était arrêtée.",
  },
};

export function AcThreeLab() {
  const t = useT(T);
  const { ref: rootRef, visible } = useRunWhileVisible();
  const [idx, setIdx] = useState(0);
  const [playing, setPlaying] = useState(false);

  const frame = FRAMES[Math.min(idx, FRAMES.length - 1)];
  const next = FRAMES[idx + 1];
  const atBoundary = next === undefined || next.kind === "place";

  useEffect(() => {
    if (!playing || !visible) return;
    const id = setInterval(() => {
      setIdx((cur) => {
        const nf = FRAMES[cur + 1];
        return nf !== undefined && nf.kind !== "place" ? cur + 1 : cur;
      });
    }, 480);
    return () => clearInterval(id);
  }, [playing, visible]);

  useEffect(() => {
    // Auto-pause once the propagation reaches a stage boundary: placing the
    // next piece stays a user decision.
    if (playing && atBoundary) {
      // eslint-disable-next-line react-hooks/set-state-in-effect
      setPlaying(false);
    }
  }, [playing, atBoundary]);

  if (!frame) return null;

  const placedCells = new Set(PLACE_ORDER.slice(0, frame.placedCount));
  const narration =
    frame.kind === "start"
      ? t.narStart
      : frame.kind === "place"
        ? t.narPlace(cellName(frame.cell), frame.requeued)
        : frame.removed > 0
          ? t.narHit(cellName(frame.cell), cellName(frame.against), frame.removed, frame.requeued)
          : t.narMiss(cellName(frame.cell), cellName(frame.against));

  return (
    <div ref={rootRef} className="space-y-4 rounded-lg border bg-card p-4">
      <div className="space-y-1">
        <h3 className="text-sm font-semibold">{t.title}</h3>
        <p className="text-xs leading-relaxed text-muted-foreground">{t.intro}</p>
      </div>

      <div className="flex flex-wrap items-start justify-center gap-6">
        <div className="w-full max-w-72">
          <svg viewBox={`0 0 ${W} ${W}`} className="w-full" role="img" aria-label="A grid of cells showing AC-3 arc consistency propagate, each cell's candidate-domain size shrinking as one arc is revised per tick, with the current cell and the arc it is checked against highlighted">
            {/* cells */}
            {Array.from({ length: CELLS }, (_, i) => {
              const x = PAD + (i % N) * (CS + GAP);
              const y = PAD + Math.floor(i / N) * (CS + GAP);
              const size = frame.sizes[i] ?? NCAND;
              if (placedCells.has(i)) {
                return (
                  <g key={i}>
                    <PlacedTile x={x} y={y} cand={i * 4} />
                    {frame.kind === "place" && frame.cell === i && (
                      <rect x={x - 2} y={y - 2} width={CS + 4} height={CS + 4} rx={8} fill="none" className="stroke-primary" strokeWidth={2.5} />
                    )}
                  </g>
                );
              }
              const isRevised = frame.kind === "revise" && frame.cell === i;
              return (
                <g key={i}>
                  <rect
                    x={x} y={y} width={CS} height={CS} rx={6}
                    className="fill-primary"
                    fillOpacity={0.06 + 0.5 * (1 - size / NCAND)}
                  />
                  <rect
                    x={x} y={y} width={CS} height={CS} rx={6} fill="none"
                    className={cn(
                      isRevised
                        ? frame.removed > 0
                          ? "stroke-rose-500"
                          : "stroke-amber-500"
                        : "stroke-border",
                    )}
                    strokeWidth={isRevised ? 2.5 : 1}
                  />
                  <text x={x + 4} y={y + 11} fontSize={7} className="fill-muted-foreground">
                    {cellName(i)}
                  </text>
                  <text
                    x={x + CS / 2} y={y + CS / 2 + 6} fontSize={17} fontWeight={600}
                    textAnchor="middle" className="fill-foreground" style={{ fontVariantNumeric: "tabular-nums" }}
                  >
                    {size}
                  </text>
                </g>
              );
            })}
            {/* queued arcs */}
            {frame.queue.map((arc) => (
              <ArcArrow key={`${arc.i}:${arc.j}`} arc={arc} active={false} />
            ))}
            {/* current arc */}
            {frame.kind === "revise" && (
              <ArcArrow
                arc={{ i: frame.cell, j: frame.against, d: 0 }}
                active
              />
            )}
          </svg>
          <p className="mt-1 text-center text-[11px] text-muted-foreground">{t.legend}</p>
        </div>

        <div className="min-w-56 flex-1 space-y-3">
          <div className="flex flex-wrap gap-2 text-[11px]">
            <span className="rounded-md border px-2 py-1 tabular-nums">
              {frame.placedCount}/16 {t.placed}
            </span>
            <span className="rounded-md border px-2 py-1 tabular-nums">
              {frame.queue.length} {t.queueLen}
            </span>
            {frame.fixedPoint && frame.kind !== "start" && (
              <span className="rounded-md border border-emerald-300 bg-emerald-500/10 px-2 py-1">
                {t.fixedPoint}
              </span>
            )}
          </div>

          <div className="grid grid-cols-2 gap-2">
            <div className="rounded-md border px-3 py-2">
              <div className="text-[10px] font-medium uppercase tracking-wide text-muted-foreground">{t.ac3}</div>
              <div className="mt-1 text-sm tabular-nums">
                <span className="font-semibold">{frame.ac3Rev}</span>{" "}
                <span className="text-[11px] text-muted-foreground">{t.revisions}</span>
              </div>
              <div className="text-sm tabular-nums">
                <span className="font-semibold">{frame.ac3Chk.toLocaleString()}</span>{" "}
                <span className="text-[11px] text-muted-foreground">{t.checks}</span>
              </div>
            </div>
            <div className="rounded-md border px-3 py-2">
              <div className="text-[10px] font-medium uppercase tracking-wide text-muted-foreground">{t.ac1}</div>
              <div className="mt-1 text-sm tabular-nums">
                <span className="font-semibold">{frame.ac1Rev.toLocaleString()}</span>{" "}
                <span className="text-[11px] text-muted-foreground">{t.revisions}</span>
              </div>
              <div className="text-sm tabular-nums">
                <span className="font-semibold">{frame.ac1Chk.toLocaleString()}</span>{" "}
                <span className="text-[11px] text-muted-foreground">{t.checks}</span>
              </div>
            </div>
          </div>

          <p className="min-h-16 rounded-md border bg-muted/40 px-3 py-2 text-xs leading-relaxed text-muted-foreground">
            {narration}
          </p>
        </div>
      </div>

      <div className="flex flex-wrap justify-center gap-2">
        <Button
          size="sm"
          onClick={() => {
            if (next !== undefined && next.kind === "place") {
              setIdx(idx + 1);
              setPlaying(true);
            }
          }}
          disabled={!atBoundary || next === undefined}
        >
          {t.place}
        </Button>
        <Button
          size="sm"
          variant="outline"
          onClick={() => {
            setPlaying(false);
            setIdx((cur) => Math.min(cur + 1, FRAMES.length - 1));
          }}
          disabled={next === undefined}
        >
          {t.step}
        </Button>
        <Button
          size="sm"
          variant="outline"
          onClick={() => setPlaying((p) => !p)}
          disabled={!playing && atBoundary}
        >
          {playing ? t.pause : t.play}
        </Button>
        <Button
          size="sm"
          variant="outline"
          onClick={() => {
            setPlaying(false);
            setIdx(0);
          }}
        >
          {t.reset}
        </Button>
      </div>

      <p className="text-xs text-muted-foreground">{t.note}</p>
    </div>
  );
}
