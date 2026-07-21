// Hint-layout geometries — the single source of truth for the Hint Study's
// board illustrations. Every layout is a pure function (size) -> cell indices,
// mirroring the Rust generator in
// `research/experiments/../engine/src/bin/hint_variants.rs` so the diagrams show
// exactly the layouts the experiment measured. Cells are row-major indices into
// an n×n board; a hint pins one cell to its true piece.
//
// Add a new geometry here once, and every diagram, the picker, and the
// complexity plot pick it up for free.

export type Layout = {
  /** stable id, matches the experiment's variant name where one exists */
  id: string;
  /** short human label (English; diagrams localise their own captions) */
  label: string;
  /** family, for grouping and colour in plots */
  family: "spread" | "clustered" | "linear" | "clue" | "baseline";
  /** cells (row-major indices) for a given board size */
  cells: (n: number) => number[];
};

const idx = (n: number, r: number, c: number) => r * n + c;
const isBorder = (n: number, cell: number) => {
  const r = Math.floor(cell / n);
  const c = cell % n;
  return r === 0 || c === 0 || r === n - 1 || c === n - 1;
};

// --- geometry generators (each pure in n) ---------------------------------

/** every `stride`-th cell in both axes, offset off the rim. */
export function lattice(n: number, stride: number, offset = 1): number[] {
  const cells: number[] = [];
  for (let r = offset; r < n; r += stride)
    for (let c = offset; c < n; c += stride) cells.push(idx(n, r, c));
  return cells;
}

/** the first `k` cells in row-major order (fills top rows). */
export function contiguous(n: number, k: number): number[] {
  return Array.from({ length: Math.min(k, n * n) }, (_, i) => i);
}

/** a solid s×s block with top-left corner at (x0,y0). */
export function squareBlock(n: number, x0: number, y0: number, s: number): number[] {
  const cells: number[] = [];
  for (let dy = 0; dy < s; dy++)
    for (let dx = 0; dx < s; dx++) {
      const x = x0 + dx;
      const y = y0 + dy;
      if (x < n && y < n) cells.push(idx(n, y, x));
    }
  return cells;
}

/** a full row r. */
export function row(n: number, r: number): number[] {
  return Array.from({ length: n }, (_, c) => idx(n, r, c));
}

/** a full column c. */
export function col(n: number, c: number): number[] {
  return Array.from({ length: n }, (_, r) => idx(n, r, c));
}

/** a plus/cross centred at (cx,cy): the row and column through it. */
export function cross(n: number, cx: number, cy: number): number[] {
  const set = new Set<number>([...row(n, cy), ...col(n, cx)]);
  return [...set].sort((a, b) => a - b);
}

/** interior-only lattice, first `k` cells. */
export function interiorLattice(n: number, stride: number, k: number): number[] {
  return lattice(n, stride, 1).filter((c) => !isBorder(n, c)).slice(0, k);
}

/** border ring cells, first `k`. */
export function borderRing(n: number, k: number): number[] {
  const cells: number[] = [];
  for (let cell = 0; cell < n * n && cells.length < k; cell++)
    if (isBorder(n, cell)) cells.push(cell);
  return cells;
}

/** the E2 five-clue SHAPE: centre + four inset from the corners. */
export function clueShape(n: number): number[] {
  const q = Math.floor(n / 4);
  return [
    idx(n, Math.floor(n / 2), Math.floor(n / 2)),
    idx(n, q, q),
    idx(n, q, n - 1 - q),
    idx(n, n - 1 - q, q),
    idx(n, n - 1 - q, n - 1 - q),
  ];
}

/** E2's clue geometry, clustered: a k×k block at each of the five clue anchors. */
export function clusteredClues(n: number, k: number): number[] {
  const off = 1;
  const anchors: Array<[number, number]> = [
    [off, off],
    [n - off - k, off],
    [off, n - off - k],
    [n - off - k, n - off - k],
    // Integer-division per axis to match the Rust generator's `n/2 - k/2` (two
    // usize divisions), NOT floor(n/2 - k/2) — they differ for odd k (e.g. k=3:
    // 8−1=7 vs floor(6.5)=6), which would desync this diagram from the board.
    [Math.floor(n / 2) - Math.floor(k / 2), Math.floor(n / 2) - Math.floor(k / 2)],
  ];
  const set = new Set<number>();
  for (const [x0, y0] of anchors) for (const c of squareBlock(n, x0, y0, k)) set.add(c);
  return [...set].sort((a, b) => a - b);
}

/** A genuinely spread lattice whose count is *closest* to `target`. Mirrors the
 * Rust generator's `spread_count` EXACTLY: pick the stride whose full offset-1
 * lattice count is nearest to the target and return it unmodified — never a
 * within-lattice downsample, which collapses low counts into near-linear rows (a
 * geometry change masquerading as a count change). So the returned count is a
 * lattice count near `target`, not `target` exactly. Keeping this identical to
 * the Rust means a diagram can never disagree with the measured board. */
export function spreadCount(n: number, target: number): number[] {
  let best = lattice(n, 2, 1);
  let bestGap = Math.abs(best.length - target);
  for (let stride = 2; stride <= n; stride++) {
    const cells = lattice(n, stride, 1);
    if (cells.length === 0) continue;
    const gap = Math.abs(cells.length - target);
    if (gap < bestGap) { best = cells; bestGap = gap; }
  }
  return best;
}

// --- the catalogue --------------------------------------------------------
// The named layouts the study illustrates. Keep ids aligned with the experiment
// variant names so a diagram and its measured row share a key.

export const LAYOUTS: Record<string, Layout> = {
  geom_scattered: { id: "geom_scattered", label: "Scattered lattice", family: "spread", cells: (n) => lattice(n, 4, 1) },
  geom_contiguous: { id: "geom_contiguous", label: "Contiguous block", family: "clustered", cells: (n) => contiguous(n, lattice(n, 4, 1).length) },
  geom_interior: { id: "geom_interior", label: "Interior only", family: "spread", cells: (n) => interiorLattice(n, 3, lattice(n, 4, 1).length) },
  geom_border: { id: "geom_border", label: "Border only", family: "linear", cells: (n) => borderRing(n, lattice(n, 4, 1).length) },
  clue_shape_5: { id: "clue_shape_5", label: "The five-clue shape", family: "clue", cells: (n) => clueShape(n) },
  ladder_clustered_k2: { id: "ladder_clustered_k2", label: "3×3 blocks (k=2)", family: "clustered", cells: (n) => clusteredClues(n, 2) },
  ladder_clustered_k3: { id: "ladder_clustered_k3", label: "3×3 blocks (k=3)", family: "clustered", cells: (n) => clusteredClues(n, 3) },
  ladder_clustered_k4: { id: "ladder_clustered_k4", label: "4×4 blocks (k=4)", family: "clustered", cells: (n) => clusteredClues(n, 4) },
  cross_center: { id: "cross_center", label: "Central cross", family: "linear", cells: (n) => cross(n, Math.floor(n / 2), Math.floor(n / 2)) },
  rows_odd: { id: "rows_odd", label: "Odd rows", family: "linear", cells: (n) => [1, 3, 5].flatMap((r) => row(n, r)) },
  baseline_00: { id: "baseline_00", label: "No hints", family: "baseline", cells: () => [] },
};

/** the pinned-seam FLOOR: interior seams with BOTH endpoints pinned (guaranteed
 * correct, so free score). This is the confound the study corrects for; exposing
 * it here lets a diagram show the free score a layout banks before any search. */
export function pinnedSeamFloor(n: number, cells: number[]): number {
  const pinned = new Set(cells);
  let banked = 0;
  for (let r = 0; r < n; r++)
    for (let c = 0; c < n; c++) {
      const cell = idx(n, r, c);
      if (c < n - 1 && pinned.has(cell) && pinned.has(cell + 1)) banked++;
      if (r < n - 1 && pinned.has(cell) && pinned.has(cell + n)) banked++;
    }
  return banked;
}

/** total interior seams on an n×n board (the score denominator). */
export const maxScore = (n: number) => 2 * n * (n - 1);

// --- fill paths + the frontier ---------------------------------------------
// The open frontier of a (path, hints) pair is computable from geometry alone —
// no solver. Its MEAN over the fill (the quantity the study's phase chart plots,
// emitted by the pipeline into hint-study.json) tracks the measured score closely
// (r ≈ −0.96); the peak below is the simpler summary and correlates a touch less
// (r ≈ −0.93). These path builders mirror the DFS engine's, and are used both by
// the fill animation and to derive the frontier a layout forces.

const nbrs4 = (n: number, cell: number): number[] => {
  const r = Math.floor(cell / n);
  const c = cell % n;
  const out: number[] = [];
  if (r > 0) out.push(cell - n);
  if (c > 0) out.push(cell - 1);
  if (c < n - 1) out.push(cell + 1);
  if (r < n - 1) out.push(cell + n);
  return out;
};

export function rowMajorSeq(n: number): number[] {
  return Array.from({ length: n * n }, (_, i) => i);
}
export function rowMajorBottomUpSeq(n: number): number[] {
  const v: number[] = [];
  for (let r = n - 1; r >= 0; r--) for (let c = 0; c < n; c++) v.push(idx(n, r, c));
  return v;
}
export function spiralInSeq(n: number): number[] {
  let [top, bottom, left, right] = [0, n - 1, 0, n - 1];
  const v: number[] = [];
  while (top <= bottom && left <= right) {
    for (let c = left; c <= right; c++) v.push(idx(n, top, c));
    for (let r = top + 1; r <= bottom; r++) v.push(idx(n, r, right));
    if (top < bottom) for (let c = right - 1; c >= left; c--) v.push(idx(n, bottom, c));
    if (left < right) for (let r = bottom - 1; r > top; r--) v.push(idx(n, r, left));
    top++; bottom--; left++; right--;
  }
  return v;
}
export function spiralOutSeq(n: number): number[] {
  return spiralInSeq(n).reverse();
}
export function borderFirstSeq(n: number): number[] {
  const b: number[] = [];
  const i: number[] = [];
  for (let r = 0; r < n; r++)
    for (let c = 0; c < n; c++)
      (r === 0 || r === n - 1 || c === 0 || c === n - 1 ? b : i).push(idx(n, r, c));
  return [...b, ...i];
}
/** multi-source BFS out of the hint cells — the hint-seeking order. */
export function connectHintsSeq(n: number, hints: number[]): number[] {
  const seen = new Array<boolean>(n * n).fill(false);
  const order: number[] = [];
  const queue: number[] = [];
  for (const h of hints) { seen[h] = true; order.push(h); queue.push(h); }
  let head = 0;
  while (head < queue.length) {
    const cell = queue[head++]!;
    for (const nb of nbrs4(n, cell))
      if (!seen[nb]) { seen[nb] = true; order.push(nb); queue.push(nb); }
  }
  return order;
}

/** The peak open frontier while filling `seq` (skipping `hints`, which start
 * filled). The frontier is the set of filled cells still adjacent to an empty
 * one; its peak is the branching pressure the search must survive. */
export function peakFrontier(n: number, seq: number[], hints: number[]): number {
  const hintSet = new Set(hints);
  const filled = new Set(hints);
  let peak = 0;
  for (const cell of seq) {
    if (hintSet.has(cell)) continue;
    filled.add(cell);
    let f = 0;
    for (const fc of filled)
      if (nbrs4(n, fc).some((nb) => !filled.has(nb))) f++;
    if (f > peak) peak = f;
  }
  return peak;
}
